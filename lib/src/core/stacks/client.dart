import 'dart:convert';
import 'package:http/http.dart' as http;
import 'package:logging/logging.dart';

/// A modern client for interacting with the Stacks blockchain API
class StacksClient {
  final String baseUrl;
  final http.Client _httpClient;
  final Logger _logger = Logger('StacksClient');

  StacksClient({
    this.baseUrl = 'https://stacks-node-api.mainnet.stacks.co',
    http.Client? httpClient,
  }) : _httpClient = httpClient ?? http.Client();

  /// Get account information for a specific Stacks address
  Future<Map<String, dynamic>> getAccountInfo(String address) async {
    final response = await _get('/v2/accounts/$address');
    return response;
  }

  /// Get account balances for a specific Stacks address
  Future<Map<String, dynamic>> getAccountBalances(String address) async {
    final response = await _get('/v2/accounts/$address/balances');
    return response;
  }

  /// Get transaction information by transaction ID
  Future<Map<String, dynamic>> getTransaction(String txId) async {
    final response = await _get('/v2/transactions/$txId');
    return response;
  }

  /// Get block information by block hash or height
  Future<Map<String, dynamic>> getBlock(String hashOrHeight) async {
    final response = await _get('/v2/blocks/$hashOrHeight');
    return response;
  }

  /// Get smart contract information
  Future<Map<String, dynamic>> getContract(String address, String contractName) async {
    final response = await _get('/v2/contracts/$address.$contractName');
    return response;
  }

  /// Get smart contract source code
  Future<Map<String, dynamic>> getContractSource(String address, String contractName) async {
    final response = await _get('/v2/contracts/source/$address.$contractName');
    return response;
  }

  /// Get mempool transactions
  Future<List<Map<String, dynamic>>> getMempoolTransactions({int? limit, int? offset}) async {
    final queryParams = <String, String>{};
    if (limit != null) queryParams['limit'] = limit.toString();
    if (offset != null) queryParams['offset'] = offset.toString();
    
    final response = await _get('/v2/mempool', queryParams: queryParams);
    return List<Map<String, dynamic>>.from(response['results']);
  }

  /// Broadcast a signed transaction
  Future<Map<String, dynamic>> broadcastTransaction(String signedTx) async {
    final response = await _post(
      '/v2/transactions',
      body: signedTx,
      headers: {'Content-Type': 'application/octet-stream'},
    );
    return response;
  }

  /// Get fee estimation
  Future<Map<String, dynamic>> getFeeEstimate() async {
    final response = await _get('/v2/fees/transfer');
    return response;
  }

  /// Internal method to make GET requests
  Future<dynamic> _get(String path, {Map<String, String>? queryParams}) async {
    final uri = Uri.parse('$baseUrl$path').replace(queryParameters: queryParams);
    _logger.fine('GET $uri');
    
    try {
      final response = await _httpClient.get(uri);
      return _handleResponse(response);
    } catch (e) {
      _logger.severe('Error making GET request to $uri', e);
      rethrow;
    }
  }

  /// Internal method to make POST requests
  Future<dynamic> _post(String path, {
    Object? body,
    Map<String, String>? headers,
  }) async {
    final uri = Uri.parse('$baseUrl$path');
    _logger.fine('POST $uri');
    
    try {
      final response = await _httpClient.post(
        uri,
        body: body,
        headers: headers,
      );
      return _handleResponse(response);
    } catch (e) {
      _logger.severe('Error making POST request to $uri', e);
      rethrow;
    }
  }

  /// Handle HTTP response and parse JSON
  dynamic _handleResponse(http.Response response) {
    if (response.statusCode >= 200 && response.statusCode < 300) {
      if (response.body.isEmpty) return null;
      return json.decode(response.body);
    }

    final error = json.decode(response.body);
    throw StacksApiException(
      statusCode: response.statusCode,
      message: error['error'] ?? 'Unknown error',
    );
  }

  /// Clean up resources
  void dispose() {
    _httpClient.close();
  }
}

/// Custom exception for Stacks API errors
class StacksApiException implements Exception {
  final int statusCode;
  final String message;

  StacksApiException({required this.statusCode, required this.message});

  @override
  String toString() => 'StacksApiException: [$statusCode] $message';
}
