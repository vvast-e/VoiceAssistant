use std::collections::HashMap;
use std::process::Command;


fn handle_open(target:&str){
    match target{
        "браузер"=>open_browser("https://www.yandex.ru"),
        "блокнот"=>open_application("notepad"),
        "калькулятор"=>open_application("calc"),
        _=>println!("Неизвестный объект для команды 'открой': {}", target),
    }
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
    let target=parts.next().unwrap_or("");

    let mut handlers:HashMap<&str, fn(&str)>=HashMap::new();
    handlers.insert("открой", handle_open as fn(&str));
    if let Some(handler) = handlers.get(action) {
        handler(target);
    } else {
        println!("Неизвестная команда: {}", action);
    }
}


