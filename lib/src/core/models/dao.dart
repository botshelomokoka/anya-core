import 'package:json_annotation/json_annotation.dart';
import 'package:equatable/equatable.dart';
import 'wallet.dart';

part 'dao.g.dart';

@JsonSerializable()
class DAO extends Equatable {
  final String id;
  final String name;
  final String description;
  final String ownerDid;
  final List<String> memberDids;
  final Map<String, dynamic> governance;
  final Map<String, dynamic> metadata;
  final DateTime createdAt;
  final DateTime updatedAt;
  final String treasuryWalletId;
  final List<String>? proposalIds;

  const DAO({
    required this.id,
    required this.name,
    required this.description,
    required this.ownerDid,
    required this.memberDids,
    required this.governance,
    required this.metadata,
    required this.createdAt,
    required this.updatedAt,
    required this.treasuryWalletId,
    this.proposalIds,
  });

  factory DAO.create({
    required String name,
    required String description,
    required String ownerDid,
    required Map<String, dynamic> governance,
    required String treasuryWalletId,
    Map<String, dynamic>? metadata,
  }) {
    final now = DateTime.now();
    return DAO(
      id: DateTime.now().millisecondsSinceEpoch.toString(),
      name: name,
      description: description,
      ownerDid: ownerDid,
      memberDids: [ownerDid],
      governance: governance,
      metadata: metadata ?? {},
      createdAt: now,
      updatedAt: now,
      treasuryWalletId: treasuryWalletId,
      proposalIds: [],
    );
  }

  DAO copyWith({
    String? id,
    String? name,
    String? description,
    String? ownerDid,
    List<String>? memberDids,
    Map<String, dynamic>? governance,
    Map<String, dynamic>? metadata,
    DateTime? createdAt,
    DateTime? updatedAt,
    String? treasuryWalletId,
    List<String>? proposalIds,
  }) {
    return DAO(
      id: id ?? this.id,
      name: name ?? this.name,
      description: description ?? this.description,
      ownerDid: ownerDid ?? this.ownerDid,
      memberDids: memberDids ?? this.memberDids,
      governance: governance ?? this.governance,
      metadata: metadata ?? this.metadata,
      createdAt: createdAt ?? this.createdAt,
      updatedAt: updatedAt ?? this.updatedAt,
      treasuryWalletId: treasuryWalletId ?? this.treasuryWalletId,
      proposalIds: proposalIds ?? this.proposalIds,
    );
  }

  Map<String, dynamic> toJson() => _$DAOToJson(this);

  factory DAO.fromJson(Map<String, dynamic> json) => _$DAOFromJson(json);

  @override
  List<Object?> get props => [
        id,
        name,
        description,
        ownerDid,
        memberDids,
        governance,
        metadata,
        createdAt,
        updatedAt,
        treasuryWalletId,
        proposalIds,
      ];

  bool canMemberVote(String did) => memberDids.contains(did);

  bool isOwner(String did) => ownerDid == did;

  bool hasMember(String did) => memberDids.contains(did);
}
