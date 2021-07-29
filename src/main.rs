mod encode;
mod fetch;


use std::{io, env};
use tokio::io::{AsyncWriteExt, AsyncWrite};
use std::io::Read;

#[tokio::main]
async fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help(args[0].clone());
        return Ok(())
    }

    let (mode, src, dst) = (args[1].clone(), args[2].clone(), args[3].clone());
    
    let mut single_file = true;
    let mut sync  = false;
    
    match mode.as_str() {
        "-f" => (),
        "-d" => single_file = false,
        "-fs" | "-sf" => sync = true,
        "-ds" | "-sd" => {
            single_file = false;
            sync = true;
        }
        "--help" | "-h" => {
            print_help(args[0].clone());
            return Ok(());
        }
        _ => return Err(io::Error::new(io::ErrorKind::Unsupported, "wrong mode"))
    }

    let (src, dst) = (args[2].clone(), args[3].clone());

    // let encoded = encode::get_uri(uml)?;
    // let diagram = fetch::fetch(encoded, fetch::Format::Png).await?;
    //
    // let mut file = tokio::fs::File::create("out.png").await?;
    // file.write_all(diagram.as_slice()).await?;

    Ok(())
}

fn print_help(cmd: String) {
    println!("Usage:");

    println!("{} --help", cmd);
    println!("{} -h", cmd);
    println!("\tPrint this help");

    println!("{} [mode] [src] [dst]", cmd);
    println!("\t")
}
