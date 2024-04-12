use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Room {
    room_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct RoomInvite {
    pub room_id: String,
    pub user_id: String,
}
