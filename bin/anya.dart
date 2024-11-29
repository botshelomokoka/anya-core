import 'package:args/command_runner.dart';
import 'package:logging/logging.dart';
import '../lib/src/api/server.dart';
import '../lib/src/core/bitcoin/wallet.dart';
import '../lib/src/core/web5/identity.dart';

void main(List<String> arguments) async {
  _setupLogging();

  final runner = CommandRunner('anya', 'Enterprise-grade Bitcoin Infrastructure')
    ..addCommand(ServeCommand())
    ..addCommand(InitCommand());

  try {
    await runner.run(arguments);
  } catch (e, stackTrace) {
    Logger('anya').severe('Error running command', e, stackTrace);
    exit(1);
  }
}

void _setupLogging() {
  Logger.root.level = Level.INFO;
  Logger.root.onRecord.listen((record) {
    print('${record.level.name}: ${record.time}: ${record.message}');
  });
}

class ServeCommand extends Command {
  @override
  final name = 'serve';
  @override
  final description = 'Start the Anya server';

  ServeCommand() {
    argParser
      ..addOption('host', defaultsTo: 'localhost')
      ..addOption('port', defaultsTo: '8080');
  }

  @override
  Future<void> run() async {
    final wallet = BitcoinWallet();
    final identity = IdentityManager(Web5Client());
    final server = AnyaServer(wallet, identity);

    final host = argResults?['host'] as String;
    final port = int.parse(argResults?['port'] as String);

    await server.start(host: host, port: port);
    print('Server running on http://$host:$port/');
  }
}

class InitCommand extends Command {
  @override
  final name = 'init';
  @override
  final description = 'Initialize a new Anya project';

  @override
  Future<void> run() async {
    // Project initialization logic
    print('Initialized new Anya project');
  }
}
