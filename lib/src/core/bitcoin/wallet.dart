import 'dart:async';
import 'dart:typed_data';
import 'package:web5/web5.dart';
import 'package:bitcoindart/bitcoindart.dart';
import '../models/wallet.dart';
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
    _walletData = await _createWalletRecord(name, ownerDid);
  }

  /// Create wallet record in Web5 storage
  Future<Wallet> _createWalletRecord(String name, String ownerDid) async {
    final wallet = Wallet.create(
      name: name,
      type: 'bitcoin',
      ownerDid: ownerDid,
      address: _hdWallet.address,
      metadata: {
        'network': _network.toString(),
        'xpub': _hdWallet.base58,
        'addressType': 'p2wpkh', // Default to native segwit
      },
      encryptedData: await _encryptSeedData(),
    );

    final id = await _repository.createWallet(wallet);
    return wallet.copyWith(id: id);
  }

  /// Encrypt sensitive wallet data
  Future<String> _encryptSeedData() async {
    final data = {
      'seed': _hdWallet.seed.toString(),
      'privateKey': _hdWallet.privKey,
      'mnemonic': _hdWallet.mnemonic,
    };

    // Encrypt using Web5's encryption
    final encrypted = await _web5.encrypt(
      data: data,
      recipients: [_walletData?.ownerDid ?? ''],
    );

    return encrypted;
  }

  /// Get wallet balance
  Future<int> getBalance() async {
    // Implement balance check using bitcoindart
    return 0;
  }

  /// Create transaction
  Future<Transaction> createTransaction({
    required String toAddress,
    required int amount,
    int? feeRate,
  }) async {
    // Verify wallet is initialized
    if (_walletData == null) {
      throw WalletNotInitializedException();
    }

    // Create and sign transaction
    final tx = Transaction(network: _network)
      ..from(await _getUtxos())
      ..to(toAddress, amount);

    if (feeRate != null) {
      tx.feePerByte(feeRate);
    }

    // Sign the transaction
    tx.sign(_hdWallet.privKey);

    // Store transaction in Web5
    await _storeTransaction(tx);

    return tx;
  }

  /// Store transaction in Web5
  Future<void> _storeTransaction(Transaction tx) async {
    final txData = {
      'txid': tx.id,
      'walletId': _walletData?.id,
      'timestamp': DateTime.now().toIso8601String(),
      'hex': tx.toHex(),
      'status': 'pending',
    };

    await _web5.dwn.records.create(
      data: txData,
      message: {
        'schema': 'anya/bitcoin/transaction',
        'dataFormat': 'application/json',
      },
    );
  }

  /// Get UTXOs for wallet
  Future<List<Utxo>> _getUtxos() async {
    // Implement UTXO fetching
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
