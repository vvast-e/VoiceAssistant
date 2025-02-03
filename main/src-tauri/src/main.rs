use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::sync::mpsc;
use tauri::Manager;

mod voiceinput;
mod commands;
mod math;
mod voiceoutput;
static IGNORE_AUDIO: AtomicBool = AtomicBool::new(false);
lazy_static::lazy_static! {
    static ref GENERAL_LISTENING_ACTIVE: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
}

#[tauri::command]
async fn toggle_general_listening() -> String {
    let is_active = GENERAL_LISTENING_ACTIVE.load(Ordering::Relaxed);
    if is_active {
        GENERAL_LISTENING_ACTIVE.store(false, Ordering::Relaxed);
        "Общий поток выключен".to_string()
    } else {
        GENERAL_LISTENING_ACTIVE.store(true, Ordering::Relaxed);
        let (command_sender, mut command_receiver) = mpsc::channel(1024);

        let general_listening_active = Arc::clone(&GENERAL_LISTENING_ACTIVE);
        tokio::spawn(async move {
            voiceinput::listen(command_sender, general_listening_active, true, false).await;
        });

        println!("Общий поток включен. Ожидание ключевого слова 'Риша'...");

        // Асинхронная обработка команд
        tokio::spawn(async move {
            while let Some(command) = command_receiver.recv().await {
                commands::handle_command(&command);
            }
        });

        "Общий поток включен".to_string()
    }
}

#[tauri::command]
async fn activate_command_listening() -> String {
    if GENERAL_LISTENING_ACTIVE.load(Ordering::Relaxed) {
        let (command_sender, mut command_receiver) = mpsc::channel(1024);

        tokio::spawn(async move {
            voiceinput::listen(command_sender, Arc::clone(&GENERAL_LISTENING_ACTIVE), false, true).await;
        });

        println!("Активировано прослушивание команд. Говорите команды...");

        tokio::spawn(async move {
            while let Some(command) = command_receiver.recv().await {
                commands::handle_command(&command);
            }
        });

        "Прослушивание команд активировано".to_string()
    } else {
        "Общий поток не активен. Сначала включите общий поток.".to_string()
    }
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .setup(|app| {
            voiceinput::init_vosk(); // Инициализация Vosk
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![toggle_general_listening, activate_command_listening])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}