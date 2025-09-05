extern crate may_minihttp;
use may_mini::routes::{health_check, ImprovedRouter};
use may_minihttp::{HttpServer};

fn main() {
    let mut improved_router = ImprovedRouter::new();
    improved_router.register_auth_routes();
    improved_router.register("GET", "/api/health", health_check);
    println!("Starting function-based router on http://0.0.0.0:5000");
    let server = HttpServer(improved_router).start("0.0.0.0:5000").unwrap();
    server.join().unwrap();
}
