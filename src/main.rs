mod compile;


use std::{io, env};
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use tokio::fs::OpenOptions;
use crate::compile::Format;
use std::ffi::OsString;
use std::path::Path;

#[tokio::main]
async fn main() -> io::Result<()> {
    match mode().as_str() {
        "-f" => {
            let (src, dst) = src_and_dst()?;
            compile_file(src, dst).await?;
        },
        "-d" => {
            let (src, dst) = src_and_dst()?;
            compile_dir(src, dst).await?;
        },
        "-fs" | "-sf" => (),
        "-ds" | "-sd" => (),
        "-h" => {
            print_help();
        },
        k => return Err(io::Error::new(io::ErrorKind::Other, format!("wrong mode: {}", k)))
    }

    Ok(())
}

fn mode() -> String {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { "-h".to_string() } else { args[1].clone() }
}

fn src_and_dst() -> io::Result<(String, String)> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        return Err(io::Error::new(io::ErrorKind::Other, "not enough args".to_string()))
    }

    Ok((args[2].clone(), args[3].clone()))
}

fn build_tree(src: OsString, dst: OsString) -> io::Result<Vec<(OsString, OsString)>> {
    let mut files = Vec::new();
    let mut directory = std::fs::read_dir(src)?;

    let dst_path = Path::new(dst.as_os_str());

    while let Some(Ok(entry)) = directory.next() {
        if entry.metadata()?.is_dir() {
            let mut nested =  build_tree(entry.path().into(), dst_path.into())?;
            files.append(&mut nested);
        } else {
            files.push((entry.path().into_os_string(), dst_path.join(entry.file_name()).into_os_string()));
        }
    }

    Ok(files)
}

async fn compile_dir(src: String, dst: String) -> io::Result<()> {
    let task = build_tree(src.into(), dst.into())?;

    for (src_file, dst_file) in task.into_iter() {
        compile_file(src_file.to_str().unwrap().into(), dst_file.to_str().unwrap().into()).await?;
    }

    Ok(())
}

async fn compile_file(src: String, dst: String) -> io::Result<()> {
    if !src.ends_with(".wsd") {
        return Ok(())
    }

    let mut buf = Vec::new();

    let mut src_file = tokio::fs::File::open(src.clone()).await?;
    src_file.read_to_end(&mut buf).await?;
    let uml = String::from_utf8(buf).map_err(|_| {
        io::Error::new(io::ErrorKind::Other, format!("Cannot use file content as UTF-8 string: {}", src).as_str())
    })?;

    let diagram = compile::compile(uml, Format::Png).await?;

    OpenOptions::new().create(true).truncate(true).write(true).open(dst).await?.write_all(diagram.as_slice()).await?;

    Ok(())
}

fn print_help() {
    let args: Vec<String> = env::args().collect();
    let cmd = args[0].clone();

    println!("Usage:\n\t{} [mode] [src] [dst]\n", cmd);

    println!("Modes:");
    println!("\t-h\t\t–\tprint this help");
    println!("\t-f\t\t–\tcompile a file");
    println!("\t-d\t\t–\tcompile all files in a directory");
    println!("\t-fs\t\t–\tcompile a file each time it is changed");
    println!("\t-ds\t\t–\tcompile all files in directory each time these files are changed");
}
