use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::thread;
use crossbeam_channel::bounded;

mod voiceinput;
mod commands;

fn main() {
    voiceinput::init_vosk();
    let (command_sender, command_receiver) = bounded::<String>(1024);
    let should_stop = Arc::new(AtomicBool::new(false));

    let audio_thread = std::thread::spawn({
        let should_stop = Arc::clone(&should_stop);
        move || {
            voiceinput::voice_input(command_sender, should_stop);
        }
    });

    for command in command_receiver {
        commands::handle_command(&command);

        if command.to_lowercase().contains("стоп запись") {
            should_stop.store(true, Ordering::SeqCst);
            break;
        }
    }

    audio_thread.join().unwrap();
}

