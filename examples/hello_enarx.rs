use chrono::{DateTime, Utc};
use http::header::{CACHE_CONTROL, CONTENT_TYPE, LAST_MODIFIED};
use http::StatusCode;
use simple_logger::SimpleLogger;

fn enarx_logo() -> &'static [u8] {
    include_bytes!("enarx-logo.svg")
}

fn index_page() -> &'static [u8] {
    include_bytes!("index.html")
}

const NOT_FOUND: &str = r#"
<!DOCTYPE HTML PUBLIC "-//IETF//DTD HTML 2.0//EN">
<html><head>
<title>404 Not Found</title>
</head><body>
<h1>Not Found</h1>
<p>The requested URL was not found on this server.</p>
</body></html>
"#;

fn run() -> Result<(), Box<dyn std::error::Error>> {
    SimpleLogger::new().init().unwrap();

    mini_http::Server::preopened()?
        .tcp_nodelay(true)
        .start(|req| {
            let now: DateTime<Utc> = Utc::now();
            let now_str = now.format("%a, %d %b %Y %H:%M:%S GMT").to_string();
            match req.uri().path() {
                "/enarx-logo.svg" => mini_http::Response::builder()
                    .status(StatusCode::OK)
                    .header(CONTENT_TYPE, "image/svg+xml")
                    .header(LAST_MODIFIED, &now_str)
                    .header(CACHE_CONTROL, "max-age=60, public")
                    .body(enarx_logo().to_vec())
                    .unwrap(),
                "/" => mini_http::Response::builder()
                    .status(StatusCode::OK)
                    .header(CONTENT_TYPE, "text/html; charset=UTF-8")
                    .header(LAST_MODIFIED, &now_str)
                    .header(CACHE_CONTROL, "max-age=60, public")
                    .body(index_page().to_vec())
                    .unwrap(),
                _ => mini_http::Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .header(CONTENT_TYPE, "text/html; charset=UTF-8")
                    .header(LAST_MODIFIED, &now_str)
                    .header(CACHE_CONTROL, "no-store, must-revalidate")
                    .body(NOT_FOUND.as_bytes().to_vec())
                    .unwrap(),
            }
        })?;
    Ok(())
}

pub fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {:?}", e);
    }
}
