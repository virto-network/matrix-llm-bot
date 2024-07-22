use ruma::events::macros::EventContent;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Deserialize, Serialize, Clone, Default, Debug)]
pub enum MediumOptions {
    #[default]
    Wallet,
}

#[derive(PartialEq, Deserialize, Serialize, Clone, Default, Debug)]
pub struct MemberItem {
    pub medium: MediumOptions,
    pub account: String,
}

pub type Members = Vec<MemberItem>;

#[derive(PartialEq, Deserialize, Serialize, Clone, Debug, Default)]
pub struct AddMembersAction {
    pub members: Members,
}

#[derive(PartialEq, Deserialize, Serialize, Clone, Debug, Default)]
pub struct RemoveMembersAction {
    pub members: Members,
}

#[derive(PartialEq, Deserialize, Serialize, Clone, Debug)]
#[serde(tag = "action_type")]
pub enum ActionItem {
    AddMembers(AddMembersAction),
    RemoveMembers(RemoveMembersAction),
}

#[derive(PartialEq, Clone, Debug, Deserialize, Serialize, EventContent, Default)]
#[ruma_event(type = "m.virto.initiative_init", kind = MessageLike)]
pub struct InitiativeInitContent {
    pub sender: String,
    pub is_admin: bool,
}

#[derive(PartialEq, Clone, Debug, Deserialize, Serialize, EventContent, Default)]
#[ruma_event(type = "m.virto.initiative_info", kind = MessageLike)]
pub struct InitiativeInfoContent {
    pub name: String,
    pub description: String,
    pub tags: Vec<String>,
    pub actions: Vec<ActionItem>,
}

#[derive(PartialEq, Deserialize, Serialize, Clone, Debug)]
pub enum VoteOf {
    Yes,
    No,
}

#[derive(PartialEq, Deserialize, Serialize, Clone, Debug)]
pub enum Vote {
    Standard(VoteOf),
}

#[derive(PartialEq, Clone, Debug, Deserialize, Serialize)]
pub struct InitiativeVoteData {
    pub user: String,
    pub room: String,
    pub vote: Vote,
}

#[derive(PartialEq, Clone, Debug, Deserialize, Serialize, EventContent)]
#[ruma_event(type = "m.virto.initiative_vote", kind = MessageLike)]
pub struct InitiativeVoteContent {
    pub user: String,
    pub vote: Vote,
}

#[derive(PartialEq, Deserialize, Serialize, Debug)]
pub struct InitiativeData {
    pub init: InitiativeInitContent,
    pub info: InitiativeInfoContent,
}

#[derive(PartialEq, Deserialize, Serialize, Debug, Default)]
pub struct InitiativeHistory {
    pub init: InitiativeInitContent,
    pub info: InitiativeInfoContent,
    pub votes: Vec<InitiativeVoteContent>,
}

#[derive(Serialize)]
pub struct InitiativeMatrixId {
    pub id: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetInitiative {
    pub room_id: String,
}
