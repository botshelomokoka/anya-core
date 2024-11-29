import 'package:web5/web5.dart';
import 'package:logging/logging.dart';

/// DWN-based storage implementation
class DWNStore {
  final Logger _logger = Logger('DWNStore');
  final Web5Client _client;
  final String _did;

  DWNStore(this._client, this._did);

  Future<String> store(String collection, Map<String, dynamic> data) async {
    try {
      final record = await _client.dwn.records.create(
        data: data,
        message: {
          'schema': 'https://anya.io/schemas/$collection',
          'dataFormat': 'application/json',
        },
        did: _did,
      );
      return record.id;
    } catch (e) {
      _logger.severe('Failed to store data: $e');
      rethrow;
    }
  }

  Future<Map<String, dynamic>?> get(String collection, String id) async {
    try {
      final record = await _client.dwn.records.read(
        message: {
          'filter': {
            'recordId': id,
            'schema': 'https://anya.io/schemas/$collection',
          },
        },
        did: _did,
      );
      return record?.data as Map<String, dynamic>?;
    } catch (e) {
      _logger.severe('Failed to get data: $e');
      rethrow;
    }
  }

  Future<List<Map<String, dynamic>>> query(
    String collection, {
    Map<String, dynamic>? filter,
  }) async {
    try {
      final records = await _client.dwn.records.query(
        message: {
          'filter': {
            'schema': 'https://anya.io/schemas/$collection',
            ...?filter,
          },
        },
        did: _did,
      );
      return records.map((r) => r.data as Map<String, dynamic>).toList();
    } catch (e) {
      _logger.severe('Failed to query data: $e');
      rethrow;
    }
  }

  Future<void> update(
    String collection,
    String id,
    Map<String, dynamic> data,
  ) async {
    try {
      await _client.dwn.records.update(
        message: {
          'recordId': id,
          'schema': 'https://anya.io/schemas/$collection',
          'data': data,
        },
        did: _did,
      );
    } catch (e) {
      _logger.severe('Failed to update data: $e');
      rethrow;
    }
  }

  Future<void> delete(String collection, String id) async {
    try {
      await _client.dwn.records.delete(
        message: {
          'recordId': id,
          'schema': 'https://anya.io/schemas/$collection',
        },
        did: _did,
      );
    } catch (e) {
      _logger.severe('Failed to delete data: $e');
      rethrow;
    }
  }
}
