import '../models/stacks_wallet.dart';
import '../services/web5_storage.dart';
import '../errors/repository_error.dart';
import 'wallet_repository.dart';

class StacksWalletRepository extends WalletRepository {
  final Web5Storage _storage;

  StacksWalletRepository({
    required Web5Storage storage,
  }) : _storage = storage;

  @override
  Future<void> createWallet(StacksWallet wallet) async {
    try {
      // Store wallet data in Web5 DWN
      final walletData = wallet.toJson();
      await _storage.storeWalletData(walletData);
    } catch (e) {
      throw RepositoryError('Failed to create Stacks wallet: $e');
    }
  }

  @override
  Future<StacksWallet?> getWallet(String id) async {
    try {
      final data = await _storage.getWalletData(id);
      if (data == null) return null;
      return StacksWallet.fromJson(data);
    } catch (e) {
      throw RepositoryError('Failed to get Stacks wallet: $e');
    }
  }

  Future<StacksWallet?> getWalletByStacksAddress(String stacksAddress) async {
    try {
      final wallets = await listWallets();
      return wallets.firstWhere(
        (w) => w is StacksWallet && w.stacksAddress == stacksAddress,
        orElse: () => null,
      ) as StacksWallet?;
    } catch (e) {
      throw RepositoryError('Failed to get wallet by Stacks address: $e');
    }
  }

  @override
  Future<List<StacksWallet>> listWallets({String? ownerDid}) async {
    try {
      final wallets = await _storage.listWallets();
      return wallets
          .where((w) => w['type'] == 'stacks' && 
                       (ownerDid == null || w['ownerDid'] == ownerDid))
          .map((w) => StacksWallet.fromJson(w))
          .toList();
    } catch (e) {
      throw RepositoryError('Failed to list Stacks wallets: $e');
    }
  }

  @override
  Future<void> updateWallet(String id, StacksWallet wallet) async {
    try {
      final walletData = wallet.copyWith(
        updatedAt: DateTime.now(),
      ).toJson();
      await _storage.updateWallet(id, walletData);
    } catch (e) {
      throw RepositoryError('Failed to update Stacks wallet: $e');
    }
  }

  @override
  Future<void> deleteWallet(String id, String ownerDid) async {
    try {
      final wallet = await getWallet(id);
      if (wallet == null) {
        throw RepositoryError('Wallet not found');
      }
      if (wallet.ownerDid != ownerDid) {
        throw RepositoryError('Not authorized to delete wallet');
      }
      await _storage.deleteWallet(id);
    } catch (e) {
      throw RepositoryError('Failed to delete Stacks wallet: $e');
    }
  }
}
