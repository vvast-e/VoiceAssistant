use std::collections::HashMap;
use std::process::Command;
use crate::math;

fn handle_shutdown(target:&str){
    #[cfg(windows)]
    {
        Command::new("shutdown")
            .args(&["/s", "/t", "0"])
            .spawn()
            .expect("Не удалось завершить работу компьютера");
    }
}
fn sleep_computer(target: &str) {
    #[cfg(windows)]
    {
        Command::new("rundll32.exe")
            .args(&["powrprof.dll,SetSuspendState", "0,1,0"])
            .spawn()
            .expect("Не удалось перевести компьютер в режим сна");
    }
}
fn handle_open(target:&str){
    match target{
        "яндекс"=>open_browser("https://www.yandex.ru"),
        "блокнот"=>open_application("notepad"),
        "калькулятор"=>open_application("calc"),
        "ютуб"=>open_browser("https://www.youtube.com"),
        "телеграмм"=>open_application("C:/Users/User/AppData/Roaming/Telegram Desktop/Telegram.exe"),
        "музыку"=>open_browser("https://vk.com/audios539944585?block=my_playlists"),
        _=>println!("Неизвестный объект для команды 'открой': {}", target),
    }
}

fn handle_find(target:&str){
    let full_url=format!("https://yandex.ru/search/?text={}", target);
    #[cfg(windows)]
    {
        Command::new("cmd").args(&["/C","start",full_url.as_str()]).spawn().expect("Could not open browser");
    }
}
fn handle_math(target:&str){
    let result=math::math(target);
    println!("будет {}",result);
}
fn open_browser(url:&str){
    #[cfg(windows)]
    {
        Command::new("cmd").args(&["/C","start", url]).spawn().expect("Could not open browser");
    }
}
fn open_application(target:&str){
    Command::new(target).spawn().expect("Could not open application");
}

pub fn handle_command(command:&str){
    let mut parts=command.split_whitespace();
    let action=parts.next().unwrap_or("");
    let target=if action=="найди"{
        parts.collect::<Vec<_>>().join("+")
    }
    else if action == "посчитай" {
        parts.collect::<Vec<_>>().join(" ")
    }
    else {
        parts.next().unwrap_or("").parse().unwrap()
    };

    let mut handlers:HashMap<&str, fn(&str)>=HashMap::new();
    handlers.insert("открой", handle_open as fn(&str));
    handlers.insert("найди", handle_find as fn(&str));
    handlers.insert("посчитай", handle_math as fn(&str));
    handlers.insert("заверши", handle_shutdown as fn(&str));
    handlers.insert("режим", sleep_computer as fn(&str));
    if let Some(handler) = handlers.get(action) {
        handler(&*target);
    } else {
        println!("Неизвестная команда: {}", action);
    }
}


