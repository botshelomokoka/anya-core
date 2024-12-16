import 'package:intl/intl.dart';

enum TransactionType {
  send,
  receive,
  swap,
  lightning,     // Lightning Network payment
  rgbTransfer,   // RGB asset transfer
  rskContract,   // RSK smart contract interaction
  stacksCall,    // Stacks contract call
  bankTransfer,  // Open Banking transfer
}

enum TransactionStatus {
  pending,
  completed,
  failed,
  confirming,    // Waiting for Bitcoin confirmations
  routing,       // Lightning Network routing
  settling,      // Cross-chain settlement
}

enum TransactionPriority {
  low,      // Lower fee, slower confirmation
  medium,   // Standard fee
  high,     // Higher fee, faster confirmation
}

class Transaction {
  final String id;
  final TransactionType type;
  final String fromAddress;
  final String toAddress;
  final double amount;
  final String chain;
  final String? symbol;
  final DateTime timestamp;
  final TransactionStatus status;
  final Map<String, dynamic>? metadata;
  final double? feeAmount;
  final String? feeSymbol;
  final int? confirmations;
  final TransactionPriority? priority;
  final String? lightningInvoice;    // Lightning payment invoice
  final String? contractAddress;      // For RSK/Stacks contract interactions
  final String? rgbAssetId;          // RGB asset identifier
  final String? bankReference;        // Banking transfer reference

  Transaction({
    required this.id,
    required this.type,
    required this.fromAddress,
    required this.toAddress,
    required this.amount,
    required this.chain,
    this.symbol,
    required this.timestamp,
    required this.status,
    this.metadata,
    this.feeAmount,
    this.feeSymbol,
    this.confirmations,
    this.priority,
    this.lightningInvoice,
    this.contractAddress,
    this.rgbAssetId,
    this.bankReference,
  });

  String get formattedAmount {
    final numberFormat = NumberFormat.currency(
      symbol: symbol ?? '',
      decimalDigits: chain == 'OpenBanking' ? 2 : 8,
    );
    return numberFormat.format(amount);
  }

  String get formattedFee {
    if (feeAmount == null) return '';
    final numberFormat = NumberFormat.currency(
      symbol: feeSymbol ?? '',
      decimalDigits: 8,
    );
    return numberFormat.format(feeAmount!);
  }

  String get formattedDate {
    return DateFormat.yMMMd().add_jm().format(timestamp);
  }

  String get statusString {
    switch (status) {
      case TransactionStatus.confirming:
        return 'Confirming ($confirmations/6)';
      case TransactionStatus.routing:
        return 'Routing Payment';
      case TransactionStatus.settling:
        return 'Settling';
      default:
        return status.toString().split('.').last;
    }
  }

  bool get isOutgoing => type == TransactionType.send;

  bool get isLightning => type == TransactionType.lightning;

  bool get isDeFi => type == TransactionType.rgbTransfer || 
                     type == TransactionType.rskContract || 
                     type == TransactionType.stacksCall;

  bool get isBanking => type == TransactionType.bankTransfer;

  bool get requiresConfirmation => chain == 'Bitcoin' || isDeFi;

  String get displayAddress {
    if (isLightning) return 'Lightning Network';
    if (isBanking) return bankReference ?? toAddress;
    return toAddress;
  }

  factory Transaction.fromJson(Map<String, dynamic> json) {
    return Transaction(
      id: json['id'] as String,
      type: TransactionType.values.firstWhere(
        (e) => e.toString().split('.').last == json['type'],
      ),
      fromAddress: json['fromAddress'] as String,
      toAddress: json['toAddress'] as String,
      amount: json['amount'] as double,
      chain: json['chain'] as String,
      symbol: json['symbol'] as String?,
      timestamp: DateTime.parse(json['timestamp'] as String),
      status: TransactionStatus.values.firstWhere(
        (e) => e.toString().split('.').last == json['status'],
      ),
      metadata: json['metadata'] as Map<String, dynamic>?,
      feeAmount: json['feeAmount'] as double?,
      feeSymbol: json['feeSymbol'] as String?,
      confirmations: json['confirmations'] as int?,
      priority: json['priority'] != null 
        ? TransactionPriority.values.firstWhere(
            (e) => e.toString().split('.').last == json['priority'],
          )
        : null,
      lightningInvoice: json['lightningInvoice'] as String?,
      contractAddress: json['contractAddress'] as String?,
      rgbAssetId: json['rgbAssetId'] as String?,
      bankReference: json['bankReference'] as String?,
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'id': id,
      'type': type.toString().split('.').last,
      'fromAddress': fromAddress,
      'toAddress': toAddress,
      'amount': amount,
      'chain': chain,
      'symbol': symbol,
      'timestamp': timestamp.toIso8601String(),
      'status': status.toString().split('.').last,
      'metadata': metadata,
      'feeAmount': feeAmount,
      'feeSymbol': feeSymbol,
      'confirmations': confirmations,
      'priority': priority?.toString().split('.').last,
      'lightningInvoice': lightningInvoice,
      'contractAddress': contractAddress,
      'rgbAssetId': rgbAssetId,
      'bankReference': bankReference,
    };
  }
}
