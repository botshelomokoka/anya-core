import 'package:web5/web5.dart';

/// Web5 configuration for cross-platform compatibility
class Web5Config {
  /// Default DWN protocol configuration
  static const dwnProtocol = {
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
        '$schema': 'http://json-schema.org/draft-07/schema#',
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
        '$schema': 'http://json-schema.org/draft-07/schema#',
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
  static Future<Web5> initialize() async {
    final web5 = await Web5.connect();

    // Configure DWN protocol
    await web5.dwn.protocols.configure(dwnProtocol);

    return web5;
  }

  /// Default Web5 options
  static const defaultOptions = {
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
