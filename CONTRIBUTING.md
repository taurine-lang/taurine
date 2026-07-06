# Contributing to Taurine

Thank you for your interest in contributing to Taurine!

Taurine is a young scripting language with big ambitions: to be a simple, powerful, and embeddable alternative to Lua. It's designed to be easy to learn, fast to execute, and flexible enough for games, tools, automation, and more.

Every contribution helps shape its future — whether it's fixing a typo, improving performance, or adding a new feature.

## Project Vision & Guiding Principles
Before contributing, please keep these core values in mind:
- Simplicity first – Taurine should be easy to learn and read
- General-purpose, but game-ready – Built to work anywhere, optimized for embedding
- Performance matters – Written in Rust for speed, safety, and small binaries
- Stability over breaking changes – No silent breaking changes without major version bumps and community discussion

## Reporting Bugs
If you find a bug, please open an issue with:
- A clear, descriptive title
- Steps to reproduce the issue
- Expected vs. actual behavior
- A minimal code example (if applicable)
- Your environment (OS, Taurine version, how you built/installed it)

## Suggesting Features
We love new ideas! Please open an issue with:
- What the feature is and why it's useful (for any use case)
- Example usage/syntax
- Potential trade-offs or alternatives considered
- Major syntax or core behavior changes require discussion before implementation. Please open an issue first.

## Code Contributions
1. Fork the repository
2. Create a branch:
   git checkout -b feature/your-feature-name
   # or
   git checkout -b fix/your-bug-fix
3. Make your changes
4. Test:
   cargo test
5. Format & lint:
   cargo fmt
   cargo clippy -- -D warnings
6. Commit with a clear message:
   git commit -m "feat: add nil-safe array indexing"
   # or
   git commit -m "fix: correct f-string interpolation edge case"
7. Push to your fork and open a Pull Request

## Pull Request Guidelines
- One PR = one logical change
- Include tests for new/changed functionality
- Update documentation/README if applicable
- Follow Rust conventions (cargo fmt, clippy clean)
- Be patient – I review every PR personally

### We Accept
- Bug fixes & performance improvements
- New standard library functions (general-purpose or domain-specific)
- Better error messages & diagnostics
- Documentation & example improvements
- CI/CD & tooling enhancements
- Embedding API improvements (C/Rust)

### We Don't Accept (Without Prior Discussion)
- Breaking changes to syntax or core behavior
- Features that add unnecessary complexity
- Code without tests or with linting warnings
- Large refactors without an issue/RFC first
- Changes that significantly degrade performance or binary size

## Code Style
- Use cargo fmt for all Rust code
- Fix all cargo clippy warnings
- Comment complex logic, but prefer self-explanatory code
- Follow conventional commits: feat:, fix:, docs:, chore:, etc.

## Getting Help
- Check existing issues before opening a new one
- Use the question label for help requests
- Be specific and provide context

## License
By contributing, you agree that your contributions will be licensed under the [MIT License](LICENSE).