// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use sysinfo::System;
use battery::Manager;
#[tauri::command]
fn get_system_info() -> [String; 3] {
    let mut sys = System::new_all();
    sys.refresh_all();
    let user = System::host_name().unwrap();
    let os = System::name().unwrap();
    let battery = Manager::new().unwrap().batteries().unwrap().next().unwrap().unwrap();
    let battery_percentage = (battery.state_of_charge().value * 100.0).round().to_string();

    return [user,os,battery_percentage];
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![get_system_info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
