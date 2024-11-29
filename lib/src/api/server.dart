import 'package:shelf/shelf.dart';
import 'package:shelf/shelf_io.dart' as shelf_io;
import 'package:shelf_router/shelf_router.dart';
import '../core/bitcoin/wallet.dart';
import '../core/web5/identity.dart';

/// API server implementation
class AnyaServer {
  final Router _router = Router();
  final BitcoinWallet _wallet;
  final IdentityManager _identity;

  AnyaServer(this._wallet, this._identity) {
    _setupRoutes();
  }

  void _setupRoutes() {
    _router.post('/wallet', _createWallet);
    _router.post('/did', _createDID);
  }

  Future<Response> _createWallet(Request request) async {
    try {
      final walletId = await _wallet.createWallet();
      return Response.ok({'wallet_id': walletId});
    } catch (e) {
      return Response.internalServerError(
        body: {'error': e.toString()},
      );
    }
  }

  Future<Response> _createDID(Request request) async {
    try {
      final did = await _identity.createDID();
      return Response.ok({'did': did});
    } catch (e) {
      return Response.internalServerError(
        body: {'error': e.toString()},
      );
    }
  }

  Future<void> start({String host = 'localhost', int port = 8080}) async {
    final handler = Pipeline()
        .addMiddleware(logRequests())
        .addMiddleware(_handleCors())
        .addHandler(_router);

    await shelf_io.serve(handler, host, port);
  }

  Middleware _handleCors() {
    return createMiddleware(
      requestHandler: (Request request) {
        if (request.method == 'OPTIONS') {
          return Response.ok('', headers: _corsHeaders);
        }
        return null;
      },
      responseHandler: (Response response) {
        return response.change(headers: _corsHeaders);
      },
    );
  }

  final _corsHeaders = {
    'Access-Control-Allow-Origin': '*',
    'Access-Control-Allow-Methods': 'GET, POST, PUT, DELETE, OPTIONS',
    'Access-Control-Allow-Headers': 'Origin, Content-Type',
  };
}
