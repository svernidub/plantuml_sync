mod deflate;
mod encode;

use std::io;
use deflate::deflate;
use encode::encode;


pub fn get_uri(uml: String) -> io::Result<String> {
    deflate(uml).map(encode)
}
