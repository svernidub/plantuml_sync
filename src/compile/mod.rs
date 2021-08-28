mod deflate;
mod encode;
mod fetch;

use std::io;
use deflate::deflate;
use encode::encode;
use fetch::fetch;

#[derive(Copy, Clone)]
pub enum Format {
    Png,
    Svg,
    SvgVersionable,
    Latex
}
use Format::*;

impl Format {
    fn to_uri(&self) -> String {
        match *self {
            Png                  => "png",
            Svg | SvgVersionable => "svg",
            Latex                => "latex",
        }.to_string()
    }
}

pub async fn compile(uml: String, format: Format) -> io::Result<Vec<u8>> {
    let diagram = deflate(uml).map(encode)?;
    fetch(diagram, format).await
}
