import 'dart:async';
import 'dart:convert';
import 'package:web5/web5.dart';
import '../models/wallet.dart';
import '../security/encryption_service.dart';
import '../errors/service_errors.dart';

/// Backup format version
const int BACKUP_VERSION = 1;

/// Backup data structure
class WalletBackup {
  final int version;
  final String timestamp;
  final String ownerDid;
  final List<Map<String, dynamic>> wallets;
  final Map<String, dynamic> metadata;

  WalletBackup({
    required this.version,
    required this.timestamp,
    required this.ownerDid,
    required this.wallets,
    required this.metadata,
  });

  factory WalletBackup.fromJson(Map<String, dynamic> json) {
    return WalletBackup(
      version: json['version'],
      timestamp: json['timestamp'],
      ownerDid: json['ownerDid'],
      wallets: List<Map<String, dynamic>>.from(json['wallets']),
      metadata: Map<String, dynamic>.from(json['metadata']),
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'version': version,
      'timestamp': timestamp,
      'ownerDid': ownerDid,
      'wallets': wallets,
      'metadata': metadata,
    };
  }
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
          final data = wallet.toJson();

          // Include additional wallet-specific data
          if (wallet.type == 'bitcoin') {
            data['utxos'] = await _getWalletUTXOs(wallet);
            data['transactions'] = await _getWalletTransactions(wallet);
          } else if (wallet.type == 'lightning') {
            data['channels'] = await _getChannelBackups(wallet);
          }

          return data;
        }),
      );

      // Create backup structure
      final backup = WalletBackup(
        version: BACKUP_VERSION,
        timestamp: DateTime.now().toIso8601String(),
        ownerDid: ownerDid,
        wallets: walletData,
        metadata: metadata ?? {},
      );

      // Encrypt backup data
      final encrypted = await _encryption.encryptData(
        jsonEncode(backup.toJson()),
        password,
      );

      // Store backup in Web5
      await _storeBackup(encrypted, ownerDid);

      return encrypted;
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
      final decrypted =
          await _encryption.decryptData(encryptedBackup, password);
      final backup = WalletBackup.fromJson(jsonDecode(decrypted));

      // Verify backup version and ownership
      if (backup.version > BACKUP_VERSION) {
        throw BackupServiceError('Unsupported backup version');
      }
      if (backup.ownerDid != ownerDid) {
        throw BackupServiceError('Invalid backup ownership');
      }

      // Restore wallets
      final restoredWallets = await Future.wait(
        backup.wallets.map((data) async {
          final wallet = Wallet.fromJson(data);

          // Restore additional wallet-specific data
          if (wallet.type == 'bitcoin') {
            await _restoreWalletUTXOs(wallet, data['utxos']);
            await _restoreWalletTransactions(wallet, data['transactions']);
          } else if (wallet.type == 'lightning') {
            await _restoreChannelBackups(wallet, data['channels']);
          }

          return wallet;
        }),
      );

      return restoredWallets;
    } catch (e) {
      throw BackupServiceError('Failed to restore backup: $e');
    }
  }

  /// List available backups for a user
  Future<List<Map<String, dynamic>>> listBackups(String ownerDid) async {
    try {
      final records = await _web5.dwn.records.query({
        'message': {
          'filter': {
            'schema': 'anya/wallet/backup',
            'recipient': ownerDid,
          },
        },
      });

      return records
          .map((record) => {
                'id': record.id,
                'timestamp': record.dateCreated,
                'size': record.data.length,
              })
          .toList();
    } catch (e) {
      throw BackupServiceError('Failed to list backups: $e');
    }
  }

  /// Delete a backup
  Future<void> deleteBackup(String backupId, String ownerDid) async {
    try {
      await _web5.dwn.records.delete({
        'message': {
          'recordId': backupId,
          'authorization': {
            'did': ownerDid,
          },
        },
      });
    } catch (e) {
      throw BackupServiceError('Failed to delete backup: $e');
    }
  }

  /// Store encrypted backup in Web5
  Future<void> _storeBackup(String encryptedData, String ownerDid) async {
    try {
      await _web5.dwn.records.create({
        'data': encryptedData,
        'message': {
          'schema': 'anya/wallet/backup',
          'dataFormat': 'application/json',
          'recipient': ownerDid,
        },
      });
    } catch (e) {
      throw BackupServiceError('Failed to store backup: $e');
    }
  }

  /// Get UTXO data for backup
  Future<List<Map<String, dynamic>>> _getWalletUTXOs(Wallet wallet) async {
    // Implementation would get UTXO data from UTXOService
    return [];
  }

  /// Get transaction history for backup
  Future<List<Map<String, dynamic>>> _getWalletTransactions(
      Wallet wallet) async {
    // Implementation would get transaction history
    return [];
  }

  /// Get Lightning channel backups
  Future<List<Map<String, dynamic>>> _getChannelBackups(Wallet wallet) async {
    // Implementation would get channel backup data
    return [];
  }

  /// Restore UTXO data
  Future<void> _restoreWalletUTXOs(
    Wallet wallet,
    List<Map<String, dynamic>> utxos,
  ) async {
    // Implementation would restore UTXO data
  }

  /// Restore transaction history
  Future<void> _restoreWalletTransactions(
    Wallet wallet,
    List<Map<String, dynamic>> transactions,
  ) async {
    // Implementation would restore transaction history
  }

  /// Restore Lightning channel backups
  Future<void> _restoreChannelBackups(
    Wallet wallet,
    List<Map<String, dynamic>> channels,
  ) async {
    // Implementation would restore channel backups
  }
}

class BackupServiceError implements Exception {
  final String message;
  BackupServiceError(this.message);
  @override
  String toString() => message;
}
