use super::server::Handler;
use crate::http::{Response, Request, StatusCode};
pub struct WebsiteHandler{

}

impl Handler for WebsiteHandler{
    fn handle_request(&mut self, request: &Request) -> Response {
        Response::new(StatusCode::Ok, Some("<h1>New Implementaion Worked</h1>".to_string()))
    }
}