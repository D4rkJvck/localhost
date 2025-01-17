use {
    super::{
        Data,
        Handler,
    },
    crate::http::{
        Request,
        Resource,
        Response,
    },
    std::{
        collections::HashMap,
        env,
        fs,
    },
};

pub struct WebService;

impl WebService {
    fn load_json() -> Result<Vec<Data>, String> {
        let default_path = format!("{}/data", env!("CARGO_MANIFEST_DIR"));
        let data_path = env::var("DATA_PATH").unwrap_or(default_path);
        let full_path = format!("{data_path}/data.json");
        let json_contents = fs::read_to_string(full_path).map_err(|e| e.to_string())?;
        let data = serde_json::from_str(json_contents.as_str()).map_err(|e| e.to_string())?;

        data
    }
}

impl Handler for WebService {
    fn handle(req: &Request) -> Result<Response, String> {
        let Resource::Path(s) = &req.resource;
        let route: Vec<&str> = s.split("/").collect();

        match route[2] {
            "shipping" if route.len() > 3 && route[3] == "data" => {
                let body =
                    Some(serde_json::to_string(&Self::load_json()).map_err(|e| e.to_string())?);
                let mut headers = HashMap::new();
                headers.insert("Content-Type", "applicaion/json");
                Ok(Response::new("200", Some(headers), body))
            }
            _ => Ok(Response::new(
                "404",
                None,
                Self::load_file("pages/error.html"),
            )),
        }
    }
}
