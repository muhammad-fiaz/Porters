# Security Policy

## Supported Versions

We release patches for security vulnerabilities. Currently supported versions:

| Version | Supported          |
| ------- | ------------------ |
| latest  | :white_check_mark: |

## Reporting a Vulnerability

We take the security of Porters seriously. If you believe you have found a security vulnerability, please report it to us responsibly.

**Please do NOT report security vulnerabilities through public GitHub issues.**

Instead, please report them via one of the following methods:

### Preferred Method: Security Advisory

1. Go to the [Security tab](https://github.com/muhammad-fiaz/porters/security)
2. Click "Report a vulnerability"
3. Fill out the form with details about the vulnerability

### Alternative Method: Email

Send an email to: [contact@muhammadfiaz.com](mailto:contact@muhammadfiaz.com)

Include the following information:

- **Type of vulnerability** (e.g., code execution, denial of service, privilege escalation)
- **Full paths of source file(s)** related to the manifestation of the vulnerability
- **Location of the affected source code** (tag/branch/commit or direct URL)
- **Step-by-step instructions** to reproduce the issue
- **Proof-of-concept or exploit code** (if possible)
- **Impact** of the vulnerability, including how an attacker might exploit it

### What to Expect

- **Acknowledgment**: We'll acknowledge receipt of your vulnerability report within 48 hours
- **Progress Updates**: We'll send you regular updates about our progress (at minimum every 7 days)
- **Disclosure Timeline**: We aim to fully disclose vulnerabilities within 90 days of initial report
- **Credit**: We'll publicly credit you for the discovery (unless you prefer to remain anonymous)

## Security Update Process

1. **Assessment**: We evaluate the vulnerability and determine its severity
2. **Fix Development**: We develop and test a fix in a private branch
3. **Security Advisory**: We create a security advisory (if severity warrants it)
4. **Coordinated Disclosure**: We coordinate with you on disclosure timing
5. **Release**: We release the patched version and publish the advisory
6. **Notification**: We notify users via GitHub releases and security advisories

## Vulnerability Severity Classification

We use the CVSS (Common Vulnerability Scoring System) to assess vulnerability severity:

- **Critical** (9.0-10.0): Remote code execution, privilege escalation
- **High** (7.0-8.9): Denial of service, information disclosure
- **Medium** (4.0-6.9): Security feature bypass
- **Low** (0.1-3.9): Minor security improvements

## Security Best Practices for Porters Users

When using Porters in your projects:

1. **Keep Updated**: Always use the latest version of Porters
2. **Verify Dependencies**: Review dependencies added to your project
3. **Trusted Sources**: Only add dependencies from trusted sources
4. **Code Review**: Review generated build files before committing
5. **Permissions**: Be cautious with custom commands that require elevated permissions
6. **Secrets**: Never commit access tokens or secrets to porters.toml
7. **Registry Trust**: Verify package integrity when using third-party registries

## Security Features

Porters includes several security features:

- **Lockfile Integrity**: porters.lock ensures dependency reproducibility
- **Hash Verification**: Downloaded packages are verified using SHA-256 hashes
- **No Arbitrary Execution**: Build systems are invoked through controlled interfaces
- **Sandboxed Builds**: Dependency builds are isolated in separate directories
- **HTTPS by Default**: All registry communication uses HTTPS

## Known Security Considerations

### Build System Execution

Porters executes build systems (CMake, Make, etc.) which can run arbitrary commands:

- **Risk**: Malicious packages can execute arbitrary code during build
- **Mitigation**: Only use packages from trusted sources and review build files
- **Best Practice**: Use `--dry-run` to preview changes before applying

### Custom Commands

Custom commands defined in porters.toml can execute arbitrary shell commands:

- **Risk**: Malicious porters.toml can execute harmful commands
- **Mitigation**: Review porters.toml files from untrusted sources
- **Best Practice**: Run projects in isolated environments (containers, VMs)

### Git Dependencies

Dependencies can be pulled from Git repositories:

- **Risk**: Git repositories can change after you add them
- **Mitigation**: Use specific tags or commit hashes instead of branches
- **Best Practice**: Pin dependencies to specific versions in porters.lock

## Bug Bounty Program

We don't currently have a formal bug bounty program, but we deeply appreciate security researchers who help improve Porters' security. Significant findings may be eligible for:

- Public recognition in our Hall of Fame
- Swag and merchandise
- Direct collaboration on the fix

## Security Hall of Fame

We'd like to thank the following security researchers for their contributions:

- *Your name could be here!*

## Scope

### In Scope

- The Porters CLI tool itself
- Build system integrations (CMake, XMake, Meson, etc.)
- Dependency resolution and fetching
- Package registry interactions
- Cross-compilation features
- Custom command execution
- Extension system

### Out of Scope

- Third-party dependencies (report to their maintainers)
- Build systems themselves (CMake, Make, etc.)
- User projects created with Porters
- Social engineering attacks
- Physical attacks
- Denial of service attacks against GitHub or other external services

## Contact

For general security questions or concerns, you can:

- Open a discussion in the [Discussions tab](https://github.com/muhammad-fiaz/porters/discussions)
- Contact the maintainer: Muhammad Fiaz ([@muhammad-fiaz](https://github.com/muhammad-fiaz))

## Additional Resources

- [OWASP Secure Coding Practices](https://owasp.org/www-project-secure-coding-practices-quick-reference-guide/)
- [CWE Top 25 Most Dangerous Software Weaknesses](https://cwe.mitre.org/top25/)
- [GitHub Security Best Practices](https://docs.github.com/en/code-security)

---

**Last Updated**: November 6, 2025

Thank you for helping keep Porters and its users safe! 🔒
