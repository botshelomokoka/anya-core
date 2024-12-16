import 'package:dio/dio.dart';
import '../models/stacks_transaction.dart';
import 'key_management_service.dart';
import '../models/stacks_wallet.dart';
import '../repositories/stacks_wallet_repository.dart';
import '../errors/service_error.dart';

enum StacksNetwork {
  mainnet,
  testnet,
}

class StacksService {
  final Dio _dio;
  final String _baseUrl;
  final KeyManagementService _keyManager;
  final StacksWalletRepository _walletRepository;
  final StacksNetwork _network;

  static const String _mainnetUrl = 'https://stacks-node-api.mainnet.stacks.co';
  static const String _testnetUrl = 'https://stacks-node-api.testnet.stacks.co';

  StacksService({
    Dio? dio,
    StacksNetwork network = StacksNetwork.mainnet,
    KeyManagementService? keyManager,
    required StacksWalletRepository walletRepository,
  })  : _dio = dio ?? Dio(),
        _network = network,
        _baseUrl = network == StacksNetwork.mainnet ? _mainnetUrl : _testnetUrl,
        _keyManager = keyManager ?? KeyManagementService(),
        _walletRepository = walletRepository {
    _dio.interceptors.add(
      InterceptorsWrapper(
        onError: (error, handler) async {
          if (error.response?.statusCode == 429) {
            throw ServiceError('Rate limit exceeded. Please try again later.');
          }
          handler.next(error);
        },
      ),
    );
  }

  Future<bool> validateAddress(String address) async {
    if (!address.startsWith('SP') || address.length != 41) {
      return false;
    }
    try {
      final response = await _dio.get('$_baseUrl/v2/accounts/$address');
      return response.statusCode == 200;
    } on DioException catch (e) {
      if (e.response?.statusCode == 404) {
        return false;
      }
      throw ServiceError('Failed to validate address: ${e.message}');
    } catch (e) {
      throw ServiceError('Unexpected error validating address: $e');
    }
  }

  Future<StacksWallet> createWallet({
    required String name,
    required String ownerDid,
    Map<String, dynamic>? metadata,
  }) async {
    try {
      // Generate new wallet
      final mnemonic = await _keyManager.generateMnemonic();
      final privateKey = await _keyManager.derivePrimaryKey(mnemonic);
      
      // Generate Stacks address from private key
      final stacksAddress = await _deriveStacksAddress(privateKey);
      
      // Create wallet instance
      final wallet = StacksWallet.create(
        name: name,
        ownerDid: ownerDid,
        stacksAddress: stacksAddress,
        isTestnet: _network == StacksNetwork.testnet,
        metadata: metadata,
        stacksMetadata: {
          'network': _network.toString(),
          'mnemonic': await _keyManager.encryptMnemonic(mnemonic),
        },
      );

      // Store wallet
      await _walletRepository.createWallet(wallet);
      
      // Store private key securely
      await _keyManager.storePrivateKey(stacksAddress, privateKey);

      return wallet;
    } catch (e) {
      throw ServiceError('Failed to create wallet: $e');
    }
  }

  Future<int> getBalance(String address) async {
    try {
      final response = await _dio.get('$_baseUrl/v2/accounts/$address');
      if (response.statusCode == 200) {
        final balance = response.data['balance'] as String;
        return int.parse(balance);
      }
      throw ServiceError('Failed to get balance: Invalid response');
    } on DioException catch (e) {
      throw ServiceError('Network error getting balance: ${e.message}');
    } catch (e) {
      throw ServiceError('Unexpected error getting balance: $e');
    }
  }

  Future<String> sendTransaction(StacksTransaction transaction) async {
    try {
      // Get the wallet
      final wallet = await _walletRepository.getWalletByStacksAddress(transaction.from);
      if (wallet == null) {
        throw ServiceError('Wallet not found for address: ${transaction.from}');
      }

      // Get the private key
      final privateKey = await _keyManager.getPrivateKey(transaction.from);
      if (privateKey == null) {
        throw ServiceError('Private key not found for address: ${transaction.from}');
      }

      // Add network-specific data
      final enrichedTransaction = {
        ...transaction.toJson(),
        'network': _network == StacksNetwork.mainnet ? 'mainnet' : 'testnet',
        'signature': await _signTransaction(transaction, privateKey),
      };

      final response = await _dio.post(
        '$_baseUrl/v2/transactions',
        data: enrichedTransaction,
      );

      if (response.statusCode == 200) {
        final txId = response.data['txid'] as String;
        
        // Update wallet metadata with transaction
        final updatedMetadata = {
          ...wallet.stacksMetadata ?? {},
          'lastTransaction': {
            'txId': txId,
            'timestamp': DateTime.now().toIso8601String(),
          },
        };
        
        await _walletRepository.updateWallet(
          wallet.id,
          wallet.copyWith(stacksMetadata: updatedMetadata),
        );
        
        return txId;
      }
      throw ServiceError('Failed to send transaction: ${response.statusMessage}');
    } on DioException catch (e) {
      throw ServiceError('Network error sending transaction: ${e.message}');
    } catch (e) {
      throw ServiceError('Unexpected error sending transaction: $e');
    }
  }

  Future<String> _signTransaction(StacksTransaction transaction, String privateKey) async {
    // TODO: Implement actual transaction signing using the Stacks SDK
    throw UnimplementedError('Transaction signing not yet implemented');
  }

  Future<String> _deriveStacksAddress(String privateKey) async {
    // TODO: Implement address derivation from private key
    throw UnimplementedError('Address derivation not yet implemented');
  }
}

class StacksException implements Exception {
  final String message;
  StacksException(this.message);

  @override
  String toString() => 'StacksException: $message';
}
