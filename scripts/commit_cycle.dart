import 'dart:io';

/// Commit cycle utility for managing submodule and main repository commits
class CommitCycle {
  final String rootDir;
  final List<String> submodules = ['dash33', 'dependencies', 'enterprise'];

  CommitCycle(this.rootDir);

  /// Execute the full commit cycle
  Future<bool> execute() async {
    try {
      // 1. Verify clean working state
      if (!await _verifyCleanState()) {
        print(
            '❌ Working directory not clean. Please stash or commit changes first.');
        return false;
      }

      // 2. Update and commit submodules
      for (final submodule in submodules) {
        if (!await _commitSubmodule(submodule)) {
          print('❌ Failed to commit submodule: $submodule');
          return false;
        }
      }

      // 3. Update main repository references
      if (!await _updateMainReferences()) {
        print('❌ Failed to update main repository references');
        return false;
      }

      // 4. Commit main repository changes
      if (!await _commitMainRepository()) {
        print('❌ Failed to commit main repository');
        return false;
      }

      print('✅ Commit cycle completed successfully');
      return true;
    } catch (e) {
      print('❌ Error during commit cycle: $e');
      return false;
    }
  }

  /// Verify clean working state across all repositories
  Future<bool> _verifyCleanState() async {
    // Check main repository
    if (!await _isClean(rootDir)) {
      print('Main repository has uncommitted changes');
      return false;
    }

    // Check submodules
    for (final submodule in submodules) {
      final submodulePath = '$rootDir/$submodule';
      if (!await _isClean(submodulePath)) {
        print('Submodule $submodule has uncommitted changes');
        return false;
      }
    }

    return true;
  }

  /// Check if a repository is in a clean state
  Future<bool> _isClean(String path) async {
    final result = await Process.run('git', ['status', '--porcelain'],
        workingDirectory: path);
    return result.stdout.toString().trim().isEmpty;
  }

  /// Commit changes in a submodule
  Future<bool> _commitSubmodule(String submodule) async {
    final submodulePath = '$rootDir/$submodule';
    print('\n🔄 Processing submodule: $submodule');

    // Pull latest changes
    final pullResult = await Process.run('git', ['pull', 'origin', 'main'],
        workingDirectory: submodulePath);
    if (pullResult.exitCode != 0) {
      print('Failed to pull latest changes in $submodule');
      print(pullResult.stderr);
      return false;
    }

    // Stage changes
    final addResult =
        await Process.run('git', ['add', '.'], workingDirectory: submodulePath);
    if (addResult.exitCode != 0) {
      print('Failed to stage changes in $submodule');
      print(addResult.stderr);
      return false;
    }

    // Check if there are changes to commit
    final statusResult = await Process.run('git', ['status', '--porcelain'],
        workingDirectory: submodulePath);
    if (statusResult.stdout.toString().trim().isEmpty) {
      print('No changes to commit in $submodule');
      return true;
    }

    // Commit changes
    final commitResult = await Process.run(
        'git', ['commit', '-m', 'feat: Updated $submodule with latest changes'],
        workingDirectory: submodulePath);
    if (commitResult.exitCode != 0 &&
        !commitResult.stderr.toString().contains('nothing to commit')) {
      print('Failed to commit changes in $submodule');
      print(commitResult.stderr);
      return false;
    }

    // Push changes
    final pushResult = await Process.run('git', ['push', 'origin', 'main'],
        workingDirectory: submodulePath);
    if (pushResult.exitCode != 0) {
      print('Failed to push changes in $submodule');
      print(pushResult.stderr);
      return false;
    }

    print('✅ Successfully processed submodule: $submodule');
    return true;
  }

  /// Update main repository references
  Future<bool> _updateMainReferences() async {
    print('\n🔄 Updating main repository references');

    // Add submodule references
    final addResult = await Process.run('git', ['add'] + submodules,
        workingDirectory: rootDir);
    if (addResult.exitCode != 0) {
      print('Failed to add submodule references');
      print(addResult.stderr);
      return false;
    }

    // Commit submodule updates
    final commitResult = await Process.run(
        'git', ['commit', '-m', 'chore: Updated submodule references'],
        workingDirectory: rootDir);
    if (commitResult.exitCode != 0 &&
        !commitResult.stderr.toString().contains('nothing to commit')) {
      print('Failed to commit submodule references');
      print(commitResult.stderr);
      return false;
    }

    return true;
  }

  /// Commit changes in main repository
  Future<bool> _commitMainRepository() async {
    print('\n🔄 Processing main repository');

    // Pull latest changes
    final pullResult = await Process.run('git', ['pull', 'origin', 'main'],
        workingDirectory: rootDir);
    if (pullResult.exitCode != 0) {
      print('Failed to pull latest changes in main repository');
      print(pullResult.stderr);
      return false;
    }

    // Stage changes
    final addResult =
        await Process.run('git', ['add', '.'], workingDirectory: rootDir);
    if (addResult.exitCode != 0) {
      print('Failed to stage changes in main repository');
      print(addResult.stderr);
      return false;
    }

    // Check if there are changes to commit
    final statusResult = await Process.run('git', ['status', '--porcelain'],
        workingDirectory: rootDir);
    if (statusResult.stdout.toString().trim().isEmpty) {
      print('No changes to commit in main repository');
      return true;
    }

    // Commit changes
    final commitResult = await Process.run('git',
        ['commit', '-m', 'feat: Updated main repository with latest changes'],
        workingDirectory: rootDir);
    if (commitResult.exitCode != 0 &&
        !commitResult.stderr.toString().contains('nothing to commit')) {
      print('Failed to commit changes in main repository');
      print(commitResult.stderr);
      return false;
    }

    // Push changes
    final pushResult = await Process.run('git', ['push', 'origin', 'main'],
        workingDirectory: rootDir);
    if (pushResult.exitCode != 0) {
      print('Failed to push changes in main repository');
      print(pushResult.stderr);
      return false;
    }

    print('✅ Successfully processed main repository');
    return true;
  }
}

void main(List<String> args) async {
  if (args.isEmpty) {
    print('Usage: dart commit_cycle.dart <root_directory>');
    exit(1);
  }

  final rootDir = args[0];
  final commitCycle = CommitCycle(rootDir);

  final success = await commitCycle.execute();
  exit(success ? 0 : 1);
}
