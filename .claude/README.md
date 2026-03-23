# Claude Code Configuration

This directory contains configuration files for [Claude Code](https://code.claude.com/), an AI coding assistant that helps with development tasks in this repository.

## Structure

```
.claude/
├── README.md              # This file
├── settings.json          # Permissions and tool access (committed)
├── settings.local.json    # Personal permission overrides (gitignored)
│
├── commands/              # Custom slash commands
│   ├── review.md          # /project:review - Review current branch diff
│   ├── test-example.md    # /project:test-example - Build and test an example
│   └── new-example.md     # /project:new-example - Create new smart escrow
│
├── rules/                 # Modular instruction files (auto-loaded)
│   ├── wasm-contracts.md  # WASM-specific rules (scoped to contract files)
│   ├── testing.md         # Testing standards and patterns
│   └── code-style.md      # Code style and naming conventions
│
├── skills/                # Auto-invoked workflows
│   └── security-review/
│       └── SKILL.md       # Security audit skill
│
└── agents/                # Specialized subagent personas
    └── wasm-reviewer.md   # WASM contract review expert
```

## Main Configuration Files

### CLAUDE.md (Project Root)

The primary instruction file that Claude reads on every session. Contains:

- Build and test commands
- Project architecture overview
- Critical coding conventions
- Common gotchas and watch-outs

Keep this file under 200 lines for optimal performance.

### settings.json

Defines what Claude can and cannot do:

- **Allowed**: Build commands, tests, git read operations, file operations
- **Denied**: Destructive commands (rm -rf), git push/commit, network commands, .env access

## Custom Commands

Use these with `/project:command-name` in Claude Code:

### /project:review

Reviews the current branch diff for:

- Code quality issues
- WASM contract safety violations
- Testing completeness
- Documentation updates
- Naming convention compliance

### /project:test-example [name]

Builds and tests a specific smart escrow example:

- Builds in release mode
- Runs integration tests
- Checks WASM binary size
- Verifies exports

Example: `/project:test-example hello_world`

### /project:new-example [name]

Creates a new smart escrow example with proper structure:

- Directory structure
- Cargo.toml with correct settings
- src/lib.rs template
- README template
- Integration test template

Example: `/project:new-example my_escrow`

## Rules (Auto-loaded)

Rules are automatically loaded based on file paths:

### wasm-contracts.md

Applies to: `examples/smart-escrows/**/*.rs`, `e2e-tests/**/*.rs`

Enforces:

- `#![no_std]` and `#![no_main]` attributes
- No heap allocations
- Proper error handling with library's Result type
- Correct build configuration
- Size optimization techniques

### testing.md

Testing standards for all test types:

- Unit tests (Rust)
- Integration tests (JavaScript)
- End-to-end tests
- Test coverage requirements

### code-style.md

File naming and code style conventions:

- Rust: snake_case files, kebab-case crates
- JavaScript: camelCase
- Shell: kebab-case
- Documentation standards

## Skills

Skills are workflows that Claude can invoke automatically when appropriate.

### security-review

Triggered when: User mentions security, reviewing code for vulnerabilities, or before deployment.

Performs:

- Memory safety checks (heap allocations)
- Standard library usage in no_std
- Error handling validation
- Integer overflow checks
- Determinism verification
- Resource exhaustion analysis
- Access control review
- Input validation checks

## Agents

Specialized subagents with focused expertise.

### wasm-reviewer

Expert in WASM smart contract review.

Checks:

- no_std compliance
- Memory safety
- Entry point correctness
- Build configuration
- Error handling
- Determinism
- XRPL-specific patterns
- Testing requirements

Provides structured feedback with priority levels.

## Personal Overrides

Create these files for personal preferences (they're gitignored):

### CLAUDE.local.md (Project Root)

Your personal instructions that apply only to you, not the team.

### .claude/settings.local.json

Your personal permission overrides.

## Usage Tips

1. **Start with /project:review** before submitting PRs
2. **Use /project:test-example** to validate changes to examples
3. **Invoke security-review** by mentioning "security" in your request
4. **Let wasm-reviewer agent** handle detailed contract reviews
5. **Keep CLAUDE.md updated** as the project evolves

## Maintenance

- Update CLAUDE.md when adding new commands or changing architecture
- Add new rules when patterns emerge that should be enforced
- Create new commands for frequently repeated workflows
- Keep rules focused and scoped to relevant file paths

## Learn More

- [Claude Code Documentation](https://code.claude.com/docs)
- [XRPL WASM Stdlib Guide](https://ripple.github.io/xrpl-wasm-stdlib/xrpl_wasm_stdlib/guide/index.html)
- [Contributing Guidelines](../CONTRIBUTING.md)
