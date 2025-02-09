use anyhow::Result;
use std::env;
use std::fs::File;
use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use w3s::writer::{car, uploader, ChainWrite};

fn get_file_name(path: &String) -> Option<String> {
    let path = std::path::Path::new(path);
    path.file_name()
        .and_then(|name| name.to_str())
        .and_then(|x| Some(x.to_owned()))
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = env::args().collect::<Vec<_>>();
    let host = "https://api.web3.storage".to_string();
    match args.as_slice() {
        [_, path, auth_token] => upload(path, &host, auth_token).await,
        _ => panic!(
            "\n\nPlease input [the_path_to_the_file] and [web3.storage_auth_token(eyJhbG......MHlq0)]\n\n"
        ),
    }
}

async fn upload(path: &String, host: &String, auth_token: &String) -> Result<()> {
    let mut file = File::open(path)?;
    let filename = get_file_name(path).unwrap();

    let uploader = uploader::Uploader::new(
        auth_token.clone(),
        host.clone(),
        filename.clone(),
        uploader::UploadType::Car,
        2,
        Some(Arc::new(Mutex::new(|name, part, pos, total| {
            println!("name: {name} part:{part} {pos}/{total}");
        }))),
    );

    let mut car = car::Car::new(
        1,
        Arc::new(Mutex::new(vec![car::single_file_to_directory_item(
            &filename, None,
        )])),
        None,
        None,
        uploader,
    );

    io::copy(&mut file, &mut car)?;
    car.flush()?;

    let mut uploader = car.next();
    let results = uploader.finish_results().await?;
    println!("results: {:?}", results);

    Ok(())
}
