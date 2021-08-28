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
        "-h" => {
            print_help(args[0].clone());
            return Ok(());
        },
        k => return Err(io::Error::new(io::ErrorKind::Other, format!("wrong mode: {}", k)))
    }

    // let encoded = encode::get_uri(uml)?;
    // let diagram = fetch::fetch(encoded, fetch::Format::Png).await?;
    //
    // let mut file = tokio::fs::File::create("out.png").await?;
    // file.write_all(diagram.as_slice()).await?;

    Ok(())
}

fn print_help(cmd: String) {
    println!("Usage:\n\t{} [mode] [src] [dst]\n", cmd);

    println!("Modes:");
    println!("\t-h\t\t–\tprint this help");
    println!("\t-f\t\t–\tcompile a file");
    println!("\t-d\t\t–\tcompile all files in a directory");
    println!("\t-fs\t\t–\tcompile a file each time it is changed");
    println!("\t-ds\t\t–\tcompile all files in directory each time these files are changed");
}
