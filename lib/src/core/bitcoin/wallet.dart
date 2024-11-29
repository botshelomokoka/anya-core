import 'package:bitcoindart/bitcoindart.dart';
import 'package:logging/logging.dart';
import '../repositories/wallet_repository.dart';
import '../models/wallet.dart';

/// Bitcoin wallet management functionality
class BitcoinWallet {
  final Logger _logger = Logger('BitcoinWallet');
  final NetworkType _network;
  final WalletRepository _repository;

  BitcoinWallet({
    required WalletRepository repository,
    NetworkType network = NetworkType.mainnet,
  })  : _repository = repository,
        _network = network;

  Future<Wallet> createWallet({
    required String name,
    Map<String, dynamic>? metadata,
  }) async {
    try {
      // Generate Bitcoin wallet
      final hdWallet = HDWallet.generate();
      
      // Store wallet data in Web5 DWN
      final wallet = await _repository.create(
        name,
        'bitcoin',
        {
          ...?metadata,
          'network': _network.toString(),
          'mnemonic': hdWallet.mnemonic,
          'xpub': hdWallet.xpub,
        },
      );

      _logger.info('Created new Bitcoin wallet: ${wallet.id}');
      return wallet;
    } catch (e) {
      _logger.severe('Failed to create wallet: $e');
      rethrow;
    }
  }

  Future<List<Wallet>> listWallets() async {
    try {
      return await _repository.list();
    } catch (e) {
      _logger.severe('Failed to list wallets: $e');
      rethrow;
    }
  }

  Future<Wallet?> getWallet(String id) async {
    try {
      return await _repository.get(id);
    } catch (e) {
      _logger.severe('Failed to get wallet: $e');
      rethrow;
    }
  }

  Future<void> updateWallet(String id, Map<String, dynamic> updates) async {
    try {
      await _repository.update(id, updates);
    } catch (e) {
      _logger.severe('Failed to update wallet: $e');
      rethrow;
    }
  }

  Future<void> deleteWallet(String id) async {
    try {
      await _repository.delete(id);
    } catch (e) {
      _logger.severe('Failed to delete wallet: $e');
      rethrow;
    }
  }
}
