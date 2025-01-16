import 'dart:async';

import 'package:web5/web5.dart';

import '../errors/service_errors.dart';
import '../models/transaction.dart';

/// Transaction history service
class TransactionHistoryService {
  final Web5 _web5;
  final String _nodeUrl;

  TransactionHistoryService(this._web5, this._nodeUrl);

  /// Get transaction history for a wallet
  Future<List<Transaction>> getTransactionHistory({
    required String walletId,
    required String chain,
    int? limit,
    int? offset,
    DateTime? startDate,
    DateTime? endDate,
    List<TransactionType>? types,
    List<TransactionStatus>? statuses,
  }) async {
    try {
      final query = {
        'message': {
          'filter': {
            'schema': 'anya/$chain/transaction',
            'walletId': walletId,
            if (startDate != null)
              'timestamp': {'\$gte': startDate.toIso8601String()},
            if (endDate != null)
              'timestamp': {'\$lte': endDate.toIso8601String()},
            if (types != null)
              'type': {'\$in': types.map((t) => t.toString().split('.').last)},
            if (statuses != null)
              'status': {
                '\$in': statuses.map((s) => s.toString().split('.').last)
              },
          },
          'limit': limit,
          'skip': offset,
          'sort': {'timestamp': -1},
        },
      };

      final records = await _web5.dwn.records.query(query);
      return _parseTransactions(records, chain);
    } catch (e) {
      throw HistoryServiceError('Failed to get transaction history: $e');
    }
  }

  /// Get transaction details
  Future<TransactionDetails> getTransactionDetails(
    String txId,
    String chain,
  ) async {
    try {
      switch (chain.toLowerCase()) {
        case 'bitcoin':
          return await _getBitcoinTransactionDetails(txId);
        case 'lightning':
          return await _getLightningTransactionDetails(txId);
        case 'rgb':
          return await _getRGBTransactionDetails(txId);
        default:
          throw HistoryServiceError('Unsupported chain: $chain');
      }
    } catch (e) {
      throw HistoryServiceError('Failed to get transaction details: $e');
    }
  }

  /// Get transaction status
  Future<TransactionStatus> getTransactionStatus(
    String txId,
    String chain,
  ) async {
    try {
      final details = await getTransactionDetails(txId, chain);
      return details.status;
    } catch (e) {
      throw HistoryServiceError('Failed to get transaction status: $e');
    }
  }

  /// Search transactions
  Future<List<Transaction>> searchTransactions({
    required String walletId,
    String? searchTerm,
    Map<String, dynamic>? filters,
  }) async {
    try {
      final query = {
        'message': {
          'filter': {
            'walletId': walletId,
            if (searchTerm != null)
              '\$or': [
                {
                  'txid': {'\$regex': searchTerm}
                },
                {
                  'address': {'\$regex': searchTerm}
                },
                {
                  'memo': {'\$regex': searchTerm}
                },
              ],
            ...?filters,
          },
          'sort': {'timestamp': -1},
        },
      };

      final records = await _web5.dwn.records.query(query);
      return _parseTransactions(records, 'all');
    } catch (e) {
      throw HistoryServiceError('Failed to search transactions: $e');
    }
  }

  /// Get transaction statistics
  Future<TransactionStats> getTransactionStats({
    required String walletId,
    required String chain,
    DateTime? startDate,
    DateTime? endDate,
  }) async {
    try {
      final transactions = await getTransactionHistory(
        walletId: walletId,
        chain: chain,
        startDate: startDate,
        endDate: endDate,
      );

      return _calculateStats(transactions);
    } catch (e) {
      throw HistoryServiceError('Failed to get transaction stats: $e');
    }
  }

  /// Export transaction history
  Future<String> exportTransactionHistory({
    required String walletId,
    required String chain,
    required String format, // csv, json
    DateTime? startDate,
    DateTime? endDate,
  }) async {
    try {
      final transactions = await getTransactionHistory(
        walletId: walletId,
        chain: chain,
        startDate: startDate,
        endDate: endDate,
      );

      switch (format.toLowerCase()) {
        case 'csv':
          return _exportToCSV(transactions);
        case 'json':
          return _exportToJSON(transactions);
        default:
          throw HistoryServiceError('Unsupported export format: $format');
      }
    } catch (e) {
      throw HistoryServiceError('Failed to export transaction history: $e');
    }
  }

  /// Parse transactions from records
  List<Transaction> _parseTransactions(List<dynamic> records, String chain) {
    return records.map((record) {
      final data = record.data;
      return Transaction(
        id: data['txid'],
        type: _parseTransactionType(data['type']),
        fromAddress: data['fromAddress'],
        toAddress: data['toAddress'],
        amount: data['amount'].toDouble(),
        chain: chain,
        symbol: data['symbol'],
        timestamp: DateTime.parse(data['timestamp']),
        status: _parseTransactionStatus(data['status']),
        feeAmount: data['feeAmount']?.toDouble(),
        feeSymbol: data['feeSymbol'],
        metadata: data['metadata'],
      );
    }).toList();
  }

