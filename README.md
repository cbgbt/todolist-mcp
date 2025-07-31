# TodoList MCP Server

A Model Context Protocol (MCP) server that provides a structured todo list tool for AI assistants to manage tasks effectively.

## Installation

### Installing via Claude Code

```bash
$ claude mcp add todolist -- docker run -i --rm ghcr.io/cbgbt/todolist-mcp:latest
```

### Installing via Q Developer CLI

Update your MCP server configuration to match the following:

```json
{
  "mcpServers": {
    "todolist": {
      "command": "docker",
      "args": ["run", "-i", "--rm", "ghcr.io/cbgbt/todolist-mcp:latest"]
    }
  }
}
```


