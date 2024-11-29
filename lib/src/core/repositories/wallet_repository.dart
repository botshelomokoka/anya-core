import '../storage/dwn_store.dart';
import '../models/wallet.dart';

/// Repository for wallet management using DWN storage
class WalletRepository {
  final DWNStore _store;
  static const String _collection = 'wallets';

  WalletRepository(this._store);

  Future<Wallet> create(String name, String type, Map<String, dynamic> metadata) async {
    final now = DateTime.now();
    final wallet = Wallet(
      id: '', // Will be set by DWN
      name: name,
      type: type,
      metadata: metadata,
      createdAt: now,
      updatedAt: now,
    );

    final id = await _store.store(_collection, wallet.toJson());
    return wallet.copyWith(id: id);
  }

  Future<Wallet?> get(String id) async {
    final data = await _store.get(_collection, id);
    return data != null ? Wallet.fromJson(data) : null;
  }

  Future<List<Wallet>> list() async {
    final records = await _store.query(_collection);
    return records.map((r) => Wallet.fromJson(r)).toList();
  }

  Future<void> update(String id, Map<String, dynamic> updates) async {
    final existing = await get(id);
    if (existing == null) {
      throw Exception('Wallet not found');
    }

    final updated = {
      ...existing.toJson(),
      ...updates,
      'updatedAt': DateTime.now().toIso8601String(),
    };

    await _store.update(_collection, id, updated);
  }

  Future<void> delete(String id) async {
    await _store.delete(_collection, id);
  }
}
