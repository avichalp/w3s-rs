use anyhow::Result;
use std::env;
use std::fs::File;
use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use w3s::writer::{splitter, uploader, ChainWrite};

fn get_file_name(path: &String) -> Option<String> {
    let path = std::path::Path::new(path);
    path.file_name()
        .and_then(|name| name.to_str())
        .and_then(|x| Some(x.to_owned()))
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = env::args().collect::<Vec<_>>();

    match args.as_slice() {
        [_, path, auth_token] => upload(path, auth_token).await,
        _ => panic!(
            "\n\nPlease input [the_path_to_the_file] and [web3.storage_auth_token(eyJhbG......MHlq0)]\n\n"
        ),
    }
}

async fn upload(path: &String, auth_token: &String) -> Result<()> {
    let mut file = File::open(path)?;
    let filename = get_file_name(path).unwrap();

    let uploader = uploader::Uploader::new(
        auth_token.clone(),
        filename.clone(),
        uploader::UploadType::Upload,
        2,
        Some(Arc::new(Mutex::new(|name, part, pos, total| {
            println!("name: {name} part:{part} {pos}/{total}");
        }))),
    );
    let splitter = splitter::PlainSplitter::new(uploader);

    // need feature `zstd`
    let mut compressor = zstd::stream::Encoder::new(splitter, 10)?;
    io::copy(&mut file, &mut compressor)?;

    let mut splitter = compressor.finish()?;
    splitter.flush()?; // this line is needed to upload the final part of the file

    let mut uploader = splitter.next();
    let results = uploader.finish_results().await?;
    println!("results: {:?}", results);

    Ok(())
}
