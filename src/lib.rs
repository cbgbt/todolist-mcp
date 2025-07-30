//! TodoList MCP Server - A Model Context Protocol server for managing todo lists.
//!
//! This crate provides an MCP server implementation that exposes a TodoWrite tool
//! allowing LLMs to create and manage structured task lists.
//!
pub mod server;
pub mod todo;

pub use server::TodoMcpServer;
pub use todo::{Status, TodoContent, TodoId, TodoItem, TodoList};
