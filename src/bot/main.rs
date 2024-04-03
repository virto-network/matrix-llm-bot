use std::sync::Arc;

use matrix_sdk::{
    config::SyncSettings, room::Room, ruma::{events::room::{member::StrippedRoomMemberEvent, message::{MessageType, OriginalSyncRoomMessageEvent, RoomMessageEventContent}}, user_id
          }, Client
};
use tokio::time::{sleep, Duration};
use dotenv::dotenv;

extern crate client_ollama;
use client_ollama::LLMClient;

// Async function that awaits for an invitation and accepts it automatically
async fn handle_room_invitation(
    room_member: StrippedRoomMemberEvent,
    client: Client,
    room: Room,
) {
    if room_member.state_key != client.user_id().unwrap() {
        return;
    }

    if let Room::Invited(room) = room {
        tokio::spawn(async move {
            println!("Autojoining room {}", room.room_id());
            let mut delay = 2;

            while let Err(err) = room.accept_invitation().await {
                // retry autojoin due to synapse sending invites, before the
                // invited user can join for more information see
                // https://github.com/matrix-org/synapse/issues/4345
                eprintln!("Failed to join room {} ({err:?}), retrying in {delay}s", room.room_id());

                sleep(Duration::from_secs(delay)).await;
                delay *= 2;

                if delay > 3600 {
                    eprintln!("Can't join room {} ({err:?})", room.room_id());
                    break;
                }
            }
            println!("Successfully joined room {}", room.room_id());
        });
    }
}

pub fn init_client(base_url: &str) -> LLMClient {
    LLMClient::new(base_url)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let password = std::env::var("PASSWORD").expect("PASSWORD must be set.");

    let bot_user = user_id!("@virto_bot:matrix.org");
    let client = Client::builder().server_name(bot_user.server_name()).build().await?;
    let llm_client = Arc::new(init_client("http://localhost:5000"));
 
    // First we need to log in.
    client.login_username(bot_user, &password).send().await?;
    // We add an event handler that listens if our user is invited to a room
    // This event_handler should be different: it has to listen every time a guest joins and invite it into a new room where you can talk with the bot.
    client.add_event_handler(handle_room_invitation);
    // We add an event handler that listens if our user is invited to a room
    client.add_event_handler(move |ev: OriginalSyncRoomMessageEvent, room: Room| {
        let llm_client = llm_client.clone();
        
        async move {
            if let Err(e) = on_room_message(ev, room, llm_client).await {
                eprintln!("Error processing message: {}", e);
            }
        }
    });
    
    //This event handler listens and prints every message it's received
     client.add_event_handler(|ev: OriginalSyncRoomMessageEvent| async move {
        if ev.sender != user_id!("@virto_bot:matrix.org") { 
            println!("Received a message {:?}", ev); 
        }   
     });

    // Syncing is important to synchronize the client state with the server.
    // This method will never return unless there is an error.
    client.sync(SyncSettings::default()).await?;

    Ok(())
}

// Async function that gets the text content of a room and answers. 
async fn on_room_message(event: OriginalSyncRoomMessageEvent, room: Room, llm_client: Arc<LLMClient>) -> Result<(), Box<dyn std::error::Error>> {
    let room = match room {
        Room::Joined(room) => room,
        _ => return Ok(()), // For now we ignore messages unrelated with rooms.
    };

    let text_content = match event.content.msgtype {
        MessageType::Text(text_content) => text_content, 
        _ => return Ok(()), // For now the bot only processes text messages.
    };

    // Ignore bot messages in order to avoid a infinite loop.
    let target_user_id = user_id!("@virto_bot:matrix.org");
    if event.sender == target_user_id {
        return Ok(());
    }

    // We check that the message we will process is the user message.
    let user_message = text_content.body.clone();
    println!("THis is the USER MESSAGE {}", user_message);
    let mut llm_response = String::new();

    // Call to the LLM client
    match llm_client.get_chat_completion("", &user_message).await {
        Ok(response) => llm_response = response,
        Err(err) => {
            eprintln!("Error procesando la solicitud a LLM: {}", err);
            llm_response = "Error al contactar al servicio de lenguaje".to_string();
        }
    }

    // Send response to matrix
    let content = RoomMessageEventContent::text_plain(&llm_response);
    room.send(content, None).await?; 

    Ok(())
}
