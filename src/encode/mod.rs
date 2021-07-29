mod deflate;
mod encode;

use std::io;
use deflate::deflate;
use encode::encode;


pub fn get_uri(uml: String) -> io::Result<String> {
    let res = deflate(uml)?;
    Ok(encode(res))
}
