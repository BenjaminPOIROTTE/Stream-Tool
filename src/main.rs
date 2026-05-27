mod irc;
mod twitch_irc;
mod ui;


use eframe::egui::{self, Ui};
use tokio::runtime::Runtime;
use tokio::sync::mpsc;
use egui_extras;


fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "STREAM-TOOL",
        options,
        Box::new(|cc| {
    egui_extras::install_image_loaders(&cc.egui_ctx); 
    Ok(Box::new(MyApp::new()))
}),
    )
}

struct MyApp {
    token: String,
    channel: String,
    runtime: Runtime,          
    connected: bool,        
    messages: Vec<String>,
    rx: mpsc::Receiver<String>, 
    tx: mpsc::Sender<String>,
    twitch_connected: bool,
}


//constructeur de MyApp pour initialiser les champs et créer le runtime tokio
impl MyApp {
    fn new() -> Self {
        let (tx, rx) = mpsc::channel(100); // Crée un canal de communication 100 est la taille du buffer
        Self {
            token: String::new(),
            channel: String::new(),
            runtime: Runtime::new().expect("Impossible de créer le runtime tokio"),
            connected: false,
            messages: Vec::new(),
            rx,
            tx,
            twitch_connected: false,
        }
    }
}

//redirige default() vers new()
impl Default for MyApp {
    fn default() -> Self {
        Self::new()
    }
}

impl eframe::App for MyApp {
     fn ui(&mut self, ui: &mut Ui, _frame: &mut eframe::Frame) {
        crate::ui::render(self, ui);  
    }
}