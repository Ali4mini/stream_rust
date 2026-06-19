use axum::Router;
use clap::Parser;
use core::option::Option;
use core::option::Option::None;
use core::result::Result;
use core::result::Result::Ok;
use core::unreachable;
use std::fs;
use std::io;
use std::net::SocketAddr;
use std::process::Command;
use tower_http::services::ServeDir;

#[derive(Parser)]
#[command(version, about)]
struct Args {
    #[arg(short, long, group = "input")]
    file: Option<String>,

    #[arg(short, long, group = "input")]
    directory: Option<String>,
}

async fn start_server(port: u16) -> Result<(), Box<dyn std::error::Error>> {
    // serving the `tmp/` directory
    let address = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    let assets_router = Router::new().nest_service("/assets", ServeDir::new("./tmp/"));
    // let service_router = Router::new()

    let server_status = axum::serve(listener, assets_router).await?;

    Ok(server_status)
}

// an endpoint to show the available musics
fn show_dir_songs(input_dir: String) -> Result<String, io::Error> {
    for file in fs::read_dir(input_dir)? {
        let file = file?;
        let file_name = file.file_name();

        println!("{}", file_name.to_string_lossy())
    }

    let temp_string = String::from("all good");
    Ok(temp_string)
}

fn make_playlist(input_file: String) -> Result<String, io::Error> {
    const OUTPUT_DIR: &str = "./tmp/";
    let output_file = format!("{}output.m3u8", OUTPUT_DIR);
    fs::create_dir_all(OUTPUT_DIR)?; // creating the `tmp` dir

    let status = Command::new("ffmpeg")
        .args(&[
            "-i",
            &input_file,
            "-c:a",
            "aac",
            "-b:a",
            "192k",
            "-f",
            "hls",
            "-start_number",
            "0",
            "-hls_time",
            "10",
            "-hls_list_size",
            "0",
            &output_file,
        ])
        .status()?;

    if !status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "FFmpeg command failed",
        ));
    }

    Ok(output_file)
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let result: Result<String, io::Error>;

    match (args.file, args.directory) {
        (Some(file), None) => result = make_playlist(file),
        (None, Some(dir)) => result = show_dir_songs(dir),
        _ => unreachable!(),
    }

    match result {
        Err(error) => eprintln!("there was an error in the main! {}", error),
        Ok(_) => {
            const PORT: u16 = 8787; // change this for changing the workin port on the host
            let server_status = start_server(PORT).await;

            match server_status {
                Err(err) => eprintln!("ther was an error in the start_server, {}", err),
                _ => (),
            }
        }
    }
}
