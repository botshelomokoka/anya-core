import 'package:flutter_secure_storage/flutter_secure_storage.dart';
import 'package:bip39/bip39.dart' as bip39;

class KeyManagementService {
  static const _storage = FlutterSecureStorage();
  static const _stacksKeyPrefix = 'stacks_key_';
  static const _stacksMnemonicKey = 'stacks_mnemonic';

  Future<String?> getPrivateKey(String address) async {
    try {
      return await _storage.read(key: '$_stacksKeyPrefix$address');
    } catch (e) {
      throw KeyManagementException('Failed to retrieve private key: $e');
    }
  }

  Future<void> storePrivateKey(String address, String privateKey) async {
    try {
      await _storage.write(
        key: '$_stacksKeyPrefix$address',
        value: privateKey,
      );
    } catch (e) {
      throw KeyManagementException('Failed to store private key: $e');
    }
  }

  Future<String> generateMnemonic() async {
    try {
      final mnemonic = bip39.generateMnemonic();
      await _storage.write(key: _stacksMnemonicKey, value: mnemonic);
      return mnemonic;
    } catch (e) {
      throw KeyManagementException('Failed to generate mnemonic: $e');
    }
  }

  Future<String?> getMnemonic() async {
    try {
      return await _storage.read(key: _stacksMnemonicKey);
    } catch (e) {
      throw KeyManagementException('Failed to retrieve mnemonic: $e');
    }
  }

  Future<void> clearKeys() async {
    try {
      final allKeys = await _storage.readAll();
      for (final entry in allKeys.entries) {
        if (entry.key.startsWith(_stacksKeyPrefix)) {
          await _storage.delete(key: entry.key);
        }
      }
    } catch (e) {
      throw KeyManagementException('Failed to clear keys: $e');
    }
  }

  Future<bool> hasStoredKeys() async {
    try {
      final allKeys = await _storage.readAll();
      return allKeys.keys.any((key) => key.startsWith(_stacksKeyPrefix));
    } catch (e) {
      throw KeyManagementException('Failed to check stored keys: $e');
    }
  }
}

class KeyManagementException implements Exception {
  final String message;
  KeyManagementException(this.message);

  @override
  String toString() => 'KeyManagementException: $message';
}
