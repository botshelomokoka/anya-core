import '../models/dao.dart';
import '../models/wallet.dart';
import '../repositories/dao_repository.dart';
import '../repositories/wallet_repository.dart';
import '../errors/service_errors.dart';

class DAOService {
  final DAORepository _daoRepository;
  final WalletRepository _walletRepository;

  DAOService(this._daoRepository, this._walletRepository);

  Future<DAO> createDAO({
    required String name,
    required String description,
    required String ownerDid,
    required Map<String, dynamic> governance,
    Map<String, dynamic>? metadata,
  }) async {
    try {
      // Create treasury wallet
      final treasuryWallet = Wallet.create(
        name: '$name Treasury',
        type: 'dao_treasury',
        ownerDid: ownerDid,
        address: '', // Will be set by chain-specific implementation
        metadata: {
          'daoName': name,
          'isDAOTreasury': true,
          ...metadata ?? {},
        },
      );

      final walletId = await _walletRepository.createWallet(treasuryWallet);

      // Create DAO
      final dao = DAO.create(
        name: name,
        description: description,
        ownerDid: ownerDid,
        governance: governance,
        treasuryWalletId: walletId,
        metadata: metadata,
      );

      final daoId = await _daoRepository.createDAO(dao);
      return dao.copyWith(id: daoId);
    } catch (e) {
      throw ServiceError('Failed to create DAO: $e');
    }
  }

  Future<DAO?> getDAO(String id) async {
    try {
      return await _daoRepository.getDAO(id);
    } catch (e) {
      throw ServiceError('Failed to get DAO: $e');
    }
  }

  Future<List<DAO>> listDAOs({String? memberDid}) async {
    try {
      return await _daoRepository.listDAOs(memberDid: memberDid);
    } catch (e) {
      throw ServiceError('Failed to list DAOs: $e');
    }
  }

  Future<void> updateDAO(String id, DAO dao) async {
    try {
      await _daoRepository.updateDAO(id, dao);
    } catch (e) {
      throw ServiceError('Failed to update DAO: $e');
    }
  }

  Future<void> addMember(String id, String memberDid, String ownerDid) async {
    try {
      await _daoRepository.addMember(id, memberDid, ownerDid);
    } catch (e) {
      throw ServiceError('Failed to add member: $e');
    }
  }

  Future<void> removeMember(
      String id, String memberDid, String ownerDid) async {
    try {
      await _daoRepository.removeMember(id, memberDid, ownerDid);
    } catch (e) {
      throw ServiceError('Failed to remove member: $e');
    }
  }

  Future<Wallet?> getTreasuryWallet(String daoId) async {
    try {
      final dao = await _daoRepository.getDAO(daoId);
      if (dao == null) return null;

      return await _walletRepository.getWallet(dao.treasuryWalletId);
    } catch (e) {
      throw ServiceError('Failed to get treasury wallet: $e');
    }
  }

  Future<void> updateGovernance(
    String id,
    Map<String, dynamic> governance,
    String ownerDid,
  ) async {
    try {
      final dao = await _daoRepository.getDAO(id);
      if (dao == null) {
        throw ServiceError('DAO not found');
      }

      if (!dao.isOwner(ownerDid)) {
        throw ServiceError('Only owner can update governance');
      }

      final updatedDAO = dao.copyWith(
        governance: governance,
        updatedAt: DateTime.now(),
      );

      await _daoRepository.updateDAO(id, updatedDAO);
    } catch (e) {
      throw ServiceError('Failed to update governance: $e');
    }
  }
}
