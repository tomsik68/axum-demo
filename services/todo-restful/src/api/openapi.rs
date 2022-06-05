use super::route::*;
use todo_http_schemas::*;

#[derive(utoipa::OpenApi)]
#[openapi(
    handlers(list_todos, new_todo, delete_todo),
    components(RequestTodo, ResponseTodo, ApiError),
    tags(
        (name = "todo", description = "")
    ),

)]
pub struct ApiDoc;
