# Contributing to WasteFi Contracts

Thank you for your interest in contributing to WasteFi!

## Development Setup

1. Install prerequisites:
```bash
make install
```

2. Setup development environment:
```bash
make setup
```

3. Run checks before committing:
```bash
make check
```

## Code Standards

- Follow Rust naming conventions
- Write comprehensive tests for all contract functions
- Document all public functions with rustdoc comments
- Keep functions focused and modular
- Use meaningful variable names

## Testing

- Write unit tests for individual functions
- Write integration tests for contract interactions
- Aim for >80% code coverage
- Test edge cases and error conditions

## Pull Request Process

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run `make check` to ensure quality
5. Commit your changes (`git commit -m 'Add amazing feature'`)
6. Push to the branch (`git push origin feature/amazing-feature`)
7. Open a Pull Request

## Commit Messages

Use conventional commit format:
- `feat:` New feature
- `fix:` Bug fix
- `docs:` Documentation changes
- `test:` Test additions/changes
- `refactor:` Code refactoring
- `chore:` Maintenance tasks

Example: `feat: add reputation scoring algorithm`

## Security

Report security vulnerabilities privately to security@wastefi.org

Do not open public issues for security concerns.
