/// Base class for service errors
abstract class ServiceError implements Exception {
  final String message;
  final dynamic cause;
  final StackTrace? stackTrace;

  ServiceError(this.message, [this.cause, this.stackTrace]);

  @override
  String toString() => '$runtimeType: $message${cause != null ? ' ($cause)' : ''}';
}

/// Security-related errors
class SecurityError extends ServiceError {
  SecurityError(String message, [dynamic cause, StackTrace? stackTrace])
      : super(message, cause, stackTrace);
}

/// Transaction-related errors
class TransactionError extends ServiceError {
  TransactionError(String message, [dynamic cause, StackTrace? stackTrace])
      : super(message, cause, stackTrace);
}

/// Network-related errors
class NetworkError extends ServiceError {
  NetworkError(String message, [dynamic cause, StackTrace? stackTrace])
      : super(message, cause, stackTrace);
}

/// Validation-related errors
class ValidationError extends ServiceError {
  ValidationError(String message, [dynamic cause, StackTrace? stackTrace])
      : super(message, cause, stackTrace);
}

/// Storage-related errors
class StorageError extends ServiceError {
  StorageError(String message, [dynamic cause, StackTrace? stackTrace])
      : super(message, cause, stackTrace);
}

/// Repository-related errors
class RepositoryError extends ServiceError {
  RepositoryError(String message, [dynamic cause, StackTrace? stackTrace])
      : super(message, cause, stackTrace);
}

/// Lightning-related errors
class LightningError extends ServiceError {
  LightningError(String message, [dynamic cause, StackTrace? stackTrace])
      : super(message, cause, stackTrace);
}

/// RGB-related errors
class RGBError extends ServiceError {
  RGBError(String message, [dynamic cause, StackTrace? stackTrace])
      : super(message, cause, stackTrace);
}

/// UTXO-related errors
class UTXOError extends ServiceError {
  UTXOError(String message, [dynamic cause, StackTrace? stackTrace])
      : super(message, cause, stackTrace);
}

/// Fee-related errors
class FeeError extends ServiceError {
  FeeError(String message, [dynamic cause, StackTrace? stackTrace])
      : super(message, cause, stackTrace);
}

/// Backup-related errors
class BackupError extends ServiceError {
  BackupError(String message, [dynamic cause, StackTrace? stackTrace])
      : super(message, cause, stackTrace);
}

/// History-related errors
class HistoryError extends ServiceError {
  HistoryError(String message, [dynamic cause, StackTrace? stackTrace])
      : super(message, cause, stackTrace);
}

/// Insufficient funds error
class InsufficientFundsError extends TransactionError {
  InsufficientFundsError(String message, [dynamic cause, StackTrace? stackTrace])
      : super(message, cause, stackTrace);
}

/// Invalid address error
class InvalidAddressError extends ValidationError {
  InvalidAddressError(String message, [dynamic cause, StackTrace? stackTrace])
      : super(message, cause, stackTrace);
}

/// Invalid amount error
class InvalidAmountError extends ValidationError {
  InvalidAmountError(String message, [dynamic cause, StackTrace? stackTrace])
      : super(message, cause, stackTrace);
}

/// Invalid fee error
class InvalidFeeError extends ValidationError {
  InvalidFeeError(String message, [dynamic cause, StackTrace? stackTrace])
      : super(message, cause, stackTrace);
}

/// Node connection error
class NodeConnectionError extends NetworkError {
  NodeConnectionError(String message, [dynamic cause, StackTrace? stackTrace])
      : super(message, cause, stackTrace);
}

/// Sync error
class SyncError extends NetworkError {
  SyncError(String message, [dynamic cause, StackTrace? stackTrace])
      : super(message, cause, stackTrace);
}

/// Authentication error
class AuthenticationError extends SecurityError {
  AuthenticationError(String message, [dynamic cause, StackTrace? stackTrace])
      : super(message, cause, stackTrace);
}

/// Authorization error
class AuthorizationError extends SecurityError {
  AuthorizationError(String message, [dynamic cause, StackTrace? stackTrace])
      : super(message, cause, stackTrace);
}

/// Encryption error
class EncryptionError extends SecurityError {
  EncryptionError(String message, [dynamic cause, StackTrace? stackTrace])
      : super(message, cause, stackTrace);
}

/// Decryption error
class DecryptionError extends SecurityError {
  DecryptionError(String message, [dynamic cause, StackTrace? stackTrace])
      : super(message, cause, stackTrace);
}

/// Key management error
class KeyManagementError extends SecurityError {
  KeyManagementError(String message, [dynamic cause, StackTrace? stackTrace])
      : super(message, cause, stackTrace);
}

/// Lightning channel error
class LightningChannelError extends LightningError {
  LightningChannelError(String message, [dynamic cause, StackTrace? stackTrace])
      : super(message, cause, stackTrace);
}

/// Lightning payment error
class LightningPaymentError extends LightningError {
  LightningPaymentError(String message, [dynamic cause, StackTrace? stackTrace])
      : super(message, cause, stackTrace);
}

/// RGB asset error
class RGBAssetError extends RGBError {
  RGBAssetError(String message, [dynamic cause, StackTrace? stackTrace])
      : super(message, cause, stackTrace);
}

/// RGB transfer error
class RGBTransferError extends RGBError {
  RGBTransferError(String message, [dynamic cause, StackTrace? stackTrace])
      : super(message, cause, stackTrace);
}

/// UTXO selection error
class UTXOSelectionError extends UTXOError {
  UTXOSelectionError(String message, [dynamic cause, StackTrace? stackTrace])
      : super(message, cause, stackTrace);
}

/// UTXO locking error
class UTXOLockingError extends UTXOError {
  UTXOLockingError(String message, [dynamic cause, StackTrace? stackTrace])
      : super(message, cause, stackTrace);
}

/// Fee estimation error
class FeeEstimationError extends FeeError {
  FeeEstimationError(String message, [dynamic cause, StackTrace? stackTrace])
      : super(message, cause, stackTrace);
}

/// Backup creation error
class BackupCreationError extends BackupError {
  BackupCreationError(String message, [dynamic cause, StackTrace? stackTrace])
      : super(message, cause, stackTrace);
}

/// Backup restoration error
class BackupRestorationError extends BackupError {
  BackupRestorationError(String message, [dynamic cause, StackTrace? stackTrace])
      : super(message, cause, stackTrace);
}

/// History retrieval error
class HistoryRetrievalError extends HistoryError {
  HistoryRetrievalError(String message, [dynamic cause, StackTrace? stackTrace])
      : super(message, cause, stackTrace);
}

/// Export error
class ExportError extends HistoryError {
  ExportError(String message, [dynamic cause, StackTrace? stackTrace])
      : super(message, cause, stackTrace);
}
