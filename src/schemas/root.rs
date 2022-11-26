use crate::db::Pool;
use crate::schemas::fund_data::FundData;
use juniper::{EmptySubscription, EmptyMutation, FieldResult, graphql_object, RootNode};


pub struct Context {
    pub db_pool: Pool,
}

impl juniper::Context for Context {}

pub struct QueryRoot;

#[graphql_object(Context = Context)]
impl QueryRoot {
    #[graphql(description = "List of all Option files")]
    async fn fund_data(context: &Context, name: String) -> FieldResult<Vec<FundData>> {
        let conn = context.db_pool.get().await.unwrap();

        let stmt = conn
            .prepare_cached(format!(
                "SELECT eop_date, name, perf_monthly \
                FROM hedgenordic_fund_performance \
                WHERE NAME = '{name}';").as_str())
            .await
            .unwrap();
        let rows = conn.query(&stmt, &[]).await.unwrap();
        let result = rows.iter().map(FundData::from).collect();
        Ok(result)
    }
}

// pub struct MutationRoot;

// #[graphql_object(Context = Context)]
// impl MutationRoot {
    // #[graphql(description = "Input new options file in database.")]
    // async fn insert_file(context: &Context, option_files_input: OptionFilesInput) -> FieldResult<OptionFiles> {
    //     let conn = context.db_pool.get().await.unwrap();
    //
    //     let stmt = conn.prepare_cached(&*format!(
    //         "INSERT INTO option_files(storage_location)\
    //         VALUES ('{}')",option_files_input.storage_location)).await.unwrap();
    //     let res = conn.execute(&stmt, &[]).await;
    //     match res {
    //         Ok(_) => Ok(OptionFiles::default()),
    //         Err(e) => Err(FieldError::from(e)),
    //     }
    // }
// }

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation<Context>, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot, EmptyMutation::new(), EmptySubscription::new())
}
