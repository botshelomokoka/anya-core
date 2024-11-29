import 'package:test/test.dart';
import '../../lib/src/core/bitcoin/wallet.dart';

void main() {
  group('BitcoinWallet', () {
    late BitcoinWallet wallet;

    setUp(() {
      wallet = BitcoinWallet();
    });

    test('createWallet returns wallet id', () async {
      final walletId = await wallet.createWallet();
      expect(walletId, isNotEmpty);
    });
  });
}
