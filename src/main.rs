
extern crate ser_test;
extern crate hyper;
extern crate futures;

use std::fs::File;
use std::io::Read;
use hyper::server::{Http, Service, Request, Response};
use hyper::{Method, StatusCode};
use hyper::Error;
use futures::future::{ok, Future};
use ser_test_lib::get_res_vec;

type HyperResult = Box<Future<Item = Response, Error = Error>>;

fn main() {
    let addr = "127.0.0.1:8080".parse().unwrap();
    let http = Http::new().bind(&addr, || Ok(Server)).unwrap();
    http.run();
}

struct Server;

impl Service for Server {
    type Request = Request;
    type Response = Response;
    type Error = Error;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;
    fn call(&self, req: Request) -> HyperResult {
        match (req.method(), req.path()) {
            (&Method::Get, "/") => index(),
            (&Method::Get, path) => static_file(path),
            _ => Box::new(ok(Response::new().with_status(StatusCode::NotFound)))
        }
    }
}

fn index() -> HyperResult {
    let mut ret: String = String::from("<!DOCTYPE html><html><head><title>WASM Serialization Test</title></head><body><div id=\"tests\"><ul id=\"native\" class=\"result-list\">");
    for entry in get_res_vec(Box::new(now), "ns") {
        ret += &format!("<li class=\"result\">{}</li>", entry);
    }
    ret += "</ul><ul id=\"wasm\" class=\"result-list\"></ul><script src=\"./index.js\"></script></div></body></html>";
    let buf = ret.to_string();
    Box::new(
        ok(
            Response::new()
                .with_body(buf)
        )
    )
}

fn now() -> u32 {
    let start = ::std::time::SystemTime::now().duration_since(::std::time::UNIX_EPOCH);
    start.unwrap().subsec_nanos()
}
fn static_file(path: &str) -> HyperResult {
    if let Ok(mut f) = File::open(path) {
        let mut buf = Vec::new();
        Box::new(
            ok(
                match f.read_to_end(&mut buf) {
                    Ok(_size) => Response::new().with_body(buf),
                    _ => Response::new().with_status(StatusCode::InternalServerError)
                }
            )
        )
    } else {
        Box::new(
            ok(
                Response::new()
                    .with_status(StatusCode::NotFound)
            )
        )
    }
}