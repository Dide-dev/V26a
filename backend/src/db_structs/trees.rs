/*
    This program has been developed by students from the bachelor Computer Science at
    Utrecht University within the Software Project course.
    © Copyright Utrecht University (Department of Information and Computing Sciences)
*/

use crate::db_structs::cards::CardGetData;
use sqlx::postgres::PgRow;
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::{Error, FromRow, Row};
use std::collections::BTreeMap;
use utoipa::openapi::{ObjectBuilder, RefOr, Schema};
use utoipa::{PartialSchema, ToSchema};

/// All data of the tree, together with its cards.
#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TreeWithCards {
    pub data: TreeData,
    pub gen_data: TreeGeneratedData,
    pub structure: TreeStructure,
    /// The `card_id` with the corresponding card.
    pub cards: BTreeMap<i32, CardGetData>,
}

// We implement this manually, so the example contains a valid example, with `cards` matching the `structure`.
impl PartialSchema for TreeWithCards {
    fn schema() -> RefOr<Schema> {
        ObjectBuilder::new()
            .property("data", TreeData::schema())
            .property("genData", TreeGeneratedData::schema())
            .property("structure", TreeStructure::schema())
            .property("cards", BTreeMap::<i32, CardGetData>::schema())
            .examples(Some(serde_json::json!({
                "data": {
                  "title": "string",
                  "description": "string",
                  "bookName": "string",
                  "bookEdition": "string",
                  "bookIsbn": "string",
                  "bookChapter": "string",
                  "bookExercise": "string",
                  "assignmentText": "string",
                  "published": true
                },
                "genData": {
                  "id": 7,
                  "ownerId": 2,
                  "createdAt": "2026-04-12T16:31:32.855032Z",
                  "lastModified": "2026-04-12T16:31:32.855032Z",
                  "timesVisited": 0,
                  "lastVisited": null
                },
                "structure": {
                  "rootNodes": [
                    {
                      "label": "string",
                      "children": [
                        {
                          "cardId": "16",
                          "children": []
                        }
                      ]
                    }
                  ]
                },
                "cards": {
                  "16": {
                    "front": "string",
                    "back": "string"
                  }
                }
            })))
            .into()
    }
}

impl ToSchema for TreeWithCards {}

/// The tree structure with the card id's.
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TreeStructure {
    pub root_nodes: Vec<RootNode>,
}

impl TreeStructure {
    /// Retrieve the card id's in the tree.
    ///
    /// # Returns
    /// The `card_id` of each [`Node`] in the tree as a [`Vec<i32>`].
    pub fn flatten(&self) -> Vec<i32> {
        self.root_nodes.iter().flat_map(RootNode::flatten).collect()
    }
}

/// A root node of a tree.
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RootNode {
    pub label: String,
    pub children: Vec<Node>,
}

impl RootNode {
    /// Retrieve the card id's in the children of this root.
    ///
    /// # Returns
    /// The `card_id` of each [`Node`] in the children of this root as a [`Vec<i32>`].
    fn flatten(&self) -> Vec<i32> {
        self.children.iter().flat_map(Node::flatten).collect()
    }
}

/// A node (not root) in a tree.
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub card_id: i32,
    pub children: Vec<Node>,
}

// We have to define this ourselves instead of simply deriving it,
// because of the cyclic definition of `Node`.
impl PartialSchema for Node {
    fn schema() -> RefOr<Schema> {
        ObjectBuilder::new()
            .property("cardId", i32::schema())
            .required("cardId")
            .property("children", Vec::<i32>::schema())
            .examples(Some(serde_json::json!({
                "cardId": 1,
                "children": [
                    {
                        "cardId": 2,
                        "children": []
                    },
                    {
                        "cardId": 3,
                        "children": []
                    }
                ]
            })))
            .description(
                Some("A node (or card) in a tree.
                This type is recursively defined in a cyclic manner, and this schema is unable to capture that.
                `children` should contain an array of `Node`'s instead of `i32`'s.")
            )
            .into()
    }
}

impl ToSchema for Node {}

impl Node {
    /// Retrieve the card id's in the children of this node.
    ///
    /// # Returns
    /// The `card_id` of this [`Node`] and of each [`Node`] in the children of this node
    /// as a [`Vec<i32>`].
    fn flatten(&self) -> Vec<i32> {
        let mut child_ids: Vec<i32> = self.children.iter().flat_map(Node::flatten).collect();

        child_ids.push(self.card_id);

        child_ids
    }
}

/// The full metadata of the tree, includes `TreeData` and `TreeGeneratedData`.
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TreeMetadata {
    pub data: TreeData,
    pub gen_data: TreeGeneratedData,
}

impl TreeMetadata {
    pub fn from_pg_row(row: &PgRow) -> Result<Self, Error> {
        let data = TreeData::from_row(row)?;
        let gen_data = TreeGeneratedData::from_row(row)?;

        Ok(TreeMetadata { data, gen_data })
    }
}

/// A language that trees can be written in.
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Language {
    pub code: String,
    pub name: String,
}

/// A subject that trees can be made for.
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Subject {
    pub language_code: String,
    pub name: String,
}

/// The general data of the tree, NOT including its structure or autogenerated data.
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TreeData {
    pub title: String,
    pub language_code: String,
    pub tags: Vec<String>,
    pub description: String,
    pub subject_id: Option<i32>,
    pub book_name: String,
    pub book_edition: String,
    pub book_author: String,
    pub book_isbn: Option<String>,
    pub book_chapter: String,
    pub book_exercise: String,
    pub assignment_text: String,
    pub published: bool,
}

impl Default for TreeData {
    fn default() -> Self {
        Self {
            title: String::new(),
            language_code: "nl".to_owned(),
            tags: Vec::new(),
            description: String::new(),
            subject_id: None,
            book_name: String::new(),
            book_edition: String::new(),
            book_author: String::new(),
            book_isbn: None,
            book_chapter: String::new(),
            book_exercise: String::new(),
            assignment_text: String::new(),
            published: false,
        }
    }
}

impl FromRow<'_, PgRow> for TreeData {
    fn from_row(row: &PgRow) -> sqlx::Result<Self> {
        Ok(Self {
            title: row.try_get("title")?,
            language_code: row.try_get("language_code")?,
            tags: row.try_get("tags")?,
            description: row.try_get("description")?,
            subject_id: row.try_get("subject_id")?,
            book_name: row.try_get("book_name")?,
            book_edition: row.try_get("book_edition")?,
            book_author: row.try_get("book_author")?,
            book_isbn: row
                .try_get::<Option<i64>, _>("book_isbn")?
                .map(|n| n.to_string()),
            book_chapter: row.try_get("book_chapter")?,
            book_exercise: row.try_get("book_exercise")?,
            assignment_text: row.try_get("assignment_text")?,
            published: row.try_get("published")?,
        })
    }
}

