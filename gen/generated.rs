#![recursion_limit = "256"]
use subxt::ext::scale_bits::bits;
use subxt::ext::scale_value::{value, Value};
use subxt::dynamic::DecodedValueThunk;
use subxt::{OnlineClient, PolkadotConfig};
use subxt_signer::sr25519::dev;
pub fn main() {}
async fn wrapper() -> Result<(), Box<dyn std::error::Error>> {
    let remark: Value = value! {
        (8, 8, 8)
    };
    let payload = subxt::tx::dynamic("System", "remark", vec![remark]);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let pages: Value = value! {
        64
    };
    let payload = subxt::tx::dynamic("System", "set_heap_pages", vec![pages]);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let code: Value = value! {
        (8, 8, 8)
    };
    let payload = subxt::tx::dynamic("System", "set_code", vec![code]);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let code: Value = value! {
        (8, 8, 8)
    };
    let payload = subxt::tx::dynamic("System", "set_code_without_checks", vec![code]);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let items: Value = value! {
        (((8, 8, 8), (8, 8, 8)), ((8, 8, 8), (8, 8, 8)), ((8, 8, 8), (8, 8, 8)))
    };
    let payload = subxt::tx::dynamic("System", "set_storage", vec![items]);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let keys: Value = value! {
        ((8, 8, 8), (8, 8, 8), (8, 8, 8))
    };
    let payload = subxt::tx::dynamic("System", "kill_storage", vec![keys]);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let prefix: Value = value! {
        (8, 8, 8)
    };
    let subkeys: Value = value! {
        32
    };
    let payload = subxt::tx::dynamic("System", "kill_prefix", vec![prefix, subkeys]);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let remark: Value = value! {
        (8, 8, 8)
    };
    let payload = subxt::tx::dynamic("System", "remark_with_event", vec![remark]);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let now: Value = value! {
        64
    };
    let payload = subxt::tx::dynamic("Timestamp", "set", vec![now]);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let dest: Value = value! {
        Id(((8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
        8, 8, 8, 8, 8, 8, 8)))
    };
    let value: Value = value! {
        128
    };
    let payload = subxt::tx::dynamic(
        "Balances",
        "transfer_allow_death",
        vec![dest, value],
    );
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let who: Value = value! {
        Id(((8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
        8, 8, 8, 8, 8, 8, 8)))
    };
    let new_free: Value = value! {
        128
    };
    let old_reserved: Value = value! {
        128
    };
    let payload = subxt::tx::dynamic(
        "Balances",
        "set_balance_deprecated",
        vec![who, new_free, old_reserved],
    );
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let source: Value = value! {
        Id(((8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
        8, 8, 8, 8, 8, 8, 8)))
    };
    let dest: Value = value! {
        Id(((8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
        8, 8, 8, 8, 8, 8, 8)))
    };
    let value: Value = value! {
        128
    };
    let payload = subxt::tx::dynamic(
        "Balances",
        "force_transfer",
        vec![source, dest, value],
    );
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let dest: Value = value! {
        Id(((8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
        8, 8, 8, 8, 8, 8, 8)))
    };
    let value: Value = value! {
        128
    };
    let payload = subxt::tx::dynamic(
        "Balances",
        "transfer_keep_alive",
        vec![dest, value],
    );
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let dest: Value = value! {
        Id(((8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
        8, 8, 8, 8, 8, 8, 8)))
    };
    let keep_alive: Value = value! {
        true
    };
    let payload = subxt::tx::dynamic("Balances", "transfer_all", vec![dest, keep_alive]);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let who: Value = value! {
        Id(((8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
        8, 8, 8, 8, 8, 8, 8)))
    };
    let amount: Value = value! {
        128
    };
    let payload = subxt::tx::dynamic("Balances", "force_unreserve", vec![who, amount]);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let who: Value = value! {
        (((8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
        8, 8, 8, 8, 8, 8)), ((8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
        8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8)), ((8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
        8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8)))
    };
    let payload = subxt::tx::dynamic("Balances", "upgrade_accounts", vec![who]);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let dest: Value = value! {
        Id(((8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
        8, 8, 8, 8, 8, 8, 8)))
    };
    let value: Value = value! {
        128
    };
    let payload = subxt::tx::dynamic("Balances", "transfer", vec![dest, value]);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let who: Value = value! {
        Id(((8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
        8, 8, 8, 8, 8, 8, 8)))
    };
    let new_free: Value = value! {
        128
    };
    let payload = subxt::tx::dynamic(
        "Balances",
        "force_set_balance",
        vec![who, new_free],
    );
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let value: Value = value! {
        128
    };
    let payee: Value = value! {
        Staked()
    };
    let payload = subxt::tx::dynamic("Staking", "bond", vec![value, payee]);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let max_additional: Value = value! {
        128
    };
    let payload = subxt::tx::dynamic("Staking", "bond_extra", vec![max_additional]);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let value: Value = value! {
        128
    };
    let payload = subxt::tx::dynamic("Staking", "unbond", vec![value]);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let num_slashing_spans: Value = value! {
        32
    };
    let payload = subxt::tx::dynamic(
        "Staking",
        "withdraw_unbonded",
        vec![num_slashing_spans],
    );
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let prefs: Value = value! {
        { commission : (32), blocked : true }
    };
    let payload = subxt::tx::dynamic("Staking", "validate", vec![prefs]);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let targets: Value = value! {
        (Id(((8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
        8, 8, 8, 8, 8, 8, 8))), Id(((8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
        8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8))), Id(((8, 8, 8, 8, 8, 8, 8, 8, 8,
        8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8))))
    };
    let payload = subxt::tx::dynamic("Staking", "nominate", vec![targets]);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let payload = subxt::tx::dynamic("Staking", "chill", Vec::<Value>::new());
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let payee: Value = value! {
        Staked()
    };
    let payload = subxt::tx::dynamic("Staking", "set_payee", vec![payee]);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let payload = subxt::tx::dynamic("Staking", "set_controller", Vec::<Value>::new());
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let new: Value = value! {
        32
    };
    let payload = subxt::tx::dynamic("Staking", "set_validator_count", vec![new]);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let additional: Value = value! {
        32
    };
    let payload = subxt::tx::dynamic(
        "Staking",
        "increase_validator_count",
        vec![additional],
    );
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let factor: Value = value! {
        (8)
    };
    let payload = subxt::tx::dynamic("Staking", "scale_validator_count", vec![factor]);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let payload = subxt::tx::dynamic("Staking", "force_no_eras", Vec::<Value>::new());
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let payload = subxt::tx::dynamic("Staking", "force_new_era", Vec::<Value>::new());
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let invulnerables: Value = value! {
        (((8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
        8, 8, 8, 8, 8, 8)), ((8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
        8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8)), ((8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
        8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8)))
    };
    let payload = subxt::tx::dynamic(
        "Staking",
        "set_invulnerables",
        vec![invulnerables],
    );
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let stash: Value = value! {
        ((8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
        8, 8, 8, 8, 8, 8))
    };
    let num_slashing_spans: Value = value! {
        32
    };
    let payload = subxt::tx::dynamic(
        "Staking",
        "force_unstake",
        vec![stash, num_slashing_spans],
    );
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let payload = subxt::tx::dynamic(
        "Staking",
        "force_new_era_always",
        Vec::<Value>::new(),
    );
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let era: Value = value! {
        32
    };
    let slash_indices: Value = value! {
        (32, 32, 32)
    };
    let payload = subxt::tx::dynamic(
        "Staking",
        "cancel_deferred_slash",
        vec![era, slash_indices],
    );
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let validator_stash: Value = value! {
        ((8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
        8, 8, 8, 8, 8, 8))
    };
    let era: Value = value! {
        32
    };
    let payload = subxt::tx::dynamic(
        "Staking",
        "payout_stakers",
        vec![validator_stash, era],
    );
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let value: Value = value! {
        128
    };
    let payload = subxt::tx::dynamic("Staking", "rebond", vec![value]);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let stash: Value = value! {
        ((8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
        8, 8, 8, 8, 8, 8))
    };
    let num_slashing_spans: Value = value! {
        32
    };
    let payload = subxt::tx::dynamic(
        "Staking",
        "reap_stash",
        vec![stash, num_slashing_spans],
    );
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let who: Value = value! {
        (Id(((8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
        8, 8, 8, 8, 8, 8, 8))), Id(((8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
        8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8))), Id(((8, 8, 8, 8, 8, 8, 8, 8, 8,
        8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8))))
    };
    let payload = subxt::tx::dynamic("Staking", "kick", vec![who]);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let min_nominator_bond: Value = value! {
        Noop()
    };
    let min_validator_bond: Value = value! {
        Noop()
    };
    let max_nominator_count: Value = value! {
        Noop()
    };
    let max_validator_count: Value = value! {
        Noop()
    };
    let chill_threshold: Value = value! {
        Noop()
    };
    let min_commission: Value = value! {
        Noop()
    };
    let payload = subxt::tx::dynamic(
        "Staking",
        "set_staking_configs",
        vec![
            min_nominator_bond, min_validator_bond, max_nominator_count,
            max_validator_count, chill_threshold, min_commission
        ],
    );
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let controller: Value = value! {
        ((8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
        8, 8, 8, 8, 8, 8))
    };
    let payload = subxt::tx::dynamic("Staking", "chill_other", vec![controller]);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let validator_stash: Value = value! {
        ((8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
        8, 8, 8, 8, 8, 8))
    };
    let payload = subxt::tx::dynamic(
        "Staking",
        "force_apply_min_commission",
        vec![validator_stash],
    );
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let new: Value = value! {
        (32)
    };
    let payload = subxt::tx::dynamic("Staking", "set_min_commission", vec![new]);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let other_signatories: Value = value! {
        (((8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
        8, 8, 8, 8, 8, 8)), ((8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
        8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8)), ((8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
        8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8)))
    };
    let call: Value = value! {
        true
    };
    let payload = subxt::tx::dynamic(
        "Multisig",
        "as_multi_threshold_1",
        vec![other_signatories, call],
    );
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let threshold: Value = value! {
        16
    };
    let other_signatories: Value = value! {
        (((8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
        8, 8, 8, 8, 8, 8)), ((8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
        8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8)), ((8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
        8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8)))
    };
    let maybe_timepoint: Value = value! {
        None()
    };
    let call: Value = value! {
        true
    };
    let max_weight: Value = value! {
        { ref_time : 64, proof_size : 64 }
    };
    let payload = subxt::tx::dynamic(
        "Multisig",
        "as_multi",
        vec![threshold, other_signatories, maybe_timepoint, call, max_weight],
    );
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let threshold: Value = value! {
        16
    };
    let other_signatories: Value = value! {
        (((8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
        8, 8, 8, 8, 8, 8)), ((8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
        8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8)), ((8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
        8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8)))
    };
    let maybe_timepoint: Value = value! {
        None()
    };
    let call_hash: Value = value! {
        (8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
        8, 8, 8, 8, 8)
    };
    let max_weight: Value = value! {
        { ref_time : 64, proof_size : 64 }
    };
    let payload = subxt::tx::dynamic(
        "Multisig",
        "approve_as_multi",
        vec![threshold, other_signatories, maybe_timepoint, call_hash, max_weight],
    );
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let threshold: Value = value! {
        16
    };
    let other_signatories: Value = value! {
        (((8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
        8, 8, 8, 8, 8, 8)), ((8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
        8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8)), ((8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
        8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8)))
    };
    let timepoint: Value = value! {
        { height : 32, index : 32 }
    };
    let call_hash: Value = value! {
        (8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
        8, 8, 8, 8, 8)
    };
    let payload = subxt::tx::dynamic(
        "Multisig",
        "cancel_as_multi",
        vec![threshold, other_signatories, timepoint, call_hash],
    );
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let data: Value = todo!(
        "Value string is very long and will likely exceed recursion limit."
    );
    let payload = subxt::tx::dynamic("ParaInherent", "enter", vec![data]);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let key_0: Value = value! {
        ((8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
        8, 8, 8, 8, 8, 8))
    };
    let storage_query = subxt::storage::dynamic("System", "Account", vec![key_0]);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<DecodedValueThunk> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = subxt::storage::dynamic(
        "System",
        "ExtrinsicCount",
        Vec::<Value>::new(),
    );
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<DecodedValueThunk> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = subxt::storage::dynamic(
        "System",
        "BlockWeight",
        Vec::<Value>::new(),
    );
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<DecodedValueThunk> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = subxt::storage::dynamic(
        "System",
        "AllExtrinsicsLen",
        Vec::<Value>::new(),
    );
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<DecodedValueThunk> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: Value = value! {
        32
    };
    let storage_query = subxt::storage::dynamic("System", "BlockHash", vec![key_0]);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<DecodedValueThunk> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: Value = value! {
        32
    };
    let storage_query = subxt::storage::dynamic("System", "ExtrinsicData", vec![key_0]);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<DecodedValueThunk> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = subxt::storage::dynamic("System", "Number", Vec::<Value>::new());
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<DecodedValueThunk> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = subxt::storage::dynamic(
        "System",
        "ParentHash",
        Vec::<Value>::new(),
    );
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<DecodedValueThunk> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = subxt::storage::dynamic("System", "Digest", Vec::<Value>::new());
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<DecodedValueThunk> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = subxt::storage::dynamic("System", "Events", Vec::<Value>::new());
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<DecodedValueThunk> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = subxt::storage::dynamic(
        "System",
        "EventCount",
        Vec::<Value>::new(),
    );
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<DecodedValueThunk> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: Value = value! {
        ((8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
        8, 8, 8, 8, 8, 8))
    };
    let storage_query = subxt::storage::dynamic("System", "EventTopics", vec![key_0]);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<DecodedValueThunk> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = subxt::storage::dynamic(
        "System",
        "LastRuntimeUpgrade",
        Vec::<Value>::new(),
    );
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<DecodedValueThunk> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = subxt::storage::dynamic(
        "System",
        "UpgradedToU32RefCount",
        Vec::<Value>::new(),
    );
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<DecodedValueThunk> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = subxt::storage::dynamic(
        "System",
        "UpgradedToTripleRefCount",
        Vec::<Value>::new(),
    );
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<DecodedValueThunk> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = subxt::storage::dynamic(
        "System",
        "ExecutionPhase",
        Vec::<Value>::new(),
    );
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<DecodedValueThunk> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = subxt::storage::dynamic("Timestamp", "Now", Vec::<Value>::new());
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<DecodedValueThunk> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = subxt::storage::dynamic(
        "Timestamp",
        "DidUpdate",
        Vec::<Value>::new(),
    );
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<DecodedValueThunk> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = subxt::storage::dynamic(
        "Balances",
        "TotalIssuance",
        Vec::<Value>::new(),
    );
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<DecodedValueThunk> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = subxt::storage::dynamic(
        "Balances",
        "InactiveIssuance",
        Vec::<Value>::new(),
    );
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<DecodedValueThunk> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: Value = value! {
        ((8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
        8, 8, 8, 8, 8, 8))
    };
    let storage_query = subxt::storage::dynamic("Balances", "Account", vec![key_0]);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<DecodedValueThunk> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: Value = value! {
        ((8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
        8, 8, 8, 8, 8, 8))
    };
    let storage_query = subxt::storage::dynamic("Balances", "Locks", vec![key_0]);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<DecodedValueThunk> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: Value = value! {
        ((8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
        8, 8, 8, 8, 8, 8))
    };
    let storage_query = subxt::storage::dynamic("Balances", "Reserves", vec![key_0]);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<DecodedValueThunk> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: Value = value! {
        ((8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
        8, 8, 8, 8, 8, 8))
    };
    let storage_query = subxt::storage::dynamic("Balances", "Holds", vec![key_0]);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<DecodedValueThunk> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: Value = value! {
        ((8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
        8, 8, 8, 8, 8, 8))
    };
    let storage_query = subxt::storage::dynamic("Balances", "Freezes", vec![key_0]);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<DecodedValueThunk> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = subxt::storage::dynamic(
        "Staking",
        "ValidatorCount",
        Vec::<Value>::new(),
    );
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<DecodedValueThunk> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = subxt::storage::dynamic(
        "Staking",
        "MinimumValidatorCount",
        Vec::<Value>::new(),
    );
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<DecodedValueThunk> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = subxt::storage::dynamic(
        "Staking",
        "Invulnerables",
        Vec::<Value>::new(),
    );
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<DecodedValueThunk> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: Value = value! {
        ((8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
        8, 8, 8, 8, 8, 8))
    };
    let storage_query = subxt::storage::dynamic("Staking", "Bonded", vec![key_0]);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<DecodedValueThunk> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = subxt::storage::dynamic(
        "Staking",
        "MinNominatorBond",
        Vec::<Value>::new(),
    );
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<DecodedValueThunk> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = subxt::storage::dynamic(
        "Staking",
        "MinValidatorBond",
        Vec::<Value>::new(),
    );
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<DecodedValueThunk> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = subxt::storage::dynamic(
        "Staking",
        "MinimumActiveStake",
        Vec::<Value>::new(),
    );
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<DecodedValueThunk> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = subxt::storage::dynamic(
        "Staking",
        "MinCommission",
        Vec::<Value>::new(),
    );
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<DecodedValueThunk> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: Value = value! {
        ((8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
        8, 8, 8, 8, 8, 8))
    };
    let storage_query = subxt::storage::dynamic("Staking", "Ledger", vec![key_0]);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<DecodedValueThunk> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: Value = value! {
        ((8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
        8, 8, 8, 8, 8, 8))
    };
    let storage_query = subxt::storage::dynamic("Staking", "Payee", vec![key_0]);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<DecodedValueThunk> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: Value = value! {
        ((8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
        8, 8, 8, 8, 8, 8))
    };
    let storage_query = subxt::storage::dynamic("Staking", "Validators", vec![key_0]);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<DecodedValueThunk> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = subxt::storage::dynamic(
        "Staking",
        "CounterForValidators",
        Vec::<Value>::new(),
    );
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<DecodedValueThunk> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = subxt::storage::dynamic(
        "Staking",
        "MaxValidatorsCount",
        Vec::<Value>::new(),
    );
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<DecodedValueThunk> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: Value = value! {
        ((8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
        8, 8, 8, 8, 8, 8))
    };
    let storage_query = subxt::storage::dynamic("Staking", "Nominators", vec![key_0]);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<DecodedValueThunk> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = subxt::storage::dynamic(
        "Staking",
        "CounterForNominators",
        Vec::<Value>::new(),
    );
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<DecodedValueThunk> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = subxt::storage::dynamic(
        "Staking",
        "MaxNominatorsCount",
        Vec::<Value>::new(),
    );
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<DecodedValueThunk> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = subxt::storage::dynamic(
        "Staking",
        "CurrentEra",
        Vec::<Value>::new(),
    );
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<DecodedValueThunk> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = subxt::storage::dynamic(
        "Staking",
        "ActiveEra",
        Vec::<Value>::new(),
    );
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<DecodedValueThunk> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: Value = value! {
        32
    };
    let storage_query = subxt::storage::dynamic(
        "Staking",
        "ErasStartSessionIndex",
        vec![key_0],
    );
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<DecodedValueThunk> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: Value = value! {
        32
    };
    let key_1: Value = value! {
        ((8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
        8, 8, 8, 8, 8, 8))
    };
    let storage_query = subxt::storage::dynamic(
        "Staking",
        "ErasStakers",
        vec![key_0, key_1],
    );
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<DecodedValueThunk> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: Value = value! {
        32
    };
    let key_1: Value = value! {
        ((8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
        8, 8, 8, 8, 8, 8))
    };
    let storage_query = subxt::storage::dynamic(
        "Staking",
        "ErasStakersClipped",
        vec![key_0, key_1],
    );
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<DecodedValueThunk> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: Value = value! {
        32
    };
    let key_1: Value = value! {
        ((8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
        8, 8, 8, 8, 8, 8))
    };
    let storage_query = subxt::storage::dynamic(
        "Staking",
        "ErasValidatorPrefs",
        vec![key_0, key_1],
    );
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<DecodedValueThunk> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: Value = value! {
        32
    };
    let storage_query = subxt::storage::dynamic(
        "Staking",
        "ErasValidatorReward",
        vec![key_0],
    );
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<DecodedValueThunk> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: Value = value! {
        32
    };
    let storage_query = subxt::storage::dynamic(
        "Staking",
        "ErasRewardPoints",
        vec![key_0],
    );
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<DecodedValueThunk> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: Value = value! {
        32
    };
    let storage_query = subxt::storage::dynamic(
        "Staking",
        "ErasTotalStake",
        vec![key_0],
    );
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<DecodedValueThunk> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = subxt::storage::dynamic(
        "Staking",
        "ForceEra",
        Vec::<Value>::new(),
    );
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<DecodedValueThunk> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = subxt::storage::dynamic(
        "Staking",
        "SlashRewardFraction",
        Vec::<Value>::new(),
    );
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<DecodedValueThunk> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = subxt::storage::dynamic(
        "Staking",
        "CanceledSlashPayout",
        Vec::<Value>::new(),
    );
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<DecodedValueThunk> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: Value = value! {
        32
    };
    let storage_query = subxt::storage::dynamic(
        "Staking",
        "UnappliedSlashes",
        vec![key_0],
    );
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<DecodedValueThunk> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = subxt::storage::dynamic(
        "Staking",
        "BondedEras",
        Vec::<Value>::new(),
    );
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<DecodedValueThunk> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: Value = value! {
        32
    };
    let key_1: Value = value! {
        ((8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
        8, 8, 8, 8, 8, 8))
    };
    let storage_query = subxt::storage::dynamic(
        "Staking",
        "ValidatorSlashInEra",
        vec![key_0, key_1],
    );
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<DecodedValueThunk> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: Value = value! {
        32
    };
    let key_1: Value = value! {
        ((8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
        8, 8, 8, 8, 8, 8))
    };
    let storage_query = subxt::storage::dynamic(
        "Staking",
        "NominatorSlashInEra",
        vec![key_0, key_1],
    );
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<DecodedValueThunk> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: Value = value! {
        ((8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
        8, 8, 8, 8, 8, 8))
    };
    let storage_query = subxt::storage::dynamic("Staking", "SlashingSpans", vec![key_0]);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<DecodedValueThunk> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: Value = value! {
        ((8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
        8, 8, 8, 8, 8, 8))
    };
    let key_1: Value = value! {
        32
    };
    let storage_query = subxt::storage::dynamic(
        "Staking",
        "SpanSlash",
        vec![key_0, key_1],
    );
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<DecodedValueThunk> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = subxt::storage::dynamic(
        "Staking",
        "CurrentPlannedSession",
        Vec::<Value>::new(),
    );
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<DecodedValueThunk> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = subxt::storage::dynamic(
        "Staking",
        "OffendingValidators",
        Vec::<Value>::new(),
    );
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<DecodedValueThunk> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = subxt::storage::dynamic(
        "Staking",
        "ChillThreshold",
        Vec::<Value>::new(),
    );
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<DecodedValueThunk> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: Value = value! {
        ((8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
        8, 8, 8, 8, 8, 8))
    };
    let key_1: Value = value! {
        (8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8,
        8, 8, 8, 8, 8)
    };
    let storage_query = subxt::storage::dynamic(
        "Multisig",
        "Multisigs",
        vec![key_0, key_1],
    );
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<DecodedValueThunk> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = subxt::storage::dynamic(
        "ParaInherent",
        "Included",
        Vec::<Value>::new(),
    );
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<DecodedValueThunk> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = subxt::storage::dynamic(
        "ParaInherent",
        "OnChainVotes",
        Vec::<Value>::new(),
    );
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<DecodedValueThunk> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let constant_query = subxt::constants::dynamic("System", "BlockWeights");
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: DecodedValueThunk = api.constants().at(&constant_query)?;
    let constant_query = subxt::constants::dynamic("System", "BlockLength");
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: DecodedValueThunk = api.constants().at(&constant_query)?;
    let constant_query = subxt::constants::dynamic("System", "BlockHashCount");
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: DecodedValueThunk = api.constants().at(&constant_query)?;
    let constant_query = subxt::constants::dynamic("System", "DbWeight");
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: DecodedValueThunk = api.constants().at(&constant_query)?;
    let constant_query = subxt::constants::dynamic("System", "Version");
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: DecodedValueThunk = api.constants().at(&constant_query)?;
    let constant_query = subxt::constants::dynamic("System", "SS58Prefix");
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: DecodedValueThunk = api.constants().at(&constant_query)?;
    let constant_query = subxt::constants::dynamic("Timestamp", "MinimumPeriod");
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: DecodedValueThunk = api.constants().at(&constant_query)?;
    let constant_query = subxt::constants::dynamic("Balances", "ExistentialDeposit");
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: DecodedValueThunk = api.constants().at(&constant_query)?;
    let constant_query = subxt::constants::dynamic("Balances", "MaxLocks");
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: DecodedValueThunk = api.constants().at(&constant_query)?;
    let constant_query = subxt::constants::dynamic("Balances", "MaxReserves");
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: DecodedValueThunk = api.constants().at(&constant_query)?;
    let constant_query = subxt::constants::dynamic("Balances", "MaxHolds");
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: DecodedValueThunk = api.constants().at(&constant_query)?;
    let constant_query = subxt::constants::dynamic("Balances", "MaxFreezes");
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: DecodedValueThunk = api.constants().at(&constant_query)?;
    let constant_query = subxt::constants::dynamic("Staking", "MaxNominations");
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: DecodedValueThunk = api.constants().at(&constant_query)?;
    let constant_query = subxt::constants::dynamic("Staking", "HistoryDepth");
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: DecodedValueThunk = api.constants().at(&constant_query)?;
    let constant_query = subxt::constants::dynamic("Staking", "SessionsPerEra");
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: DecodedValueThunk = api.constants().at(&constant_query)?;
    let constant_query = subxt::constants::dynamic("Staking", "BondingDuration");
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: DecodedValueThunk = api.constants().at(&constant_query)?;
    let constant_query = subxt::constants::dynamic("Staking", "SlashDeferDuration");
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: DecodedValueThunk = api.constants().at(&constant_query)?;
    let constant_query = subxt::constants::dynamic(
        "Staking",
        "MaxNominatorRewardedPerValidator",
    );
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: DecodedValueThunk = api.constants().at(&constant_query)?;
    let constant_query = subxt::constants::dynamic("Staking", "MaxUnlockingChunks");
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: DecodedValueThunk = api.constants().at(&constant_query)?;
    let constant_query = subxt::constants::dynamic("Multisig", "DepositBase");
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: DecodedValueThunk = api.constants().at(&constant_query)?;
    let constant_query = subxt::constants::dynamic("Multisig", "DepositFactor");
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: DecodedValueThunk = api.constants().at(&constant_query)?;
    let constant_query = subxt::constants::dynamic("Multisig", "MaxSignatories");
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: DecodedValueThunk = api.constants().at(&constant_query)?;
    Ok(())
}
