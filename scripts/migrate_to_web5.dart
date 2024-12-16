import 'dart:async';
import 'dart:convert';
import 'dart:io';
import 'package:path/path.dart' as path;
import 'package:logging/logging.dart';
import 'package:web5/web5.dart';
import '../lib/src/core/storage/dwn_store.dart';
import '../lib/src/core/config/web5_config.dart';
import '../lib/src/core/models/wallet.dart';

final _logger = Logger('MigrationScript');

/// Cross-platform migration utility for Web5 DWN
class Web5Migrator {
  final DWNStore _store;
  final String _dataDir;
  final bool _enableCompression;
  final int _batchSize;
  final Duration _batchDelay;

  Web5Migrator(
    this._store,
    this._dataDir, {
    bool enableCompression = true,
    int batchSize = 50,
    Duration? batchDelay,
  })  : _enableCompression = enableCompression,
        _batchSize = batchSize,
        _batchDelay = batchDelay ?? const Duration(milliseconds: 100);

  /// Start the migration process
  Future<void> migrate() async {
    _logger.info('Starting Web5 migration from $_dataDir');
    
    try {
      await _setupLogging();
      await _validateEnvironment();
      
      final collections = await _getCollections();
      _logger.info('Found ${collections.length} collections to migrate');

      for (final collection in collections) {
        await _migrateCollection(collection);
      }

      _logger.info('Migration completed successfully');
    } catch (e, stack) {
      _logger.severe('Migration failed', e, stack);
      rethrow;
    }
  }

  /// Set up logging with platform-specific paths
  Future<void> _setupLogging() async {
    final logsDir = path.join(_dataDir, 'logs');
    await Directory(logsDir).create(recursive: true);

    final logFile = File(path.join(logsDir, 'migration_${DateTime.now().toIso8601String()}.log'));
    
    Logger.root.level = Level.ALL;
    Logger.root.onRecord.listen((record) {
      final message = '${record.time}: ${record.level.name}: ${record.message}';
      print(message);
      logFile.writeAsStringSync('$message\n', mode: FileMode.append);
    });
  }

  /// Validate environment and permissions
  Future<void> _validateEnvironment() async {
    final dir = Directory(_dataDir);
    if (!await dir.exists()) {
      throw Exception('Data directory does not exist: $_dataDir');
    }

    try {
      final testFile = File(path.join(_dataDir, 'test_write'));
      await testFile.writeAsString('test');
      await testFile.delete();
    } catch (e) {
      throw Exception('Insufficient permissions in data directory: $_dataDir');
    }
  }

  /// Get all collections to migrate
  Future<List<String>> _getCollections() async {
    final dir = Directory(_dataDir);
    final collections = <String>[];

    await for (final entity in dir.list()) {
      if (entity is Directory) {
        final name = path.basename(entity.path);
        if (!name.startsWith('.') && !name.startsWith('logs')) {
          collections.add(name);
        }
      }
    }

    return collections;
  }

  /// Migrate a single collection
  Future<void> _migrateCollection(String collection) async {
    _logger.info('Migrating collection: $collection');
    
    final collectionDir = Directory(path.join(_dataDir, collection));
    final files = await collectionDir
        .list()
        .where((e) => e is File && path.extension(e.path) == '.json')
        .toList();

    _logger.info('Found ${files.length} files to migrate in $collection');

    // Process files in batches
    for (var i = 0; i < files.length; i += _batchSize) {
      final batch = files.skip(i).take(_batchSize);
      await _processBatch(collection, batch);
      
      if (i + _batchSize < files.length) {
        await Future.delayed(_batchDelay);
      }
    }
  }

  /// Process a batch of files
  Future<void> _processBatch(String collection, Iterable<FileSystemEntity> batch) async {
    final futures = batch.map((file) => _migrateFile(collection, file as File));
    await Future.wait(futures);
  }

