import 'package:logging/logging.dart';
import 'client.dart';
import 'models.dart';

/// High-level service for interacting with the Stacks blockchain
class StacksService {
  final StacksClient _client;
  final Logger _logger = Logger('StacksService');

  StacksService({StacksClient? client})
      : _client = client ?? StacksClient();

  /// Get account information and parse it into a strongly-typed model
  Future<StacksAccount> getAccount(String address) async {
    try {
      final response = await _client.getAccountInfo(address);
      return StacksAccount.fromJson(response);
    } catch (e) {
      _logger.severe('Error getting account info for $address', e);
      rethrow;
    }
  }

  /// Get transaction details and parse them into a strongly-typed model
  Future<StacksTransaction> getTransaction(String txId) async {
    try {
      final response = await _client.getTransaction(txId);
      return StacksTransaction.fromJson(response);
    } catch (e) {
      _logger.severe('Error getting transaction $txId', e);
      rethrow;
    }
  }

  /// Get contract details and parse them into a strongly-typed model
  Future<StacksContract> getContract(String address, String contractName) async {
    try {
      final response = await _client.getContract(address, contractName);
      return StacksContract.fromJson(response);
    } catch (e) {
      _logger.severe('Error getting contract $address.$contractName', e);
      rethrow;
    }
  }

  /// Get fee estimation and parse it into a strongly-typed model
  Future<StacksFeeEstimate> getFeeEstimate() async {
    try {
      final response = await _client.getFeeEstimate();
      return StacksFeeEstimate.fromJson(response);
    } catch (e) {
      _logger.severe('Error getting fee estimate', e);
      rethrow;
    }
  }

  /// Broadcast a signed transaction and return the transaction ID
  Future<String> broadcastTransaction(String signedTx) async {
    try {
      final response = await _client.broadcastTransaction(signedTx);
      return response['txid'] as String;
    } catch (e) {
      _logger.severe('Error broadcasting transaction', e);
      rethrow;
    }
  }

  /// Get recent mempool transactions
  Future<List<StacksTransaction>> getMempoolTransactions({int limit = 20}) async {
    try {
      final response = await _client.getMempoolTransactions(limit: limit);
      return response
          .map((tx) => StacksTransaction.fromJson(tx))
          .toList();
    } catch (e) {
      _logger.severe('Error getting mempool transactions', e);
      rethrow;
    }
  }

  /// Clean up resources
  void dispose() {
    _client.dispose();
  }
}
