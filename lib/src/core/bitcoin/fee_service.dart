import 'dart:async';
import 'package:freezed_annotation/freezed_annotation.dart';
import 'package:json_annotation/json_annotation.dart';
import 'package:web5/web5.dart';
import '../errors/service_errors.dart';

part 'fee_service.g.dart';
part 'fee_service.freezed.dart';

/// Fee estimation modes
enum FeeEstimateMode {
  /// Conservative estimation (more likely to be confirmed within target)
  conservative,

  /// Economical estimation (may take longer to confirm)
  economical,
}

/// Fee rate recommendation
@freezed
class FeeRecommendation with _$FeeRecommendation {
  const factory FeeRecommendation({
    /// Fee rate in satoshis per vbyte
    required int feeRate,

    /// Estimated blocks until confirmation
    required int blocks,

    /// Estimated minutes until confirmation
    required int minutes,

    /// Confidence level (0-1)
    required double confidence,
  }) = _FeeRecommendation;

  factory FeeRecommendation.fromJson(Map<String, dynamic> json) =>
      _$FeeRecommendationFromJson(json);
}

/// Fee estimation service
class FeeService {
  final Web5 _web5;
  final String _nodeUrl;
  final Duration _cacheDuration;

  /// Cache duration for fee estimates
  static const defaultCacheDuration = Duration(minutes: 10);

  /// Cached fee recommendations
  final Map<String, _CachedFeeRecommendation> _cache = {};

  FeeService(
    this._web5,
    this._nodeUrl, {
    Duration? cacheDuration,
  }) : _cacheDuration = cacheDuration ?? defaultCacheDuration;

  /// Get fee recommendation for different priorities
  Future<Map<String, FeeRecommendation>> getFeeRecommendations({
    FeeEstimateMode mode = FeeEstimateMode.conservative,
  }) async {
    try {
      // Check cache first
      final cacheKey = mode.toString();
      final cached = _cache[cacheKey];
      if (cached != null && !cached.isExpired(_cacheDuration)) {
        return cached.recommendations;
      }

      // Get fresh estimates from node
      final Map<String, dynamic> estimates = await _getFeeEstimates(mode);

      // Process estimates into recommendations
      final recommendations = <String, FeeRecommendation>{
        'high': FeeRecommendation(
          feeRate: _validateFeeRate(estimates['high_fee_rate']),
          blocks: 1,
          minutes: 10,
          confidence: 0.95,
        ),
        'medium': FeeRecommendation(
          feeRate: _validateFeeRate(estimates['medium_fee_rate']),
          blocks: 3,
          minutes: 30,
          confidence: 0.90,
        ),
        'low': FeeRecommendation(
          feeRate: _validateFeeRate(estimates['low_fee_rate']),
          blocks: 6,
          minutes: 60,
          confidence: 0.80,
        ),
        'minimum': FeeRecommendation(
          feeRate: _validateFeeRate(estimates['minimum_fee_rate']),
          blocks: 100,
          minutes: 1000,
          confidence: 0.50,
        ),
      };

      // Update cache
      _cache[cacheKey] = _CachedFeeRecommendation(
        recommendations: recommendations,
        timestamp: DateTime.now(),
      );

      return recommendations;
    } on Web5Exception catch (e) {
      throw FeeServiceError('Web5 error: ${e.message}');
    } on FormatException catch (e) {
      throw FeeServiceError('Invalid fee rate format: ${e.message}');
    } catch (e) {
      throw FeeServiceError('Failed to get fee recommendations: $e');
    }
  }

  /// Estimate fee for specific target in blocks
  Future<FeeRecommendation> estimateFee(
    int targetBlocks, {
    FeeEstimateMode mode = FeeEstimateMode.conservative,
  }) async {
    try {
      final Map<String, dynamic> response = await _makeRequest(
        'estimatesmartfee',
        [targetBlocks, mode.toString().split('.').last],
      );

      if (!response.containsKey('feerate')) {
        throw FeeServiceError('Invalid response from node');
      }

      final double feeRate = response['feerate'] as double;
      final int blocks = response['blocks'] as int;

      return FeeRecommendation(
        feeRate: _convertBtcKbToSatVb(feeRate),
        blocks: blocks,
        minutes: blocks * 10, // Assuming 10 min block time
        confidence: 0.90,
      );
    } on Web5Exception catch (e) {
      throw FeeServiceError('Web5 error: ${e.message}');
    } catch (e) {
      throw FeeServiceError('Failed to estimate fee: $e');
    }
  }

  /// Calculate fee for transaction size
  int calculateFee(int vsize, FeeRecommendation recommendation) {
    if (vsize <= 0) {
      throw FeeServiceError('Invalid transaction size');
    }
    return vsize * recommendation.feeRate;
  }

  /// Get minimum relay fee
  Future<int> getMinimumRelayFee() async {
    try {
      final Map<String, dynamic> response = await _makeRequest('getnetworkinfo', []);
      final double relayFee = response['relayfee'] as double;
      return _convertBtcKbToSatVb(relayFee);
    } catch (e) {
      throw FeeServiceError('Failed to get minimum relay fee: $e');
    }
  }

  // Private helper methods
  Future<Map<String, dynamic>> _getFeeEstimates(FeeEstimateMode mode) async {
    final response = await _makeRequest('estimatefeerates', [mode.toString().split('.').last]);
    return Map<String, dynamic>.from(response);
  }

  Future<Map<String, dynamic>> _makeRequest(String method, List<dynamic> params) async {
    final response = await _web5.rpc.call(_nodeUrl, method, params);
    return Map<String, dynamic>.from(response);
  }

  int _validateFeeRate(dynamic rate) {
    if (rate == null) {
      throw FeeServiceError('Fee rate cannot be null');
    }
    final feeRate = rate is int ? rate : (rate as num).toInt();
    if (feeRate < 0) {
      throw FeeServiceError('Fee rate cannot be negative');
    }
    return feeRate;
  }

  int _convertBtcKbToSatVb(double btcPerKb) {
    return (btcPerKb * 100000000 / 1000).round(); // Convert BTC/kB to sat/vB
  }
}

/// Cached fee recommendation with timestamp
@immutable
class _CachedFeeRecommendation {
  final Map<String, FeeRecommendation> recommendations;
  final DateTime timestamp;

  const _CachedFeeRecommendation({
    required this.recommendations,
    required this.timestamp,
  });

  bool isExpired(Duration cacheDuration) {
    return DateTime.now().difference(timestamp) > cacheDuration;
  }
}

/// Error thrown by FeeService
class FeeServiceError implements Exception {
  final String message;
  
  const FeeServiceError(this.message);
  
  @override
  String toString() => 'FeeServiceError: $message';
}