  /// Migrate a single file
  Future<void> _migrateFile(String collection, File file) async {
    try {
      final content = await file.readAsString();
      final data = jsonDecode(content);
      
      // Transform data based on collection type
      final transformed = await _transformData(collection, data);
      
      // Store in Web5 DWN
      final recordId = await _store.store(collection, transformed);
      
      _logger.info('Migrated ${file.path} to record: $recordId');
      
      // Verify migration
      await _verifyMigration(collection, recordId, transformed);
      
      // Create backup
      await _backupFile(file);
    } catch (e, stack) {
      _logger.warning('Failed to migrate ${file.path}', e, stack);
      await _handleMigrationError(file, e);
    }
  }

  /// Transform data based on collection type
  Future<Map<String, dynamic>> _transformData(
    String collection,
    Map<String, dynamic> data,
  ) async {
    switch (collection) {
      case 'wallets':
        return _migrateWalletData(data);
      case 'transactions':
        return _migrateTransactionData(data);
      case 'metadata':
        return _migrateMetadata(data);
      default:
        return data;
    }
  }

  /// Migrate wallet-specific data
  Map<String, dynamic> _migrateWalletData(Map<String, dynamic> data) {
    return {
      ...data,
      'version': Web5Config.currentVersion,
      'migrated_at': DateTime.now().toIso8601String(),
      'platform': Platform.operatingSystem,
    };
  }

  /// Migrate transaction-specific data
  Map<String, dynamic> _migrateTransactionData(Map<String, dynamic> data) {
    return {
      ...data,
      'version': Web5Config.currentVersion,
      'migrated_at': DateTime.now().toIso8601String(),
      'platform': Platform.operatingSystem,
    };
  }

  /// Migrate metadata
  Map<String, dynamic> _migrateMetadata(Map<String, dynamic> data) {
    return {
      ...data,
      'version': Web5Config.currentVersion,
      'migrated_at': DateTime.now().toIso8601String(),
      'platform': Platform.operatingSystem,
    };
  }

  /// Verify migrated data
  Future<void> _verifyMigration(
    String collection,
    String recordId,
    Map<String, dynamic> expectedData,
  ) async {
    final migrated = await _store.get(collection, recordId);
    if (migrated == null) {
      throw Exception('Verification failed: migrated record not found');
    }

    if (!_compareData(migrated, expectedData)) {
      throw Exception('Verification failed: data mismatch');
    }
  }

  /// Compare data objects for equality
  bool _compareData(
    Map<String, dynamic> a,
    Map<String, dynamic> b,
  ) {
    return jsonEncode(a) == jsonEncode(b);
  }

  /// Create backup of original file
  Future<void> _backupFile(File file) async {
    final backupDir = Directory(path.join(
      path.dirname(file.path),
      'backups',
    ));
    
    await backupDir.create(recursive: true);
    
    final backupPath = path.join(
      backupDir.path,
      '${path.basenameWithoutExtension(file.path)}_${DateTime.now().millisecondsSinceEpoch}.json',
    );
    
    await file.copy(backupPath);
  }

  /// Handle migration errors
  Future<void> _handleMigrationError(File file, dynamic error) async {
    final errorDir = Directory(path.join(
      path.dirname(file.path),
      'errors',
    ));
    
    await errorDir.create(recursive: true);
    
    final errorFile = File(path.join(
      errorDir.path,
      '${path.basenameWithoutExtension(file.path)}_error.txt',
    ));
    
    await errorFile.writeAsString('''
Timestamp: ${DateTime.now().toIso8601String()}
File: ${file.path}
Error: $error
''');
  }
}

void main(List<String> args) async {
  if (args.isEmpty) {
    print('Usage: dart migrate_to_web5.dart <data_directory>');
    exit(1);
  }

  final dataDir = args[0];
  final web5 = await Web5.connect();
  final did = await web5.did.create();
  final store = DWNStore(web5, did);

  final migrator = Web5Migrator(
    store,
    dataDir,
    enableCompression: true,
    batchSize: 50,
  );

  try {
    await migrator.migrate();
    exit(0);
  } catch (e) {
    print('Migration failed: $e');
    exit(1);
  }
}
