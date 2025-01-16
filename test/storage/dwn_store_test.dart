import 'dart:convert';

import 'package:mockito/mockito.dart';
import 'package:test/test.dart';
import 'package:web5/web5.dart';

import '../../lib/src/core/config/web5_config.dart';
import '../../lib/src/core/errors/storage_errors.dart';
import '../../lib/src/core/storage/dwn_store.dart';

class MockWeb5 extends Mock implements Web5 {}

class MockDWN extends Mock implements DWN {}

class MockRecords extends Mock implements Records {}

class MockPermissions extends Mock implements Permissions {}

class MockDID extends Mock implements DID {}

class MockRecord extends Mock implements Record {}

void main() {
  late DWNStore store;
  late MockWeb5 mockWeb5;
  late MockDWN mockDwn;
  late MockRecords mockRecords;
  late MockPermissions mockPermissions;
  late MockDID mockDid;

  setUp(() {
    mockWeb5 = MockWeb5();
    mockDwn = MockDWN();
    mockRecords = MockRecords();
    mockPermissions = MockPermissions();
    mockDid = MockDID();

    when(mockWeb5.dwn).thenReturn(mockDwn);
    when(mockDwn.records).thenReturn(mockRecords);
    when(mockDwn.permissions).thenReturn(mockPermissions);
    when(mockDid.uri).thenReturn('did:example:123');

    store = DWNStore(mockWeb5, mockDid);
  });

  group('Store Operations', () {
    test('store compresses and stores data correctly', () async {
      final testData = {'test': 'data'};
      final mockRecord = MockRecord();
      when(mockRecord.id).thenReturn('test_id');

      when(mockRecords.create(
        data: any,
        message: any,
      )).thenAnswer((_) async => mockRecord);

      final id = await store.store('test_collection', testData);

      verify(mockRecords.create(
        data: any,
        message: {
          'schema': Web5Config.getSchemaUri('test_collection'),
          'dataFormat': 'application/octet-stream',
        },
      )).called(1);

      expect(id, equals('test_id'));
    });

    test('store handles errors correctly', () async {
      when(mockRecords.create(
        data: any,
        message: any,
      )).thenThrow(Exception('Test error'));

      expect(
        () => store.store('test_collection', {}),
        throwsA(isA<StorageError>()),
      );
    });
  });

  group('Retrieval Operations', () {
    test('get retrieves and decompresses data correctly', () async {
      final mockRecord = MockRecord();
      final testData = utf8.encode(jsonEncode({'test': 'data'}));

      when(mockRecord.data).thenReturn(testData);
      when(mockRecords.read(any)).thenAnswer((_) async => mockRecord);

      final result = await store.get('test_collection', 'test_id');

      expect(result, equals({'test': 'data'}));
      verify(mockRecords.read('test_id')).called(1);
    });

    test('get returns null for non-existent record', () async {
      when(mockRecords.read(any)).thenAnswer((_) async => null);

      final result = await store.get('test_collection', 'test_id');

      expect(result, isNull);
    });

    test('get uses cache when available', () async {
      final mockRecord = MockRecord();
      final testData = utf8.encode(jsonEncode({'test': 'data'}));

      when(mockRecord.data).thenReturn(testData);
      when(mockRecord.id).thenReturn('test_id');
      when(mockRecords.read(any)).thenAnswer((_) async => mockRecord);

      // First call - should hit the DWN
      await store.get('test_collection', 'test_id');

      // Second call - should use cache
      final result = await store.get('test_collection', 'test_id');

      expect(result, equals({'test': 'data'}));
      verify(mockRecords.read('test_id')).called(1);
    });
  });

  group('Query Operations', () {
    test('query returns filtered results', () async {
      final mockRecord1 = MockRecord();
      final mockRecord2 = MockRecord();
      final testData1 = utf8.encode(jsonEncode({'id': 1}));
      final testData2 = utf8.encode(jsonEncode({'id': 2}));

      when(mockRecord1.data).thenReturn(testData1);
      when(mockRecord2.data).thenReturn(testData2);

      when(mockRecords.query(
        message: any,
      )).thenAnswer((_) async => [mockRecord1, mockRecord2]);

      final results = await store.query(
        'test_collection',
        filter: {'type': 'test'},
      );

      expect(results.length, equals(2));
      expect(results[0]['id'], equals(1));
      expect(results[1]['id'], equals(2));
    });

    test('query uses cache for unfiltered queries', () async {
      final mockRecord = MockRecord();
      final testData = utf8.encode(jsonEncode({'test': 'data'}));

      when(mockRecord.data).thenReturn(testData);
      when(mockRecords.query(message: any))
          .thenAnswer((_) async => [mockRecord]);

      // First call - should hit the DWN
      await store.query('test_collection');

      // Second call - should use cache
      final results = await store.query('test_collection');

      expect(results.length, equals(1));
      verify(mockRecords.query(message: any)).called(1);
    });
  });

  group('Update Operations', () {
    test('update compresses and updates data correctly', () async {
      final testData = {'test': 'updated'};

      await store.update('test_collection', 'test_id', testData);

      verify(mockRecords.update(
        'test_id',
        data: any,
        message: {
          'schema': Web5Config.getSchemaUri('test_collection'),
          'dataFormat': 'application/octet-stream',
        },
      )).called(1);
    });

    test('update invalidates cache', () async {
      final mockRecord = MockRecord();
      final testData = utf8.encode(jsonEncode({'test': 'data'}));

      when(mockRecord.data).thenReturn(testData);
      when(mockRecord.id).thenReturn('test_id');
      when(mockRecords.read(any)).thenAnswer((_) async => mockRecord);

      // Prime the cache
      await store.get('test_collection', 'test_id');

      // Update should invalidate cache
      await store.update('test_collection', 'test_id', {'test': 'updated'});

      // Next get should hit the DWN
      await store.get('test_collection', 'test_id');

      verify(mockRecords.read('test_id')).called(2);
    });
  });

  group('Permission Operations', () {
    test('verifyPermissions checks owner and permissions', () async {
      final mockRecord = MockRecord();
      when(mockRecord.owner).thenReturn('did:example:456');
      when(mockRecords.read(any)).thenAnswer((_) async => mockRecord);

      when(mockPermissions.check(
        recordId: any,
        did: any,
      )).thenAnswer((_) async => PermissionResponse(granted: true));

      final hasPermission = await store.verifyPermissions(
        'test_id',
        'did:example:789',
      );

      expect(hasPermission, isTrue);
      verify(mockPermissions.check(
        recordId: 'test_id',
        did: 'did:example:789',
      )).called(1);
    });

    test('verifyPermissions returns false for non-existent record', () async {
      when(mockRecords.read(any)).thenAnswer((_) async => null);

      final hasPermission = await store.verifyPermissions(
        'test_id',
        'did:example:789',
      );

      expect(hasPermission, isFalse);
    });
  });

  group('Cache Management', () {
    test('cache respects size limit', () async {
      final mockRecord = MockRecord();
      final testData = utf8.encode(jsonEncode({'test': 'data'}));

      when(mockRecord.data).thenReturn(testData);
      when(mockRecord.id).thenReturn('test_id');
      when(mockRecords.read(any)).thenAnswer((_) async => mockRecord);

      // Add many records to trigger cache limit
      for (var i = 0; i < 1100; i++) {
        await store.get('test_collection_$i', 'test_id_$i');
      }

      // Verify oldest cache entries were removed
      final result = await store.get('test_collection_0', 'test_id_0');
      verify(mockRecords.read('test_id_0')).called(2);
    });
  });
}
