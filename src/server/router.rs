use {
    super::handler::{ErrorPage, Handler, StaticPage, WebService},
    crate::http::{Method, Request, Resource},
    serde::{Deserialize, Serialize},
    std::{collections::HashMap, io::prelude::*},
};

#[derive(Serialize, Deserialize)]
pub struct Route {
    path: Option<String>,
    method: Option<Vec<String>>,
    default_file: Option<String>,
    check_session: Option<bool>,
    redirect: Option<HashMap<String, String>>,
}

impl Route {
    pub fn has_valid_config(&self) -> bool {
        self.path.is_some()
            && self.method.is_some()
            && self.default_file.is_some()
            && self.check_session.is_some()
    }
}

pub struct Router;

impl Router {
    pub fn run(request: Request, stream: &mut impl Write) -> Result<(), String> {
        match (&request.method, &request.resource) {
            (Method::GET, Resource::Path(s)) => {
                let route: Vec<&str> = s.split("/").collect();

                match route[1] {
                    "api" => WebService::handle(&request)?
                        .send_response(stream)
                        .map_err(|e| e.to_string()),
                    _ => StaticPage::handle(&request)?
                        .send_response(stream)
                        .map_err(|e| e.to_string()),
                }
            }
            _ => ErrorPage::handle(&request)?
                .send_response(stream)
                .map_err(|e| e.to_string()),
        }
    }
}
