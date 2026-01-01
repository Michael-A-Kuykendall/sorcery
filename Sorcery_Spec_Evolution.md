# Evolving Sorcery: From Glyph Notation to Specification-Driven Development

## Overview of Current Notation and Ideas

The Sorcery project uses a custom notation called Glyph for "dehydrating and hydrating intent." This involves specifying complex logic, architectures, and processes in a concise, symbolic format (referred to as "spells"), which can then be "rehydrated" into code or validated against implementations. The notation draws from experiments in slice-gated engineering, where minimal representations capture full application intent.

### Key Elements of Glyph Notation
Based on the syntax examples and files like `glyph_verify.invocation.spell`, Glyph uses symbols to define specifications:
- `#`: Spell/module name
- `^`: Intent (required description of purpose)
- `@`: Entity/Component (e.g., Parser, Comparator)
- `:`: Contract (input → output, defining interfaces)
- `!`: Guarantee (must-have properties or behaviors)
- `~`: Assumption (contextual expectations)
- `-`: Exclusion (prohibited elements)
- `>`: Dependency (links to other spells or components)
- `?`: Open question (unresolved aspects)

Example from `glyph_verify.invocation.spell`:
```
#Spell: ConstraintGate
^ Intent: deterministically verify that an artifact description covers a specification’s declared constraints

@Parser
  : constraint_text -> ast
  ! deterministic
  ! stable_normalization
  - semantic_inference
  - network
  - filesystem_writes
  ~ grammar_is_#^@:!~->?

@Comparator
  : (spec_ast, artifact_ast) -> verdict
  ! fails_on_missing_intent
  ! fails_on_missing_guarantees
  ! fails_on_missing_exclusions
  ! fails_on_missing_dependencies
  ! emits_actionable_diff
  - guessing
  - nondeterminism
  ~ artifact_description_written_from_output

@Verdict
  : input -> pass_fail
  ! binary
  - partial_success
```

This notation has proven capable of representing diverse logic (e.g., parsing, inference, event handling) without needing external frameworks. Successes include effective dehydration/hydration for features and shorthand representation. Challenges include confusing terminology ("spells," "glyphs"), self-validation issues in AST parsing (AI checking itself without external handshake), and inconsistent application in CICD.

### Vision: Glyph as an AI-Teachable Language for Spec-Driven Development
Transform Glyph into a structured language that AI can learn and use consistently, enabling:
- **Dehydration in Discussions**: Capture complex ideas (e.g., cloud architectures) precisely during large-scale talks, converting them into Glyph specs without loss.
- **Rehydration**: Generate code/tools systematically from specs, or validate existing code against specs for compliance. If code doesn't match, it's invalid—creating an "AI assistant development language" where AI outputs are enforced against intent.
- **AI Integration**: Teach AI the grammar and semantics via examples and prompts, treating it as untrusted input. Use validation to ensure AI-generated code adheres to guarantees/exclusions, preventing hallucinations.

This positions Glyph as a bridge between human intent and machine execution, with rehydration as the "missing nut" for enforcement.

### Comparison to Modern Spec Builders
- **Similarities**: Glyph resembles specification-driven systems like OpenAPI (API contracts), Protocol Buffers (data schemas), or BDD tools like Cucumber (executable specs). It defines contracts, constraints, and dependencies declaratively, aiming for spec-first development.
- **Differences**: Modern systems often use JSON/YAML for tooling and readability, while Glyph's symbols prioritize brevity. They emphasize ecosystems (e.g., OpenAPI's code generators), whereas Glyph is experimental and self-contained. The "AI self-checking" problem mirrors validation gaps in spec tools, often addressed via automated linters or CI.

### Proposed Evolution
To transform Glyph into a specification-driven process:
1. **Formalize the Grammar**: Use EBNF or tools like ANTLR to define a complete, machine-parseable grammar.
2. **Decouple Spec from Hydration**: Treat spells as immutable specs; build decanting tools to generate code from them.
3. **Enhance Validation and CI**: Implement external linters and CI checks for consistency and spec adherence.
4. **Professionalize Terminology**: Shift from "spells" to "specs" or "modules" for clarity.
5. **Automatic Spec Building**: Develop reverse-engineering tools to generate Glyph specs from existing codebases.
6. **Prototype Workflow**: Start with small specs, hydrate to code, and validate iteratively.

This could position Glyph as a lightweight alternative to JSON-based spec builders, focusing on intent dehydration.

## Formalizing the Grammar with EBNF and ANTLR

The next critical step is formalizing Glyph's grammar. This involves defining the syntax rules precisely, enabling automated parsing, validation, and tooling. Two key approaches are Extended Backus-Naur Form (EBNF) for defining the grammar and ANTLR (ANother Tool for Language Recognition) for generating parsers from it.

### What is EBNF?
EBNF is a meta-syntax for describing the grammar of formal languages, including programming languages, DSLs, and notations like Glyph. It extends the original Backus-Naur Form (BNF) with operators for repetition, optionality, and grouping, making it more expressive.

- **Purpose**: EBNF provides a human-readable, unambiguous definition of language rules. For Glyph, it would specify how symbols like `#`, `^`, `@`, etc., combine to form valid "spells."
- **Key Elements**:
  - **Terminals**: Literal symbols (e.g., `#`, `^`).
  - **Non-terminals**: Named rules (e.g., `<spell>`, `<intent>`).
  - **Operators**:
    - `|` : Alternation (choice, e.g., `a | b`).
    - `[]` : Optionality (e.g., `[optional]`).
    - `{}` : Repetition (zero or more, e.g., `{item}`).
    - `()` : Grouping.
