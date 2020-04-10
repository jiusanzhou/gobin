extern crate cfg_if;
extern crate wasm_bindgen;

#[macro_use]
extern crate serde_derive;

mod gobin;
mod utils;
mod worker;

use cfg_if::cfg_if;
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;

use gobin::gen_script;
use worker::{Request, Response};

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

// add a serve to serve

// why we splite body with request and response
// auto convert between request/response and jsvalue
#[wasm_bindgen]
pub fn handle(reqv: JsValue) -> Result<JsValue, JsValue> {
    let mut req: Request<String> = from_value(reqv)?;

    // Server::new().handle(req)

    let path = req.uri().path();

    let mut resp = Response::new("404 Page not found");

    // fake router
    match path {
        "" | "/" | "index.html" | "index.htm" => {
            resp.body = "Welcome gobin.zoe.im";
            return Ok(to_value(&resp)?);
        }
        "/_health" => {
            resp.body = "OK";
            return Ok(to_value(&resp)?);
        }
        _ => {
            // handle the gobin
            resp.body = gen_script(&path);
            return Ok(to_value(&resp)?);
        }
    }
}
