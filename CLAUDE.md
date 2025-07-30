# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Rules

See the following rules docs:
* @.agents/rules/00-building.md
* @.agents/rules/01-implementing-features.md
* @.agents/rules/02-using-git.md
* @.agents/rules/03-refactoring.md
* @.agents/rules/04-keeping-secrets.md
* Follow the rust style guide while developing features: @.agents/style-guides/rust.style-guide.md

## Project Overview

todo-mcp is a Model Context Protocol server that provides a `todo_write` tool for managing structured task lists. The server communicates via stdio and is designed to help LLMs create and manage todo lists through the MCP protocol.

## Development Commands

### Building
```bash
cargo build -q              # Build debug version
cargo build --release -q    # Build release version
```

### Running
```bash
cargo run -q                # Run server (communicates via stdio)
cargo run --release -q      # Run optimized version
```

### Testing
```bash
cargo test -q               # Run all tests
cargo test -q -- --nocapture # Run tests with output
cargo test -q todo::        # Run tests for todo module
```

### Linting and Formatting
```bash
cargo fmt -q                # Format code
cargo clippy -q             # Run linter
cargo clippy -q -- -D warnings # Fail on warnings
```

## Architecture

### MCP Protocol Implementation
The server implements the Model Context Protocol using the `rmcp` crate:
- **ServerHandler trait**: Implemented in `src/server.rs` to handle MCP protocol methods
- **Tool routing**: Manual implementation in `call_tool` method (not using tool_router macro)
- **Protocol version**: Uses `V_2024_11_05` constant from rmcp

### Domain Model
The codebase follows domain-driven design with strong typing:

1. **Validated Types** (`src/todo/validation.rs`):
   - `TodoId`: Non-empty string validated with nutype
   - `TodoContent`: Non-empty string validated with nutype

2. **Core Types** (`src/todo/mod.rs`):
   - `TodoItem`: Uses bon builder pattern with public fields for serialization
   - `TodoList`: Encapsulates vector of items with manipulation methods
   - `Priority`: Enum with `high`, `medium`, `low` (lowercase in JSON)
   - `Status`: Enum with `pending`, `in_progress`, `completed` (snake_case in JSON)

3. **Tool Schema** (`src/tool.rs`):
   - `TodoWriteParams`: Deserializes the tool input
   - `todo_write_schema()`: Generates JSON schema for the tool

### Error Handling
Uses SNAFU for error handling with rmcp's `ErrorData`:
- `McpError::invalid_params()` for parameter validation errors
- `McpError::invalid_request()` for unknown tool requests
- JSON deserialization errors are mapped to invalid_params

### Key Implementation Details
- The server stores todo list in `Arc<Mutex<TodoList>>` for thread-safe access
- Tool accepts complete todo list replacement (not incremental updates)
- Server instructs LLMs to display todo lists as markdown checklists after updates
- All communication happens through stdio using JSON-RPC messages
