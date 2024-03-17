/// Custom return type for DB queries
pub enum QueryResult<T> {
    Success,
    Failed(String),
    Record(T),
    Records(Vec<T>),
}
