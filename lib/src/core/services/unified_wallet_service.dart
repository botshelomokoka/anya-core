import '../errors/service_errors.dart';
import '../models/wallet.dart';
import '../repositories/wallet_repository.dart';
import '../security/biometric_service.dart';
import '../security/encryption_service.dart';
import 'chain_coordinator.dart';

class UnifiedWalletService {
  final ChainCoordinator _coordinator;
  final WalletRepository _repository;
  final EncryptionService _encryption;
  final BiometricService _biometric;

  UnifiedWalletService({
    required ChainCoordinator coordinator,
    required WalletRepository repository,
    required EncryptionService encryption,
    required BiometricService biometric,
  })  : _coordinator = coordinator,
        _repository = repository,
        _encryption = encryption,
        _biometric = biometric;

  Future<Map<String, Wallet>> createMultiChainWallet({
    required String ownerDid,
    required List<String> chains,
    required String name,
    Map<String, dynamic>? metadata,
  }) async {
    try {
      // Create wallets on all specified chains
      final addresses = await _coordinator.createWallets(
        ownerDid: ownerDid,
        chains: chains,
      );

      final wallets = <String, Wallet>{};
      for (final entry in addresses.entries) {
        final wallet = Wallet.create(
          name: '$name (${entry.key})',
          type: entry.key,
          ownerDid: ownerDid,
          address: entry.value,
          metadata: {
            'chain': entry.key,
            ...?metadata,
          },
        );

        final id = await _repository.createWallet(wallet);
        wallets[entry.key] = wallet.copyWith(id: id);
      }

      return wallets;
    } catch (e) {
      throw ServiceError('Failed to create multi-chain wallet: $e');
    }
  }

  Future<void> backupWallets(String ownerDid, String password) async {
    try {
      final wallets = await _repository.listWallets(ownerDid: ownerDid);

      // Encrypt wallet data
      final encryptedData = await _encryption.encryptWallets(
        wallets,
        password,
      );

      // Store backup
      await _repository.storeBackup(ownerDid, encryptedData);
    } catch (e) {
      throw ServiceError('Failed to backup wallets: $e');
    }
  }

  Future<void> restoreWallets(
    String ownerDid,
    String password,
    List<String> chains,
  ) async {
    try {
      // Get encrypted backup
      final encryptedData = await _repository.getBackup(ownerDid);
      if (encryptedData == null) {
        throw ServiceError('No backup found');
      }

      // Decrypt wallet data
      final wallets = await _encryption.decryptWallets(
        encryptedData,
        password,
      );

      // Validate and restore wallets
      for (final wallet in wallets) {
        if (chains.contains(wallet.type)) {
          await _coordinator.validateAddresses({
            wallet.type: wallet.address,
          });
          await _repository.createWallet(wallet);
        }
      }
    } catch (e) {
      throw ServiceError('Failed to restore wallets: $e');
    }
  }

  Future<Map<String, int>> getBalances(String ownerDid) async {
    try {
      return await _coordinator.getAllBalances(ownerDid);
    } catch (e) {
      throw ServiceError('Failed to get balances: $e');
    }
  }

  Future<void> unlockWallet(String id) async {
    try {
      if (!await _biometric.authenticate()) {
        throw ServiceError('Biometric authentication failed');
      }

      final wallet = await _repository.getWallet(id);
      if (wallet == null) {
        throw ServiceError('Wallet not found');
      }

      // Decrypt wallet data if needed
      if (wallet.encryptedData != null) {
        await _encryption.decryptWalletData(wallet);
      }
    } catch (e) {
      throw ServiceError('Failed to unlock wallet: $e');
    }
  }

  Future<void> lockWallet(String id) async {
    try {
      final wallet = await _repository.getWallet(id);
      if (wallet == null) {
        throw ServiceError('Wallet not found');
      }

      // Encrypt sensitive data before locking
      if (wallet.encryptedData != null) {
        await _encryption.encryptWalletData(wallet);
      }
    } catch (e) {
      throw ServiceError('Failed to lock wallet: $e');
    }
  }

  Set<String> get supportedChains => _coordinator.supportedChains;
}
