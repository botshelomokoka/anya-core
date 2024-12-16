import 'dart:async';
import 'dart:typed_data';
import 'package:web5/web5.dart';
import 'package:bitcoindart/bitcoindart.dart';
import '../models/wallet.dart';
import '../models/transaction.dart';
import '../repositories/wallet_repository.dart';
import '../storage/dwn_store.dart';

/// Bitcoin wallet implementation with Web5 integration
class BitcoinWallet {
  final WalletRepository _repository;
  final Web5 _web5;
  final NetworkType _network;
  
  late final HDWallet _hdWallet;
  Wallet? _walletData;

  BitcoinWallet(this._repository, this._web5, this._network);

  /// Initialize wallet from seed or create new
  Future<void> initialize({
    String? seed,
    String? mnemonic,
    required String name,
    required String ownerDid,
    String addressType = 'p2wpkh', // Default to native segwit
  }) async {
    if (seed != null) {
      _hdWallet = HDWallet.fromSeed(
        Uint8List.fromList(seed.codeUnits),
        network: _network,
      );
    } else if (mnemonic != null) {
      _hdWallet = HDWallet.fromMnemonic(mnemonic, network: _network);
    } else {
      _hdWallet = HDWallet.random(network: _network);
    }

    // Create wallet record in Web5
    _walletData = await _createWalletRecord(name, ownerDid, addressType);
  }

  /// Create wallet record in Web5 storage
  Future<Wallet> _createWalletRecord(String name, String ownerDid, String addressType) async {
    final derivationPath = _getDerivationPath(addressType);
    final address = await _deriveAddress(addressType);

    final wallet = Wallet.create(
      name: name,
      type: 'bitcoin',
      ownerDid: ownerDid,
      address: address,
      metadata: {
        'network': _network.toString(),
        'xpub': _hdWallet.base58,
        'addressType': addressType,
        'derivationPath': derivationPath,
        'scriptType': _getScriptType(addressType),
        'isLightningEnabled': true,
        'isRgbEnabled': true,
      },
      encryptedData: await _encryptSeedData(),
    );

    final id = await _repository.createWallet(wallet);
    return wallet.copyWith(id: id);
  }

  String _getDerivationPath(String addressType) {
    switch (addressType) {
      case 'p2wpkh':
        return "m/84'/0'/0'/0/0"; // Native SegWit
      case 'p2sh-p2wpkh':
        return "m/49'/0'/0'/0/0"; // Nested SegWit
      case 'p2pkh':
        return "m/44'/0'/0'/0/0"; // Legacy
      default:
        return "m/84'/0'/0'/0/0"; // Default to Native SegWit
    }
  }

  Future<String> _deriveAddress(String addressType) async {
    final pubKey = _hdWallet.pubKey;
    switch (addressType) {
      case 'p2wpkh':
        return P2WPKH(pubKey: pubKey, network: _network).address;
      case 'p2sh-p2wpkh':
        return P2SH(P2WPKH(pubKey: pubKey, network: _network)).address;
      case 'p2pkh':
        return P2PKH(pubKey: pubKey, network: _network).address;
      default:
        return P2WPKH(pubKey: pubKey, network: _network).address;
    }
  }

  String _getScriptType(String addressType) {
    switch (addressType) {
      case 'p2wpkh':
        return 'witness_v0_keyhash';
      case 'p2sh-p2wpkh':
        return 'p2sh-witness_v0_keyhash';
      case 'p2pkh':
        return 'pubkeyhash';
      default:
        return 'witness_v0_keyhash';
    }
  }

  /// Encrypt sensitive wallet data
  Future<String> _encryptSeedData() async {
    final data = {
      'seed': _hdWallet.seed.toString(),
      'privateKey': _hdWallet.privKey,
      'mnemonic': _hdWallet.mnemonic,
      'masterFingerprint': _hdWallet.fingerprint,
    };

    // Encrypt using Web5's encryption
    final encrypted = await _web5.encrypt(
      data: data,
      recipients: [_walletData?.ownerDid ?? ''],
    );

    return encrypted;
  }

  /// Get wallet balance
  Future<WalletBalance> getBalance() async {
    if (_walletData == null) {
      throw WalletNotInitializedException();
    }

    final utxos = await _getUtxos();
    int confirmed = 0;
    int unconfirmed = 0;
    int lightning = 0;

    for (final utxo in utxos) {
      if (utxo.confirmations >= 6) {
        confirmed += utxo.value;
      } else {
        unconfirmed += utxo.value;
      }
    }

    // Get Lightning balance if enabled
    if (_walletData!.metadata['isLightningEnabled'] == true) {
      lightning = await _getLightningBalance();
    }

    return WalletBalance(
      confirmed: confirmed,
      unconfirmed: unconfirmed,
      lightning: lightning,
      total: confirmed + unconfirmed + lightning,
    );
  }

