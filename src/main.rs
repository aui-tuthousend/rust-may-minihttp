extern crate may_minihttp;
use may_mini::routes::{ImprovedRouter};
use may_minihttp::{HttpServer};

fn main() {
    env_logger::init();
    let mut improved_router = ImprovedRouter::new();
    improved_router.register_auth_routes();
    improved_router.register("GET", "/", |res, _body| {
        res.status_code(200, "OK");
        res.header("Content-Type: application/json");
        res.body(r#"Hello from May Mini!"#);
        Ok(())
    });
    println!("Starting function-based router on http://0.0.0.0:5000");
    let server = HttpServer(improved_router).start("0.0.0.0:5000").unwrap();
    server.join().unwrap();
}
