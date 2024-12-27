import 'package:local_auth/local_auth.dart';
import '../errors/security_errors.dart';

class BiometricService {
  final LocalAuthentication _auth;

  BiometricService() : _auth = LocalAuthentication();

  Future<bool> isAvailable() async {
    try {
      return await _auth.canCheckBiometrics && await _auth.isDeviceSupported();
    } catch (e) {
      throw SecurityError('Failed to check biometric availability: $e');
    }
  }

  Future<bool> authenticate() async {
    try {
      final isAvailable = await isAvailable();
      if (!isAvailable) {
        throw SecurityError('Biometric authentication not available');
      }

      return await _auth.authenticate(
        localizedReason: 'Authenticate to access wallet',
        options: const AuthenticationOptions(
          stickyAuth: true,
          biometricOnly: true,
        ),
      );
    } catch (e) {
      throw SecurityError('Biometric authentication failed: $e');
    }
  }

  Future<List<BiometricType>> getAvailableBiometrics() async {
    try {
      return await _auth.getAvailableBiometrics();
    } catch (e) {
      throw SecurityError('Failed to get available biometrics: $e');
    }
  }
}
