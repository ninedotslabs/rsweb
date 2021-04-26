use std::io::{self, Write};
use std::time::{Duration, SystemTime};
use templates::{statics::StaticFile, RenderRucte};
use warp::http::{Response, StatusCode};
use warp::{path, Filter, Rejection, Reply};

#[tokio::main]
async fn main() {
    let routes = warp::get()
        .and(
            path::end()
                .and_then(home_page)
                .or(path("static").and(path::param()).and_then(static_file)),
        )
        .recover(customize_error);

    let webserver = warp::serve(routes);

    webserver.run(([127, 0, 0, 1], 3030)).await;
}

static FAR: Duration = Duration::from_secs(180 * 24 * 60 * 60);

async fn home_page() -> Result<impl Reply, Rejection> {
    Response::builder().html(|o| {
        templates::page(
            o,
            &[("warp", "https://github.com/seanmonstar/warp"), ("ructe", "https://github.com/kaj/ructe")],
        )
    })
}

#[derive(Debug)]
struct SomeError;
impl std::error::Error for SomeError {}
impl warp::reject::Reject for SomeError {}

impl std::fmt::Display for SomeError {
    fn fmt(&self, out: &mut std::fmt::Formatter) -> std::fmt::Result {
        out.write_str("Some error")
    }
}

fn footer(out: &mut dyn Write) -> io::Result<()> {
    templates::footer(out, &("ahdyt", "https://alfianguide.netlify.app"))
}

async fn static_file(name: String) -> Result<impl Reply, Rejection> {
    if let Some(data) = StaticFile::get(&name) {
        let _far_expires = SystemTime::now() + FAR;
        Ok(Response::builder()
            .status(StatusCode::OK)
            .header("content-type", data.mime.as_ref())
            .body(data.content))
    } else {
        Err(warp::reject::not_found())
    }
}

async fn customize_error(err: Rejection) -> Result<impl Reply, Rejection> {
    if err.is_not_found() {
        Response::builder().status(StatusCode::NOT_FOUND).html(|o| {
            templates::error(
                o,
                StatusCode::NOT_FOUND,
                "The resource you requested could not be located.",
            )
        })
    } else {
        let code = StatusCode::INTERNAL_SERVER_ERROR;
        Response::builder()
            .status(code)
            .html(|o| templates::error(o, code, "Something went wrong."))
    }
}

include!(concat!(env!("OUT_DIR"), "/templates.rs"));
