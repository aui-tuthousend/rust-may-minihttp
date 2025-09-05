use std::{collections::HashMap, io};
use may_minihttp::{HttpService, Request, Response};
use crate::features::auth;

type HandlerFn = fn(&Request, &mut Response) -> io::Result<()>;

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
    
    fn handle_not_found(&self, _req: &Request, res: &mut Response) -> io::Result<()> {
        res.status_code(404, "Not Found");
        res.header("Content-Type: application/json");
        res.body(r#"{"error": "Route not found", "code": 404}"#);
        Ok(())
    }
    
    fn handle_method_not_allowed(&self, _req: &Request, res: &mut Response) -> io::Result<()> {
        res.status_code(405, "Method Not Allowed");
        res.header("Content-Type: application/json");
        res.body(r#"{"error": "Method not allowed", "code": 405}"#);
        Ok(())
    }
}

impl HttpService for ImprovedRouter {
    fn call(&mut self, req: Request, res: &mut Response) -> io::Result<()> {
        let method = req.method();
        let path = req.path();
        
        match self.find_handler(method, path) {
            Some(handler) => handler(&req, res),
            None => {
                if self.routes.contains_key(path) {
                    self.handle_method_not_allowed(&req, res)
                } else {
                    self.handle_not_found(&req, res)
                }
            }
        }
    }
}

pub fn health_check(_req: &Request, res: &mut Response) -> io::Result<()> {
    res.status_code(200, "OK");
    res.header("Content-Type: application/json");
    res.body(r#"{"status": "ok", "timestamp": "2024-01-01T00:00:00Z"}"#);
    Ok(())
}