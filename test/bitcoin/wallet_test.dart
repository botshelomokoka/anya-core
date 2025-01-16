import 'package:bitcoindart/bitcoindart.dart';
import 'package:mockito/mockito.dart';
import 'package:test/test.dart';
import 'package:web5/web5.dart';

import '../../lib/src/core/bitcoin/wallet.dart';
import '../../lib/src/core/models/wallet.dart' as models;
import '../../lib/src/core/repositories/wallet_repository.dart';

class MockWeb5 extends Mock implements Web5 {}

class MockDWN extends Mock implements DWN {}

class MockRecords extends Mock implements Records {}

class MockWalletRepository extends Mock implements WalletRepository {}

class MockHDWallet extends Mock implements HDWallet {}

void main() {
  late BitcoinWallet wallet;
  late MockWeb5 mockWeb5;
  late MockWalletRepository mockRepository;
  late MockDWN mockDwn;
  late MockRecords mockRecords;

  setUp(() {
    mockWeb5 = MockWeb5();
    mockRepository = MockWalletRepository();
    mockDwn = MockDWN();
    mockRecords = MockRecords();

    when(mockWeb5.dwn).thenReturn(mockDwn);
    when(mockDwn.records).thenReturn(mockRecords);

    wallet = BitcoinWallet(
      mockRepository,
      mockWeb5,
      NetworkType.testnet,
    );
  });

  group('BitcoinWallet', () {
    test('initialize creates new wallet with correct parameters', () async {
      const testName = 'Test Wallet';
      const testOwnerDid = 'did:example:123';
      const testAddress = 'bc1qtest';

      final expectedWallet = models.Wallet.create(
        name: testName,
        type: 'bitcoin',
        ownerDid: testOwnerDid,
        address: testAddress,
        metadata: {
          'network': 'testnet',
          'xpub': 'test_xpub',
          'addressType': 'p2wpkh',
        },
      );

      when(mockRepository.createWallet(any))
          .thenAnswer((_) async => 'test_wallet_id');

      when(mockWeb5.encrypt(
        data: any,
        recipients: [testOwnerDid],
      )).thenAnswer((_) async => 'encrypted_data');

      await wallet.initialize(
        name: testName,
        ownerDid: testOwnerDid,
      );

      verify(mockRepository.createWallet(any)).called(1);
      verify(mockWeb5.encrypt(
        data: any,
        recipients: [testOwnerDid],
      )).called(1);
    });

    test('createTransaction stores transaction in Web5', () async {
      const testName = 'Test Wallet';
      const testOwnerDid = 'did:example:123';

      await wallet.initialize(
        name: testName,
        ownerDid: testOwnerDid,
      );

      when(mockRecords.create(
        data: any,
        message: any,
      )).thenAnswer((_) async => Record());

      await wallet.createTransaction(
        toAddress: 'bc1qtest',
        amount: 100000,
      );

      verify(mockRecords.create(
        data: any,
        message: {
          'schema': 'anya/bitcoin/transaction',
          'dataFormat': 'application/json',
        },
      )).called(1);
    });

    test('export includes encrypted data when requested', () async {
      const testName = 'Test Wallet';
      const testOwnerDid = 'did:example:123';
      const encryptedData = 'encrypted_test_data';
      const decryptedData = {
        'seed': 'test_seed',
        'privateKey': 'test_private_key',
        'mnemonic': 'test mnemonic',
      };

      await wallet.initialize(
        name: testName,
        ownerDid: testOwnerDid,
      );

      when(mockWeb5.decrypt(encryptedData))
          .thenAnswer((_) async => decryptedData);

      final exportData = await wallet.export(includePrivateData: true);

      expect(exportData['privateData'], equals(decryptedData));
      verify(mockWeb5.decrypt(any)).called(1);
    });

    test('import validates required fields', () async {
      expect(
        () => wallet.import({}),
        throwsA(isA<InvalidWalletDataException>()),
      );

      expect(
        () => wallet.import({'name': 'Test'}),
        throwsA(isA<InvalidWalletDataException>()),
      );

      await wallet.import({
        'name': 'Test',
        'ownerDid': 'did:example:123',
      });
    });
  });

  group('Error Handling', () {
    test('handles Web5 encryption errors', () async {
      const testName = 'Test Wallet';
      const testOwnerDid = 'did:example:123';

      when(mockWeb5.encrypt(
        data: any,
        recipients: any,
      )).thenThrow(Exception('Encryption failed'));

      expect(
        () => wallet.initialize(
          name: testName,
          ownerDid: testOwnerDid,
        ),
        throwsException,
      );
    });

    test('handles repository errors', () async {
      const testName = 'Test Wallet';
      const testOwnerDid = 'did:example:123';

      when(mockRepository.createWallet(any))
          .thenThrow(Exception('Repository error'));

      expect(
        () => wallet.initialize(
          name: testName,
          ownerDid: testOwnerDid,
        ),
        throwsException,
      );
    });
  });
}
