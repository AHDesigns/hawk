#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod lib;

use std::path::Path;

use app::Buffer;
use lib::PointInSpace;

fn main() {
  println!("started!");
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      my_custom_command,
      window_fn,
      log,
      open_buffer
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn log(msg: String) {
  println!("{}", msg);
}

#[tauri::command]
fn my_custom_command(num: u32) -> u32 {
  num + 1
}

#[tauri::command]
fn window_fn(window: tauri::Window, x: u32, point_in_space: PointInSpace) {
  lib::window_fn(window.label(), x, point_in_space)
}

#[tauri::command]
fn open_buffer(path: String) -> Result<Buffer, String> {
  app::open_buffer(&Path::new(&path))
}
