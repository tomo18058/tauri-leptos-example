#[tauri::command]
fn greet(name: &str) -> String {
    format!("こんちくは, {}さん!", name)
}

// windowsの環境情報を取得する
#[tauri::command]
fn get_system_info() -> String {
    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;

    let cpu_count = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);

    let current_dir = std::env::current_dir()
        .map(|path| path.display().to_string())
        .unwrap_or_else(|_| "取得失敗".to_string());

    format!(
        "OS: {}\nアーキテクチャ: {}\nCPU並列処理数: {}\n作業ディレクトリ: {}",
        os, arch, cpu_count, current_dir
    )
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet,get_system_info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
