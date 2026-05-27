use irc::client::prelude::*;
use futures::prelude::*;
use tokio::sync::mpsc;




pub async fn irc_main(tx: mpsc::Sender<String>) {
        println!("Tentative de connexion au serveur IRC...");

    let mut client = match Client::new("config.toml").await {
        Ok(c) => c,
        Err(e) => { eprintln!("Erreur connexion : {:?}", e); return; }
    };

    if let Err(e) = client.identify() {
        eprintln!("Erreur lors de l'identification : {}", e);
        return;
    }

  
    let mut stream = match client.stream() {
        Ok(s) => s,
        Err(e) => { eprintln!("Erreur stream : {}", e); return; }
    };


  let sender = client.sender();

    while let Some(message_result) = stream.next().await {

       match message_result {

            Ok(message) => {
                match &message.command {
                    Command::PING(server, _) => {
                        let _ = sender.send_pong(server);
                    }
                    Command::PRIVMSG(channel, text) => {
                        // Récupère le pseudo de l'expéditeur
                        let user = message.source_nickname()
                            .unwrap_or("inconnu")
                            .to_string();

                        let msg = format!("{}: {}", user, text);
                        println!("{}", msg);

                        // Envoie à la GUI via le canal
                        let _ = tx.send(msg).await;
                    }
                    _ => {}
                }
            }



            Err(e) => { eprintln!("Erreur : {}", e); break; }
        }
    }

    println!("Déconnecté du serveur IRC.");




}

fn emote_url(id: &str) -> String {
    format!(
        "https://static-cdn.jtvnw.net/emoticons/v2/{}/default/dark/1.0",
        id
    )
}