import 'package:json_annotation/json_annotation.dart';

part 'stacks_transaction.g.dart';

@JsonSerializable()
class StacksTransaction {
  final String from;
  final String to;
  final int amount;
  final String? memo;
  final int? nonce;
  final int? fee;

  StacksTransaction({
    required this.from,
    required this.to,
    required this.amount,
    this.memo,
    this.nonce,
    this.fee,
  });

  factory StacksTransaction.fromJson(Map<String, dynamic> json) =>
      _$StacksTransactionFromJson(json);

  Map<String, dynamic> toJson() => _$StacksTransactionToJson(this);
}
