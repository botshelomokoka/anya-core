import 'dart:convert';
import 'dart:io';
import 'package:http/http.dart' as http;

/// Automated commit cycle trigger
class CommitCycleTrigger {
  final String githubToken;
  final String owner;
  final String repo;

  CommitCycleTrigger({
    required this.githubToken,
    required this.owner,
    required this.repo,
  });

  /// Trigger the commit cycle workflow
  Future<bool> trigger({String? commitMessage}) async {
    final url = Uri.parse(
        'https://api.github.com/repos/$owner/$repo/actions/workflows/commit_cycle.yml/dispatches');

    try {
      final response = await http.post(
        url,
        headers: {
          'Accept': 'application/vnd.github.v3+json',
          'Authorization': 'token $githubToken',
          'Content-Type': 'application/json',
        },
        body: jsonEncode({
          'ref': 'main',
          'inputs': {
            'commit_message': commitMessage ?? 'Automated update',
          },
        }),
      );

      if (response.statusCode == 204) {
        print('✅ Successfully triggered commit cycle workflow');
        return true;
      } else {
        print('❌ Failed to trigger workflow: ${response.statusCode}');
        print('Response: ${response.body}');
        return false;
      }
    } catch (e) {
      print('❌ Error triggering workflow: $e');
      return false;
    }
  }

  /// Check workflow status
  Future<Map<String, dynamic>?> checkStatus() async {
    final url = Uri.parse(
        'https://api.github.com/repos/$owner/$repo/actions/runs?event=workflow_dispatch');

    try {
      final response = await http.get(
        url,
        headers: {
          'Accept': 'application/vnd.github.v3+json',
          'Authorization': 'token $githubToken',
        },
      );

      if (response.statusCode == 200) {
        final data = jsonDecode(response.body);
        final runs = data['workflow_runs'] as List;
        if (runs.isNotEmpty) {
          return runs.first;
        }
      }
    } catch (e) {
      print('❌ Error checking workflow status: $e');
    }
    return null;
  }

  /// Wait for workflow completion
  Future<bool> waitForCompletion({Duration? timeout}) async {
    final startTime = DateTime.now();
    final maxDuration = timeout ?? const Duration(minutes: 30);

    while (true) {
      if (DateTime.now().difference(startTime) > maxDuration) {
        print('❌ Workflow timeout exceeded');
        return false;
      }

      final status = await checkStatus();
      if (status == null) {
        print('❌ Unable to get workflow status');
        return false;
      }

      final conclusion = status['conclusion'];
      if (conclusion != null) {
        final success = conclusion == 'success';
        print(success
            ? '✅ Workflow completed successfully'
            : '❌ Workflow failed');
        return success;
      }

      await Future.delayed(const Duration(seconds: 30));
    }
  }
}

void main(List<String> args) async {
  // Get GitHub token from environment
  final token = Platform.environment['GITHUB_TOKEN'];
  if (token == null) {
    print('❌ GITHUB_TOKEN environment variable not set');
    exit(1);
  }

  final trigger = CommitCycleTrigger(
    githubToken: token,
    owner: 'botshelomokoka', // Replace with your GitHub username
    repo: 'anya-core', // Replace with your repository name
  );

  final message = args.isNotEmpty ? args.join(' ') : null;

  if (await trigger.trigger(commitMessage: message)) {
    print('Waiting for workflow completion...');
    final success = await trigger.waitForCompletion();
    exit(success ? 0 : 1);
  } else {
    exit(1);
  }
}
