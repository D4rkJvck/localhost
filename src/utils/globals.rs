use {
    crate::server::handler::Http,
    lazy_static::lazy_static,
    regex::Regex,
    std::{
        collections::HashMap,
        sync::{
            LazyLock,
            RwLock,
        },
    },
    tera::Tera,
};

pub const TIMEOUT: u64 = 1000;

pub static INTERPRETERS: LazyLock<HashMap<&str, &str>> =
    LazyLock::new(|| HashMap::from([("py", "python3")]));

pub static TEMPLATES: LazyLock<Tera> = LazyLock::new(|| {
    let root = env!("CARGO_MANIFEST_DIR");
    let full_path = format!("{}/public/templates/*.html", root);
    let mut tera = match Tera::new(&full_path) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Failed to load templates: {}", e);
            std::process::exit(1);
        }
    };
    tera.autoescape_on(vec!["html"]);
    tera
});

lazy_static! {
    pub static ref HTTP: RwLock<Http> = RwLock::new(Http::new(5));

    // Improved regex definitions
    pub static ref BOUNDARY_REGEX: Regex = Regex::new(r"boundary=(.+)$").unwrap();

    pub static ref CONTENT_DISPOSITION_REGEX: Regex =
        Regex::new(r#"Content-Disposition: form-data; name="([^"]+)"(; filename="([^"]+)")?"#).unwrap();

    pub static ref CONTENT_TYPE_REGEX: Regex =
        Regex::new(r"Content-Type: (.+)\r\n").unwrap();
}
