import 'dart:convert';
import 'package:shelf/shelf.dart';
import 'package:shelf/shelf_io.dart' as shelf_io;
import 'package:shelf_router/shelf_router.dart';
import 'package:logging/logging.dart';

import '../core/web5/identity.dart';

/// API server implementation for the Anya Bitcoin Infrastructure Platform
class AnyaServer {
  final Router _router = Router();
  final IdentityManager _identity;
  final Logger _logger = Logger('AnyaServer');

  AnyaServer({
    required IdentityManager identity,
  }) : _identity = identity {
    _setupRoutes();
  }

  void _setupRoutes() {
    // Identity endpoints
    _router.post('/did', _createDID);
    _router.get('/did/<did>', _resolveDID);
    
    // Health check
    _router.get('/health', _healthCheck);
  }

  Future<Response> _createDID(Request request) async {
    try {
      _logger.info('Creating new DID');
      final did = await _identity.createDID();
      return Response.ok(
        jsonEncode({'did': did}),
        headers: {'content-type': 'application/json'},
      );
    } catch (e, stack) {
      _logger.severe('Error creating DID', e, stack);
      return Response.internalServerError(
        body: jsonEncode({'error': 'Failed to create DID: $e'}),
        headers: {'content-type': 'application/json'},
      );
    }
  }

  Future<Response> _resolveDID(Request request, String did) async {
    try {
      _logger.info('Resolving DID: $did');
      final resolution = await _identity.resolveDID(did);
      return Response.ok(
        jsonEncode(resolution),
        headers: {'content-type': 'application/json'},
      );
    } catch (e, stack) {
      _logger.severe('Error resolving DID', e, stack);
      return Response.internalServerError(
        body: jsonEncode({'error': 'Failed to resolve DID: $e'}),
        headers: {'content-type': 'application/json'},
      );
    }
  }

  Future<Response> _healthCheck(Request request) async {
    return Response.ok(
      jsonEncode({
        'status': 'healthy',
        'timestamp': DateTime.now().toIso8601String(),
      }),
      headers: {'content-type': 'application/json'},
    );
  }

  Middleware _handleCors() {
    const corsHeaders = {
      'Access-Control-Allow-Origin': '*',
      'Access-Control-Allow-Methods': 'GET, POST, PUT, DELETE, OPTIONS',
      'Access-Control-Allow-Headers': 'Origin, Content-Type',
    };

    return createMiddleware(
      requestHandler: (request) {
        if (request.method == 'OPTIONS') {
          return Response.ok('', headers: corsHeaders);
        }
        return null;
      },
      responseHandler: (response) {
        return response.change(headers: corsHeaders);
      },
    );
  }

  /// Start the server
  Future<void> start({
    String host = 'localhost',
    int port = 8080,
  }) async {
    final handler = Pipeline()
        .addMiddleware(logRequests())
        .addMiddleware(_handleCors())
        .addHandler(_router.call);

    final server = await shelf_io.serve(handler, host, port);
    _logger.info('Server running on http://${server.address.host}:${server.port}');
  }
}
