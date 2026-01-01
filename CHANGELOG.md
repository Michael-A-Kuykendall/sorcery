# Changelog

All notable changes to this repository’s doctrine and example methodology will be documented in this file.

This project is experimental and documentation-led. The primary “API surface” is the doctrine + glyph semantics.

## [1.0.0] - 2026-01-01

### Added
- **Test-Bound Doctrine**: Complete shift from AST-based semantic analysis to runtime test verification
- **Comprehensive Test Suites**: 85+ executable tests across 6 complete examples proving behavioral fidelity
- **AUDIT.md**: Repository conformity audit document
- **SORCERY_COMPLETION_PLAN.md**: Detailed completion tracking and methodology documentation

### Changed
- **Test-Bound Reset**: Complete migration from AST-based semantic analysis to runtime test verification
- Updated all spell obligations from `!`/`-` syntax to `$ require`/`$ forbid`/`$ prove` with test bindings
- Removed deprecated glyph-verify tool (AST parsing) in favor of test-only verification
- Fixed tokenize.spell obligation types (deterministic/locale_free/order_preserved now use `$ prove`)
- Updated README incantation patterns and examples to use modern `$` obligation syntax
- Corrected completion plan test counts (fast-deep-equal: 16 tests, ms: 43 tests)
- Updated shimmy spells to use `$ prove` obligations with test bindings

### Removed
- **glyph-verify tool**: Deprecated AST-based verifier superseded by test-bound approach
- **Legacy Documentation**: Removed outdated doctrine files (Glyph_Master_Plan.md, etc.)
- **Committed Dependencies**: Removed node_modules and package files from examples

## [0.1.0] - 2025-12-17

### Added
- Initial changelog.
