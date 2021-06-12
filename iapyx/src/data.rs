use chain_impl_mockchain::{certificate::VotePlanId, vote::Options};
use jormungandr_testing_utils::wallet::committee::encrypting_key_from_base32;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, convert::TryFrom, fmt, str};
pub use wallet_core::{Choice, Value};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Fund {
    pub id: i32,
    #[serde(alias = "fundName")]
    pub fund_name: String,
    #[serde(alias = "fundGoal")]
    pub fund_goal: String,
    pub voting_power_threshold: u32,
    #[serde(alias = "rewardsInfo")]
    pub rewards_info: String,
    #[serde(alias = "fundStartTime")]
    #[serde(serialize_with = "crate::utils::serde::serialize_unix_timestamp_as_rfc3339")]
    #[serde(deserialize_with = "crate::utils::serde::deserialize_unix_timestamp_from_rfc3339")]
    pub fund_start_time: i64,
    #[serde(alias = "fundEndTime")]
    #[serde(serialize_with = "crate::utils::serde::serialize_unix_timestamp_as_rfc3339")]
    #[serde(deserialize_with = "crate::utils::serde::deserialize_unix_timestamp_from_rfc3339")]
    pub fund_end_time: i64,
    #[serde(alias = "nextFundStartTime")]
    #[serde(serialize_with = "crate::utils::serde::serialize_unix_timestamp_as_rfc3339")]
    #[serde(deserialize_with = "crate::utils::serde::deserialize_unix_timestamp_from_rfc3339")]
    pub next_fund_start_time: i64,
    pub registration_snapshot_time: String,
    #[serde(alias = "chainVotePlans")]
    pub chain_vote_plans: Vec<Voteplan>,
    #[serde(alias = "chainVotePlans")]
    pub challenges: Vec<Challenge>,
    pub voting_power_info: String,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Voteplan {
    pub id: i32,
    #[serde(alias = "chainVoteplanId")]
    pub chain_voteplan_id: String,
    #[serde(alias = "chainVoteStartTime")]
    #[serde(serialize_with = "crate::utils::serde::serialize_unix_timestamp_as_rfc3339")]
    #[serde(deserialize_with = "crate::utils::serde::deserialize_unix_timestamp_from_rfc3339")]
    pub chain_vote_start_time: i64,
    #[serde(alias = "chainVoteEndTime")]
    #[serde(serialize_with = "crate::utils::serde::serialize_unix_timestamp_as_rfc3339")]
    #[serde(deserialize_with = "crate::utils::serde::deserialize_unix_timestamp_from_rfc3339")]
    pub chain_vote_end_time: i64,
    #[serde(alias = "chainCommitteeEnd")]
    #[serde(serialize_with = "crate::utils::serde::serialize_unix_timestamp_as_rfc3339")]
    #[serde(deserialize_with = "crate::utils::serde::deserialize_unix_timestamp_from_rfc3339")]
    pub chain_committee_end_time: i64,
    #[serde(alias = "chainVoteplanPayload")]
    pub chain_voteplan_payload: String,

    pub chain_vote_encryption_key: String,
    #[serde(alias = "fundId")]
    pub fund_id: i32,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]

pub struct Category {
    #[serde(alias = "categoryId")]
    pub category_id: String,
    #[serde(alias = "categoryName")]
    pub category_name: String,
    #[serde(alias = "categoryDescription")]
    pub category_description: String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct Proposer {
    #[serde(alias = "proposerName")]
    pub proposer_name: String,
    #[serde(alias = "proposerEmail")]
    pub proposer_email: String,
    #[serde(alias = "proposerUrl")]
    pub proposer_url: String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct Proposal {
    #[serde(alias = "internalId")]
    pub internal_id: i64,
    #[serde(alias = "proposalId")]
    pub proposal_id: String,
    //  #[serde(alias = "category")]
    pub proposal_category: Category,
    #[serde(alias = "proposalTitle")]
    pub proposal_title: String,
    #[serde(alias = "proposalSummary")]
    pub proposal_summary: String,
    #[serde(alias = "proposalProblem")]
    pub proposal_problem: Option<String>,
    #[serde(alias = "proposalSolution")]
    pub proposal_solution: Option<String>,
    #[serde(alias = "proposalPublicKey")]
    pub proposal_public_key: String,
    #[serde(alias = "proposalFunds")]
    pub proposal_funds: i64,
    #[serde(alias = "proposalUrl")]
    pub proposal_url: String,
    #[serde(alias = "proposalFilesUrl")]
    pub proposal_files_url: String,
    pub proposer: Proposer,
    #[serde(alias = "chainProposalId")]
    #[serde(serialize_with = "crate::utils::serde::serialize_bin_as_str")]
    #[serde(deserialize_with = "crate::utils::serde::deserialize_string_as_bytes")]
    pub chain_proposal_id: Vec<u8>,
    #[serde(alias = "chainProposalIndex")]
    pub chain_proposal_index: i64,
    #[serde(alias = "chainVoteOptions")]
    pub chain_vote_options: VoteOptions,
    #[serde(alias = "chainVoteplanId")]
    pub chain_voteplan_id: String,
    #[serde(alias = "chainVoteplanPayload")]
    pub chain_voteplan_payload: String,
    #[serde(alias = "chainVoteEncryptionKey")]
    pub chain_vote_encryption_key: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Challenge {
    pub id: i32,
    #[serde(alias = "challengeType")]
    pub challenge_type: ChallengeType,
    pub title: String,
    pub description: String,
    #[serde(alias = "rewardsTotal")]
    pub rewards_total: i64,
    pub proposers_rewards: i64,
    #[serde(alias = "fundId")]
    pub fund_id: i32,
    #[serde(alias = "challengeUrl")]
    pub challenge_url: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum ChallengeType {
    Simple,
    CommunityChoice,
}

impl Proposal {
    pub fn chain_proposal_id_as_str(&self) -> String {
        str::from_utf8(&self.chain_proposal_id).unwrap().to_string()
    }

    pub fn get_option_text(&self, choice: u8) -> String {
        self.chain_vote_options
            .0
            .iter()
            .find(|(_, y)| **y == choice)
            .map(|(x, _)| x.to_string())
            .unwrap()
    }
}

#[allow(clippy::from_over_into)]
impl Into<wallet_core::Proposal> for Proposal {
    fn into(self) -> wallet_core::Proposal {
        let chain_proposal_index = self.chain_proposal_index as u8;
        let bytes = hex::decode(self.chain_voteplan_id).unwrap();
        let mut vote_plan_id = [0; 32];
        let bytes = &bytes[..vote_plan_id.len()]; // panics if not enough data
        vote_plan_id.copy_from_slice(bytes);

        if self.chain_voteplan_payload == "public" {
            return wallet_core::Proposal::new_public(
                VotePlanId::try_from(vote_plan_id).unwrap(),
                chain_proposal_index,
                Options::new_length(self.chain_vote_options.0.len() as u8).unwrap(),
            );
        }
        wallet_core::Proposal::new_private(
            VotePlanId::try_from(vote_plan_id).unwrap(),
            chain_proposal_index,
            Options::new_length(self.chain_vote_options.0.len() as u8).unwrap(),
            encrypting_key_from_base32(&self.chain_vote_encryption_key).unwrap(),
        )
    }
}

#[derive(Serialize, Deserialize)]
pub struct ServiceVersion {
    pub service_version: String,
}

pub struct SimpleVoteStatus {
    pub chain_proposal_id: String,
    pub proposal_title: String,
    pub choice: String,
}

impl fmt::Display for SimpleVoteStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "# {}, '{}' -> Choice:  {}",
            self.chain_proposal_id, self.proposal_title, self.choice
        )
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct VoteOptions(pub VoteOptionsMap);
pub type VoteOptionsMap = HashMap<String, u8>;

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct VitVersion {
    service_version: String,
}

impl VitVersion {
    pub fn new(service_version: String) -> Self {
        Self { service_version }
    }

    pub fn version(&self) -> String {
        self.service_version.clone()
    }
}

impl Default for VitVersion {
    fn default() -> Self {
        Self {
            service_version: "2.0".to_string(),
        }
    }
}
