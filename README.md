<p align="center">
  <img src="assets/sorcery-logo.png" alt="Sorcery" width="350">
</p>

<p align="center">
  <strong>🧙‍♂️ Compress genius into code. Dehydrate big ideas into spells that AI can actually execute.</strong>
</p>

<p align="center">
  <em>The missing link between your brilliant architecture vision and code that doesn't suck.</em>
</p>

<p align="center">
  <a href="#the-problem">The Problem</a> •
  <a href="#the-solution">The Solution</a> •
  <a href="#quick-start">Quick Start</a> •
  <a href="#how-it-works">How It Works</a> •
  <a href="#examples">Examples</a> •
  <a href="#get-started">Get Started</a>
</p>

---

> **⚠️ Heads up:** Sorcery is a *design doctrine* for architecture handoff—not a framework, language, or product. Think of it as a superpower for making your intent bulletproof.

---

## 🔥 The Problem

Ever poured your soul into explaining a complex system, only to watch it get mangled by:

- 🤖 AI assistants that "get it" but then hallucinate nonsense
- 👶 Junior devs who nod along but miss the crucial tradeoffs
- 🕰️ Future-you who stares at the code like "WTF was I thinking?"

**Your genius evaporates at handoff.** Deep constraints, architectural vision, and "obvious" decisions vanish into thin air. Teams waste weeks rediscovering what you already figured out.

**Software development eats context for breakfast.** And it's starving your projects.

---

## ✨ The Solution

**Sorcery** is shorthand notation for dehydrating large thoughts into reusable, executable spells.

- **Dehydrate:** Compress high-context genius into tiny, unambiguous artifacts
- **Rehydrate:** Let AI/juniors expand them into perfect code without guessing

It's like giving your brain a lossless compression algorithm for software design.

**One sentence:** *Sorcery casts high-context intent into constrained spells so lower-context agents can act correctly without shared memory.*

---

## 🚀 Quick Start

Want to see the magic? Here's a spell that defines a tokenizer:

```glyph
#Spell: Tokenize
^ Intent: produce stable, cross-platform tokens for deterministic inference

@Tokenizer
  : utf8 -> tokens
  ! deterministic
  ! locale_free
  - network
  - persistence
  ~ valid_utf8
```

**This isn't code—it's intent.** It tells AI exactly what must happen (and what must *never* happen) so it can write the implementation flawlessly.

No more "I think you meant..."—just perfect execution.

---

## 🛠️ How It Works

### The Core Asymmetry

Sorcery enforces genius at the top, automation below:

| Phase | Who Does It | What Happens |
|-------|-------------|--------------|
| **🧠 Casting** | You (the architect) | Compress reasoning into a spell |
| **🤖 Invocation** | AI/Juniors | Expand spell into code/tests/docs |

**Casting destroys excess context.**  
**Invocation never invents missing intent.**

### Spells: Your Atomic Design Units

A **Spell** is one focused capability, locked down tight:

- **🎯 Intent:** The "why" that survives all tradeoffs
- **✅ Guarantees:** What must *always* be true
- **🔍 Assumptions:** What you depend on
- **🚫 Exclusions:** What you *forbid* (crucial!)

Every spell does **exactly one thing**. No bloat, no ambiguity.

### Glyph: The Shorthand Language

**Glyph** is Sorcery's symbolic notation—designed for machines, readable by humans:

| Symbol | Meaning | Example |
|:------:|---------|---------|
| `#` | Spell name | `#Spell: Tokenize` |
| `^` | Intent (required!) | `^ produce stable tokens` |
| `@` | Component/Entity | `@Tokenizer` |
| `:` | Input → Output | `: utf8 -> tokens` |
| `!` | Guarantee | `! deterministic` |
| `~` | Assumption | `~ valid_utf8` |
| `-` | Exclusion | `- network` |
| `>` | Dependency | `> @Tokenizer` |
| `?` | Open question | `? performance reqs` |

**Why symbols?** Prose lies. Symbols enforce structure. AI parses them perfectly.

### Slice Gating: Quality Control

A spell can't execute until it's **complete**:

- ✅ Intent declared
- ✅ No open questions (`?`)
- ✅ Guarantees explicit
- ✅ At least one exclusion
- ✅ Does exactly one thing

**Malformed spells block deployment.** Quality by design.

---

## 📚 Examples

### Basic Spell: Tokenization

```glyph
#Spell: Tokenize
^ Intent: produce stable, cross-platform tokens to enable deterministic inference and caching

@Tokenizer
  : utf8 -> tokens
  ! deterministic
  ! locale_free
  - network
  - persistence
  ~ valid_utf8
```

**What it declares:**
- ✓ What it does (utf8 → tokens)
- ✓ Why it exists (deterministic inference)
- ✓ What must hold (deterministic, locale-free)
- ✓ What it forbids (network, persistence)
- ✓ What it assumes (valid utf8)

### Composed Spells: Building Systems

```glyph
#Spell: Parse
^ Intent: build stable syntax tree for semantic analysis

@Parser
  : tokens -> ast
  > @Tokenizer  # depends on Tokenize spell
  ! no_side_effects
  - io
  ~ valid_token_stream
```

Spells compose through **explicit contracts**—no hidden dependencies, no "as discussed" nonsense.

### Advanced: Multi-Entity Spell

```glyph
#Spell: InferenceABI
^ Intent: define stable contract boundary for model inference that survives version changes

@InferenceEndpoint
  : request -> response
  ! version_tagged
  ! backward_compatible
  - streaming
  ~ authenticated_caller

@RequestSchema
  : json -> validated_input
  ! schema_versioned
  - unknown_fields

@ResponseSchema
  : inference_result -> json
  ! deterministic_serialization
  - stack_traces
```

**See the full examples in** [`examples/`](examples/)

---

## 🎯 Why This Works

### Negative Space Matters

**What you forbid is as important as what you allow.**

Exclusions are first-class. A spell without them is probably incomplete.

### Boundary with Testing

| Layer | Job |
|-------|-----|
| **Sorcery** | Defines *what must be true* |
| **Tests** | Ensures *it stays true* |

Spells drive tests. Tests don't define spells.

### The Philosophy

```
Compression     over  Verbosity
Constraints     over  Explanation
Interfaces      over  Narrative
Explicit forbid over  Implied scope
Artifacts       over  Memory
```

**Canonical rule:** *Write spells that say exactly what must happen and what must never happen—so weaker agents can act without guessing.*

---

## 🚀 Get Started

1. **Pick a component** in your system that keeps breaking
2. **Write its Intent** (the "why" that survives tradeoffs)
3. **Declare guarantees, assumptions, exclusions**
4. **Feed to AI** for implementation

**Pro tip:** Start small. One spell. Watch the magic happen.

### Repository Structure

```
sorcery/
├── README.md           # This guide
├── examples/           # Spell examples
│   ├── tokenize.spell
│   ├── parse.spell
│   └── inference_abi.spell
├── LICENSE             # MIT
└── sorcery-logo.png    # Because why not?
```

### Contributing

Got a spell? Want to improve the doctrine? Open an issue/PR. Let's make architecture handoff not suck.

---

<p align="center">
  <strong>🧙‍♂️ Sorcery: Because your ideas deserve to survive handoff.</strong>
</p>

<p align="center">
  <sub>Design doctrine for intent compression. Not magic—just disciplined genius.</sub>
</p>
