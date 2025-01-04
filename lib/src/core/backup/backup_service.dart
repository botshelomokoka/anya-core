import 'dart:async';
import 'dart:convert';

import 'package:freezed_annotation/freezed_annotation.dart';
import 'package:json_annotation/json_annotation.dart';
import 'package:web5/web5.dart';

import '../errors/service_errors.dart';
import '../models/wallet.dart';
import '../security/encryption_service.dart';

part 'backup_service.g.dart';
part 'backup_service.freezed.dart';

/// Backup format version
const int backupVersion = 1;

/// Backup data structure
@freezed
class WalletBackup with _$WalletBackup {
  const factory WalletBackup({
    required int version,
    required String timestamp,
    required String ownerDid,
    required List<Map<String, dynamic>> wallets,
    required Map<String, dynamic> metadata,
  }) = _WalletBackup;

  factory WalletBackup.fromJson(Map<String, dynamic> json) =>
      _$WalletBackupFromJson(json);
}

/// Backup service for wallet data
class BackupService {
  final Web5 _web5;
  final EncryptionService _encryption;

  BackupService(this._web5, this._encryption);

  /// Create encrypted backup of wallets
  Future<String> createBackup({
    required List<Wallet> wallets,
    required String ownerDid,
    required String password,
    Map<String, dynamic>? metadata,
  }) async {
    try {
      // Prepare wallet data for backup
      final walletData = await Future.wait(
        wallets.map((wallet) async {
          final Map<String, dynamic> data = wallet.toJson();

          // Include additional wallet-specific data
          switch (wallet.type) {
            case WalletType.bitcoin:
              data['utxos'] = await _getWalletUTXOs(wallet);
              data['transactions'] = await _getWalletTransactions(wallet);
              break;
            case WalletType.lightning:
              data['channels'] = await _getChannelBackups(wallet);
              break;
            default:
              // No additional data needed for other wallet types
              break;
          }

          return data;
        }),
      );

      // Create backup structure
      final backup = WalletBackup(
        version: backupVersion,
        timestamp: DateTime.now().toIso8601String(),
        ownerDid: ownerDid,
        wallets: walletData,
        metadata: metadata ?? {},
      );

      // Encrypt backup data
      final encrypted = await _encryption.encrypt(
        jsonEncode(backup.toJson()),
        password,
      );

      // Store backup in Web5
      await _storeBackup(encrypted, ownerDid);

      return encrypted;
    } on EncryptionException catch (e) {
      throw BackupServiceError('Encryption failed: ${e.message}');
    } on Web5Exception catch (e) {
      throw BackupServiceError('Web5 storage failed: ${e.message}');
    } catch (e) {
      throw BackupServiceError('Failed to create backup: $e');
    }
  }

  /// Restore wallets from encrypted backup
  Future<List<Wallet>> restoreBackup({
    required String encryptedBackup,
    required String password,
    required String ownerDid,
  }) async {
    try {
      // Decrypt backup data
      final decrypted = await _encryption.decrypt(encryptedBackup, password);
      final Map<String, dynamic> jsonData = jsonDecode(decrypted) as Map<String, dynamic>;
      final backup = WalletBackup.fromJson(jsonData);

      // Verify backup version and ownership
      if (backup.version > backupVersion) {
        throw BackupServiceError('Unsupported backup version');
      }
      if (backup.ownerDid != ownerDid) {
        throw BackupServiceError('Invalid backup ownership');
      }

      // Restore wallets
      return Future.wait(
        backup.wallets.map((data) async {
          final wallet = Wallet.fromJson(data);

          // Restore additional wallet-specific data
          switch (wallet.type) {
            case WalletType.bitcoin:
              await _restoreWalletUTXOs(wallet, data['utxos'] as List<Map<String, dynamic>>);
              await _restoreWalletTransactions(wallet, data['transactions'] as List<Map<String, dynamic>>);
              break;
            case WalletType.lightning:
              await _restoreChannelBackups(wallet, data['channels'] as List<Map<String, dynamic>>);
              break;
            default:
              // No additional data to restore for other wallet types
              break;
          }

          return wallet;
        }),
      );
    } on EncryptionException catch (e) {
      throw BackupServiceError('Decryption failed: ${e.message}');
    } on FormatException catch (e) {
      throw BackupServiceError('Invalid backup format: ${e.message}');
    } catch (e) {
      throw BackupServiceError('Failed to restore backup: $e');
    }
  }

  /// List available backups for a user
  Future<List<BackupMetadata>> listBackups(String ownerDid) async {
    try {
      final records = await _web5.records.query({
        'message.type': 'wallet-backup',
        'message.owner': ownerDid,
      });

      return records.map((record) => BackupMetadata.fromJson(record)).toList();
    } on Web5Exception catch (e) {
      throw BackupServiceError('Failed to list backups: ${e.message}');
    }
  }

  // Private helper methods
  Future<List<Map<String, dynamic>>> _getWalletUTXOs(Wallet wallet) async {
    // Implementation
    throw UnimplementedError();
  }

  Future<List<Map<String, dynamic>>> _getWalletTransactions(Wallet wallet) async {
    // Implementation
    throw UnimplementedError();
  }

  Future<List<Map<String, dynamic>>> _getChannelBackups(Wallet wallet) async {
    // Implementation
    throw UnimplementedError();
  }

  Future<void> _restoreWalletUTXOs(
    Wallet wallet,
    List<Map<String, dynamic>> utxos,
  ) async {
    // Implementation
    throw UnimplementedError();
  }

  Future<void> _restoreWalletTransactions(
    Wallet wallet,
    List<Map<String, dynamic>> transactions,
  ) async {
    // Implementation
    throw UnimplementedError();
  }

  Future<void> _restoreChannelBackups(
    Wallet wallet,
    List<Map<String, dynamic>> channels,
  ) async {
    // Implementation
    throw UnimplementedError();
  }

  Future<void> _storeBackup(String encrypted, String ownerDid) async {
    await _web5.records.create({
      'message': {
        'type': 'wallet-backup',
        'owner': ownerDid,
        'data': encrypted,
        'timestamp': DateTime.now().toIso8601String(),
      }
    });
  }
}

/// Error thrown by BackupService
class BackupServiceError implements Exception {
  final String message;
  
  const BackupServiceError(this.message);
  
  @override
  String toString() => 'BackupServiceError: $message';
}
