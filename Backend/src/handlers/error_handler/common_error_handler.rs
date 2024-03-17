// TODO: Added generic error handler
// pub fn check_db_response_and_handle<T>(db_response: QueryResult<T>, get_response_closure: F) -> HttpResponse
//     where T: Fn(T) -> R
//     {
//     let _ = get_response_closure;
//     match db_response {
//         QueryResult::Success => HttpResponse::Ok().json(get_response_closure()),
//         QueryResult::Failed(_) => todo!(),
//         QueryResult::Record(_) => todo!(),
//         QueryResult::Records(_) => todo!(),
//     }
// }
