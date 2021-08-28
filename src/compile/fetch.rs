use std::io;

use super::{Format};

pub(super) async fn fetch(encoded_diagram: String, format: Format) -> Result<Vec<u8>, io::Error> {
    make_request(uri(encoded_diagram, format)).await.map_err(|_| {
        io::Error::new(io::ErrorKind::ConnectionAborted, "Connection issue".to_string())
    })
}

async fn make_request(to_uri: String) -> Result<Vec<u8>, reqwest::Error> {
    Ok(reqwest::get(to_uri).await?.bytes().await?.to_vec())
}

fn uri(encoded_diagram: String, format: Format) -> String {
    format!("http://www.plantuml.com/plantuml/{}/{}", format.to_uri(), encoded_diagram)
}
