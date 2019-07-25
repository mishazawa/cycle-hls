use std::process::{ Command, Stdio, Child, Output };
use std::io::{ Error };
use std::{fs};
use std::path::Path;

const PATH_LEN_THRESHOLD: usize = 8;

fn main() {
  let sample_dir = Path::new("./sample");
  let tmp_dir = Path::new("./tmp");
  let playlist_dir = Path::new(".");
  clear_tmp(tmp_dir);
  loop {
    gen_hls(sample_dir);
    generate_playlist(playlist_dir, tmp_dir);
  }

}

fn clear_tmp (path: &Path) {
  let paths: Vec<_> = fs::read_dir(path).unwrap().map(|r| r.unwrap()).collect();
  for file in &paths {
    fs::remove_file(file.path()).unwrap();
  }
}

fn generate_playlist (path: &Path, tmp_path: &Path) {
  let mut paths: Vec<_> = fs::read_dir(tmp_path).unwrap().map(|r| r.unwrap()).collect();
  paths.sort_by_key(|dir| dir.path());


}

fn gen_hls (path: &Path) {
  let paths: Vec<_> = fs::read_dir(path).unwrap().map(|r| r.unwrap()).collect();
  for file in paths {
    if let Ok(mut c) = spawn_ffmpeg(file.path().to_str().unwrap()) {
      println!("Exit with: {:?}", c.wait());
    } else {
      panic!("Can't spawn ffmpeg");
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
      "-hls_flags", "+omit_endlist +delete_segments",
      "-hls_list_size", "3",
      "-hls_time", "4",
      "-hls_playlist_type", "event",
      "-hls_segment_filename", "tmp/%03d.ts",
    ])
    .arg(format!("tmp/{}playlist.m3u8", target))
    .spawn()
}

fn dash (target: &str) -> Result<Child, Error> {
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
      "-f", "dash"
    ])
    .arg("tmp/playlist.mpd")
    .spawn()
}

fn get_segment_duration (path: &str) -> String {
  let Output {stdout, ..} = Command::new("ffprobe")
    .args(&[
      "-v", "error",
      "-show_entries", "format=duration",
      "-of", "default=noprint_wrappers=1:nokey=1"
    ])
    .arg(path)
    .output().unwrap();
    std::str::from_utf8(&stdout).unwrap().trim().to_string()
}
