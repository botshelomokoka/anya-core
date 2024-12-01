import '../models/dao.dart';
import '../storage/dwn_store.dart';
import '../errors/repository_errors.dart';
import 'wallet_repository.dart';

class DAORepository {
  static const String _collection = 'daos';
  final DWNStore _store;
  final WalletRepository _walletRepository;

  DAORepository(this._store, this._walletRepository);

  Future<String> createDAO(DAO dao) async {
    try {
      // Verify treasury wallet exists
      final wallet = await _walletRepository.getWallet(dao.treasuryWalletId);
      if (wallet == null) {
        throw RepositoryError('Treasury wallet not found');
      }

      final id = await _store.store(_collection, dao.toJson());
      return id;
    } catch (e) {
      throw RepositoryError('Failed to create DAO: $e');
    }
  }

  Future<DAO?> getDAO(String id) async {
    try {
      final data = await _store.get(_collection, id);
      if (data == null) return null;
      
      return DAO.fromJson(data);
    } catch (e) {
      throw RepositoryError('Failed to get DAO: $e');
    }
  }

  Future<List<DAO>> listDAOs({String? memberDid}) async {
    try {
      final filter = memberDid != null ? {'memberDids': memberDid} : null;
      final records = await _store.query(_collection, filter: filter);
      
      return records.map((data) => DAO.fromJson(data)).toList();
    } catch (e) {
      throw RepositoryError('Failed to list DAOs: $e');
    }
  }

  Future<void> updateDAO(String id, DAO dao) async {
    try {
      // Verify permissions
      if (!await _hasPermission(id, dao.ownerDid)) {
        throw RepositoryError('Permission denied');
      }

      await _store.update(_collection, id, dao.toJson());
    } catch (e) {
      throw RepositoryError('Failed to update DAO: $e');
    }
  }

  Future<void> deleteDAO(String id, String ownerDid) async {
    try {
      // Verify permissions
      if (!await _hasPermission(id, ownerDid)) {
        throw RepositoryError('Permission denied');
      }

      await _store.delete(_collection, id);
    } catch (e) {
      throw RepositoryError('Failed to delete DAO: $e');
    }
  }

  Future<void> addMember(String id, String memberDid, String ownerDid) async {
    try {
      final dao = await getDAO(id);
      if (dao == null) {
        throw RepositoryError('DAO not found');
      }

      if (!dao.isOwner(ownerDid)) {
        throw RepositoryError('Only owner can add members');
      }

      if (dao.hasMember(memberDid)) {
        throw RepositoryError('Member already exists');
      }

      final updatedDAO = dao.copyWith(
        memberDids: [...dao.memberDids, memberDid],
        updatedAt: DateTime.now(),
      );

      await updateDAO(id, updatedDAO);
    } catch (e) {
      throw RepositoryError('Failed to add member: $e');
    }
  }

  Future<void> removeMember(String id, String memberDid, String ownerDid) async {
    try {
      final dao = await getDAO(id);
      if (dao == null) {
        throw RepositoryError('DAO not found');
      }

      if (!dao.isOwner(ownerDid)) {
        throw RepositoryError('Only owner can remove members');
      }

      if (dao.ownerDid == memberDid) {
        throw RepositoryError('Cannot remove owner');
      }

      if (!dao.hasMember(memberDid)) {
        throw RepositoryError('Member not found');
      }

      final updatedDAO = dao.copyWith(
        memberDids: dao.memberDids.where((did) => did != memberDid).toList(),
        updatedAt: DateTime.now(),
      );

      await updateDAO(id, updatedDAO);
    } catch (e) {
      throw RepositoryError('Failed to remove member: $e');
    }
  }

  Future<bool> _hasPermission(String id, String did) async {
    final dao = await getDAO(id);
    if (dao == null) return false;
    return dao.isOwner(did);
  }
}
