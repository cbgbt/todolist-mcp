//! Todo domain types and operations.
use bon::Builder;
use nutype::nutype;
use schemars::{JsonSchema, Schema, SchemaGenerator};
use serde::{Deserialize, Serialize};

#[nutype(
    validate(not_empty),
    derive(
        Debug,
        Clone,
        PartialEq,
        Eq,
        Hash,
        Serialize,
        Deserialize,
        AsRef,
        TryFrom
    )
)]
pub struct TodoId(String);

impl JsonSchema for TodoId {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "TodoId".into()
    }

    fn json_schema(r#gen: &mut SchemaGenerator) -> Schema {
        // Generate a string schema with minLength: 1
        let mut schema = r#gen.subschema_for::<String>();
        schema.insert("minLength".to_string(), 1.into());
        schema
    }
}

#[nutype(
    validate(not_empty),
    derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, AsRef, TryFrom)
)]
pub struct TodoContent(String);

impl JsonSchema for TodoContent {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "TodoContent".into()
    }

    fn json_schema(r#gen: &mut SchemaGenerator) -> Schema {
        // Generate a string schema with minLength: 1
        let mut schema = r#gen.subschema_for::<String>();
        schema.insert("minLength".to_string(), 1.into());
        schema
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Pending,
    InProgress,
    Completed,
}

#[derive(Debug, Clone, PartialEq, Eq, Builder, Serialize, Deserialize, JsonSchema)]
#[builder(on(String, into), on(&str, into))]
#[non_exhaustive]
pub struct TodoItem {
    pub id: TodoId,
    pub content: TodoContent,
    pub status: Status,
}

#[derive(Debug, Clone, Default)]
pub struct TodoList {
    items: Vec<TodoItem>,
}

impl TodoList {
    /// Creates a new empty todo list.
    #[must_use = "constructors return new instances that must be used"]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns all todo items.
    #[must_use = "getters return borrowed data that should be used"]
    pub fn items(&self) -> &[TodoItem] {
        &self.items
    }

    /// Replaces all items with the provided vector.
    pub fn set_items(&mut self, items: Vec<TodoItem>) {
        self.items = items;
    }
}

impl From<Vec<TodoItem>> for TodoList {
    fn from(items: Vec<TodoItem>) -> Self {
        Self { items }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_todo_list_from_vec() {
        // Given: A vector of todo items
        let items = vec![
            TodoItem::builder()
                .id(TodoId::try_new("1").unwrap())
                .content(TodoContent::try_new("Test todo").unwrap())
                .status(Status::Pending)
                .build(),
        ];

        // When: Creating a TodoList from the vector
        let list = TodoList::from(items.clone());

        // Then: The list should contain the items
        assert_eq!(list.items().len(), 1);
        assert_eq!(list.items()[0].id.as_ref(), "1");
    }

    #[test]
    fn test_todo_list_set_items() {
        // Given: An empty todo list
        let mut list = TodoList::new();

        // When: Setting items
        let items = vec![
            TodoItem::builder()
                .id(TodoId::try_new("1").unwrap())
                .content(TodoContent::try_new("First todo").unwrap())
                .status(Status::InProgress)
                .build(),
            TodoItem::builder()
                .id(TodoId::try_new("2").unwrap())
                .content(TodoContent::try_new("Second todo").unwrap())
                .status(Status::Completed)
                .build(),
        ];
        list.set_items(items);

        // Then: The list should contain the new items
        assert_eq!(list.items().len(), 2);
        assert_eq!(list.items()[0].id.as_ref(), "1");
        assert_eq!(list.items()[1].id.as_ref(), "2");
    }
}
