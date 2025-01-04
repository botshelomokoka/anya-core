import 'package:web5_dart/web5_dart.dart' as web5;
import '../web5/web5_service.dart';

/// Web5 configuration for cross-platform compatibility
class Web5Config {
  /// Default DWN protocol configuration
  static final dwnProtocol = {
    'protocol': 'anya',
    'published': true,
    'types': {
      'wallet': {
        'schema': 'anya/wallet',
        'dataFormats': ['application/json'],
      },
      'transaction': {
        'schema': 'anya/bitcoin/transaction',
        'dataFormats': ['application/json'],
      },
      'metadata': {
        'schema': 'anya/metadata',
        'dataFormats': ['application/json'],
      },
    },
    'structure': {
      'wallet': {
        r'$schema': 'http://json-schema.org/draft-07/schema#',
        'type': 'object',
        'required': ['id', 'name', 'type', 'ownerDid', 'address'],
        'properties': {
          'id': {'type': 'string'},
          'name': {'type': 'string'},
          'type': {'type': 'string'},
          'ownerDid': {'type': 'string'},
          'address': {'type': 'string'},
          'metadata': {'type': 'object'},
          'encryptedData': {'type': 'string'},
          'permissions': {
            'type': 'array',
            'items': {'type': 'string'},
          },
        },
      },
      'transaction': {
        r'$schema': 'http://json-schema.org/draft-07/schema#',
        'type': 'object',
        'required': ['txid', 'walletId', 'timestamp', 'hex'],
        'properties': {
          'txid': {'type': 'string'},
          'walletId': {'type': 'string'},
          'timestamp': {'type': 'string'},
          'hex': {'type': 'string'},
          'status': {'type': 'string'},
        },
      },
    },
  };

  /// Initialize Web5 with default configuration
  static Future<Web5Service> initialize() async {
    final service = await Web5Service.connect();

    // Configure DWN protocol
    await service.createRecord(
      collection: 'protocols',
      data: dwnProtocol,
      schema: 'https://anya.io/schemas/protocol/v1',
    );

    return service;
  }

  /// Default Web5 options
  static final defaultOptions = {
    'enableEncryption': true,
    'enableCompression': true,
    'maxRecordSize': 1024 * 1024 * 10, // 10MB
  };

  /// Schema versions for data types
  static const schemaVersions = {
    'wallet': 'v1.0.0',
    'transaction': 'v1.0.0',
    'metadata': 'v1.0.0',
  };

  /// Collection names
  static const collections = {
    'wallets': 'wallets',
    'transactions': 'transactions',
    'metadata': 'metadata',
  };

  /// Get schema URI for a data type
  static String getSchemaUri(String type) {
    final version = schemaVersions[type];
    return 'https://anya.io/schemas/$type/$version';
  }

  /// Get collection name for a data type
  static String getCollection(String type) {
    return collections[type] ?? type;
  }
}
