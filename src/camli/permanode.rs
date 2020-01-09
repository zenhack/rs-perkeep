use serde::{Serialize, Deserialize};
use super::claim;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Permanode {
    random: String,
    claim_data: claim::Shared,
}
