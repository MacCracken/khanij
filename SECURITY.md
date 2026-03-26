# Security Policy

## Supported Versions

| Version | Supported |
|---------|-----------|
| 0.1.x   | Yes       |

## Reporting a Vulnerability

If you discover a security vulnerability in Khanij, please report it responsibly:

1. **Do not** open a public issue
2. Email the maintainers at the address listed in the repository
3. Include a description of the vulnerability and steps to reproduce
4. Allow reasonable time for a fix before public disclosure

We aim to acknowledge reports within 48 hours and provide a fix or mitigation
within 7 days for critical issues.

## Scope

Khanij is a scientific computation library. Security concerns primarily involve:

- **Denial of service**: Inputs that cause excessive computation or memory usage
- **Numerical overflow**: Calculations that produce incorrect results silently
- **Dependency vulnerabilities**: Issues in upstream crates

We run `cargo audit` and `cargo deny` in CI to catch known dependency
vulnerabilities.
