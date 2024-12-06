use ic_cdk::export::candid::CandidType;
use ic_cdk_macros::*;

#[derive(CandidType, Default)]
pub struct Time {
    pub hours: u8,
    pub minutes: u8,
    pub seconds: u8,
}

#[query]
fn get_time() -> Time {
    let now = ic_cdk::api::time();
    let secs = now / 1_000_000_000;
    let hours = (secs / 3600 % 24) as u8;
    let minutes = (secs / 60 % 60) as u8;
    let seconds = (secs % 60) as u8;

    Time { hours, minutes, seconds }
}