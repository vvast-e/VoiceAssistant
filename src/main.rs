use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::thread;
use crossbeam_channel::bounded;
static IGNORE_AUDIO: AtomicBool = AtomicBool::new(false);
mod voiceinput;
mod commands;
mod math;
mod voiceoutput;

fn main() {
    voiceinput::init_vosk();
    let (command_sender, command_receiver) = bounded::<String>(1024);

    thread::spawn(move || {
        voiceinput::voice_input(command_sender);
    });

    println!("Программа запущена. Говорите команды...");
    for command in command_receiver {
        commands::handle_command(&command);
    }
}

