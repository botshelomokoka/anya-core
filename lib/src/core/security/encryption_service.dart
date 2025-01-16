import 'dart:convert';
import 'dart:typed_data';

import 'package:crypto/crypto.dart';
import 'package:encrypt/encrypt.dart';

import '../errors/security_errors.dart';
import '../models/wallet.dart';

class EncryptionService {
  final Key _masterKey;
  final IV _iv;

  EncryptionService()
      : _masterKey = Key.fromSecureRandom(32),
        _iv = IV.fromSecureRandom(16);

  Future<String> encryptWallets(
    List<Wallet> wallets,
    String password,
  ) async {
    try {
      final encrypter = _getEncrypter(password);
      final data = jsonEncode(wallets.map((w) => w.toJson()).toList());

      final encrypted = encrypter.encrypt(data, iv: _iv);
      return encrypted.base64;
    } catch (e) {
      throw SecurityError('Failed to encrypt wallets: $e');
    }
  }

  Future<List<Wallet>> decryptWallets(
    String encryptedData,
    String password,
  ) async {
    try {
      final encrypter = _getEncrypter(password);
      final encrypted = Encrypted.fromBase64(encryptedData);

      final decrypted = encrypter.decrypt(encrypted, iv: _iv);
      final data = jsonDecode(decrypted) as List;

      return data.map((json) => Wallet.fromJson(json)).toList();
    } catch (e) {
      throw SecurityError('Failed to decrypt wallets: $e');
    }
  }

  Future<void> encryptWalletData(Wallet wallet) async {
    try {
      final encrypter = _getEncrypter(_masterKey.base64);
      final data = jsonEncode(wallet.metadata);

      final encrypted = encrypter.encrypt(data, iv: _iv);
      wallet.copyWith(encryptedData: encrypted.base64);
    } catch (e) {
      throw SecurityError('Failed to encrypt wallet data: $e');
    }
  }

  Future<Map<String, dynamic>> decryptWalletData(Wallet wallet) async {
    try {
      if (wallet.encryptedData == null) {
        return {};
      }

      final encrypter = _getEncrypter(_masterKey.base64);
      final encrypted = Encrypted.fromBase64(wallet.encryptedData!);

      final decrypted = encrypter.decrypt(encrypted, iv: _iv);
      return jsonDecode(decrypted) as Map<String, dynamic>;
    } catch (e) {
      throw SecurityError('Failed to decrypt wallet data: $e');
    }
  }

  Encrypter _getEncrypter(String password) {
    final key = _deriveKey(password);
    return Encrypter(AES(key));
  }

  Key _deriveKey(String password) {
    final bytes = utf8.encode(password);
    final hash = sha256.convert(bytes);
    return Key(hash.bytes as Uint8List);
  }
}
