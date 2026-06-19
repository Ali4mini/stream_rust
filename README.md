# stream_rust

Single-file Rust binary that takes a media file, converts it to HLS with FFmpeg, and serves it via Axum.

## Usage

```bash
cargo run -- -f <input_file>
```

The server starts on port **8787**. HLS output is available at:

```
http://localhost:8787/assets/output.m3u8
```

Open this URL in VLC or an HLS-compatible player. For browsers, use an HLS.js-based player pointing to the same URL.




## TODO
- [ ] JIT audio procecing 
     the user provides a directory as the main songs lib/playlist, and we will process the AUDIO THAT
     THE USER REQUESTS from that dir,
     later we store the processed audio and add a cachine layer
