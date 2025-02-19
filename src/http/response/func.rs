use {
    super::Response,
    crate::utils::AppResult,
    std::{
        collections::HashMap,
        io::Write,
    },
};

impl<'a> Response<'a> {
    pub fn new(
        status_code: &'a str,
        headers: Option<HashMap<&'a str, &'a str>>,
        body: Option<String>,
    ) -> Response<'a> {
        let mut response = Response::default();
        if status_code != "200" {
            response.status_code = status_code;
        };

        response.headers = match &headers {
            Some(_h) => headers,
            None => {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            }
        };

        response.status_text = match response.status_code {
            "200" => "OK",
            "400" => "Bad Request",
            "404" => "Not Found",
            "500" => "Internal Server Error",
            _ => "Not Found",
        };

        response.body = body;

        response
    }

    pub fn send_response(
        &self,
        write_stream: &mut impl Write,
    ) -> AppResult<()> {
        let res = self.clone();
        let response_string: String = String::from(res);
        let _ = write!(write_stream, "{}", response_string);
        Ok(())
    }

    pub fn status_code(&self) -> &str { self.status_code }

    pub fn status_text(&self) -> &str { self.status_text }

    pub fn headers(&self) -> String {
        let map: HashMap<&str, &str> = self.headers.clone().unwrap();
        let mut header_string: String = "".into();
        for (k, v) in map.iter() {
            header_string = format!("{}{}:{}\r\n", header_string, k, v);
        }
        header_string
    }

    pub fn body(&self) -> &str {
        match &self.body {
            Some(b) => b.as_str(),
            None => "",
        }
    }
}
