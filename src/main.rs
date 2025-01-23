use clap::Parser;
use std::fs;
use std::io;
use std::process::Command;

#[derive(Parser)]
#[command(version, about)]
struct Args {
    #[arg(short, long)]
    file: String,
}

fn make_playlist(input_file: String) -> Result<String, io::Error> {
    const OUTPUT_DIR: &str = "./tmp/";
    let output_file = format!("{}output.m3u8", OUTPUT_DIR);
    fs::create_dir_all(OUTPUT_DIR)?; // creating the `tmp` dir

    let status = Command::new("ffmpeg")
        .args(&[
            "-i",
            &input_file,
            "-codec",
            "copy",
            "-start_number",
            "0",
            "-hls_time",
            "10",
            "-hls_list_size",
            "0",
            "-f",
            "hls",
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

fn main() {
    let args = Args::parse();

    let result = make_playlist(args.file);

    match result {
        Err(error) => eprintln!("there was an error in the main! {}", error),
        _ => (),
    }
}
