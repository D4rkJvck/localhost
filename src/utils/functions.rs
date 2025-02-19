use {
    super::AppResult,
    crate::{
        http::{
            Method,
            Resource,
        },
        server::Server,
    },
    std::{
        io::{
            BufRead,
            BufReader,
            ErrorKind,
        },
        net::{
            TcpListener,
            TcpStream,
        },
    },
};
#[cfg(target_os = "macos")]
use {
    libc::{
        c_long,
        time_t,
        timespec,
    },
    std::time::Duration,
};

#[cfg(target_os = "macos")]
pub fn timeout(timeout_in_ms: u64) -> *const timespec {
    let duration = Duration::from_millis(timeout_in_ms);
    let secs = duration.as_secs() as time_t;
    let nanos = duration.subsec_nanos() as c_long;

    &timespec {
        tv_sec:  secs,
        tv_nsec: nanos,
    }
}

pub fn process_req_line(s: &str) -> (Method, Resource) {
    let mut words = s.split_whitespace();

    let method = words.next().unwrap();
    let resource = words.next().unwrap();

    (
        method.into(),
        Resource::Path(resource.to_string()),
    )
}

pub fn process_header_line(s: &str) -> (String, String) {
    let mut header_items = s.split(':');
    let key = header_items
        .next()
        .unwrap_or("")
        .trim()
        .to_string();

    let value = header_items
        .collect::<Vec<&str>>()
        .join(":")
        .trim()
        .to_string();

    (key, value)
}

pub fn get_listeners(
    servers: &Vec<Server>,
) -> AppResult<Vec<TcpListener>> {
    let mut mux_listeners = vec![];
    for server in servers {
        mux_listeners.push(server.listeners()?);
    }

    // Flattens all listeners.
    Ok(mux_listeners
        .into_iter()
        .flatten()
        .collect())
}

pub fn read_buffer(stream: &TcpStream) -> Option<String> {
    let mut buf_reader = BufReader::new(stream);
    let mut req_str = String::new();

    loop {
        let mut line = String::new();
        match buf_reader.read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => {
                req_str.push_str(&line);

                if line == "\r\n" || line == "\n" {
                    break;
                }
            }
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => continue,
            Err(_) => break,
        }
    }

    if req_str.is_empty() {
        None
    }
    else {
        Some(req_str)
    }
}
