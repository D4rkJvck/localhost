use libc::{epoll_ctl, epoll_event, epoll_wait, EPOLLIN, EPOLL_CTL_ADD};
use std::{
    net::{TcpListener, TcpStream},
    os::{fd::AsRawFd, unix::io::RawFd},
};
use crate::{http::Request, loader::Config, server::Server, server::cgi::CGI, server::router::Router};


#[derive(Debug)]
pub struct Multiplexer {
    epoll_fd: RawFd,
    servers: Vec<Server>,
    listeners: Vec<TcpListener>,
    streams: Vec<TcpStream>,
}
type ErrorAddFd = Result<(), String>;
impl Multiplexer {
    pub fn new(config: Config) -> Result<Self, String> {
        let servers = config.servers();
        let epoll_fd = unsafe { libc::epoll_create1(0) };
        if epoll_fd == -1 {
            return Err(std::io::Error::last_os_error().to_string());
        };

        let mut mux_listeners = vec![];

        for server in &servers {
            match server.listeners() {
                Ok(server_listeners) => mux_listeners.push(server_listeners),
                Err(error) => return Err(error.to_string()),
            }
        }

        let listeners = mux_listeners
            .into_iter()
            .flatten()
            .collect();

        Ok(Self {
            epoll_fd,
            servers,
            listeners,
            streams: vec![],
        })
    }

    pub fn epoll_fd(&self) -> RawFd {
        self.epoll_fd
    }

    pub fn servers(&self) -> &Vec<Server> {
        &self.servers
    }

    pub fn add_fd(&self) -> ErrorAddFd {
        for listener in self.listeners.iter() {
            let fd = listener.as_raw_fd();

            listener
                .set_nonblocking(true)
                .map_err(|e| e.to_string())?;

            let mut event: epoll_event = unsafe { std::mem::zeroed() };
            event.events = EPOLLIN as u32;
            event.u64 = fd as u64;

            if unsafe {
                epoll_ctl(
                    self.epoll_fd,
                    libc::EPOLL_CTL_ADD,
                    fd,
                    &mut event,
                )
            } < 0
            {
                return Err(std::io::Error::last_os_error().to_string());
            }
        }
        Ok(())
    }

    pub fn run(&self) {
        let mut events: Vec<epoll_event> = Vec::with_capacity(32);
        unsafe { events.set_len(32) };

        loop {
            let nfds = unsafe {
                libc::epoll_wait(
                    self.epoll_fd,
                    events.as_mut_ptr(),
                    events.len() as i32,
                    -1,
                )
            };

            if nfds < 0 {
                dbg!(std::io::Error::last_os_error().to_string());
                continue;
            }

            for n in 0..nfds as usize {
                let fd = events[n].u64 as RawFd;

                for listener in self.listeners.iter() {
                    if listener.as_raw_fd() == fd {
                        match listener.accept() {
                            Ok((stream, addr)) => {
                                dbg!(stream, addr);
                                let stream_fd = stream.as_raw_fd();
                                let buf_reader = BufReader::new(&stream);
                                let mut request_string = String::new();
                                for line in buf_reader.lines() {
                                    let line = line.map_err(|e| e.to_string())?;
                                    request_string.push_str(&line);
                                    request_string.push_str("\n");
                                    if line.is_empty() {
                                        break;
                                    }
                                }
                                
                                let request = Request::from(request_string);
                                let cgi = CGI;


                                match cgi.is_cgi_request(&request, self.servers) {
                                    Ok(Some(cgi_py)) => {
                                        let cgi_script = cgi.execute_cgi(&cgi_py, &request,&mut  stream);
                                        if let Err(error) = cgi_script {
                                            dbg!(error);
                                            continue;
                                        }
                                    },
                                    Ok(None) => {
                                        let router = Router;
                                        router.route(request, &mut stream);
                                        continue;
                                    }
                                    Err(error) => {
                                        dbg!(error);
                                        continue;
                                    }
                                    
                                };
                                let mut event = unsafe { std::mem::zeroed() };
                                event.events = EPOLL_IN as u32;
                                event.u64 = stream_fd as u64;

                                if unsafe {
                                    epoll_ctl(
                                        self.epoll_fd,
                                        EPOLL_CTL_ADD,
                                        stream_fd,
                                        &mut event,
                                    )
                                } < 0
                                {
                                    dbg!(io::Error::last_os_error().to_string())
                                }
                            }
                            Err(error) => {
                                dbg!(error);
                                continue;
                            }
                        }
                    }
                }
            }
        }
    }
}
