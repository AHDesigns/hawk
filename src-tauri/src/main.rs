#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

fn main() {
  println!("started!");
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![my_custom_command, window_fn, log])
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
  println!("x {}", x);
  println!("p {:?}", point_in_space);
  println!("Window {}", window.label())
}

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct PointInSpace {
  x: u32,
  y: u32,
}
