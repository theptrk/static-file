#![crate_id = "staticfile"]
#![deny(missing_doc)]
#![feature(phase)]

//! Static file-serving middleware.

#[phase(plugin)]
extern crate regex_macros;
extern crate regex;

extern crate iron;
#[phase(plugin, link)]
extern crate log;

use std::path::BytesContainer;
use std::str::from_utf8;

use iron::{Request, Response, Middleware, Alloy};
use iron::mixin::{GetUrl, Serve};
use iron::middleware::{Status, Continue, Unwind};

/// The static file-serving `Middleware`.
#[deriving(Clone)]
pub struct Static {
    root_path: Path
}

#[deriving(Clone)]
#[doc(hidden)]
struct Favicon {
    favicon_path: Path
}

impl Static {
    /// Create a new instance of `Static` with a given root path.
    ///
    /// This will attempt to serve static files from the given root path.
    /// The path may be relative or absolute. If `Path::new("")` is given,
    /// files will be served from the current directory.
    ///
    /// If a static file exists and can be read from, `enter` will serve it to
    /// the `Response` and `Unwind` the middleware stack with a status of `200`.
    ///
    /// In the case of any error, it will `Continue` through the stack.
    /// If a file should have been read but cannot, due to permissions or
    /// read errors, a different `Middleware` should handle it.
    ///
    /// If the path is '/', it will attempt to serve `index.html`.
    pub fn new(root_path: Path) -> Static {
        Static {
            root_path: root_path
        }
    }

    /// Create a favicon server from the given filepath.
    ///
    /// This will serve your favicon, as specified by `favicon_path`,
    /// to every request ending in "/favicon.ico" that it sees,
    /// and then unwind the middleware stack for those requests.
    ///
    /// It should be linked first in order to avoid additional processing
    /// for simple favicon requests.
    ///
    /// Unlike normally served static files, favicons are given a max-age
    /// of one day.
    #[allow(visible_private_types)]
    pub fn favicon(favicon_path: Path) -> Favicon {
        Favicon {
            favicon_path: favicon_path
        }
    }
}

impl Middleware for Static {
    fn enter(&mut self, req: &mut Request, res: &mut Response, _alloy: &mut Alloy) -> Status {
        match req.url() {
            Some(path) => {
                debug!("Serving static file at {}{}.", from_utf8(self.root_path.container_as_bytes()).unwrap(), path);
                let mut relative_path = path.clone();
                if relative_path.eq(&"/".to_string()) {
                    relative_path = "index.html".to_string();
                } else {
                    relative_path.shift_char();
                }
                match res.serve_file(&self.root_path.join(Path::new(relative_path.to_string()))) {
                    Ok(()) => { Unwind },
                    Err(_) => { Continue }
                }
            },
            None => {
                Continue
            }
        }
    }
}

impl Middleware for Favicon {
    fn enter(&mut self, req: &mut Request, res: &mut Response, _alloy: &mut Alloy) -> Status {
        match req.request_uri {
            AbsolutePath(ref path) => {
                if regex!("/favicon$").is_match(path.as_slice()) {
                    match res.serve_file(&self.favicon_path) {
                        Ok(()) => {
                            res.headers.cache_control = Some("max-age=86400".to_str());
                            return Unwind },
                        Err(_) => { return Continue }
                    }
                }
                Continue
            },
            _ => {
                Continue
            }
        }
    }
}
