mod irc;
use eframe::egui::Ui;
use tokio::runtime::Runtime;
use tokio::sync::mpsc;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "STREAM-TOOL",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::new()))),
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
        ui.heading("STREAM-TOOL");
        ui.separator();

        ui.horizontal(|ui| {
            ui.label("Token: ");
            ui.text_edit_singleline(&mut self.token);
        });
        ui.separator();

        ui.horizontal(|ui| {
            ui.label("Channel: ");
            ui.text_edit_singleline(&mut self.channel);
        });
        ui.separator();

        
        let btn_label = if self.connected { "Déconnecter" } else { "Connecter" };

        if ui.button(btn_label).clicked() {
            if !self.connected {
                // Lance irc_main() dans une tâche async séparée
                self.runtime.spawn(irc::irc_main());
                self.connected = true;
            }
        }

        if self.connected {
            ui.label("Connecté à Twitch IRC...");
        }

        ui.separator();




    }
}