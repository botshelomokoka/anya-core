import 'dart:async';
import 'dart:convert';
import 'package:http/http.dart' as http;
import 'package:logging/logging.dart';

/// Repository monitoring and health check system
class RepoMonitor {
  final String githubToken;
  final String owner;
  final String repo;
  final Logger _logger = Logger('RepoMonitor');
  Timer? _healthCheckTimer;

  RepoMonitor({
    required this.githubToken,
    required this.owner,
    required this.repo,
  }) {
    _setupLogging();
  }

  void _setupLogging() {
    Logger.root.level = Level.ALL;
    Logger.root.onRecord.listen((record) {
      print('${record.level.name}: ${record.time}: ${record.message}');
    });
  }

  /// Start monitoring system
  void startMonitoring({
    Duration checkInterval = const Duration(minutes: 5),
  }) {
    _healthCheckTimer?.cancel();
    _healthCheckTimer = Timer.periodic(checkInterval, (_) => runHealthCheck());
  }

  /// Stop monitoring system
  void stopMonitoring() {
    _healthCheckTimer?.cancel();
    _healthCheckTimer = null;
  }

  /// Run comprehensive health check
  Future<Map<String, dynamic>> runHealthCheck() async {
    final results = <String, dynamic>{};

    try {
      // Check workflow status
      results['workflows'] = await _checkWorkflows();

      // Check repository status
      results['repository'] = await _checkRepository();

      // Check dependencies
      results['dependencies'] = await _checkDependencies();

      // Check submodules
      results['submodules'] = await _checkSubmodules();

      // Check issues and PRs
      results['issues'] = await _checkIssues();

      _logger.info('Health check completed: ${jsonEncode(results)}');
    } catch (e, stack) {
      _logger.severe('Health check failed', e, stack);
      results['error'] = e.toString();
    }

    return results;
  }

  /// Check workflow status
  Future<Map<String, dynamic>> _checkWorkflows() async {
    final url =
        Uri.parse('https://api.github.com/repos/$owner/$repo/actions/runs');

    final response = await _githubGet(url);
    final runs = jsonDecode(response.body)['workflow_runs'] as List;

    return {
      'total': runs.length,
      'successful': runs.where((r) => r['conclusion'] == 'success').length,
      'failed': runs.where((r) => r['conclusion'] == 'failure').length,
      'latest': runs.isNotEmpty ? runs.first : null,
    };
  }

  /// Check repository status
  Future<Map<String, dynamic>> _checkRepository() async {
    final url = Uri.parse('https://api.github.com/repos/$owner/$repo');

    final response = await _githubGet(url);
    final data = jsonDecode(response.body);

    return {
      'size': data['size'],
      'open_issues': data['open_issues_count'],
      'watchers': data['watchers_count'],
      'last_push': data['pushed_at'],
    };
  }

  /// Check dependencies
  Future<Map<String, dynamic>> _checkDependencies() async {
    final url = Uri.parse(
        'https://api.github.com/repos/$owner/$repo/dependency-graph/snapshots');

    try {
      final response = await _githubGet(url);
      final data = jsonDecode(response.body);

      return {
        'total': data['total_count'],
        'vulnerable': data['vulnerable_count'],
        'outdated': data['outdated_count'],
      };
    } catch (e) {
      _logger.warning('Failed to check dependencies', e);
      return {'error': e.toString()};
    }
  }

  /// Check submodules
  Future<Map<String, dynamic>> _checkSubmodules() async {
    final submodules = ['dash33', 'dependencies', 'enterprise'];
    final results = <String, dynamic>{};

    for (final submodule in submodules) {
      try {
        final url = Uri.parse('https://api.github.com/repos/$owner/$submodule');

        final response = await _githubGet(url);
        final data = jsonDecode(response.body);

        results[submodule] = {
          'status': 'healthy',
          'last_update': data['updated_at'],
          'open_issues': data['open_issues_count'],
        };
      } catch (e) {
        results[submodule] = {'status': 'error', 'error': e.toString()};
      }
    }

    return results;
  }

  /// Check issues and PRs
  Future<Map<String, dynamic>> _checkIssues() async {
    final issuesUrl = Uri.parse(
        'https://api.github.com/repos/$owner/$repo/issues?state=open');

    final prsUrl =
        Uri.parse('https://api.github.com/repos/$owner/$repo/pulls?state=open');

    final issuesResponse = await _githubGet(issuesUrl);
    final prsResponse = await _githubGet(prsUrl);

    final issues = jsonDecode(issuesResponse.body) as List;
    final prs = jsonDecode(prsResponse.body) as List;

    return {
      'open_issues': issues.length,
      'open_prs': prs.length,
      'stale_issues': issues.where((i) => _isStale(i['updated_at'])).length,
      'stale_prs': prs.where((p) => _isStale(p['updated_at'])).length,
    };
  }

  /// Check if an item is stale (no updates in 30 days)
  bool _isStale(String dateStr) {
    final date = DateTime.parse(dateStr);
    final staleThreshold = DateTime.now().subtract(const Duration(days: 30));
    return date.isBefore(staleThreshold);
  }

  /// Make authenticated GitHub API request
  Future<http.Response> _githubGet(Uri url) async {
    final response = await http.get(
      url,
      headers: {
        'Accept': 'application/vnd.github.v3+json',
        'Authorization': 'token $githubToken',
      },
    );

    if (response.statusCode != 200) {
      throw Exception(
          'GitHub API error: ${response.statusCode} ${response.body}');
    }

    return response;
  }

  /// Generate health report
  Future<String> generateReport() async {
    final health = await runHealthCheck();
    final buffer = StringBuffer();

    buffer.writeln('Repository Health Report');
    buffer.writeln('======================');
    buffer.writeln('Generated: ${DateTime.now()}');
    buffer.writeln();

    // Workflows
    buffer.writeln('Workflows:');
    buffer.writeln('- Total: ${health['workflows']['total']}');
    buffer.writeln('- Successful: ${health['workflows']['successful']}');
    buffer.writeln('- Failed: ${health['workflows']['failed']}');
    buffer.writeln();

    // Repository
    buffer.writeln('Repository:');
    buffer.writeln('- Open Issues: ${health['repository']['open_issues']}');
    buffer.writeln('- Watchers: ${health['repository']['watchers']}');
    buffer.writeln('- Last Push: ${health['repository']['last_push']}');
    buffer.writeln();

    // Dependencies
    buffer.writeln('Dependencies:');
    buffer.writeln('- Total: ${health['dependencies']['total']}');
    buffer.writeln('- Vulnerable: ${health['dependencies']['vulnerable']}');
    buffer.writeln('- Outdated: ${health['dependencies']['outdated']}');
    buffer.writeln();

    // Submodules
    buffer.writeln('Submodules:');
    health['submodules'].forEach((name, data) {
      buffer.writeln('- $name: ${data['status']}');
      if (data['status'] == 'healthy') {
        buffer.writeln('  Last Update: ${data['last_update']}');
        buffer.writeln('  Open Issues: ${data['open_issues']}');
      }
    });

    return buffer.toString();
  }
}
