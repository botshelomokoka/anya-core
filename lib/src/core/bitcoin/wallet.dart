import 'dart:typed_data';
import 'package:bip32/bip32.dart' as bip32;
import 'package:bip39/bip39.dart' as bip39;
import 'package:bitcoin_base/bitcoin_base.dart';
import 'package:hex/hex.dart';
import 'package:logging/logging.dart';
import 'package:pointycastle/export.dart' as pc;
import 'package:web5/web5.dart' hide Ed25519;

import 'models.dart';
import 'fee_service.dart';
import 'utxo_service.dart';

/// A Bitcoin wallet implementation supporting multiple address types and HD derivation
class BitcoinWallet {
  final Logger _logger = Logger('BitcoinWallet');
  final FeeService _feeService;
  final UtxoService _utxoService;
  final NetworkType _network;
  
  late bip32.BIP32 _masterKey;
  late String _mnemonic;
  
  BitcoinWallet({
    required NetworkType network,
    FeeService? feeService,
    UtxoService? utxoService,
  }) : _network = network,
       _feeService = feeService ?? FeeService(),
       _utxoService = utxoService ?? UtxoService();

  /// Create a new wallet with a randomly generated seed
  static Future<BitcoinWallet> create({
    NetworkType network = NetworkType.mainnet,
    String? passphrase,
  }) async {
    final wallet = BitcoinWallet(network: network);
    await wallet._generateNewWallet(passphrase);
    return wallet;
  }

  /// Restore a wallet from an existing mnemonic
  static Future<BitcoinWallet> fromMnemonic(
    String mnemonic, {
    NetworkType network = NetworkType.mainnet,
    String? passphrase,
  }) async {
    final wallet = BitcoinWallet(network: network);
    await wallet._restoreFromMnemonic(mnemonic, passphrase);
    return wallet;
  }

  /// Generate a new wallet with random entropy
  Future<void> _generateNewWallet(String? passphrase) async {
    try {
      // Generate random mnemonic
      final entropy = _generateSecureEntropy();
      _mnemonic = bip39.generateMnemonic(entropy: entropy);
      
      // Convert mnemonic to seed
      final seed = await _mnemonicToSeed(_mnemonic, passphrase);
      
      // Generate master key
      _masterKey = _deriveMasterKey(seed);
      
      _logger.info('New wallet generated successfully');
    } catch (e) {
      _logger.severe('Error generating new wallet', e);
      rethrow;
    }
  }

  /// Restore wallet from existing mnemonic
  Future<void> _restoreFromMnemonic(String mnemonic, String? passphrase) async {
    try {
      if (!bip39.validateMnemonic(mnemonic)) {
        throw Exception('Invalid mnemonic');
      }
      
      _mnemonic = mnemonic;
      final seed = await _mnemonicToSeed(mnemonic, passphrase);
      _masterKey = _deriveMasterKey(seed);
      
      _logger.info('Wallet restored successfully from mnemonic');
    } catch (e) {
      _logger.severe('Error restoring wallet from mnemonic', e);
      rethrow;
    }
  }

  /// Generate secure random entropy for mnemonic generation
  List<int> _generateSecureEntropy() {
    final random = pc.SecureRandom('Fortuna')
      ..seed(pc.KeyParameter(
        Uint8List.fromList(DateTime.now().microsecondsSinceEpoch.toRadixString(16).codeUnits),
      ));
    
    return List<int>.generate(32, (i) => random.nextUint8());
  }

  /// Convert mnemonic to seed with optional passphrase
  Future<Uint8List> _mnemonicToSeed(String mnemonic, String? passphrase) async {
    return Uint8List.fromList(
      bip39.mnemonicToSeed(mnemonic, passphrase: passphrase ?? ''),
    );
  }

  /// Derive master key from seed
  bip32.BIP32 _deriveMasterKey(Uint8List seed) {
    return bip32.BIP32.fromSeed(
      seed,
      bip32.NetworkType(
        wif: _network == NetworkType.mainnet ? 0x80 : 0xef,
        bip32: _network == NetworkType.mainnet
            ? bip32.NetworkType.bitcoin.bip32
            : bip32.NetworkType.testnet.bip32,
      ),
    );
  }

  /// Get the mnemonic phrase for backup
  String get mnemonic => _mnemonic;

  /// Get a receiving address for the given derivation path
  Future<String> getAddress(int index, {AddressType type = AddressType.p2wpkh}) async {
    try {
      final keyPair = _deriveKeyPair(index);
      
      switch (type) {
        case AddressType.p2wpkh:
          return _createP2WPKHAddress(keyPair);
        case AddressType.p2sh:
          return _createP2SHAddress(keyPair);
        case AddressType.p2pkh:
          return _createP2PKHAddress(keyPair);
        default:
          throw Exception('Unsupported address type: $type');
      }
    } catch (e) {
      _logger.severe('Error generating address', e);
      rethrow;
    }
  }

  /// Derive key pair at specific index
  ECPair _deriveKeyPair(int index) {
    final child = _masterKey.derive(index);
    return ECPair(
      privateKey: child.privateKey!,
      network: _network == NetworkType.mainnet ? Networks.bitcoin : Networks.testnet,
    );
  }

  /// Create Native SegWit (P2WPKH) address
  String _createP2WPKHAddress(ECPair keyPair) {
    final p2wpkh = P2WPKH(
      pubkey: keyPair.publicKey,
      network: _network == NetworkType.mainnet ? Networks.bitcoin : Networks.testnet,
    );
    return p2wpkh.address;
  }

  /// Create Nested SegWit (P2SH-P2WPKH) address
  String _createP2SHAddress(ECPair keyPair) {
    final p2sh = P2SH(
      redeem: P2WPKH(
        pubkey: keyPair.publicKey,
        network: _network == NetworkType.mainnet ? Networks.bitcoin : Networks.testnet,
      ),
    );
    return p2sh.address;
  }

  /// Create Legacy (P2PKH) address
  String _createP2PKHAddress(ECPair keyPair) {
    final p2pkh = P2PKH(
      pubkey: keyPair.publicKey,
      network: _network == NetworkType.mainnet ? Networks.bitcoin : Networks.testnet,
    );
    return p2pkh.address;
  }
}

/// Supported address types
enum AddressType {
  p2wpkh,  // Native SegWit
  p2sh,    // Nested SegWit
  p2pkh,   // Legacy
}

/// Network type
enum NetworkType {
  mainnet,
  testnet,
}
