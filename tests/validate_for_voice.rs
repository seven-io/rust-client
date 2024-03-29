use testutil::*;
use seven_client::voice::{Voice, VoiceParams};
use seven_client::validate_for_voice::{ValidateForVoice, ValidateForVoiceParams};

mod testutil;

fn client() -> ValidateForVoice {
    ValidateForVoice::new(get_client())
}

fn params() -> ValidateForVoiceParams {
    ValidateForVoiceParams {
        callback: None,
        number: "+491716992343".to_string(),
    }
}

#[test]
fn post() {
    assert!(client().post(params()).is_ok());
}
