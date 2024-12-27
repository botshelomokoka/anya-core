/// Custom error types for Bitcoin operations
abstract class BitcoinError implements Exception {
  final String message;
  final dynamic cause;

  const BitcoinError(this.message, [this.cause]);

  @override
  String toString() => cause != null
      ? 'BitcoinError: $message (Cause: $cause)'
      : 'BitcoinError: $message';
}

/// RPC communication errors
class RpcError extends BitcoinError {
  const RpcError(String message, [dynamic cause]) : super(message, cause);
}

/// Transaction validation errors
class ValidationError extends BitcoinError {
  const ValidationError(String message, [dynamic cause])
      : super(message, cause);
}

/// Transaction signing errors
class SigningError extends BitcoinError {
  const SigningError(String message, [dynamic cause]) : super(message, cause);
}

/// Privacy-related errors
class PrivacyError extends BitcoinError {
  const PrivacyError(String message, [dynamic cause]) : super(message, cause);
}

/// Taproot-related errors
class TaprootError extends BitcoinError {
  const TaprootError(String message, [dynamic cause]) : super(message, cause);
}

/// PSBT-related errors
class PsbtError extends BitcoinError {
  const PsbtError(String message, [dynamic cause]) : super(message, cause);
}

/// Lightning Network errors
class LightningError extends BitcoinError {
  const LightningError(String message, [dynamic cause]) : super(message, cause);
}

/// Web5 storage errors
class StorageError extends BitcoinError {
  const StorageError(String message, [dynamic cause]) : super(message, cause);
}

/// Wallet initialization errors
class WalletError extends BitcoinError {
  const WalletError(String message, [dynamic cause]) : super(message, cause);
}
