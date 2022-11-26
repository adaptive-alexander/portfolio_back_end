use chrono::NaiveDateTime;
use serde::Deserialize;
use juniper::{GraphQLObject};
use tokio_postgres::Row;


#[derive(GraphQLObject, Debug, Deserialize, Default)]
/// Information about option file storage
pub struct FundData {
    pub eop_date: NaiveDateTime,
    pub name: String,
    pub perf_monthly: f64,
}

impl FundData {}

impl From<&Row> for FundData {
    fn from(row: &Row) -> Self {
        Self {
            eop_date: row.get(0),
            name: row.get(1),
            perf_monthly: row.get(2),
        }
    }
}

// #[derive(GraphQLInputObject, Debug, Deserialize)]
// #[graphql(description = "option_files table input")]
// pub struct OptionFilesInput {
//     pub storage_location: String,
// }
