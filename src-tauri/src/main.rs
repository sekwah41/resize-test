#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{PhysicalSize, Runtime};
use tauri::Size::Physical;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}


#[tauri::command]
fn set_max<R: Runtime>(window: tauri::Window<R>) {
    println!("Max resizeable!");
    window.set_max_size(Some(PhysicalSize { width: 500, height: 500 })).unwrap();
}


#[tauri::command]
fn set_min<R: Runtime>(window: tauri::Window<R>) {
    println!("Max resizeable!");
    window.set_min_size(Some(PhysicalSize { width: 400, height: 400 })).unwrap();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, set_max, set_min])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