/// The autogenerated data of the tree.
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize, ToSchema, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct TreeGeneratedData {
    pub id: i32,
    pub owner_id: i32,
    pub created_at: DateTime<Utc>,
    pub last_modified: DateTime<Utc>,
    pub times_visited: i64,
    pub last_visited: Option<DateTime<Utc>>,
}

/// The data needed to update a tree.
#[derive(serde::Deserialize, serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TreeUpdateStructureData {
    pub id: i32,
    pub new_structure: TreeStructure,
}

/// The data needed to remove a card from a tree.
#[derive(serde::Deserialize, serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TreeUpdateData {
    pub id: i32,
    pub new_data: TreeData,
}

/// The data needed to remove a card from a tree.
///
/// The `new_structure` must be passed, as the card has to be removed simultaneously
/// with the update of the structure in a single database transaction to ensure the database
/// remains in a valid state.
#[derive(serde::Deserialize, serde::Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TreeRemoveCardData {
    pub tree_id: i32,
    pub card_id: i32,
    pub new_structure: TreeStructure,
}

/// The data needed to initialize a tree in the database.
#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TreeAddData {
    pub tree_data: TreeData,
    pub tree_structure: TreeStructure,
}

// We implement this manually, so the example contains a legal, empty structure.
impl PartialSchema for TreeAddData {
    fn schema() -> RefOr<Schema> {
        ObjectBuilder::new()
            .property("treeData", TreeData::schema())
            .property("treeStructure", TreeStructure::schema())
            .examples(Some(serde_json::json!({
                "treeData": {
                  "assignmentText": "string",
                  "bookChapter": "string",
                  "bookEdition": "string",
                  "bookExercise": "string",
                  "bookIsbn": "string",
                  "bookName": "string",
                  "description": "string",
                  "published": true,
                  "title": "string"
                },
                "treeStructure": {
                  "rootNodes": [
                    {
                      "label": "string",
                      "children": []
                    },
                    {
                      "label": "string",
                      "children": []
                    }
                  ]
                }
            })))
            .into()
    }
}

impl ToSchema for TreeAddData {}

/// The data returned when requesting a list of trees.
#[derive(serde::Serialize, serde::Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TreeListResponse {
    /// The total amount of trees that are visible in the query.
    pub total_trees: i32,
    pub trees: Vec<TreeMetadata>,
}

/// The filters which can be applied to the public list of trees.
#[derive(serde::Deserialize, ToSchema, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct TreeFilters {
    pub owner_id: Option<i32>,

    // Needed for optional list in query params, see https://docs.rs/axum-extra/latest/axum_extra/extract/struct.Query.html
    #[serde(default)]
    pub book: Vec<String>,
    pub created_after: Option<DateTime<Utc>>,
    pub created_before: Option<DateTime<Utc>>,

    // Needed for optional boolean
    #[serde(default)]
    pub show_own_private: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flatten_tree_structure() {
        let tree_structure = TreeStructure {
            root_nodes: vec![
                RootNode {
                    label: "Orientation".to_owned(),
                    children: vec![
                        Node {
                            card_id: 1,
                            children: vec![],
                        },
                        Node {
                            card_id: 2,
                            children: vec![Node {
                                card_id: 3,
                                children: vec![],
                            }],
                        },
                    ],
                },
                RootNode {
                    label: "Problem".to_owned(),
                    children: vec![Node {
                        card_id: 5,
                        children: vec![],
                    }],
                },
            ],
        };

        let mut flattened = tree_structure.flatten();
        flattened.sort_unstable();
        assert_eq!(&flattened, &[1, 2, 3, 5]);
    }
}
