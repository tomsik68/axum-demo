use super::model::*;
use super::route::*;

#[derive(utoipa::OpenApi)]
#[openapi(
    handlers(list_todos, new_todo, delete_todo),
    components(RequestTodo, ResponseTodo, ApiError),
    tags(
        (name = "todo", description = "")
    ),

)]
pub struct ApiDoc;
