import 'dart:typed_data';
import 'package:equatable/equatable.dart';

/// Bitcoin network types
enum BitcoinNetwork {
  mainnet,
  testnet,
  regtest,
}

/// Transaction priority levels
enum TransactionPriority {
  low,
  medium,
  high,
}

/// Address types supported by the wallet
enum AddressType {
  legacy, // P2PKH
  segwit, // P2SH-P2WPKH
  nativeSegwit, // P2WPKH
  taproot, // P2TR
}

/// Represents a Bitcoin UTXO
class Utxo extends Equatable {
  final String txid;
  final int vout;
  final String address;
  final String script;
  final int value;
  final int height;
  final bool coinbase;

  const Utxo({
    required this.txid,
    required this.vout,
    required this.address,
    required this.script,
    required this.value,
    required this.height,
    this.coinbase = false,
  });

  @override
  List<Object?> get props =>
      [txid, vout, address, script, value, height, coinbase];
}

/// Represents a Bitcoin transaction
class BitcoinTransaction extends Equatable {
  final String txid;
  final List<TxInput> inputs;
  final List<TxOutput> outputs;
  final int version;
  final int locktime;
  final Uint8List? witness;

  const BitcoinTransaction({
    required this.txid,
    required this.inputs,
    required this.outputs,
    this.version = 2,
    this.locktime = 0,
    this.witness,
  });

  @override
  List<Object?> get props =>
      [txid, inputs, outputs, version, locktime, witness];
}

/// Transaction input
class TxInput extends Equatable {
  final String txid;
  final int vout;
  final String script;
  final int sequence;
  final Uint8List? witness;

  const TxInput({
    required this.txid,
    required this.vout,
    required this.script,
    this.sequence = 0xffffffff,
    this.witness,
  });

  @override
  List<Object?> get props => [txid, vout, script, sequence, witness];
}

/// Transaction output
class TxOutput extends Equatable {
  final int value;
  final String script;
  final String? address;

  const TxOutput({
    required this.value,
    required this.script,
    this.address,
  });

  @override
  List<Object?> get props => [value, script, address];
}

/// Wallet credentials
class WalletCredentials extends Equatable {
  final Uint8List publicKey;
  final Uint8List privateKey;
  final String address;
  final String mnemonic;

  const WalletCredentials({
    required this.publicKey,
    required this.privateKey,
    required this.address,
    required this.mnemonic,
  });

  @override
  List<Object> get props => [publicKey, privateKey, address, mnemonic];
}

/// Wallet balance information
class WalletBalance extends Equatable {
  final int total;
  final int confirmed;
  final int unconfirmed;
  final int locked;

  const WalletBalance({
    required this.total,
    required this.confirmed,
    required this.unconfirmed,
    required this.locked,
  });

  @override
  List<Object> get props => [total, confirmed, unconfirmed, locked];
}
