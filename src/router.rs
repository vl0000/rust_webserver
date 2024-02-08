use std::collections::HashMap;

use crate::messaging::{Request, Response};

/// Stores routes by method, then route.
pub struct Router {
        pub get: HashMap<String, Box<dyn Fn(Request) -> Response >>,
}

impl Router {

        pub fn get (&mut self, path: &str, handler: &'static dyn Fn(Request) -> Response) {
                self.get.insert(path.to_string(), Box::new(handler));
        }
}