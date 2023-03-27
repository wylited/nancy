// Prevents additional console window on Windows in release, DO NOT REMOVE!!

#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

fn main() {
    nancy::AppBuilder::new().run();
}
