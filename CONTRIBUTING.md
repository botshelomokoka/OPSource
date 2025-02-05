# Contributing to OPSource

We love your input! We want to make contributing to OPSource as easy and transparent as possible.

## Development Process

1. Fork the repo and create your branch from `main`.
2. If you've added code that should be tested, add tests.
3. If you've changed APIs, update the documentation.
4. Ensure the test suite passes.
5. Make sure your code lints.
6. Issue that pull request!

## Repository Structure and Submodules

### Component Organization

The project is organized into several key components, each maintained as a separate repository:

1. **Core AI Components** (`anya-core/`):
   - `dash33/`: AI decision engine and analytics
   - `enterprise/`: Enterprise integration and business logic
   - `mobile/`: Mobile interface and platform-specific code
   - `web5-rs/`: Web5 implementation in Rust

2. **Enterprise Components** (`enterprise/`):
   - Focused enterprise-specific implementations
   - Direct integration with core components
   - Custom business logic and workflows

### Working with Submodules

When contributing to the project, follow these guidelines for working with submodules:

1. **Initial Setup**:
   ```bash
   # Clone the repository with all submodules
   git clone --recursive git@github.com:botshelomokoka/OPSource.git
   
   # If already cloned, initialize and update submodules
   git submodule update --init --recursive
   ```

2. **Making Changes**:
   - Work in the appropriate component directory
   - Commit changes in the submodule first
   - Update the parent repository to point to the new commit

3. **Updating Submodules**:
   ```bash
   # Update all submodules to their latest versions
   git submodule update --remote
   
   # Update a specific submodule
   git submodule update --remote anya-core/[component]
   ```

4. **Creating Pull Requests**:
   - Submit separate PRs for submodule and parent repository changes
   - Reference related PRs in your commit messages
   - Ensure CI passes in both repositories

### Best Practices

1. **Code Organization**:
   - Keep related code within its appropriate component
   - Avoid duplicate implementations across components
   - Use clear and consistent naming conventions

2. **Dependencies**:
   - Minimize cross-component dependencies
   - Document any new dependencies clearly
   - Keep dependency versions in sync across components

3. **Testing**:
   - Write tests for both component and integration levels
   - Ensure all tests pass before submitting PRs
   - Include documentation updates with code changes

## Pull Request Process

1. Update the README.md with details of changes to the interface
2. Update the docs/ with any new information
3. The PR will be merged once you have the sign-off of two other developers

## Any contributions you make will be under the MIT Software License

In short, when you submit code changes, your submissions are understood to be under the same [MIT License](http://choosealicense.com/licenses/mit/) that covers the project. Feel free to contact the maintainers if that's a concern.

## Report bugs using Github's [issue tracker](https://github.com/botshelomokoka/OPSource/issues)

We use GitHub issues to track public bugs. Report a bug by [opening a new issue](https://github.com/botshelomokoka/OPSource/issues/new/choose).

## Write bug reports with detail, background, and sample code

**Great Bug Reports** tend to have:

- A quick summary and/or background
- Steps to reproduce
  - Be specific!
  - Give sample code if you can.
- What you expected would happen
- What actually happens
- Notes (possibly including why you think this might be happening, or stuff you tried that didn't work)

## License

By contributing, you agree that your contributions will be licensed under its MIT License.
