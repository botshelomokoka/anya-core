import 'package:test/test.dart';
import 'package:mockito/mockito.dart';
import 'package:mockito/annotations.dart';
import 'package:bitcoin_core/bitcoin_core.dart';
import 'package:anya/src/core/bitcoin/wallet.dart';

@GenerateMocks([BitcoinNetwork])
void main() {
  group('BitcoinWallet Tests', () {
    late BitcoinWallet wallet;
    late MockBitcoinNetwork mockNetwork;

    setUp(() {
      mockNetwork = MockBitcoinNetwork();
      wallet = BitcoinWallet(network: mockNetwork);
    });

    test('wallet initialization', () {
      expect(wallet, isNotNull);
      expect(wallet.network, equals(mockNetwork));
    });

    test('create new wallet', () async {
      final mnemonic = await wallet.generateMnemonic();
      expect(mnemonic.split(' ').length, equals(24));
    });

    test('restore from seed', () async {
      const testMnemonic =
          'abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about';
      final restored = await wallet.restoreFromMnemonic(testMnemonic);
      expect(restored, isTrue);
    });

    test('get balance', () async {
      when(mockNetwork.getBalance(any))
          .thenAnswer((_) async => BigInt.from(100000));

      final balance = await wallet.getBalance();
      expect(balance, equals(BigInt.from(100000)));
    });

    test('send transaction', () async {
      const address = 'bc1qxy2kgdygjrsqtzq2n0yrf2493p83kkfjhx0wlh';
      final amount = BigInt.from(50000);

      when(mockNetwork.sendTransaction(
        address: address,
        amount: amount,
        fee: any,
      )).thenAnswer((_) async => 'txid');

      final txid = await wallet.sendTransaction(
        toAddress: address,
        amount: amount,
      );

      expect(txid, isNotNull);
      verify(mockNetwork.sendTransaction(
        address: address,
        amount: amount,
        fee: any,
      )).called(1);
    });
  });
}
