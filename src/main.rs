use std::process::{ Command, Stdio, Child };
use std::io::{ Error };
use std::{env, fs};

fn main() {
  env::set_current_dir("sample/").unwrap();
  loop {
    let dir = fs::read_dir(env::current_dir().unwrap()).unwrap();
    for i in dir {
      if let Ok(mut c) = spawn_ffmpeg(i.unwrap().path().to_str().unwrap()) {
        println!("Exit with: {:?}", c.wait());
      } else {
        panic!("Can't spawn ffmpeg");
      }
    }
  }
}


fn spawn_ffmpeg (target: &str) -> Result<Child, Error> {
  Command::new("ffmpeg")
    .stdin(Stdio::piped())
    .args(&["-y", "-re"])
    .args(&["-i", target])
    .args(&[
      "-c:v", "libx264",
      "-pix_fmt", "yuv420p",
      "-preset", "veryfast"
    ])
    .args(&[
      "-hls_list_size", "3",
      "-hls_time", "4",
      "-hls_playlist_type", "event",
      "-hls_segment_filename", "../tmp/%03d.ts",
      "-hls_flags", "+delete_segments +append_list +omit_endlist",
    ])
    .arg("../tmp/playlist.m3u8")
    .spawn()
}