- **Example for Glyph** (simplified sketch based on syntax):
  ```
  <spell> ::= '#' <name> <intent> {<entity>} {<dependency>} {<question>}
  <intent> ::= '^' <text>
  <entity> ::= '@' <name> <contract> {<guarantee>} {<exclusion>} {<assumption>}
  <contract> ::= ':' <input> '->' <output>
  <guarantee> ::= '!' <text>
  <exclusion> ::= '-' <text>
  <assumption> ::= '~' <text>
  <dependency> ::= '>' <name>
  <question> ::= '?' <text>
  ```
  This defines the structure: A spell starts with `#` and a name, followed by an intent (`^`), then zero or more entities (`@`), etc. It ensures rules like "every entity must have a contract" or "guarantees are lists under entities."

- **Benefits**: Clarity and precision. EBNF acts as documentation and a blueprint for implementation. It helps identify ambiguities (e.g., is `~` always optional?) and ensures the notation is deterministic.

### What is ANTLR?
ANTLR is a powerful parser generator tool that takes a grammar definition (often in EBNF-like syntax) and automatically generates parsers in languages like Java, C#, Python, or TypeScript. It's widely used for building compilers, interpreters, and DSL tools.

- **How It Works**:
  1. **Input**: You write a grammar file (e.g., `Glyph.g4`) in ANTLR's syntax, which is EBNF-based.
  2. **Generation**: ANTLR processes the file and outputs parser code (e.g., a lexer and parser in your target language).
  3. **Usage**: Integrate the generated parser into your app to parse Glyph text into an AST (Abstract Syntax Tree), which can then be analyzed, validated, or transformed.
- **Example Workflow for Glyph**:
  - Define grammar in `Glyph.g4` (similar to the EBNF sketch above).
  - Run ANTLR: `antlr4 -Dlanguage=TypeScript Glyph.g4` (generates TypeScript files).
  - Use the parser: Input a spell string, get an AST representing the structure (e.g., nodes for intent, entities, guarantees).
- **Features**: Supports advanced parsing (e.g., error handling, tree walking), and can generate visitors/listeners for processing the AST.

### What Does Formalizing with EBNF/ANTLR Give Us?
- **Automated Parsing**: Instead of manual AST parsing (which failed due to self-validation), you get a reliable, generated parser that handles syntax errors and produces structured data. This enables deterministic hydration/dehydration.
- **Validation and Tooling**: Build linters to enforce rules (e.g., "all entities must have guarantees"). Integrate with IDEs for syntax highlighting, auto-completion, and error detection. Use in CI to validate specs against code.
- **Code Generation and Decanting**: Traverse the AST to generate code (e.g., convert a Glyph spec to TypeScript classes or Rust structs). This automates the "decanting from spec" you mentioned, similar to how OpenAPI generates API clients.
- **Extensibility and Maintenance**: Changes to the grammar (e.g., adding new symbols) are centralized. The generated parser ensures consistency across tools.
- **Interoperability**: Export ASTs to formats like JSON for integration with other systems (e.g., feed into code generators or validation frameworks).
- **Error Handling**: ANTLR provides detailed error messages (e.g., "unexpected symbol at line 5"), improving on the "theater" of AI self-checks.

### What It Looks Like at the End
Once defined, you have:
- A grammar file (e.g., `Glyph.g4`) as the single source of truth for Glyph syntax.
- Generated parser code (e.g., in TypeScript) that you can import and use: `const ast = parseGlyph(spellText);`.
- Tools built on top: A CLI for validation (`glyph-verify spell.spell`), a code generator (`glyph-decant spell.spell --lang=ts`), and CI scripts.
- A mature DSL: Glyph becomes a professional spec language, capable of driving full application development without frameworks. For example, write a Glyph spec for an API, parse it, validate against implementation, and generate boilerplate code.

This step addresses the AST parser's shortcomings by providing external, automated validation. It preserves Glyph's minimalist power while making it toolable and scalable.

### Next Steps
- Draft a full EBNF grammar based on existing spells.
- Set up ANTLR in the project (e.g., add to `package.json` or as a tool).
- Prototype parsing a sample spell and generating a simple AST.
- Discuss integrating with hydration processes or CI.

## Turning Glyph into an AI-Teachable Language

To realize Glyph as a language for AI-assisted development:
1. **Formalize and Teach Grammar**: Use EBNF/ANTLR to define unambiguous rules. Create AI training data: Examples of Glyph specs with explanations (e.g., "This ! guarantee means..."). Prompt AI with: "Convert this discussion into Glyph spec, ensuring all ! guarantees are explicit."
2. **Build Rehydration Tools**:
   - **Validation Mode**: Parse code ASTs and compare to Glyph ASTs—fail if guarantees/exclusions/assumptions are missing or violated. This enforces "if it doesn't match, you didn't do it."
   - **Codegen Mode**: Generate boilerplate code from Glyph (e.g., TypeScript interfaces from @ entities). Start minimal: Contracts → function signatures, guarantees → assertions.
3. **Enforce Compliance**: Integrate into workflows—AI outputs must pass validation. Use CI to check PRs: "Does this code satisfy the Glyph spec?"
4. **Iterate with AI**: Test by having AI generate Glyph from prompts, then validate/re-generate code. Refine grammar for clarity (e.g., avoid ambiguous symbols).
5. **Prototype**: Draft a "Glyph Compliance Contract" (per GPT 5.2), then implement a basic parser and validator in TypeScript.

This makes Glyph a "development language" where AI is guided by specs, not free-form, ensuring intent preservation at scale.</content>
<parameter name="filePath">c:\Users\micha\repos\sorcery\Sorcery_Spec_Evolution.md