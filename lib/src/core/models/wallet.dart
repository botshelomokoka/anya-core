/// Cross-platform wallet model
@JsonSerializable()
class Wallet extends Equatable {
  final String id;
  final String name;
  final String type;
  final String ownerDid;
  final String address;
  final Map<String, dynamic> metadata;
  final DateTime createdAt;
  final DateTime updatedAt;
  final String? encryptedData;
  final List<String>? permissions;

  const Wallet({
    required this.id,
    required this.name,
    required this.type,
    required this.ownerDid,
    required this.address,
    required this.metadata,
    required this.createdAt,
    required this.updatedAt,
    this.encryptedData,
    this.permissions,
  });

  /// Create a new wallet instance
  factory Wallet.create({
    required String name,
    required String type,
    required String ownerDid,
    required String address,
    Map<String, dynamic>? metadata,
    String? encryptedData,
    List<String>? permissions,
  }) {
    final now = DateTime.now();
    return Wallet(
      id: '', // Will be set by storage layer
      name: name,
      type: type,
      ownerDid: ownerDid,
      address: address,
      metadata: metadata ?? {},
      createdAt: now,
      updatedAt: now,
      encryptedData: encryptedData,
      permissions: permissions,
    );
  }

  /// Create a copy with updated fields
  Wallet copyWith({
    String? id,
    String? name,
    String? type,
    String? ownerDid,
    String? address,
    Map<String, dynamic>? metadata,
    DateTime? createdAt,
    DateTime? updatedAt,
    String? encryptedData,
    List<String>? permissions,
  }) {
    return Wallet(
      id: id ?? this.id,
      name: name ?? this.name,
      type: type ?? this.type,
      ownerDid: ownerDid ?? this.ownerDid,
      address: address ?? this.address,
      metadata: metadata ?? this.metadata,
      createdAt: createdAt ?? this.createdAt,
      updatedAt: updatedAt ?? this.updatedAt,
      encryptedData: encryptedData ?? this.encryptedData,
      permissions: permissions ?? this.permissions,
    );
  }

  /// Create a wallet from JSON
  factory Wallet.fromJson(Map<String, dynamic> json) => _$WalletFromJson(json);

  /// Convert wallet to JSON
  Map<String, dynamic> toJson() => _$WalletToJson(this);

  @override
  List<Object?> get props => [
        id,
        name,
        type,
        ownerDid,
        address,
        metadata,
        createdAt,
        updatedAt,
        encryptedData,
        permissions,
      ];

  @override
  String toString() => 'Wallet(id: $id, name: $name, type: $type)';
}
