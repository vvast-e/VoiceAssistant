use rodio::{source::Source, Decoder, OutputStream};
use std::collections::HashMap;
use std::fs;
use std::io::BufReader;
use std::path::Path;
use std::fs::File;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use crate::IGNORE_AUDIO;

fn play_audio(file_name:&str){
    IGNORE_AUDIO.store(true, Ordering::Relaxed);

    let file_path=Path::new("audio").join(file_name);
    let (_stream, stream_handle) = OutputStream::try_default().expect("Не удалось создать аудиопоток");
    let file = File::open(&file_path).expect("Не удалось открыть аудиофайл");
    let source = Decoder::new(BufReader::new(file)).expect("Не удалось декодировать аудио");
    let metadata = fs::metadata(file_path).expect("Не удалось получить метаданные файла");

    let file_size = metadata.len()/1024;
    stream_handle.play_raw(source.convert_samples()).expect("Не удалось воспроизвести аудио");
    if file_size>=50{
        std::thread::sleep(std::time::Duration::from_secs(2));
    }
    std::thread::sleep(std::time::Duration::from_secs(2));
    IGNORE_AUDIO.store(false, Ordering::Relaxed);
}

pub fn current_audio(text:&str){
        let phrases:HashMap<&str,&str> = [
            ("слушаю", "listen.mp3"),
            ("открываю","open.mp3"),
            ("поиск", "search.mp3"),
            ("люблю", "love.mp3"),
            ("приветствие", "greetings.mp3")
        ].iter().cloned().collect();
    if let Some(audio_file) = phrases.get(text) {
        play_audio(audio_file);
    } else {
        println!("Фраза для команды '{}' не найдена", text);
    }
}