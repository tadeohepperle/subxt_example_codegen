#[subxt::subxt(runtime_metadata_path = "polkadot.scale")]
pub mod interface {}
use subxt::{OnlineClient, PolkadotConfig};
use subxt_signer::sr25519::dev;
pub fn main() {}
async fn wrapper() -> Result<(), Box<dyn std::error::Error>> {
    let remark: ::std::vec::Vec<::core::primitive::u8> = vec![8, 8];
    let payload = interface::tx().system().remark(remark);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    Ok(())
}
