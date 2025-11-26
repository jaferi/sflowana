# Security Policy

## Reporting a Vulnerability

Please do not report security vulnerabilities through public GitHub issues.

For a vulnerability affecting Sflowana, please use GitHub's private security reporting mechanism or contact the project maintainers privately.

Please include:

* A description of the vulnerability
* Steps to reproduce the issue
* Affected versions or commits
* Potential impact
* Any suggested mitigation, if available

## Supported Versions

Sflowana is currently under active development.

Security support is provided for the latest development version. Because the project is pre-release, APIs, architecture, and security characteristics may change as development progresses.

Older versions may not receive security fixes.

## Scope

Security reports are particularly relevant to:

* Memory safety issues
* Remote code execution
* Denial-of-service vulnerabilities
* Malformed transaction or instruction parsing
* Protocol decoder vulnerabilities
* RPC or streaming input handling
* WebSocket or network-service vulnerabilities
* Unexpected data exposure
* Authentication or authorization bypasses, where applicable
* Resource exhaustion and unbounded memory growth
* Container or deployment security issues

## Out of Scope

The following are generally outside the scope of Sflowana's security policy:

* Vulnerabilities in external Solana RPC providers
* Vulnerabilities in Solana itself
* Vulnerabilities in third-party protocols integrated by Sflowana
* Issues requiring physical access to the host
* General performance issues without a security impact

However, if Sflowana's handling of an external dependency creates a security vulnerability in Sflowana itself, please report it.

## Disclosure

Please allow reasonable time for investigation and remediation before publicly disclosing a vulnerability.

Security fixes may be released as a patch, configuration change, or architectural change depending on the severity and nature of the issue.
