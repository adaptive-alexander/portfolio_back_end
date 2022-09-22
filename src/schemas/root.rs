use crate::db::Pool;
use crate::schemas::option_files::{OptionFiles, OptionFilesInput};
use juniper::{EmptySubscription, FieldError, FieldResult, graphql_object, RootNode};


pub struct Context {
    pub db_pool: Pool,
}

impl juniper::Context for Context {}

pub struct QueryRoot;

#[graphql_object(Context = Context)]
impl QueryRoot {
    #[graphql(description = "List of all Option files")]
    async fn option_files(context: &Context) -> FieldResult<Vec<OptionFiles>> {
        let conn = context.db_pool.get().await.unwrap();

        let stmt = conn
            .prepare_cached("SELECT * FROM option_files")
            .await
            .unwrap();
        let rows = conn.query(&stmt, &[]).await.unwrap();
        let result = rows.iter().map(OptionFiles::from).collect();
        Ok(result)
    }
}

pub struct MutationRoot;

#[graphql_object(Context = Context)]
impl MutationRoot {
    #[graphql(description = "Input new options file in database.")]
    async fn insert_file(context: &Context, option_files_input: OptionFilesInput) -> FieldResult<OptionFiles> {
        let conn = context.db_pool.get().await.unwrap();

        let stmt = conn.prepare_cached(&*format!(
            "INSERT INTO option_files(storage_location)\
            VALUES ('{}')",option_files_input.storage_location)).await.unwrap();
        let res = conn.execute(&stmt, &[]).await;
        match res {
            Ok(_) => Ok(OptionFiles::default()),
            Err(e) => Err(FieldError::from(e)),
        }
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot, MutationRoot, EmptySubscription::new())
}
