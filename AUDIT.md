# Sorcery Repository Audit: Conformity to Test-Bound Process

This document lists all files and references that do not conform to the new test-bound Sorcery process. Items marked for removal, update, or review.

## Deprecated Tools
- **tools/glyph-verify/**: Entire directory. Old AST-based verifier using !/- semantic matching. Superseded by tools/glyph/ which uses $ obligations and test enforcement.
  - Remove directory and all references.

## Documentation Updates Needed
- **README.md**:
  - Lines 143,147,153: References to `glyph-verify` should be updated to `glyph` (the new tool).
  - Line 377: "Declare guarantees, assumptions, exclusions" → "Declare obligations, assumptions".
  - Section "Slice Gating": Update to mention obligations instead of guarantees/exclusions.
- **Glyph_Master_Plan.md**: Entire file appears to be old doctrine. References ! guarantees, exclusions. Either update or remove if superseded.
- **examples/WHITEPAPER_EXPERIMENT_ASSESSMENT.md**: References "guarantees and exclusions" – update to obligations.
- **examples/REHYDRATION_COMPARISON_V1_V2.md**: Check for old terms.
- **examples/LESSONS_CONTEXT_ASYMMETRY.md**: Check for old terms.

## Example Files Needing Updates
- **examples/syntax/glyph_verify.invocation.spell**: Still uses !/- syntax. Update to $ obligations.
- **examples/syntax/glyph_verify.spell**: Same.
- **examples/syntax/inference_abi.spell**: Check syntax.
- **examples/syntax/parse.spell**: Check.
- **examples/syntax/tokenize.spell**: Check.
- **examples/inference_abi.spell**: Check.
- **examples/parse.spell**: Check.
- **examples/tokenize.spell**: Check.

## Whitepapers with Old References
- All example WHITEPAPER.md files: While updated with new sections, some may retain old terminology in original sections. Ensure consistency.
- **examples/shimmy/WHITEPAPER.md**: Title updated, but content may need review for old terms.

## Code Files
- **tools/glyph/src/lib.rs**: Appears updated, but verify no remnants of old parsing.
- **tools/glyph/demo.glyph**: Updated to $.
- **tools/glyph/glyph.ebnf**: Updated.

## Case Studies
- **case-studies/**: Review for conformity. May be production examples, but ensure they use new syntax if applicable.

## Other
- **examples/WHITEPAPER_EXPERIMENT_ASSESSMENT.md**: Line 57 references "guarantees and exclusions".
- Any remaining .md files in examples/ subdirs: Check for old doctrine.

## Action Plan
1. ✅ Remove tools/glyph-verify/ directory.
2. ✅ Update README.md references from glyph-verify to glyph.
3. ✅ Convert all syntax examples from !/- to $ obligations with test bindings.
4. ✅ Update references to obligations instead of guarantees/exclusions in main docs.
5. ✅ Remove Glyph_Master_Plan.md as superseded.
6. ✅ Update whitepaper references to new terminology.
7. ✅ Update all .spell files in examples/ to $ syntax.

**Audit Complete:** Repository cleaned of deprecated AST-based glyph-verify code and updated to conform to test-bound Sorcery process with $ obligations.
6. Ensure all spells use $ obligations.
7. Test that tools/glyph/ works with updated examples.