use proc_macro::TokenStream;

mod scylla;
mod utils;


/// 查询方法
/// db_query<T: ToString>(session: &Arc<scylla::Session>, where_sql: String, where_in_vars: &Vec<T>, limit_v: Option<isize>) -> R<Vec<Self>>
#[proc_macro_derive(ScyllaDBQuery)]
pub fn db_query(input: TokenStream) -> TokenStream{
    scylla::db_query(input)
}


#[test]
fn test() {}
