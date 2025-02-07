# Release Process

## Preparing a Release Candidate

1. Ensure all features and fixes are merged into the `develop` branch.
2. Run comprehensive tests to verify stability.
3. Update version numbers and changelog.

## Merging to `main`

1. Create a PR from `develop` to `main`.
2. Review and approve the PR.
3. Merge the PR into `main`.

## Tagging and Publishing

1. Tag the release with the version number, e.g., `v0.1`.
2. Publish the release on GitHub.
3. Update documentation and notify stakeholders.

## Post-Release

1. Create a new `develop` branch from `main` for future development.
2. Update `requirements.txt` with stable dependencies.
3. Begin work on the next release cycle.
