// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'stacks_transaction.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

StacksTransaction _$StacksTransactionFromJson(Map<String, dynamic> json) =>
    StacksTransaction(
      from: json['from'] as String,
      to: json['to'] as String,
      amount: (json['amount'] as num).toInt(),
      memo: json['memo'] as String?,
      nonce: (json['nonce'] as num?)?.toInt(),
      fee: (json['fee'] as num?)?.toInt(),
    );

Map<String, dynamic> _$StacksTransactionToJson(StacksTransaction instance) =>
    <String, dynamic>{
      'from': instance.from,
      'to': instance.to,
      'amount': instance.amount,
      'memo': instance.memo,
      'nonce': instance.nonce,
      'fee': instance.fee,
    };
