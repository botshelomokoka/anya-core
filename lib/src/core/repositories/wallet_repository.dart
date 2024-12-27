import 'dart:async';
import '../models/wallet.dart';
import '../storage/dwn_store.dart';
import '../errors/repository_errors.dart';

/// Cross-platform wallet repository implementation
class WalletRepository {
  static const String _collection = 'wallets';
  final DWNStore _store;

  WalletRepository(this._store);

  /// Create a new wallet
  Future<String> createWallet(Wallet wallet) async {
    try {
      final id = await _store.store(_collection, wallet.toJson());
      return id;
    } catch (e) {
      throw RepositoryError('Failed to create wallet: $e');
    }
  }

  /// Get wallet by ID
  Future<Wallet?> getWallet(String id) async {
    try {
      final data = await _store.get(_collection, id);
      if (data == null) return null;

      return Wallet.fromJson(data);
    } catch (e) {
      throw RepositoryError('Failed to get wallet: $e');
    }
  }

  /// List all wallets for a user
  Future<List<Wallet>> listWallets({String? ownerDid}) async {
    try {
      final filter = ownerDid != null ? {'owner': ownerDid} : null;
      final records = await _store.query(_collection, filter: filter);

      return records.map((data) => Wallet.fromJson(data)).toList();
    } catch (e) {
      throw RepositoryError('Failed to list wallets: $e');
    }
  }

  /// Update wallet
  Future<void> updateWallet(String id, Wallet wallet) async {
    try {
      // Verify permissions
      if (!await _store.verifyPermissions(id, wallet.ownerDid)) {
        throw RepositoryError('Permission denied');
      }

      await _store.update(_collection, id, wallet.toJson());
    } catch (e) {
      throw RepositoryError('Failed to update wallet: $e');
    }
  }

  /// Delete wallet
  Future<void> deleteWallet(String id, String ownerDid) async {
    try {
      // Verify permissions
      if (!await _store.verifyPermissions(id, ownerDid)) {
        throw RepositoryError('Permission denied');
      }

      await _store.delete(_collection, id);
    } catch (e) {
      throw RepositoryError('Failed to delete wallet: $e');
    }
  }

  /// Get wallet by address
  Future<Wallet?> getWalletByAddress(String address) async {
    try {
      final records = await _store.query(
        _collection,
        filter: {'address': address},
      );

      if (records.isEmpty) return null;
      return Wallet.fromJson(records.first);
    } catch (e) {
      throw RepositoryError('Failed to get wallet by address: $e');
    }
  }

  /// Get wallets by type
  Future<List<Wallet>> getWalletsByType(
    String type, {
    String? ownerDid,
  }) async {
    try {
      final filter = {
        'type': type,
        if (ownerDid != null) 'owner': ownerDid,
      };

      final records = await _store.query(_collection, filter: filter);
      return records.map((data) => Wallet.fromJson(data)).toList();
    } catch (e) {
      throw RepositoryError('Failed to get wallets by type: $e');
    }
  }
}
