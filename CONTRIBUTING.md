# Contributing to E9TH

Thank you for your interest in contributing to E9TH! This document provides guidelines for contributing to the project.

## Code of Conduct

By participating in this project, you agree to abide by our Code of Conduct. We are committed to providing a welcoming and inspiring community for all.

## How to Contribute

### Reporting Bugs

- Use the GitHub issue tracker to report bugs
- Include as much detail as possible in your bug report
- Use the bug report template when creating issues

### Suggesting Enhancements

- Use the GitHub issue tracker to suggest enhancements
- Use the feature request template when creating issues
- Provide clear descriptions of the proposed changes

### Pull Requests

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Development Setup

### Prerequisites

- Node.js 18+
- Rust 1.70+
- Solana CLI
- Git

### Local Development

1. Clone the repository
   ```bash
   git clone https://github.com/VltrnOne/E9th.git
   cd E9th
   ```

2. Run the static site locally
   ```bash
   cd static-site
   python3 -m http.server 8000
   ```

3. Build the Solana program
   ```bash
   cd program
   cargo build-sbf
   ```

## Project Structure

- `program/` - Solana program (Rust)
- `static-site/` - Static website
- `E9th app/` - React frontend (alternative)

## Coding Standards

- Follow existing code style
- Use meaningful commit messages
- Add comments for complex logic
- Test your changes thoroughly

## License

By contributing to E9TH, you agree that your contributions will be licensed under the same license as the project.

## Questions?

If you have any questions about contributing, please open an issue or contact us at [your-email@example.com].
