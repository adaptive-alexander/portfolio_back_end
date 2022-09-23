use chrono::NaiveDateTime;
use serde::Deserialize;
use juniper::{GraphQLInputObject, GraphQLObject};
use tokio_postgres::Row;


#[derive(GraphQLObject, Debug, Deserialize, Default)]
/// Information about option file storage
pub struct OptionFiles {
    pub id: String,
    pub storage_location: String,
    pub created_at: NaiveDateTime,
}

impl OptionFiles {}

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
