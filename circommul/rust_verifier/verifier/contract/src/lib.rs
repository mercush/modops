use electron_rs::verifier::near::{parse_verification_key, get_prepared_verifying_key, verify_proof};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Verifier {

}
impl Default for Verifier {
    fn default() -> Self {
        Self {

        }
    }
}

#[near_bindgen]
impl Verifier {
    pub fn verify(proof: String, verification: String, pub_inputs: String) -> bool{
        env::log_str("verifying");
        let parsed_key = parse_verification_key(verification.to_string()).unwrap();
        let prepared_key = get_prepared_verifying_key(parsed_key);
        verify_proof(prepared_key, proof.to_string(), pub_inputs.to_string()).unwrap()
    }
}