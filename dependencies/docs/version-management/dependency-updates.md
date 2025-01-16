# Dependency Updates

This page documents how to update dependencies in the Anya Core repository.

## When to Update Dependencies

There are two reasons to update dependencies:

1. A new version of a dependency has been released with a security fix.
2. A new version of a dependency has been released with a feature we want or need.

## How to Update Dependencies

To update a dependency, follow these steps:

1. Check the [Crate page](https://crates.io/) for the dependency to see if there is a new version available.
2. Run `cargo update` to update the dependency.
3. Test your changes to make sure they work as expected.
4. Commit your changes with a message that includes the words "update dependency [dependency name]".

## Best Practices for Updating Dependencies

Here are some best practices to follow when updating dependencies:

* Only update dependencies when necessary. If a new version of a dependency has been released with a security fix, update the dependency. If a new version of a dependency has been released with a feature we want or need, update the dependency.
* Test your changes to make sure they work as expected. This includes running the test suite and testing your code manually.
* Commit your changes with a message that includes the words "update dependency [dependency name]". This makes it easier for other developers to know why the dependency was updated.
