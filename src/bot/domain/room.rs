use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct CommunityData {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub industry: String,
}

#[derive(Serialize)]
pub struct CommunityMatrixId {
    pub id: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetCommunity {
    pub room_id: String,
}

#[derive(Serialize)]
pub struct Uri {
    pub uri: String
}