  /// Get Bitcoin transaction details
  Future<TransactionDetails> _getBitcoinTransactionDetails(String txId) async {
    try {
      final response = await _makeRequest('gettransaction', [txId]);
      return TransactionDetails(
        txId: txId,
        status: _getBitcoinTransactionStatus(response),
        confirmations: response['confirmations'],
        blockHeight: response['blockheight'],
        blockTime:
            DateTime.fromMillisecondsSinceEpoch(response['blocktime'] * 1000),
        fee: response['fee'].abs() * 100000000, // Convert to sats
        size: response['size'],
        vsize: response['vsize'],
        replaceable: response['bip125-replaceable'] == 'yes',
        inputs: _parseBitcoinInputs(response['vin']),
        outputs: _parseBitcoinOutputs(response['vout']),
      );
    } catch (e) {
      throw HistoryServiceError(
          'Failed to get Bitcoin transaction details: $e');
    }
  }

  /// Get Lightning transaction details
  Future<TransactionDetails> _getLightningTransactionDetails(
      String txId) async {
    // Implementation would get Lightning payment details
    throw UnimplementedError();
  }

  /// Get RGB transaction details
  Future<TransactionDetails> _getRGBTransactionDetails(String txId) async {
    // Implementation would get RGB transfer details
    throw UnimplementedError();
  }

  /// Calculate transaction statistics
  TransactionStats _calculateStats(List<Transaction> transactions) {
    double totalSent = 0;
    double totalReceived = 0;
    double totalFees = 0;
    int successfulCount = 0;
    int failedCount = 0;

    for (final tx in transactions) {
      if (tx.type == TransactionType.send) {
        totalSent += tx.amount;
        totalFees += tx.feeAmount ?? 0;
      } else if (tx.type == TransactionType.receive) {
        totalReceived += tx.amount;
      }

      if (tx.status == TransactionStatus.completed) {
        successfulCount++;
      } else if (tx.status == TransactionStatus.failed) {
        failedCount++;
      }
    }

    return TransactionStats(
      totalSent: totalSent,
      totalReceived: totalReceived,
      totalFees: totalFees,
      successfulCount: successfulCount,
      failedCount: failedCount,
      totalCount: transactions.length,
    );
  }

  /// Export transactions to CSV format
  String _exportToCSV(List<Transaction> transactions) {
    final buffer = StringBuffer();

    // Add header
    buffer.writeln('Date,Type,Amount,Fee,Status,ID,From,To');

    // Add transactions
    for (final tx in transactions) {
      buffer.writeln(
        '${tx.timestamp.toIso8601String()},'
        '${tx.type.toString().split('.').last},'
        '${tx.amount},'
        '${tx.feeAmount ?? ''},'
        '${tx.status.toString().split('.').last},'
        '${tx.id},'
        '${tx.fromAddress},'
        '${tx.toAddress}',
      );
    }

    return buffer.toString();
  }

  /// Export transactions to JSON format
  String _exportToJSON(List<Transaction> transactions) {
    return jsonEncode(transactions.map((tx) => tx.toJson()).toList());
  }

  TransactionType _parseTransactionType(String type) {
    return TransactionType.values.firstWhere(
      (t) => t.toString().split('.').last == type,
      orElse: () => TransactionType.send,
    );
  }

  TransactionStatus _parseTransactionStatus(String status) {
    return TransactionStatus.values.firstWhere(
      (s) => s.toString().split('.').last == status,
      orElse: () => TransactionStatus.pending,
    );
  }

  Future<dynamic> _makeRequest(String method, [List<dynamic>? params]) async {
    try {
      // Implementation would use appropriate RPC client
      throw UnimplementedError('RPC communication not implemented');
    } catch (e) {
      throw HistoryServiceError('Request failed: $e');
    }
  }
}

/// Transaction details
class TransactionDetails {
  final String txId;
  final TransactionStatus status;
  final int confirmations;
  final int? blockHeight;
  final DateTime? blockTime;
  final double fee;
  final int size;
  final int vsize;
  final bool replaceable;
  final List<Map<String, dynamic>> inputs;
  final List<Map<String, dynamic>> outputs;

  TransactionDetails({
    required this.txId,
    required this.status,
    required this.confirmations,
    this.blockHeight,
    this.blockTime,
    required this.fee,
    required this.size,
    required this.vsize,
    required this.replaceable,
    required this.inputs,
    required this.outputs,
  });
}

/// Transaction statistics
class TransactionStats {
  final double totalSent;
  final double totalReceived;
  final double totalFees;
  final int successfulCount;
  final int failedCount;
  final int totalCount;

  TransactionStats({
    required this.totalSent,
    required this.totalReceived,
    required this.totalFees,
    required this.successfulCount,
    required this.failedCount,
    required this.totalCount,
  });

  double get successRate => totalCount > 0 ? successfulCount / totalCount : 0;
}

class HistoryServiceError implements Exception {
  final String message;
  HistoryServiceError(this.message);
  @override
  String toString() => message;
}
