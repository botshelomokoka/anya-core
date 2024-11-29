import 'package:test/test.dart';
import 'package:mockito/mockito.dart';
import 'package:web5/web5.dart';
import '../../lib/src/core/web5/identity.dart';

class MockWeb5Client extends Mock implements Web5Client {}

void main() {
  group('IdentityManager', () {
    late MockWeb5Client mockClient;
    late IdentityManager manager;

    setUp(() {
      mockClient = MockWeb5Client();
      manager = IdentityManager(mockClient);
    });

    test('createDID returns a valid DID', () async {
      final did = await manager.createDID();
      expect(did, startsWith('did:'));
    });
  });
}
