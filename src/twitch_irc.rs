
use tokio::sync::mpsc;
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::ClientConfig;
use twitch_irc::SecureTCPTransport;
use twitch_irc::TwitchIRCClient;
use twitch_irc::message::ServerMessage;



pub async fn twitch_irc_main(tx: mpsc::Sender<String>,channelName: String) {
 

//On va juste se connecter au chat de Twitch en anonyme et recuperer les messages 
   let config = ClientConfig::default();
    let (mut incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);


let join_handle = tokio::spawn(async move {
    while let Some(message) = incoming_messages.recv().await {
        match message {
            ServerMessage::Privmsg(msg) => {

           for emote in &msg.emotes {
             let url = emote_url(&emote.id);
                println!("Emote: {} (id: {})  url: {}", emote.code, emote.id, url);
            }           
 


                let user = &msg.sender.name;
                let text = &msg.message_text;

        

             
                let _ = tx.send(format!("{}: {}",  user, text)).await;
            }   
            _ => {} 
        }
    }
});

client.join(channelName.to_owned()).unwrap();
join_handle.await.unwrap();
}

fn emote_url(id: &str) -> String {
    format!(
        "https://static-cdn.jtvnw.net/emoticons/v2/{}/default/dark/1.0",
        id
    )
}