#[derive(serde::Serialize)]
pub struct ErrorResponse {
    pub error: &'static str,
    pub message: String,
}

pub fn parse_json<T: serde::de::DeserializeOwned>(json: &str) -> Result<T, serde_json::Error> {
    match serde_json::from_str::<T>(&json) {
        Ok(data) => {
            Ok(data)
        },
        Err(e) => {
            Err(e)
        }
    }
}