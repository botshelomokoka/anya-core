import 'dart:async';
import 'dart:convert';
import 'package:logging/logging.dart';
import 'monitoring.dart';
import 'auto_fixer.dart';

/// Workflow orchestration and automation management
class AutomationOrchestrator {
  final String githubToken;
  final String owner;
  final String repo;
  final Logger _logger = Logger('Orchestrator');

  late final RepoMonitor _monitor;
  late final AutoFixer _fixer;
  Timer? _automationTimer;

  AutomationOrchestrator({
    required this.githubToken,
    required this.owner,
    required this.repo,
  }) {
    _setupLogging();
    _monitor = RepoMonitor(
      githubToken: githubToken,
      owner: owner,
      repo: repo,
    );
    _fixer = AutoFixer(
      githubToken: githubToken,
      owner: owner,
      repo: repo,
    );
  }

  void _setupLogging() {
    Logger.root.level = Level.ALL;
    Logger.root.onRecord.listen((record) {
      print('${record.level.name}: ${record.time}: ${record.message}');
    });
  }

  /// Start automation system
  void startAutomation({
    Duration checkInterval = const Duration(hours: 1),
  }) {
    _logger.info('Starting automation system');

    // Start monitoring
    _monitor.startMonitoring();

    // Schedule automated fixes
    _automationTimer?.cancel();
    _automationTimer =
        Timer.periodic(checkInterval, (_) => _runAutomationCycle());
  }

  /// Stop automation system
  void stopAutomation() {
    _logger.info('Stopping automation system');
    _monitor.stopMonitoring();
    _automationTimer?.cancel();
    _automationTimer = null;
  }

  /// Run complete automation cycle
  Future<Map<String, dynamic>> _runAutomationCycle() async {
    final results = <String, dynamic>{};

    try {
      // Run health check
      final health = await _monitor.runHealthCheck();
      results['health'] = health;

      // Check if fixes are needed
      if (_needsFixes(health)) {
        _logger.info('Issues detected, running auto-fixes');

        // Run auto-fixes
        final fixes = await _fixer.runAutoFix();
        results['fixes'] = fixes;

        // Create PR if fixes were made
        if (_fixesApplied(fixes)) {
          final prUrl = await _fixer.createFixPR();
          results['pull_request'] = prUrl;
        }
      }

      // Generate report
      results['report'] = await _generateAutomationReport(results);

      _logger.info('Automation cycle completed successfully');
    } catch (e, stack) {
      _logger.severe('Automation cycle failed', e, stack);
      results['error'] = {
        'message': e.toString(),
        'stack': stack.toString(),
      };
    }

    return results;
  }

  /// Check if fixes are needed based on health check
  bool _needsFixes(Map<String, dynamic> health) {
    // Check workflows
    if (health['workflows']?['failed'] > 0) return true;

    // Check dependencies
    if (health['dependencies']?['vulnerable'] > 0) return true;
    if (health['dependencies']?['outdated'] > 0) return true;

    // Check issues
    if (health['issues']?['stale_issues'] > 0) return true;
    if (health['issues']?['stale_prs'] > 0) return true;

    return false;
  }

  /// Check if fixes were applied
  bool _fixesApplied(Map<String, dynamic> fixes) {
    return fixes['dependencies']?['status'] == 'success' ||
        fixes['code_style']?['status'] == 'success' ||
        fixes['documentation']?['status'] == 'success' ||
        fixes['tests']?['status'] == 'success';
  }

  /// Generate automation report
  Future<String> _generateAutomationReport(Map<String, dynamic> results) async {
    final buffer = StringBuffer();

    buffer.writeln('Automation Report');
    buffer.writeln('=================');
    buffer.writeln('Generated: ${DateTime.now()}');
    buffer.writeln();

    // Health Status
    if (results.containsKey('health')) {
      buffer.writeln('Health Status:');
      buffer.writeln('--------------');
      _formatHealthStatus(buffer, results['health']);
      buffer.writeln();
    }

    // Fixes Applied
    if (results.containsKey('fixes')) {
      buffer.writeln('Fixes Applied:');
      buffer.writeln('-------------');
      _formatFixes(buffer, results['fixes']);
      buffer.writeln();
    }

    // Pull Request
    if (results.containsKey('pull_request')) {
      buffer.writeln('Pull Request:');
      buffer.writeln('-------------');
      buffer.writeln(results['pull_request'] ?? 'No PR created');
      buffer.writeln();
    }

    // Errors
    if (results.containsKey('error')) {
      buffer.writeln('Errors:');
      buffer.writeln('-------');
      buffer.writeln(results['error']['message']);
      buffer.writeln();
    }

    return buffer.toString();
  }

  /// Format health status for report
  void _formatHealthStatus(StringBuffer buffer, Map<String, dynamic> health) {
    // Workflows
    buffer.writeln('Workflows:');
    buffer.writeln('- Total: ${health['workflows']['total']}');
    buffer.writeln('- Successful: ${health['workflows']['successful']}');
    buffer.writeln('- Failed: ${health['workflows']['failed']}');

    // Dependencies
    buffer.writeln('\nDependencies:');
    buffer.writeln('- Vulnerable: ${health['dependencies']['vulnerable']}');
    buffer.writeln('- Outdated: ${health['dependencies']['outdated']}');

    // Issues
    buffer.writeln('\nIssues:');
    buffer.writeln('- Open Issues: ${health['issues']['open_issues']}');
    buffer.writeln('- Open PRs: ${health['issues']['open_prs']}');
    buffer.writeln('- Stale Issues: ${health['issues']['stale_issues']}');
    buffer.writeln('- Stale PRs: ${health['issues']['stale_prs']}');
  }

  /// Format fixes for report
  void _formatFixes(StringBuffer buffer, Map<String, dynamic> fixes) {
    fixes.forEach((category, data) {
      buffer.writeln('$category:');
      buffer.writeln('- Status: ${data['status']}');

      if (data['status'] == 'error') {
        buffer.writeln('- Error: ${data['error']}');
      } else {
        if (data.containsKey('conflicts')) {
          buffer.writeln('- Conflicts Resolved: ${data['conflicts'].length}');
        }
        if (data.containsKey('removed')) {
          buffer.writeln('- Dependencies Removed: ${data['removed'].length}');
        }
        if (data.containsKey('lint_fixes')) {
          buffer.writeln('- Lint Issues Fixed: ${data['lint_fixes'].length}');
        }
      }
      buffer.writeln();
    });
  }
}
