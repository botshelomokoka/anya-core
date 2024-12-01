import 'dart:async';
import 'package:web5/web5.dart';
import 'service_errors.dart';

/// Error severity levels
enum ErrorSeverity {
  debug,
  info,
  warning,
  error,
  critical,
}

/// Error categories
enum ErrorCategory {
  network,
  security,
  validation,
  transaction,
  storage,
  sync,
  protocol,
  unknown,
}

/// Error context
class ErrorContext {
  final String component;
  final String operation;
  final Map<String, dynamic>? metadata;
  final StackTrace? stackTrace;
  final DateTime timestamp;

  ErrorContext({
    required this.component,
    required this.operation,
    this.metadata,
    this.stackTrace,
    DateTime? timestamp,
  }) : timestamp = timestamp ?? DateTime.now();

  Map<String, dynamic> toJson() {
    return {
      'component': component,
      'operation': operation,
      'metadata': metadata,
      'stackTrace': stackTrace?.toString(),
      'timestamp': timestamp.toIso8601String(),
    };
  }
}

/// Wallet error handler
class WalletErrorHandler {
  final Web5 _web5;
  final void Function(String)? _logger;
  
  WalletErrorHandler(this._web5, [this._logger]);

  /// Handle error with context
  Future<void> handleError(
    dynamic error,
    ErrorContext context, {
    ErrorSeverity severity = ErrorSeverity.error,
    ErrorCategory category = ErrorCategory.unknown,
  }) async {
    try {
      // Log error
      _logError(error, context, severity, category);

      // Store error if significant
      if (severity >= ErrorSeverity.error) {
        await _storeError(error, context, severity, category);
      }

      // Handle specific error types
      if (error is SecurityError) {
        await _handleSecurityError(error, context);
      } else if (error is TransactionError) {
        await _handleTransactionError(error, context);
      } else if (error is NetworkError) {
        await _handleNetworkError(error, context);
      } else if (error is ValidationError) {
        await _handleValidationError(error, context);
      }

      // Rethrow critical errors
      if (severity == ErrorSeverity.critical) {
        throw error;
      }
    } catch (e) {
      // Last resort error logging
      _logger?.call('Error handler failed: $e');
    }
  }

  /// Log error with context
  void _logError(
    dynamic error,
    ErrorContext context,
    ErrorSeverity severity,
    ErrorCategory category,
  ) {
    final message = _formatErrorMessage(error, context, severity, category);
    _logger?.call(message);
  }

  /// Store error in Web5
  Future<void> _storeError(
    dynamic error,
    ErrorContext context,
    ErrorSeverity severity,
    ErrorCategory category,
  ) async {
    try {
      await _web5.dwn.records.create({
        'data': {
          'error': error.toString(),
          'context': context.toJson(),
          'severity': severity.toString(),
          'category': category.toString(),
        },
        'message': {
          'schema': 'anya/errors',
          'dataFormat': 'application/json',
        },
      });
    } catch (e) {
      _logger?.call('Failed to store error: $e');
    }
  }

  /// Handle security-related errors
  Future<void> _handleSecurityError(
    SecurityError error,
    ErrorContext context,
  ) async {
    // Implement security error handling
    // - Lock affected wallets
    // - Notify user
    // - Trigger security audit
  }

  /// Handle transaction-related errors
  Future<void> _handleTransactionError(
    TransactionError error,
    ErrorContext context,
  ) async {
    // Implement transaction error handling
    // - Check transaction status
    // - Attempt recovery if possible
    // - Update transaction status
  }

  /// Handle network-related errors
  Future<void> _handleNetworkError(
    NetworkError error,
    ErrorContext context,
  ) async {
    // Implement network error handling
    // - Check network status
    // - Attempt reconnection
    // - Queue operations for retry
  }

  /// Handle validation-related errors
  Future<void> _handleValidationError(
    ValidationError error,
    ErrorContext context,
  ) async {
    // Implement validation error handling
    // - Log validation details
    // - Update UI with validation message
  }

  /// Format error message
  String _formatErrorMessage(
    dynamic error,
    ErrorContext context,
    ErrorSeverity severity,
    ErrorCategory category,
  ) {
    return '[${severity.toString().split('.').last}] '
        '[${category.toString().split('.').last}] '
        '[${context.component}] '
        '[${context.operation}] '
        '$error';
  }
}

/// Error recovery service
class ErrorRecoveryService {
  final Web5 _web5;
  final void Function(String)? _logger;

  ErrorRecoveryService(this._web5, [this._logger]);

  /// Attempt to recover from error
  Future<bool> attemptRecovery(
    dynamic error,
    ErrorContext context,
  ) async {
    try {
      if (error is TransactionError) {
        return await _recoverTransaction(error, context);
      } else if (error is NetworkError) {
        return await _recoverNetwork(error, context);
      } else if (error is StorageError) {
        return await _recoverStorage(error, context);
      }
      return false;
    } catch (e) {
      _logger?.call('Recovery failed: $e');
      return false;
    }
  }

  /// Recover from transaction error
  Future<bool> _recoverTransaction(
    TransactionError error,
    ErrorContext context,
  ) async {
    // Implement transaction recovery
    // - Check mempool
    // - Attempt RBF if possible
    // - Rebroadcast if necessary
    return false;
  }

  /// Recover from network error
  Future<bool> _recoverNetwork(
    NetworkError error,
    ErrorContext context,
  ) async {
    // Implement network recovery
    // - Check alternate endpoints
    // - Attempt reconnection
    // - Verify chain state
    return false;
  }

  /// Recover from storage error
  Future<bool> _recoverStorage(
    StorageError error,
    ErrorContext context,
  ) async {
    // Implement storage recovery
    // - Check data integrity
    // - Attempt repair
    // - Restore from backup if needed
    return false;
  }
}

/// Error monitoring service
class ErrorMonitoringService {
  final Web5 _web5;
  final void Function(String)? _logger;

  ErrorMonitoringService(this._web5, [this._logger]);

  /// Monitor for errors
  Stream<Map<String, dynamic>> monitorErrors({
    ErrorSeverity? minSeverity,
    List<ErrorCategory>? categories,
  }) async* {
    try {
      final query = {
        'message': {
          'filter': {
            'schema': 'anya/errors',
            if (minSeverity != null)
              'severity': {'\$gte': minSeverity.toString()},
            if (categories != null)
              'category': {
                '\$in': categories.map((c) => c.toString()).toList(),
              },
          },
          'sort': {'timestamp': -1},
        },
      };

      final subscription = _web5.dwn.records.query(query);
      await for (final record in subscription) {
        yield record.data;
      }
    } catch (e) {
      _logger?.call('Error monitoring failed: $e');
    }
  }

  /// Get error statistics
  Future<Map<String, int>> getErrorStats({
    DateTime? startDate,
    DateTime? endDate,
  }) async {
    try {
      final query = {
        'message': {
          'filter': {
            'schema': 'anya/errors',
            if (startDate != null || endDate != null)
              'timestamp': {
                if (startDate != null)
                  '\$gte': startDate.toIso8601String(),
                if (endDate != null)
                  '\$lte': endDate.toIso8601String(),
              },
          },
        },
      };

      final records = await _web5.dwn.records.query(query);
      
      final stats = <String, int>{};
      for (final record in records) {
        final category = record.data['category'] as String;
        stats[category] = (stats[category] ?? 0) + 1;
      }
      
      return stats;
    } catch (e) {
      _logger?.call('Failed to get error stats: $e');
      return {};
    }
  }
}
