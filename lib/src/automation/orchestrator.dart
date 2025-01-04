import 'dart:async';

import 'package:logging/logging.dart';

import 'auto_fixer.dart';
import 'monitoring.dart';

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
      _logger.info('${record.level.name}: ${record.time}: ${record.message}');
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
    final workflows = health['workflows'] as Map<String, dynamic>?;
    final dependencies = health['dependencies'] as Map<String, dynamic>?;
    final issues = health['issues'] as Map<String, dynamic>?;

    // Check workflows
    if (workflows != null && (workflows['failed'] as int? ?? 0) > 0) return true;

    // Check dependencies
    if (dependencies != null) {
      if ((dependencies['vulnerable'] as int? ?? 0) > 0) return true;
      if ((dependencies['outdated'] as int? ?? 0) > 0) return true;
    }

    // Check issues
    if (issues != null) {
      if ((issues['stale_issues'] as int? ?? 0) > 0) return true;
      if ((issues['stale_prs'] as int? ?? 0) > 0) return true;
    }

    return false;
  }

  /// Check if fixes were applied
  bool _fixesApplied(Map<String, dynamic> fixes) {
    final dependencies = fixes['dependencies'] as Map<String, dynamic>?;
    final codeStyle = fixes['code_style'] as Map<String, dynamic>?;
    final documentation = fixes['documentation'] as Map<String, dynamic>?;
    final tests = fixes['tests'] as Map<String, dynamic>?;

    return (dependencies?['status'] as String? == 'success') ||
           (codeStyle?['status'] as String? == 'success') ||
           (documentation?['status'] as String? == 'success') ||
           (tests?['status'] as String? == 'success');
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
      final health = results['health'];
      if (health is Map<String, dynamic>) {
        buffer.writeln('Health Status:');
        buffer.writeln('--------------');
        _formatHealthStatus(buffer, health);
        buffer.writeln();
      }
    }

    // Fixes Applied
    if (results.containsKey('fixes')) {
      final fixes = results['fixes'];
      if (fixes is Map<String, dynamic>) {
        buffer.writeln('Fixes Applied:');
        buffer.writeln('-------------');
        _formatFixes(buffer, fixes);
        buffer.writeln();
      }
    }

    // Pull Request
    if (results.containsKey('pull_request')) {
      buffer.writeln('Pull Request:');
      buffer.writeln('-------------');
      buffer.writeln(results['pull_request'] as String? ?? 'No PR created');
      buffer.writeln();
    }

    // Errors
    if (results.containsKey('error')) {
      final error = results['error'] as Map<String, dynamic>?;
      if (error != null) {
        buffer.writeln('Errors:');
        buffer.writeln('-------');
        buffer.writeln(error['message'] as String? ?? 'Unknown error');
        buffer.writeln();
      }
    }

    return buffer.toString();
  }

  /// Format health status for report
  void _formatHealthStatus(StringBuffer buffer, Map<String, dynamic> health) {
    final workflows = health['workflows'] as Map<String, dynamic>?;
    final dependencies = health['dependencies'] as Map<String, dynamic>?;
    final issues = health['issues'] as Map<String, dynamic>?;

    // Workflows
    buffer.writeln('Workflows:');
    buffer.writeln('- Total: ${workflows?['total'] ?? 0}');
    buffer.writeln('- Successful: ${workflows?['successful'] ?? 0}');
    buffer.writeln('- Failed: ${workflows?['failed'] ?? 0}');

    // Dependencies
    buffer.writeln('\nDependencies:');
    buffer.writeln('- Vulnerable: ${dependencies?['vulnerable'] ?? 0}');
    buffer.writeln('- Outdated: ${dependencies?['outdated'] ?? 0}');

    // Issues
    buffer.writeln('\nIssues:');
    buffer.writeln('- Open Issues: ${issues?['open_issues'] ?? 0}');
    buffer.writeln('- Open PRs: ${issues?['open_prs'] ?? 0}');
    buffer.writeln('- Stale Issues: ${issues?['stale_issues'] ?? 0}');
    buffer.writeln('- Stale PRs: ${issues?['stale_prs'] ?? 0}');
  }

  /// Format fixes for report
  void _formatFixes(StringBuffer buffer, Map<String, dynamic> fixes) {
    fixes.forEach((category, data) {
      if (data is Map<String, dynamic>) {
        buffer.writeln('$category:');
        buffer.writeln('- Status: ${data['status'] as String? ?? 'unknown'}');

        if (data['status'] == 'error') {
          buffer.writeln('- Error: ${data['error'] as String? ?? 'Unknown error'}');
        } else if (data['status'] == 'success') {
          final changes = data['changes'] as Map<String, dynamic>?;
          if (changes != null) {
            buffer.writeln('- Changes:');
            changes.forEach((key, value) {
              buffer.writeln('  - $key: $value');
            });
          }
        }
      }
    });
  }
}
