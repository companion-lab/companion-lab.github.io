use dioxus::{
    fullstack::{http::header::USER_AGENT, HeaderMap, ServerFnError},
    prelude::*,
};

#[get("/api/detect_os", headers: HeaderMap)]
pub async fn detect_os() -> Result<i32, ServerFnError> {
    let ua = headers
        .get(USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    if ua.contains("Mac") {
        Ok(1)
    } else if ua.contains("Linux") || ua.contains("X11") {
        Ok(2)
    } else {
        Ok(0)
    }
}
