import 'package:test/test.dart';
import 'package:mockito/mockito.dart';
import '../../lib/src/core/bitcoin/wallet.dart';
import '../../lib/src/core/repositories/wallet_repository.dart';
import '../../lib/src/core/models/wallet.dart';

class MockWalletRepository extends Mock implements WalletRepository {}

void main() {
  group('BitcoinWallet', () {
    late MockWalletRepository mockRepository;
    late BitcoinWallet wallet;

    setUp(() {
      mockRepository = MockWalletRepository();
      wallet = BitcoinWallet(repository: mockRepository);
    });

    test('createWallet stores wallet in repository', () async {
      final testWallet = Wallet(
        id: 'test-id',
        name: 'Test Wallet',
        type: 'bitcoin',
        metadata: {
          'network': 'mainnet',
          'mnemonic': 'test mnemonic',
          'xpub': 'test xpub',
        },
        createdAt: DateTime.now(),
        updatedAt: DateTime.now(),
      );

      when(mockRepository.create(
        any,
        any,
        any,
      )).thenAnswer((_) async => testWallet);

      final result = await wallet.createWallet(name: 'Test Wallet');
      
      verify(mockRepository.create(
        'Test Wallet',
        'bitcoin',
        any,
      )).called(1);

      expect(result.id, equals(testWallet.id));
      expect(result.name, equals(testWallet.name));
      expect(result.type, equals('bitcoin'));
    });

    test('listWallets returns all wallets', () async {
      final testWallets = [
        Wallet(
          id: 'test-id-1',
          name: 'Test Wallet 1',
          type: 'bitcoin',
          metadata: {'network': 'mainnet'},
          createdAt: DateTime.now(),
          updatedAt: DateTime.now(),
        ),
        Wallet(
          id: 'test-id-2',
          name: 'Test Wallet 2',
          type: 'bitcoin',
          metadata: {'network': 'testnet'},
          createdAt: DateTime.now(),
          updatedAt: DateTime.now(),
        ),
      ];

      when(mockRepository.list()).thenAnswer((_) async => testWallets);

      final result = await wallet.listWallets();
      
      verify(mockRepository.list()).called(1);
      expect(result.length, equals(2));
      expect(result.first.id, equals('test-id-1'));
      expect(result.last.id, equals('test-id-2'));
    });

    test('getWallet returns specific wallet', () async {
      final testWallet = Wallet(
        id: 'test-id',
        name: 'Test Wallet',
        type: 'bitcoin',
        metadata: {'network': 'mainnet'},
        createdAt: DateTime.now(),
        updatedAt: DateTime.now(),
      );

      when(mockRepository.get('test-id')).thenAnswer((_) async => testWallet);

      final result = await wallet.getWallet('test-id');
      
      verify(mockRepository.get('test-id')).called(1);
      expect(result?.id, equals('test-id'));
      expect(result?.name, equals('Test Wallet'));
    });

    test('updateWallet updates wallet in repository', () async {
      final updates = {'name': 'Updated Wallet'};
      
      await wallet.updateWallet('test-id', updates);
      
      verify(mockRepository.update('test-id', updates)).called(1);
    });

    test('deleteWallet removes wallet from repository', () async {
      await wallet.deleteWallet('test-id');
      
      verify(mockRepository.delete('test-id')).called(1);
    });
  });
}
