use std::io;

#[derive(Copy, Clone)]
pub enum Format {
    Png,
    Svg,
    SvgVersionable,
    Latex
}
use Format::*;
use reqwest::{Response, Error};
use std::io::Read;

pub(super) async fn fetch(encoded_diagram: String, format: Format) -> Result<Vec<u8>, io::Error> {
    match make_request(uri(encoded_diagram, format)).await {
        Ok(data) => Ok(data),
        Err(e) => Err(io::Error::new(io::ErrorKind::ConnectionAborted, "something".to_string()))
    }
}

async fn make_request(to_uri: String) -> Result<Vec<u8>, reqwest::Error> {
    let  res = reqwest::get(to_uri).await?;
    let bytes = res.bytes().await?;
    Ok(bytes.to_vec())
}

fn format_uri(format: Format) -> String {
    match format {
        Png                  => "png",
        Svg | SvgVersionable => "svg",
        Latex                => "latex",
    }.to_string()
}

fn uri(encoded_diagram: String, format: Format) -> String {
    format!("http://www.plantuml.com/plantuml/{}/{}", format_uri(format), encoded_diagram)
}
