
use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AdecoderCheckPair {
  pub owner_id: String,
  pub encoded_phrase: String,
  pub should_be: String,
}