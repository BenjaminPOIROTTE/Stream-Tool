use irc::client::prelude::*;
use futures::prelude::*;





pub async fn irc_main() {
        println!("Tentative de connexion au serveur IRC...");

    let mut client = match Client::new("config.toml").await {
        Ok(client) => {
            println!("Connexion au serveur IRC réussie.");
            client
        }

        Err(e) => {
            eprintln!("Impossible de se connecter : {}", e);
            return;
        }
    };

    if let Err(e) = client.identify() {
        eprintln!("Erreur lors de l'identification : {}", e);
        return;
    }

    println!("Identifié sur le serveur.");

    let mut stream = match client.stream() {
        Ok(stream) => stream,

        Err(e) => {
            eprintln!("Impossible de créer le stream IRC : {}", e);
            return;
        }
    };

    while let Some(message_result) = stream.next().await {

       match message_result {

            Ok(message) => {
                            println!("MSG BRUT: {:?}", message.command);
                if let Command::PRIVMSG(channel, text) = &message.command {

                    println!("[{}] {}", channel, text);

                    if text.contains(client.current_nickname()) {
                        println!("On parle de moi !");
                    }
                }
            }

            Err(e) => {
                eprintln!("Erreur IRC : {}", e);
                break;
            }
        }
    }

    println!("Déconnecté du serveur IRC.");




}