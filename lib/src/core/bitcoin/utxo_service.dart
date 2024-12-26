import 'dart:async';
import 'package:web5/web5.dart';
import 'package:bitcoindart/bitcoindart.dart';
import '../errors/service_errors.dart';

class UTXO {
  final String txid;
  final int vout;
  final String address;
  final String scriptPubKey;
  final int value;
  final int confirmations;
  final bool isSpendable;
  final bool isSolvable;
  final String? redeemScript;
  final String? witnessScript;
  final bool isChange;
  final Map<String, dynamic>? metadata;

  UTXO({
    required this.txid,
    required this.vout,
    required this.address,
    required this.scriptPubKey,
    required this.value,
    required this.confirmations,
    required this.isSpendable,
    required this.isSolvable,
    this.redeemScript,
    this.witnessScript,
    this.isChange = false,
    this.metadata,
  });

  factory UTXO.fromJson(Map<String, dynamic> json) {
    return UTXO(
      txid: json['txid'],
      vout: json['vout'],
      address: json['address'],
      scriptPubKey: json['scriptPubKey'],
      value: json['value'],
      confirmations: json['confirmations'],
      isSpendable: json['spendable'],
      isSolvable: json['solvable'],
      redeemScript: json['redeemScript'],
      witnessScript: json['witnessScript'],
      isChange: json['isChange'] ?? false,
      metadata: json['metadata'],
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'txid': txid,
      'vout': vout,
      'address': address,
      'scriptPubKey': scriptPubKey,
      'value': value,
      'confirmations': confirmations,
      'spendable': isSpendable,
      'solvable': isSolvable,
      'redeemScript': redeemScript,
      'witnessScript': witnessScript,
      'isChange': isChange,
      'metadata': metadata,
    };
  }

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

  UTXOService(this._web5, this._nodeUrl, this._network);

  /// Get all UTXOs for an address
  Future<List<UTXO>> getUTXOs(String address) async {
    try {
      final response = await _makeRequest('listunspent', [
        0, // minconf
        9999999, // maxconf
        [address],
      ]);

      return List<Map<String, dynamic>>.from(response)
          .map((utxo) => UTXO.fromJson(utxo))
          .toList();
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
        final currentFee = _calculateFee(selectedUTXOs.length, 2,
            feeRate); // Assume 2 outputs (recipient + change)

        // Check if we have enough including fees
        if (selectedAmount >= targetAmount + currentFee) {
          // Check if change output would be dust
          final change = selectedAmount - targetAmount - currentFee;
          if (change >= baseFee) {
            break;
          }
        }
      }

      if (selectedAmount <
          targetAmount + _calculateFee(selectedUTXOs.length, 2, feeRate)) {
        throw InsufficientFundsError(
            'Not enough funds to cover amount and fees');
      }

      return selectedUTXOs;
    } catch (e) {
      throw UTXOServiceError('Failed to select UTXOs: $e');
    }
  }

  /// Lock UTXOs to prevent double-spending
  Future<void> lockUTXOs(List<UTXO> utxos) async {
    try {
      final utxoRefs = utxos
          .map((utxo) => {
                'txid': utxo.txid,
                'vout': utxo.vout,
              })
          .toList();

      await _makeRequest('lockunspent', [false, utxoRefs]);
    } catch (e) {
      throw UTXOServiceError('Failed to lock UTXOs: $e');
    }
  }

  /// Unlock previously locked UTXOs
  Future<void> unlockUTXOs(List<UTXO> utxos) async {
    try {
      final utxoRefs = utxos
          .map((utxo) => {
                'txid': utxo.txid,
                'vout': utxo.vout,
              })
          .toList();

      await _makeRequest('lockunspent', [true, utxoRefs]);
    } catch (e) {
      throw UTXOServiceError('Failed to unlock UTXOs: $e');
    }
  }

  /// Get all locked UTXOs
  Future<List<UTXO>> getLockedUTXOs() async {
    try {
      final response = await _makeRequest('listlockunspent');
      return List<Map<String, dynamic>>.from(response)
          .map((utxo) => UTXO.fromJson(utxo))
          .toList();
    } catch (e) {
      throw UTXOServiceError('Failed to get locked UTXOs: $e');
    }
  }

  /// Sort UTXOs based on selection strategy
  List<UTXO> _sortUTXOs(List<UTXO> utxos, UTXOSelectionStrategy strategy) {
    switch (strategy) {
      case UTXOSelectionStrategy.minimizeInputs:
        // Sort by value descending to use fewer inputs
        return List.from(utxos)..sort((a, b) => b.value.compareTo(a.value));

      case UTXOSelectionStrategy.maximizePrivacy:
        // Shuffle UTXOs to use random inputs
        return List.from(utxos)..shuffle();

      case UTXOSelectionStrategy.oldestFirst:
        // Sort by confirmations descending
        return List.from(utxos)
          ..sort((a, b) => b.confirmations.compareTo(a.confirmations));

      case UTXOSelectionStrategy.closestAmount:
        // Implementation depends on target amount, handled in selectUTXOs
        return utxos;
    }
  }

  /// Calculate transaction fee based on size
  int _calculateFee(int numInputs, int numOutputs, int feeRate) {
    // Approximate size calculation:
    // Input: ~148 bytes for legacy, ~68 bytes for SegWit
    // Output: ~34 bytes
    // Other: ~10 bytes
    const int inputSize = 68; // Assuming SegWit
    const int outputSize = 34;
    const int otherSize = 10;

    final int totalSize =
        (inputSize * numInputs) + (outputSize * numOutputs) + otherSize;
    return totalSize * feeRate;
  }

  /// Estimate base fee for dust calculation
  int _estimateBaseFee(int feeRate) {
    // Minimum size for a typical output
    return 34 * feeRate;
  }

  Future<dynamic> _makeRequest(String method, [List<dynamic>? params]) async {
    try {
      // Implementation would use Bitcoin Core RPC
      throw UnimplementedError(
          'Bitcoin Core RPC communication not implemented');
    } catch (e) {
      throw UTXOServiceError('Request failed: $e');
    }
  }
}

class UTXOServiceError implements Exception {
  final String message;
  UTXOServiceError(this.message);
  @override
  String toString() => message;
}

class InsufficientFundsError implements Exception {
  final String message;
  InsufficientFundsError(this.message);
  @override
  String toString() => message;
}
