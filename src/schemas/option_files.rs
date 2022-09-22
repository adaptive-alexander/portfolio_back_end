use chrono::NaiveDateTime;
use serde::Deserialize;
use juniper::{GraphQLInputObject, GraphQLObject};
use tokio_postgres::Row;
use uuid::Uuid;


#[derive(GraphQLObject, Debug, Deserialize, Default)]
/// Information about option file storage
pub struct OptionFiles {
    pub id: Uuid,
    pub storage_location: String,
    pub created_at: NaiveDateTime,
}

impl OptionFiles {
    fn id(&self) -> &Uuid {
        &self.id
    }
    fn storage_location(&self) -> &str {
        &self.storage_location
    }
    fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }
}

impl From<&Row> for OptionFiles {
    fn from(row: &Row) -> Self {
        Self {
            id: row.get(0),
            storage_location: row.get(1),
            created_at: row.get(2),
        }
    }
}

#[derive(GraphQLInputObject, Debug, Deserialize)]
#[graphql(description = "option_files table input")]
pub struct OptionFilesInput {
    pub storage_location: String,
}
