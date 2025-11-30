use rust_embed::RustEmbed;
use warp::{Rejection, Reply, http::header::HeaderValue, path::Tail, reply};

#[derive(RustEmbed)]
#[folder = "dist"]
struct Asset;

pub async fn serve_index() -> Result<impl Reply, Rejection> {
    serve_impl("index.html").await
}

pub async fn serve(path: Tail) -> Result<impl Reply, Rejection> {
    serve_impl(path.as_str()).await
}

async fn serve_impl(path: &str) -> Result<impl Reply + 'static + use<>, Rejection> {
    let asset = Asset::get(path).ok_or_else(warp::reject::not_found)?;
    let mime = mime_guess::from_path(path).first_or_octet_stream();

    let mut res = reply::Response::new(asset.data.into());
    res.headers_mut().insert(
        "content-type",
        HeaderValue::from_str(mime.as_ref()).unwrap(),
    );
    Ok(res)
}
