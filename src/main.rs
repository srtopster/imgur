#![windows_subsystem = "windows"]
use curl::easy::{List,Easy,Form};
use std::{fs,env,process,thread,time,str};
use serde_json::Value;
use base64;
use clipboard::{ClipboardProvider,ClipboardContext};
use webbrowser;
use soloud::*;

fn play_sound(){
    let sl = Soloud::default().unwrap();
    let mut wav = audio::Wav::default();
    wav.load_mem(include_bytes!("upload.mp3")).unwrap();
    sl.play(&wav);
    while sl.voice_count() > 0 {
        thread::sleep(time::Duration::from_millis(100))
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1{
        println!("Uso: imgur_up IMAGEM.jpg");
        process::exit(1)
    }

    let client_id = "yourclientid";
    let b64file = base64::encode(fs::read(&args[1]).unwrap());

    let mut form = Form::new();
    form.part("image").contents(&b64file.as_bytes()).add().unwrap();
    form.part("type").contents("base64".as_bytes()).add().unwrap();

    let mut list  = List::new();
    list.append(&format!("Authorization: Client-ID {}",client_id)).unwrap();

    let mut response: Vec<u8> = Vec::new();
    let mut easy = Easy::new();
    easy.url("https://api.imgur.com/3/upload.json").unwrap();{
    easy.post(true).unwrap();
    easy.http_headers(list).unwrap();
    easy.httppost(form).unwrap();
    let mut transfer = easy.transfer();
    transfer.write_function(|data|{
        response.extend_from_slice(data);
        Ok(data.len())
    }).unwrap();
    transfer.perform().unwrap();
    }
    let v: Value = serde_json::from_str(str::from_utf8(&response).unwrap()).unwrap();
    let link = &v["data"]["link"].to_string().replace('"',"");

    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(link.to_string()).unwrap();
    thread::spawn(||play_sound());
    let notify = process::Command::new("notifu64.exe")
        .args(["/m",&format!("Link copiado: {}",link),"/p","Upload completo","/i","icon.ico","/d","30000"])
        .spawn()
        .expect("falha a executar notifu64.exe")
        .wait();
    if notify.unwrap().code().unwrap() == 3 {
        webbrowser::open(link).unwrap();
    }
}