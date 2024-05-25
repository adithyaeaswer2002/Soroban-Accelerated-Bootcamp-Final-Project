use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use serde_wasm_bindgen::from_value;
use serde_wasm_bindgen::to_value;

#[derive(Serialize, Deserialize)]
pub struct Campaign {
    target_amount: u64,
    collected_amount: u64,
    creator_public_key: String,
    donors: Vec<Donation>,
}

#[derive(Serialize, Deserialize)]
pub struct Donation {
    donor_public_key: String,
    amount: u64,
}

#[wasm_bindgen]
pub fn create_campaign(target_amount: u64, creator_public_key: String) -> JsValue {
    let campaign = Campaign {
        target_amount,
        collected_amount: 0,
        creator_public_key,
        donors: vec![],
    };
    to_value(&campaign).unwrap()
}

#[wasm_bindgen]
pub fn donate(campaign: JsValue, donor_public_key: String, amount: u64) -> JsValue {
    let mut campaign: Campaign = from_value(campaign).unwrap();
    campaign.collected_amount += amount;
    campaign.donors.push(Donation {
        donor_public_key,
        amount,
    });
    to_value(&campaign).unwrap()
}
