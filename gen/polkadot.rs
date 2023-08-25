#[subxt::subxt(runtime_metadata_path = "polkadot.scale")]
pub mod polkadot {}
use polkadot::runtime_types;
use subxt::{OnlineClient, PolkadotConfig};
use subxt_signer::sr25519::dev;
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()> = ::subxt::utils::MultiAddress::Id(
        ::subxt::utils::AccountId32([8; 32usize]),
    );
    let value: ::core::primitive::u128 = 128;
    let payload = polkadot::tx().balances().transfer(dest, value);
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
