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

#[derive(PartialEq, Clone, Default, Deserialize, Serialize, Debug)]
pub struct KusamaTreasury {
    pub date: String,
    pub amount: u64,
}

pub type KusamaTreasuryPeriods = Vec<KusamaTreasury>;

#[derive(PartialEq, Clone, Debug, Deserialize, Serialize, Default)]
pub struct KusamaTreasuryAction {
    pub periods: KusamaTreasuryPeriods,
}

#[derive(PartialEq, Clone, Default, Deserialize, Serialize, Debug)]
pub enum ConvictionVote {
    #[default]
    None,
    Locked1x,
    Locked2x,
    Locked3x,
    Locked4x,
    Locked5x,
    Locked6x,
}

#[derive(PartialEq, Clone, Debug, Deserialize, Serialize, Default)]
pub struct StandardVote {
    pub aye: bool,
    pub conviction: ConvictionVote,
    pub balance: u64,
}

#[derive(PartialEq, Clone, Debug, Deserialize, Serialize, Default)]
pub struct SplitVote {
    pub aye: u64,
    pub nay: u64,
}

#[derive(PartialEq, Clone, Debug, Deserialize, Serialize, Default)]
pub struct SplitAbstainVote {
    pub aye: u64,
    pub nay: u64,
    pub abstain: u64,
}

#[derive(PartialEq, Clone, Deserialize, Serialize, Debug)]
pub enum VoteType {
    Standard(StandardVote),
    Split(SplitVote),
    SplitAbstain(SplitAbstainVote),
}

#[derive(PartialEq, Clone, Deserialize, Serialize, Debug)]
pub struct VotingOpenGov {
    pub poll_index: u64,
    pub vote: VoteType,
}

pub type VotingOpenGovActionProposals = Vec<VotingOpenGov>;

#[derive(PartialEq, Clone, Debug, Deserialize, Serialize, Default)]
pub struct VotingOpenGovAction {
    pub proposals: VotingOpenGovActionProposals,
}

#[derive(PartialEq, Clone, Default, Deserialize, Serialize, Debug)]
pub struct TransferItem {
    pub account: String,
    pub value: u64
}
pub type Transfers = Vec<TransferItem>;
#[derive(PartialEq, Clone, Debug, Deserialize, Serialize, Default)]
pub struct CommunityTransferAction {
    pub transfers: Transfers,
}

#[derive(PartialEq, Deserialize, Serialize, Clone, Debug)]
#[serde(tag = "action_type")]
pub enum ActionItem {
    AddMembers(AddMembersAction),
    RemoveMembers(RemoveMembersAction),
    KusamaTreasury(KusamaTreasuryAction),
    VotingOpenGov(VotingOpenGovAction),
    CommunityTransfer(CommunityTransferAction),
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
