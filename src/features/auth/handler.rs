use std::io;

use may_minihttp::{Request, Response};

pub fn create_user(res: &mut Response) -> io::Result<()> {
    res.status_code(200, "OK");
    res.header("Content-Type: application/json");
    res.body(r#"{"message": "User created successfully", "id": 4, "name": "New User"}"#);
    Ok(())
}

pub fn get_auth(res: &mut Response) -> io::Result<()> {
    res.status_code(200, "OK");
    res.header("Content-Type: application/json");
    res.body(r#"{"message": "Sup Nigga"}"#);
    Ok(())
}