/// Represents account information from the Stacks blockchain
class StacksAccount {
  final String address;
  final int nonce;
  final String balance;
  final List<String> assets;

  StacksAccount({
    required this.address,
    required this.nonce,
    required this.balance,
    required this.assets,
  });

  factory StacksAccount.fromJson(Map<String, dynamic> json) {
    return StacksAccount(
      address: json['address'] as String,
      nonce: json['nonce'] as int,
      balance: json['balance'] as String,
      assets: List<String>.from(json['assets'] ?? []),
    );
  }

  Map<String, dynamic> toJson() => {
    'address': address,
    'nonce': nonce,
    'balance': balance,
    'assets': assets,
  };
}

/// Represents a transaction on the Stacks blockchain
class StacksTransaction {
  final String txId;
  final String status;
  final int blockHeight;
  final String blockHash;
  final String burnBlockTime;

  StacksTransaction({
    required this.txId,
    required this.status,
    required this.blockHeight,
    required this.blockHash,
    required this.burnBlockTime,
  });

  factory StacksTransaction.fromJson(Map<String, dynamic> json) {
    return StacksTransaction(
      txId: json['tx_id'] as String,
      status: json['tx_status'] as String,
      blockHeight: json['block_height'] as int,
      blockHash: json['block_hash'] as String,
      burnBlockTime: json['burn_block_time'] as String,
    );
  }

  Map<String, dynamic> toJson() => {
    'tx_id': txId,
    'tx_status': status,
    'block_height': blockHeight,
    'block_hash': blockHash,
    'burn_block_time': burnBlockTime,
  };
}

/// Represents a smart contract on the Stacks blockchain
class StacksContract {
  final String contractId;
  final String source;
  final bool published;

  StacksContract({
    required this.contractId,
    required this.source,
    required this.published,
  });

  factory StacksContract.fromJson(Map<String, dynamic> json) {
    return StacksContract(
      contractId: json['contract_id'] as String,
      source: json['source'] as String,
      published: json['published'] as bool,
    );
  }

  Map<String, dynamic> toJson() => {
    'contract_id': contractId,
    'source': source,
    'published': published,
  };
}

/// Represents fee estimation for Stacks transactions
class StacksFeeEstimate {
  final int fee;
  final String feeRate;
  final int estimatedCost;

  StacksFeeEstimate({
    required this.fee,
    required this.feeRate,
    required this.estimatedCost,
  });

  factory StacksFeeEstimate.fromJson(Map<String, dynamic> json) {
    return StacksFeeEstimate(
      fee: json['fee'] as int,
      feeRate: json['fee_rate'] as String,
      estimatedCost: json['estimated_cost'] as int,
    );
  }

  Map<String, dynamic> toJson() => {
    'fee': fee,
    'fee_rate': feeRate,
    'estimated_cost': estimatedCost,
  };
}
