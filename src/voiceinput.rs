use once_cell::sync::OnceCell;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Instant, Duration};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{StreamConfig, SampleRate};
use vosk::{Model, Recognizer};
use crossbeam_channel::Sender;
use crate::{voiceoutput, IGNORE_AUDIO};

static IS_PROCESSING: AtomicBool = AtomicBool::new(false);
static MODEL: OnceCell<Model> = OnceCell::new();
static RECOGNIZER: OnceCell<Mutex<Recognizer>> = OnceCell::new();

pub fn init_vosk() {
    if RECOGNIZER.get().is_none() {
        let model = Model::new("D:/VoiceAssistant/vosk-model-ru-0.22").unwrap();
        let mut recognizer = Recognizer::new(&model, 16000.0).unwrap();

        recognizer.set_max_alternatives(10);
        recognizer.set_words(true);
        recognizer.set_partial_words(true);

        MODEL.set(model);
        RECOGNIZER.set(Mutex::new(recognizer));
    }
}

pub fn voice_input(command_sender: Sender<String>) {
    let host = cpal::default_host();
    let input_device = host.default_input_device().expect("No input device available");

    let supported_config = input_device
        .supported_input_configs()
        .expect("Failed to get supported input configs")
        .next()
        .expect("No supported input configs")
        .with_max_sample_rate();

    let config = StreamConfig {
        channels: supported_config.channels(),
        sample_rate: SampleRate(48000),
        buffer_size: cpal::BufferSize::Default,
    };

    let should_stop = Arc::new(AtomicBool::new(false));
    let should_stop_clone = Arc::clone(&should_stop);



    let stream = input_device
        .build_input_stream(
            &config,
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                if IGNORE_AUDIO.load(Ordering::Relaxed) {
                    return;
                }
                let mono_data: Vec<f32> = data.chunks(2).map(|chunk| (chunk[0] + chunk[1]) / 2.0).collect();
                let resampled_data: Vec<f32> = mono_data.iter().step_by(3).cloned().collect();
                let pcm_data: Vec<i16> = resampled_data.iter().map(|&sample| (sample * i16::MAX as f32) as i16).collect();

                let mut recognizer = RECOGNIZER.get().unwrap().lock().unwrap();
                recognizer.accept_waveform(&pcm_data).expect("Failed to process audio");

                let partial_result = recognizer.partial_result();
                //println!("Partial result: {}", partial_result.partial);

                if partial_result.partial.to_lowercase().contains("хватит") {
                    should_stop_clone.store(true, Ordering::SeqCst);
                }
            },
            move |err| {
                eprintln!("An error occurred in the audio stream: {}", err);
            },
            None,
        )
        .expect("Failed to build input stream");

    stream.play().expect("Failed to start stream");
    println!("Listening for audio...");
    voiceoutput::current_audio("приветствие");

    'outer: loop {
        if IS_PROCESSING.load(Ordering::Relaxed) {
            std::thread::sleep(Duration::from_millis(100));
            continue;
        }

        let result = {
            let mut recognizer = RECOGNIZER.get().unwrap().lock().unwrap();
            let final_result = recognizer.final_result();
            final_result.multiple().unwrap().alternatives.first().unwrap().text.to_owned()
        };


        if result.contains("риша") {
            println!("Ключевое слово 'риша' обнаружено. Ожидание команды...");
            IS_PROCESSING.store(true, Ordering::Relaxed);
            voiceoutput::current_audio("слушаю");
            loop {

                let command = {
                    let mut recognizer = RECOGNIZER.get().unwrap().lock().unwrap();
                    //std::thread::sleep(Duration::from_secs(3));
                    let final_result = recognizer.final_result();
                    final_result.multiple().unwrap().alternatives.first().unwrap().text.to_owned()
                };
                let corrected_command = command.replace("телеграмму", "телеграмм")
                    .replace("телеграммы", "телеграмм").replace("откроем","открой");
                if should_stop.load(Ordering::SeqCst) || command == "хватит" {
                    println!("Остановка прослушивания...");
                    break;
                }
                else if corrected_command=="стоп запись" {
                    voiceoutput::current_audio("люблю");
                    break 'outer;
                }

                if !corrected_command.is_empty() {
                    println!("Команда получена: {}", command);
                    if corrected_command.contains("найди"){
                        voiceoutput::current_audio("поиск");
                    }
                    else if corrected_command.contains("открой"){
                        voiceoutput::current_audio("открываю");
                    }
                    command_sender.send(corrected_command).unwrap();

                    break;
                }
                std::thread::sleep(Duration::from_secs(3));
            }
            IS_PROCESSING.store(false, Ordering::Relaxed);
            println!("Риша не слушает")
        }

        std::thread::sleep(Duration::from_secs(3));
    }

    println!("Запись остановлена.");
}
