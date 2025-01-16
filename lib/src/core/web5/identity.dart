import 'package:logging/logging.dart';
import 'package:web5_dart/web5_dart.dart' as web5;

/// Web5 DID management
class IdentityManager {
  final Logger _logger = Logger('IdentityManager');
  final web5.Web5Client _client;

  IdentityManager(this._client);

  Future<String> createDID({String method = 'key'}) async {
    try {
      final did = await web5.DID.create(method: method);
      _logger.info('Created DID: ${did.toString()}');
      return did.toString();
    } catch (e) {
      _logger.severe('Failed to create DID: $e');
      rethrow;
    }
  }

  Future<Map<String, dynamic>> resolveDID(String did) async {
    try {
      final resolution = await web5.DID.resolve(did);
      _logger.info('Resolved DID: $did');
      return resolution.toJson();
    } catch (e) {
      _logger.severe('Failed to resolve DID: $e');
      rethrow;
    }
  }

  Future<bool> verifyDID(String did) async {
    try {
      final resolution = await web5.DID.resolve(did);
      return resolution != null;
    } catch (e) {
      _logger.warning('DID verification failed: $e');
      return false;
    }
  }
}
