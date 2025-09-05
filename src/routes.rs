use std::{collections::HashMap, io::{self, Read}, time::Instant};
use may_minihttp::{HttpService, Request, Response};
use crate::features::auth;

type HandlerFn = fn(&mut Response) -> io::Result<()>;

#[derive(Clone)]
pub struct ImprovedRouter {
    routes: HashMap<String, HashMap<String, HandlerFn>>,
}

impl ImprovedRouter {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }
    
    pub fn register(&mut self, method: &str, path: &str, handler: HandlerFn) {
        self.routes
            .entry(path.to_string())
            .or_insert_with(HashMap::new)
            .insert(method.to_uppercase(), handler);
    }
    
    pub fn register_auth_routes(&mut self) {
        // self.register("POST", "/api/auth/register", auth::create_user);
        // self.register("POST", "/api/auth/login", auth::login_user);
        self.register("GET", "/api/auth", auth::handler::get_auth);
        self.register("POST", "/api/auth", auth::handler::create_user);
    }
    
    fn find_handler(&self, method: &str, path: &str) -> Option<HandlerFn> {
        self.routes
            .get(path)?
            .get(&method.to_uppercase())
            .copied()
    }
    
    fn handle_not_found(&self, res: &mut Response) -> io::Result<()> {
        res.status_code(404, "Not Found")
        .header("Content-Type: application/json")
        .body(r#"{"error": "Route not found", "code": 404}"#);
        Ok(())
    }
    
    fn handle_method_not_allowed(&self, res: &mut Response) -> io::Result<()> {
        res.status_code(405, "Method Not Allowed")
        .header("Content-Type: application/json")
        .body(r#"{"error": "Method not allowed", "code": 405}"#);
        Ok(())
    }
}

impl HttpService for ImprovedRouter {
    fn call(&mut self, req: Request, res: &mut Response) -> io::Result<()> {
        // let body = req.body();
        let method = req.method().to_owned();
        let path = req.path().to_owned();
        // let body = req.body();

        let mut body_content = String::new();
        if let Err(e) = req.body().read_to_string(&mut body_content) {
            println!("Error reading body: {}", e);
            res.status_code(400, "Bad Request");
            res.body(r#"{"error": "Could not read request body"}"#);
            return Ok(());
        }

        println!("Body content: {}", body_content);

        let start = Instant::now();

        println!("[REQ] {} {}", method, path);
        // log::info!("[REQ] {} {}", &req.method(), &req.path());

        // let mut body_content = String::new();
        // if let Err(e) = req.body().read_to_end(&mut body_content) {
        //     println!("Error reading body: {}", e);
        //     res.status_code(400, "Bad Request");
        //     res.body(r#"{"error": "Could not read request body"}"#);
        //     return Ok(());
        // }
        
        
        let result = match self.find_handler(&method, &path) {
            Some(handler) => handler(res),
            None => {
                if self.routes.contains_key(&path) {
                    self.handle_method_not_allowed(res)
                } else {
                    self.handle_not_found(res)
                }
            }
        };

        println!("[RES] {} {} -> took {:?})", &method, &path, start.elapsed());
        // log::info!("[RES] {} {} -> took {:?})", &req.method(), &req.path(), start.elapsed());

        result
    }
}
