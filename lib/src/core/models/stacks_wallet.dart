import 'package:json_annotation/json_annotation.dart';
import 'wallet.dart';

part 'stacks_wallet.g.dart';

@JsonSerializable()
class StacksWallet extends Wallet {
  final String stacksAddress;
  final bool isTestnet;
  final Map<String, dynamic>? stacksMetadata;

  StacksWallet({
    required super.id,
    required super.name,
    required super.ownerDid,
    required this.stacksAddress,
    required super.createdAt,
    super.updatedAt,
    super.metadata,
    super.encryptedData,
    super.permissions,
    this.isTestnet = false,
    this.stacksMetadata,
  }) : super(
          type: 'stacks',
          address: stacksAddress,
        );

  factory StacksWallet.create({
    required String name,
    required String ownerDid,
    required String stacksAddress,
    bool isTestnet = false,
    Map<String, dynamic>? metadata,
    Map<String, dynamic>? stacksMetadata,
  }) {
    return StacksWallet(
      id: DateTime.now().millisecondsSinceEpoch.toString(),
      name: name,
      ownerDid: ownerDid,
      stacksAddress: stacksAddress,
      createdAt: DateTime.now(),
      metadata: metadata,
      stacksMetadata: stacksMetadata,
      isTestnet: isTestnet,
    );
  }

  @override
  Map<String, dynamic> toJson() => _$StacksWalletToJson(this);

  factory StacksWallet.fromJson(Map<String, dynamic> json) =>
      _$StacksWalletFromJson(json);

  @override
  StacksWallet copyWith({
    String? id,
    String? name,
    String? ownerDid,
    String? stacksAddress,
    Map<String, dynamic>? metadata,
    Map<String, dynamic>? stacksMetadata,
    bool? isTestnet,
    DateTime? createdAt,
    DateTime? updatedAt,
    String? encryptedData,
    List<String>? permissions,
  }) {
    return StacksWallet(
      id: id ?? this.id,
      name: name ?? this.name,
      ownerDid: ownerDid ?? this.ownerDid,
      stacksAddress: stacksAddress ?? this.stacksAddress,
      metadata: metadata ?? this.metadata,
      stacksMetadata: stacksMetadata ?? this.stacksMetadata,
      isTestnet: isTestnet ?? this.isTestnet,
      createdAt: createdAt ?? this.createdAt,
      updatedAt: updatedAt ?? DateTime.now(),
      encryptedData: encryptedData ?? this.encryptedData,
      permissions: permissions ?? this.permissions,
    );
  }
}
