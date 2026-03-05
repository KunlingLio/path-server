# Changelog

All notable changes to the Path Server will be documented in this file.

## [Unreleased]
### Fixed
- **Zed Extension**: Fix version-compatibility check — correctly parse the major version so `v10.x.x` is not mistaken for `v1.x.x`.

## [0.2.0] - 2026-03-04
### Added
- **VS Code Extension**: Initial release of VS Code Extension with Path Server support.
    - Self-contained extension providing Path Server integration.
    - Basic path auto-completion for relative and absolute paths across programming languages from Path Server.
    - Log redirection to the "Output" panel.
- **Zed Extension**: Initial release of Zed Extension with Path Server support.
    - Auto-download and automatic upgrades of the Path Server executable.
    - Basic path auto-completion for relative and absolute paths across programming languages from Path Server.

### Changed
- **Path Server**: Refactored completion logic to improve maintainability.
- Repository reorganized into a monorepo (consolidated `path-server-zed` and `path-server-vscode` into `path-server`).
- Change release assets naming style from `-` to `_` for readability.
- Improved README readability.

## [0.1.0] - 2026-03-03
Initial release of **Path Server**.

### Added
- **Path Server**: Support path completion, both relative and absolute paths.
- **Path Server**: Support relative path based on workspace root or current document.