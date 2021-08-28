use std::io;

#[derive(Copy, Clone)]
pub enum Format {
    Png,
    Svg,
    SvgVersionable,
    Latex
}
use Format::*;
use std::io::Read;

pub(super) async fn fetch(encoded_diagram: String, format: Format) -> Result<Vec<u8>, io::Error> {
    make_request(uri(encoded_diagram, format)).await.map_err(|_| {
         io::Error::new(io::ErrorKind::ConnectionAborted, "Connection issue".to_string())
    })
}

async fn make_request(to_uri: String) -> Result<Vec<u8>, reqwest::Error> {
    Ok(reqwest::get(to_uri).await?.bytes().await?.to_vec())
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
