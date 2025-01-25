use axum::Router;
use clap::Parser;
use std::fs;
use std::io;
use std::net::SocketAddr;
use std::process::Command;
use tower_http::services::ServeDir;

#[derive(Parser)]
#[command(version, about)]
struct Args {
    #[arg(short, long)]
    file: String,
}

async fn start_server(port: u16) -> Result<(), Box<dyn std::error::Error>> {
    // serving the `tmp/` directory
    let address = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    dbg!("started listener on: {}", &listener);

    let router = Router::new().nest_service("/assets", ServeDir::new("./tmp/"));

    let server_status = axum::serve(listener, router).await?;

    Ok(server_status)
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

    let result = make_playlist(args.file);

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
