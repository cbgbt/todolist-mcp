//! MCP server implementation for the TodoWrite tool.
use crate::todo::{TodoItem, TodoList};
use rmcp::{
    ErrorData as McpError,
    handler::server::{
        ServerHandler,
        tool::{Parameters, ToolRouter},
    },
    model::*,
    service::{RequestContext, RoleServer},
    tool, tool_handler, tool_router,
};
use schemars::JsonSchema;
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{debug, info, instrument};

#[derive(Debug, Clone, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TodoWriteParams {
    /// The updated todo list
    pub todos: Vec<TodoItem>,
}

#[derive(Clone)]
pub struct TodoMcpServer {
    todo_list: Arc<Mutex<TodoList>>,
    tool_router: ToolRouter<TodoMcpServer>,
}

impl Default for TodoMcpServer {
    fn default() -> Self {
        Self::new()
    }
}

impl TodoMcpServer {
    /// Creates a new TodoMcpServer instance.
    #[must_use = "constructors return new instances that must be used"]
    pub fn new() -> Self {
        Self {
            todo_list: Arc::new(Mutex::new(TodoList::new())),
            tool_router: Self::tool_router(),
        }
    }
}

#[tool_router]
impl TodoMcpServer {
    /// Manages task lists for development sessions.
    #[tool(
        description = "Manages task lists for development sessions. Best suited for: coordinating multi-phase work, organizing tasks with dependencies, tracking progress through complex implementations, handling multiple objectives, or when explicitly requested. Avoid using for: single operations, simple queries, minimal coordination tasks, or basic information exchanges. Best practices: update status as work progresses, maintain one active task at a time, create entries before starting work, and mark completion promptly. Display updated lists as markdown checklists. Use special terminal colors to indicate a recently-completed task."
    )]
    #[instrument(level = "trace", skip(self, params))]
    async fn todo_write(
        &self,
        Parameters(params): Parameters<TodoWriteParams>,
    ) -> Result<CallToolResult, McpError> {
        let mut todo_list = self.todo_list.lock().await;
        let item_count = params.todos.len();

        debug!("Updating todo list with {} items", item_count);
        todo_list.set_items(params.todos);
        info!("Todo list updated successfully with {} items", item_count);

        Ok(CallToolResult::success(vec![Content::text(
            "Todo list updated successfully",
        )]))
    }
}

#[tool_handler]
impl ServerHandler for TodoMcpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation {
                name: "todolist-mcp".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
            },
            instructions: Some(
                "The todo_write tool helps organize development work through task management. \
                 Use it for multi-step implementations, complex workflows, or when tracking \
                 progress adds value. Display tasks as markdown checklists after updates."
                    .to_string(),
            ),
        }
    }

    #[instrument(level="trace", skip(self, _context), fields(protocol_version = ?init.protocol_version))]
    async fn initialize(
        &self,
        init: InitializeRequestParam,
        _context: RequestContext<RoleServer>,
    ) -> Result<InitializeResult, McpError> {
        info!("Initialize request received");
        debug!("Client info: {:?}", init.client_info);

        Ok(self.get_info())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rmcp::{ClientHandler, ServiceExt};
    use serde_json::json;
    use tokio::io::duplex;

    // A simple test client handler
    #[derive(Clone)]
    struct TestClient {}

    impl ClientHandler for TestClient {}

    #[tokio::test]
    async fn test_server_todo_write() {
        // Create a bidirectional channel for testing
        let (server_transport, client_transport) = duplex(4096);

        // Spawn the server
        tokio::spawn(async move {
            let server = TodoMcpServer::new().serve(server_transport).await.unwrap();
            server.waiting().await.unwrap();
        });

        // Create and run the client (initialization happens automatically)
        let client = TestClient {}.serve(client_transport).await.unwrap();

        // List available tools
        let tools = client.list_tools(None).await.unwrap();
        assert_eq!(tools.tools.len(), 1);
        assert_eq!(tools.tools[0].name, "todo_write");

        // Create some todo items
        let todo_params = json!({
            "todos": [
                {
                    "id": "test-1",
                    "content": "Write unit tests",
                    "status": "pending",
                },
                {
                    "id": "test-2",
                    "content": "Review code",
                    "status": "in_progress",
                }
            ]
        });

        // Call the todo_write tool
        let result = client
            .call_tool(CallToolRequestParam {
                name: "todo_write".into(),
                arguments: Some(todo_params.as_object().unwrap().clone()),
            })
            .await
            .unwrap();

        // Verify we got a success response
        assert_eq!(result.content.len(), 1);
        let content = &result.content[0];
        assert_eq!(
            content.as_text().unwrap().text,
            "Todo list updated successfully"
        );

        // Cancel the client
        client.cancel().await.unwrap();
    }

    #[tokio::test]
    async fn test_server_info() {
        let server = TodoMcpServer::new();
        let info = server.get_info();

        assert_eq!(info.protocol_version, ProtocolVersion::V_2024_11_05);
        assert_eq!(info.server_info.name, "todolist-mcp");
        assert!(info.capabilities.tools.is_some());
        assert!(info.instructions.is_some());
    }
}