  /// Create transaction
  Future<Transaction> createTransaction({
    required String toAddress,
    required int amount,
    TransactionPriority priority = TransactionPriority.medium,
    Map<String, dynamic>? metadata,
  }) async {
    if (_walletData == null) {
      throw WalletNotInitializedException();
    }

    // Check if this is a Lightning payment
    if (toAddress.startsWith('lightning:') && _walletData!.metadata['isLightningEnabled'] == true) {
      return await _createLightningPayment(toAddress, amount);
    }

    // Check if this is an RGB transfer
    if (metadata?['rgbAssetId'] != null && _walletData!.metadata['isRgbEnabled'] == true) {
      return await _createRgbTransfer(toAddress, amount, metadata!);
    }

    // Regular on-chain transaction
    final feeRate = _getFeeRate(priority);
    final utxos = await _getUtxos();
    
    // Create and sign transaction
    final tx = Transaction(network: _network)
      ..from(utxos)
      ..to(toAddress, amount)
      ..feePerByte(feeRate);

    // Sign the transaction
    tx.sign(_hdWallet.privKey);

    // Store transaction in Web5
    final storedTx = await _storeTransaction(tx, 'onchain');

    return storedTx;
  }

  int _getFeeRate(TransactionPriority priority) {
    switch (priority) {
      case TransactionPriority.low:
        return 1; // 1 sat/byte
      case TransactionPriority.medium:
        return 5; // 5 sats/byte
      case TransactionPriority.high:
        return 10; // 10 sats/byte
      default:
        return 5;
    }
  }

  Future<Transaction> _createLightningPayment(String invoice, int amount) async {
    // Implement Lightning payment logic
    throw UnimplementedError('Lightning payments not implemented yet');
  }

  Future<Transaction> _createRgbTransfer(String toAddress, int amount, Map<String, dynamic> metadata) async {
    // Implement RGB transfer logic
    throw UnimplementedError('RGB transfers not implemented yet');
  }

  Future<int> _getLightningBalance() async {
    // Implement Lightning balance check
    return 0;
  }

  /// Store transaction in Web5
  Future<Transaction> _storeTransaction(Transaction tx, String type) async {
    final txData = {
      'txid': tx.id,
      'walletId': _walletData?.id,
      'timestamp': DateTime.now().toIso8601String(),
      'hex': tx.toHex(),
      'status': 'pending',
      'type': type,
      'fee': tx.fee,
      'vsize': tx.virtualSize,
    };

    await _web5.dwn.records.create(
      data: txData,
      message: {
        'schema': 'anya/bitcoin/transaction',
        'dataFormat': 'application/json',
      },
    );

    return Transaction(
      id: tx.id,
      type: TransactionType.send,
      fromAddress: _walletData!.address,
      toAddress: tx.outputs.first.address,
      amount: tx.outputs.first.value.toDouble(),
      chain: 'Bitcoin',
      symbol: 'BTC',
      timestamp: DateTime.now(),
      status: TransactionStatus.pending,
      feeAmount: tx.fee.toDouble(),
      feeSymbol: 'BTC',
      metadata: txData,
    );
  }

  /// Get UTXOs for wallet
  Future<List<Utxo>> _getUtxos() async {
    // Implement UTXO fetching using bitcoindart
    return [];
  }

  /// Export wallet data
  Future<Map<String, dynamic>> export({bool includePrivateData = false}) async {
    if (_walletData == null) {
      throw WalletNotInitializedException();
    }

    final exportData = {
      'id': _walletData!.id,
      'name': _walletData!.name,
      'address': _walletData!.address,
      'network': _network.toString(),
      'metadata': _walletData!.metadata,
    };

    if (includePrivateData) {
      // Decrypt and include private data
      final decrypted = await _web5.decrypt(_walletData!.encryptedData!);
      exportData['privateData'] = decrypted;
    }

    return exportData;
  }

  /// Import wallet data
  Future<void> import(Map<String, dynamic> data) async {
    // Verify data format
    _verifyImportData(data);

    // Initialize wallet
    await initialize(
      name: data['name'],
      ownerDid: data['ownerDid'],
      seed: data['privateData']?['seed'],
      mnemonic: data['privateData']?['mnemonic'],
      addressType: data['metadata']?['addressType'] ?? 'p2wpkh',
    );
  }

  void _verifyImportData(Map<String, dynamic> data) {
    final requiredFields = ['name', 'ownerDid'];
    for (final field in requiredFields) {
      if (!data.containsKey(field)) {
        throw InvalidWalletDataException('Missing required field: $field');
      }
    }
  }
}

class WalletBalance {
  final int confirmed;
  final int unconfirmed;
  final int lightning;
  final int total;

  WalletBalance({
    required this.confirmed,
    required this.unconfirmed,
    this.lightning = 0,
    required this.total,
  });
}

class WalletNotInitializedException implements Exception {
  final String message = 'Wallet not initialized';
  @override
  String toString() => message;
}

class InvalidWalletDataException implements Exception {
  final String message;
  InvalidWalletDataException(this.message);
  @override
  String toString() => message;
}
