import 'package:web5/web5.dart';
import '../lib/src/core/storage/dwn_store.dart';

/// Utility to migrate data from SQL to Web5 DWN
class Web5Migrator {
  final DWNStore _dwnStore;
  final String _sqlConnectionString;

  Web5Migrator(this._dwnStore, this._sqlConnectionString);

  Future<void> migrateTable(String tableName, String collection) async {
    print('Migrating $tableName to Web5 collection: $collection');

    try {
      // 1. Read from SQL
      final sqlData = await _readFromSQL(tableName);

      // 2. Transform data if needed
      final transformedData = _transformData(sqlData);

      // 3. Store in Web5 DWN
      for (final record in transformedData) {
        await _dwnStore.store(collection, record);
      }

      print('Successfully migrated ${sqlData.length} records');
    } catch (e) {
      print('Error migrating $tableName: $e');
      rethrow;
    }
  }

  Future<List<Map<String, dynamic>>> _readFromSQL(String tableName) async {
    // Implementation will depend on your SQL driver
    // This is just a placeholder
    return [];
  }

  List<Map<String, dynamic>> _transformData(List<Map<String, dynamic>> data) {
    // Transform SQL data format to Web5 format
    // This is where you'd add any necessary data transformations
    return data.map((record) {
      return {
        ...record,
        'updatedAt': DateTime.now().toIso8601String(),
        'schema': 'https://anya.io/schemas/${record['type']}',
      };
    }).toList();
  }
}

void main(List<String> args) async {
  if (args.length != 2) {
    print('Usage: dart migrate_to_web5.dart <sql_connection_string> <did>');
    return;
  }

  final sqlConnectionString = args[0];
  final did = args[1];

  // Initialize Web5 client
  final web5 = Web5Client();
  final dwnStore = DWNStore(web5, did);

  // Create migrator
  final migrator = Web5Migrator(dwnStore, sqlConnectionString);

  // Define migration mappings
  final migrations = {
    'wallets': 'wallets',
    'transactions': 'transactions',
    'users': 'users',
    // Add more table-to-collection mappings as needed
  };

  // Run migrations
  for (final entry in migrations.entries) {
    await migrator.migrateTable(entry.key, entry.value);
  }

  print('Migration completed successfully');
}
