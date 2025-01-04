import 'dart:async';
import 'package:freezed_annotation/freezed_annotation.dart';
import 'package:json_annotation/json_annotation.dart';
import 'package:web5/web5.dart';
import 'package:bitcoin_base/bitcoin_base.dart';
import '../errors/service_errors.dart';

part 'utxo_service.g.dart';
part 'utxo_service.freezed.dart';

/// Unspent Transaction Output (UTXO) model
@freezed
class UTXO with _$UTXO {
  const factory UTXO({
    required String txid,
    required int vout,
    required String address,
    required String scriptPubKey,
    required int value,
    required int confirmations,
    required bool isSpendable,
    required bool isSolvable,
    String? redeemScript,
    String? witnessScript,
    @Default(false) bool isChange,
    Map<String, dynamic>? metadata,
  }) = _UTXO;

  factory UTXO.fromJson(Map<String, dynamic> json) => _$UTXOFromJson(json);

  /// Get outpoint string (txid:vout)
  String get outpoint => '$txid:$vout';
}

/// UTXO selection strategies
enum UTXOSelectionStrategy {
  /// Select UTXOs to minimize the number of inputs
  minimizeInputs,

  /// Select UTXOs to maximize privacy by using different addresses
  maximizePrivacy,

  /// Select oldest UTXOs first to minimize dust
  oldestFirst,

  /// Select UTXOs closest to target amount to minimize change
  closestAmount,
}

/// UTXO management service
class UTXOService {
  final Web5 _web5;
  final String _nodeUrl;
  final NetworkType _network;
  final int _dustThreshold;

  UTXOService(
    this._web5,
    this._nodeUrl,
    this._network, {
    int? dustThreshold,
  }) : _dustThreshold = dustThreshold ?? 546; // Default Bitcoin dust threshold

  /// Get all UTXOs for an address
  Future<List<UTXO>> getUTXOs(String address) async {
    try {
      _validateAddress(address);

      final response = await _makeRequest('listunspent', [
        0, // minconf
        9999999, // maxconf
        [address],
      ]);

      if (response is! List) {
        throw UTXOServiceError('Invalid response format from node');
      }

      return List<Map<String, dynamic>>.from(response)
          .map((json) => UTXO.fromJson(json))
          .toList();
    } on Web5Exception catch (e) {
      throw UTXOServiceError('Web5 error: ${e.message}');
    } on FormatException catch (e) {
      throw UTXOServiceError('Invalid data format: ${e.message}');
    } catch (e) {
      throw UTXOServiceError('Failed to get UTXOs: $e');
    }
  }

  /// Select UTXOs for a transaction
  Future<List<UTXO>> selectUTXOs({
    required int targetAmount,
    required int feeRate,
    required List<UTXO> availableUTXOs,
    UTXOSelectionStrategy strategy = UTXOSelectionStrategy.minimizeInputs,
  }) async {
    try {
      if (targetAmount <= 0) {
        throw UTXOServiceError('Target amount must be positive');
      }
      if (feeRate <= 0) {
        throw UTXOServiceError('Fee rate must be positive');
      }
      if (availableUTXOs.isEmpty) {
        throw InsufficientFundsError('No UTXOs available');
      }

      // Sort UTXOs based on strategy
      final sortedUTXOs = _sortUTXOs(availableUTXOs, strategy);

      int selectedAmount = 0;
      final selectedUTXOs = <UTXO>[];

      // Calculate approximate fee for a basic transaction
      final baseFee = _estimateBaseFee(feeRate);

      for (final utxo in sortedUTXOs) {
        if (!utxo.isSpendable || !utxo.isSolvable) continue;

        selectedUTXOs.add(utxo);
        selectedAmount += utxo.value;

        // Recalculate fee with current inputs
        final currentFee = _calculateFee(
          selectedUTXOs.length,
          2, // Assume 2 outputs (recipient + change)
          feeRate,
        );

        // Check if we have enough including fees
        if (selectedAmount >= targetAmount + currentFee) {
          // Check if change output would be dust
          final change = selectedAmount - targetAmount - currentFee;
          if (change >= _dustThreshold) {
            break;
          }
        }
      }

      final totalFee = _calculateFee(selectedUTXOs.length, 2, feeRate);
      if (selectedAmount < targetAmount + totalFee) {
        throw InsufficientFundsError(
          'Insufficient funds: need ${targetAmount + totalFee}, '
          'have $selectedAmount',
        );
      }

      return selectedUTXOs;
    } catch (e) {
      if (e is InsufficientFundsError) rethrow;
      throw UTXOServiceError('UTXO selection failed: $e');
    }
  }

  /// Get UTXO details by outpoint
  Future<UTXO?> getUTXOByOutpoint(String txid, int vout) async {
    try {
      _validateTxid(txid);
      if (vout < 0) {
        throw UTXOServiceError('Invalid vout');
      }

      final response = await _makeRequest('gettxout', [txid, vout]);
      if (response == null) return null;

      return UTXO.fromJson(response as Map<String, dynamic>);
    } catch (e) {
      throw UTXOServiceError('Failed to get UTXO details: $e');
    }
  }

  // Private helper methods
  List<UTXO> _sortUTXOs(List<UTXO> utxos, UTXOSelectionStrategy strategy) {
    switch (strategy) {
      case UTXOSelectionStrategy.minimizeInputs:
        return List.from(utxos)
          ..sort((a, b) => b.value.compareTo(a.value));
      case UTXOSelectionStrategy.maximizePrivacy:
        return List.from(utxos)
          ..sort((a, b) => a.address.compareTo(b.address));
      case UTXOSelectionStrategy.oldestFirst:
        return List.from(utxos)
          ..sort((a, b) => b.confirmations.compareTo(a.confirmations));
      case UTXOSelectionStrategy.closestAmount:
        return utxos; // Implement closest amount logic
    }
  }

  int _calculateFee(int numInputs, int numOutputs, int feeRate) {
    // P2WPKH input: 68 vbytes, P2WPKH output: 31 vbytes, Other: 11 vbytes
    final int vsize = (numInputs * 68) + (numOutputs * 31) + 11;
    return vsize * feeRate;
  }

  int _estimateBaseFee(int feeRate) {
    // Estimate fee for a basic P2WPKH output
    return 31 * feeRate;
  }

  void _validateAddress(String address) {
    if (address.isEmpty) {
      throw UTXOServiceError('Empty address');
    }
    try {
      Address.fromAddress(address, _network);
    } catch (e) {
      throw UTXOServiceError('Invalid address format: $e');
    }
  }

  void _validateTxid(String txid) {
    if (!RegExp(r'^[a-fA-F0-9]{64}$').hasMatch(txid)) {
      throw UTXOServiceError('Invalid transaction ID format');
    }
  }

  Future<dynamic> _makeRequest(String method, List<dynamic> params) async {
    final response = await _web5.rpc.call(_nodeUrl, method, params);
    return response;
  }
}

/// Error thrown by UTXOService
class UTXOServiceError implements Exception {
  final String message;
  
  const UTXOServiceError(this.message);
  
  @override
  String toString() => 'UTXOServiceError: $message';
}

/// Error thrown when insufficient funds are available
class InsufficientFundsError implements Exception {
  final String message;
  
  const InsufficientFundsError(this.message);
  
  @override
  String toString() => 'InsufficientFundsError: $message';
}
