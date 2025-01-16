# Version Control

Version control is a critical part of the Anya Project's development process. We use a combination of Git and GitHub to manage our codebase. This document outlines the best practices for using version control in the Anya Project.

## Git Repositories

The Anya Project has multiple Git repositories. These are split into two main categories: core and dependencies.

### Core Repositories

The core repositories are the main codebases for the Anya Project. They are owned by the Anya Project GitHub organization.

- `anya-core`: The main Anya Project codebase.
- `anya-enterprise`: The enterprise version of the Anya Project.

### Dependencies

The dependencies repositories are third-party libraries that are used by the Anya Project. These are not owned by the Anya Project GitHub organization.

- `bitcoin-rs`: A Rust library for Bitcoin.
- `lightning-rs`: A Rust library for the Lightning Network.
- `web5-rs`: A Rust library for Web5.
- `stacks-rs`: A Rust library for Stacks.

## Git Branches

The Anya Project uses a Git branching strategy to manage the development of new features and bug fixes. This strategy is based on the Git Flow model.

- `main`: The main branch of the Anya Project. This is the branch that will be used to build the production version of the Anya Project.
- `develop`: The development branch of the Anya Project. This is the branch that is used to develop new features and bug fixes.
- `feature/*`: The feature branches of the Anya Project. These are branches that are used to develop new features.
- `release/*`: The release branches of the Anya Project. These are branches that are used to prepare a new release of the Anya Project.
- `hotfix/*`: The hotfix branches of the Anya Project. These are branches that are used to develop bug fixes.

## Git Commits

The Anya Project uses a Git commit message format to ensure that the commit messages are consistent and easy to understand.

- The first line of the commit message should be a brief summary of the changes.
- The second line of the commit message should be a blank line.
- The third line of the commit message should include a more detailed description of the changes.
- The fourth line of the commit message should include a list of the files that were modified.
- The fifth line of the commit message should include a reference to the issue that the commit is intended to fix.

## Git Worktrees

The Anya Project uses Git worktrees to manage different features and versions of the project. Worktrees are used to create a new branch for a feature or bug fix.

- Create a new worktree for a feature:

  
