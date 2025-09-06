use std::io;

use may_minihttp::{Response};

use crate::{features::auth::model::LoginRequest, utils::parsejson::parse_json};

pub fn create_user(res: &mut Response, _body: String) -> io::Result<()> {
    res.status_code(200, "OK");
    res.header("Content-Type: application/json");
    res.body(r#"{"message": "User created successfully", "id": 4, "name": "New User"}"#);
    Ok(())
}

pub fn login(res: &mut Response, body: String) -> io::Result<()> {

    match parse_json::<LoginRequest>(&body) {
        Ok(user) => {
            // println!("Parsed user: {}", user.username);
            res.status_code(200, "Login successful");
            res.header("Content-Type: application/json");
            res.body(r#"{"message": "Login successful"}"#);
        },
        Err(e) => {
            // println!("JSON parse error: {}", e);
            res.status_code(400, "Bad Request");
            res.header("Content-Type: application/json");
            res.body(r#"{"error": "Invalid JSON format", "message": "{}"}"#);
        }
    }

    Ok(())
}