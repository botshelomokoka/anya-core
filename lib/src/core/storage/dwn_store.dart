import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'package:web5/web5.dart';
import '../config/web5_config.dart';
import '../errors/storage_errors.dart';

/// Enhanced cross-platform DWN store implementation
class DWNStore {
  final Web5 _web5;
  final DID _did;
  final Map<String, List<Record>> _cache = {};
  final Duration _cacheDuration = const Duration(minutes: 5);
  final int _maxCacheSize = 1000;
  
  DWNStore(this._web5, this._did);

  /// Platform-optimized store operation
  Future<String> store(String collection, Map<String, dynamic> data) async {
    try {
      // Platform-specific compression
      final compressedData = await _compressData(jsonEncode(data));
      
      final record = await _web5.dwn.records.create(
        data: compressedData,
        message: {
          'schema': Web5Config.getSchemaUri(collection),
          'dataFormat': 'application/octet-stream',
        },
      );

      // Update cache
      _updateCache(collection, record);
      
      return record.id;
    } catch (e) {
      throw StorageError('Failed to store data: $e');
    }
  }

  /// Platform-optimized data retrieval
  Future<Map<String, dynamic>?> get(String collection, String recordId) async {
    try {
      // Check cache first
      final cachedRecord = _getFromCache(collection, recordId);
      if (cachedRecord != null) {
        return _decompressAndDecode(cachedRecord.data);
      }

      final record = await _web5.dwn.records.read(recordId);
      if (record == null) return null;

      // Update cache
      _updateCache(collection, record);
      
      return _decompressAndDecode(record.data);
    } catch (e) {
      throw StorageError('Failed to retrieve data: $e');
    }
  }

  /// Platform-optimized query operation
  Future<List<Map<String, dynamic>>> query(
    String collection, {
    Map<String, dynamic>? filter,
  }) async {
    try {
      // Check if we have a complete cache for this collection
      if (_hasValidCache(collection) && filter == null) {
        return _cache[collection]!
            .map((record) => _decompressAndDecode(record.data))
            .toList();
      }

      final records = await _web5.dwn.records.query(
        message: {
          'schema': Web5Config.getSchemaUri(collection),
          'filter': filter,
        },
      );

      // Update cache
      _updateCollectionCache(collection, records);

      return Future.wait(
        records.map((record) => _decompressAndDecode(record.data)),
      );
    } catch (e) {
      throw StorageError('Failed to query data: $e');
    }
  }

  /// Platform-optimized update operation
  Future<void> update(
    String collection,
    String recordId,
    Map<String, dynamic> newData,
  ) async {
    try {
      final compressedData = await _compressData(jsonEncode(newData));
      
      await _web5.dwn.records.update(
        recordId,
        data: compressedData,
        message: {
          'schema': Web5Config.getSchemaUri(collection),
          'dataFormat': 'application/octet-stream',
        },
      );

      // Invalidate cache for this record
      _invalidateCacheRecord(collection, recordId);
    } catch (e) {
      throw StorageError('Failed to update data: $e');
    }
  }

  /// Platform-optimized delete operation
  Future<void> delete(String collection, String recordId) async {
    try {
      await _web5.dwn.records.delete(recordId);
      // Remove from cache
      _invalidateCacheRecord(collection, recordId);
    } catch (e) {
      throw StorageError('Failed to delete data: $e');
    }
  }

  /// Platform-specific data compression
  Future<List<int>> _compressData(String data) async {
    if (!Web5Config.defaultOptions['enableCompression']) {
      return utf8.encode(data);
    }

    try {
      final bytes = utf8.encode(data);
      return gzip.encode(bytes);
    } catch (e) {
      // Fallback to uncompressed if compression fails
      return utf8.encode(data);
    }
  }

  /// Platform-specific data decompression
  Future<Map<String, dynamic>> _decompressAndDecode(dynamic data) async {
    if (data is String) {
      return jsonDecode(data);
    }

    try {
      final decompressed = gzip.decode(data);
      final jsonStr = utf8.decode(decompressed);
      return jsonDecode(jsonStr);
    } catch (e) {
      // Fallback to direct decoding if decompression fails
      final jsonStr = utf8.decode(data);
      return jsonDecode(jsonStr);
    }
  }

  /// Cache management
  void _updateCache(String collection, Record record) {
    _cache.putIfAbsent(collection, () => []);
    _cache[collection]!.removeWhere((r) => r.id == record.id);
    _cache[collection]!.add(record);
    _enforceCacheLimit();
  }

  void _updateCollectionCache(String collection, List<Record> records) {
    _cache[collection] = records;
    _enforceCacheLimit();
  }

  Record? _getFromCache(String collection, String recordId) {
    return _cache[collection]?.firstWhere(
      (record) => record.id == recordId,
      orElse: () => null,
    );
  }

  bool _hasValidCache(String collection) {
    return _cache.containsKey(collection) && 
           _cache[collection]!.isNotEmpty;
  }

  void _invalidateCacheRecord(String collection, String recordId) {
    _cache[collection]?.removeWhere((record) => record.id == recordId);
  }

  void _enforceCacheLimit() {
    while (_getTotalCacheSize() > _maxCacheSize) {
      final oldestCollection = _cache.keys.first;
      _cache.remove(oldestCollection);
    }
  }

  int _getTotalCacheSize() {
    return _cache.values
        .expand((records) => records)
        .length;
  }

  /// Get DID for the current store
  String get did => _did.uri;

  /// Verify permissions for a record
  Future<bool> verifyPermissions(String recordId, String requesterDid) async {
    try {
      final record = await _web5.dwn.records.read(recordId);
      if (record == null) return false;
      
      return record.owner == requesterDid || 
             await _hasPermission(record, requesterDid);
    } catch (e) {
      return false;
    }
  }

  Future<bool> _hasPermission(Record record, String requesterDid) async {
    try {
      final permissions = await _web5.dwn.permissions.check(
        recordId: record.id,
        did: requesterDid,
      );
      return permissions.granted;
    } catch (e) {
      return false;
    }
  }
}
