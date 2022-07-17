#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::Manager;

fn main() {
  tauri::Builder::default()
  .setup(|app| {
    let window = app.get_window("main").unwrap();
    window_vibrancy::apply_acrylic(&window, Some((18,12,24, 150))).unwrap();
    window_shadows::set_shadow(&window, true).unwrap();
    Ok(())
  })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
