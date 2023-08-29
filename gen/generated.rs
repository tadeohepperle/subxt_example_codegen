#[subxt::subxt(runtime_metadata_path = "polkadot.scale")]
pub mod runtime {}
use runtime::runtime_types;
use subxt::{OnlineClient, PolkadotConfig};
use subxt_signer::sr25519::dev;
pub fn main() {}
async fn wrapper() -> Result<(), Box<dyn std::error::Error>> {
    let remark: ::std::vec::Vec<::core::primitive::u8> = vec![8, 8];
    let payload = runtime::tx().system().remark(remark);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let pages: ::core::primitive::u64 = 64;
    let payload = runtime::tx().system().set_heap_pages(pages);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let code: ::std::vec::Vec<::core::primitive::u8> = vec![8, 8];
    let payload = runtime::tx().system().set_code(code);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let code: ::std::vec::Vec<::core::primitive::u8> = vec![8, 8];
    let payload = runtime::tx().system().set_code_without_checks(code);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let items: ::std::vec::Vec<
        (::std::vec::Vec<::core::primitive::u8>, ::std::vec::Vec<::core::primitive::u8>),
    > = vec![(vec![8, 8], vec![8, 8]), (vec![8, 8], vec![8, 8])];
    let payload = runtime::tx().system().set_storage(items);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>> = vec![
        vec![8, 8], vec![8, 8]
    ];
    let payload = runtime::tx().system().kill_storage(keys);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let prefix: ::std::vec::Vec<::core::primitive::u8> = vec![8, 8];
    let subkeys: ::core::primitive::u32 = 32;
    let payload = runtime::tx().system().kill_prefix(prefix, subkeys);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let remark: ::std::vec::Vec<::core::primitive::u8> = vec![8, 8];
    let payload = runtime::tx().system().remark_with_event(remark);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let now: ::core::primitive::u64 = 64;
    let payload = runtime::tx().timestamp().set(now);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()> = ::subxt::utils::MultiAddress::Id(
        ::subxt::utils::AccountId32([8; 32usize]),
    );
    let value: ::core::primitive::u128 = 128;
    let payload = runtime::tx().balances().transfer_allow_death(dest, value);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()> = ::subxt::utils::MultiAddress::Id(
        ::subxt::utils::AccountId32([8; 32usize]),
    );
    let new_free: ::core::primitive::u128 = 128;
    let old_reserved: ::core::primitive::u128 = 128;
    let payload = runtime::tx()
        .balances()
        .set_balance_deprecated(who, new_free, old_reserved);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let source: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()> = ::subxt::utils::MultiAddress::Id(
        ::subxt::utils::AccountId32([8; 32usize]),
    );
    let dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()> = ::subxt::utils::MultiAddress::Id(
        ::subxt::utils::AccountId32([8; 32usize]),
    );
    let value: ::core::primitive::u128 = 128;
    let payload = runtime::tx().balances().force_transfer(source, dest, value);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()> = ::subxt::utils::MultiAddress::Id(
        ::subxt::utils::AccountId32([8; 32usize]),
    );
    let value: ::core::primitive::u128 = 128;
    let payload = runtime::tx().balances().transfer_keep_alive(dest, value);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()> = ::subxt::utils::MultiAddress::Id(
        ::subxt::utils::AccountId32([8; 32usize]),
    );
    let keep_alive: ::core::primitive::bool = false;
    let payload = runtime::tx().balances().transfer_all(dest, keep_alive);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()> = ::subxt::utils::MultiAddress::Id(
        ::subxt::utils::AccountId32([8; 32usize]),
    );
    let amount: ::core::primitive::u128 = 128;
    let payload = runtime::tx().balances().force_unreserve(who, amount);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let who: ::std::vec::Vec<::subxt::utils::AccountId32> = vec![
        ::subxt::utils::AccountId32([8; 32usize],), ::subxt::utils::AccountId32([8;
        32usize],)
    ];
    let payload = runtime::tx().balances().upgrade_accounts(who);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()> = ::subxt::utils::MultiAddress::Id(
        ::subxt::utils::AccountId32([8; 32usize]),
    );
    let value: ::core::primitive::u128 = 128;
    let payload = runtime::tx().balances().transfer(dest, value);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()> = ::subxt::utils::MultiAddress::Id(
        ::subxt::utils::AccountId32([8; 32usize]),
    );
    let new_free: ::core::primitive::u128 = 128;
    let payload = runtime::tx().balances().force_set_balance(who, new_free);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let value: ::core::primitive::u128 = 128;
    let payee: runtime_types::pallet_staking::RewardDestination<
        ::subxt::utils::AccountId32,
    > = runtime_types::pallet_staking::RewardDestination::Staked;
    let payload = runtime::tx().staking().bond(value, payee);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let max_additional: ::core::primitive::u128 = 128;
    let payload = runtime::tx().staking().bond_extra(max_additional);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let value: ::core::primitive::u128 = 128;
    let payload = runtime::tx().staking().unbond(value);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let num_slashing_spans: ::core::primitive::u32 = 32;
    let payload = runtime::tx().staking().withdraw_unbonded(num_slashing_spans);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let prefs: runtime_types::pallet_staking::ValidatorPrefs = runtime_types::pallet_staking::ValidatorPrefs {
        commission: runtime_types::sp_arithmetic::per_things::Perbill(32),
        blocked: false,
    };
    let payload = runtime::tx().staking().validate(prefs);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let targets: ::std::vec::Vec<
        ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
    > = vec![
        ::subxt::utils::MultiAddress::Id(::subxt::utils::AccountId32([8; 32usize],),),
        ::subxt::utils::MultiAddress::Id(::subxt::utils::AccountId32([8; 32usize],),)
    ];
    let payload = runtime::tx().staking().nominate(targets);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let payload = runtime::tx().staking().chill();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let payee: runtime_types::pallet_staking::RewardDestination<
        ::subxt::utils::AccountId32,
    > = runtime_types::pallet_staking::RewardDestination::Staked;
    let payload = runtime::tx().staking().set_payee(payee);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let payload = runtime::tx().staking().set_controller();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let new: ::core::primitive::u32 = 32;
    let payload = runtime::tx().staking().set_validator_count(new);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let additional: ::core::primitive::u32 = 32;
    let payload = runtime::tx().staking().increase_validator_count(additional);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let factor: runtime_types::sp_arithmetic::per_things::Percent = runtime_types::sp_arithmetic::per_things::Percent(
        8,
    );
    let payload = runtime::tx().staking().scale_validator_count(factor);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let payload = runtime::tx().staking().force_no_eras();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let payload = runtime::tx().staking().force_new_era();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let invulnerables: ::std::vec::Vec<::subxt::utils::AccountId32> = vec![
        ::subxt::utils::AccountId32([8; 32usize],), ::subxt::utils::AccountId32([8;
        32usize],)
    ];
    let payload = runtime::tx().staking().set_invulnerables(invulnerables);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let stash: ::subxt::utils::AccountId32 = ::subxt::utils::AccountId32([8; 32usize]);
    let num_slashing_spans: ::core::primitive::u32 = 32;
    let payload = runtime::tx().staking().force_unstake(stash, num_slashing_spans);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let payload = runtime::tx().staking().force_new_era_always();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let era: ::core::primitive::u32 = 32;
    let slash_indices: ::std::vec::Vec<::core::primitive::u32> = vec![32, 32];
    let payload = runtime::tx().staking().cancel_deferred_slash(era, slash_indices);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let validator_stash: ::subxt::utils::AccountId32 = ::subxt::utils::AccountId32(
        [8; 32usize],
    );
    let era: ::core::primitive::u32 = 32;
    let payload = runtime::tx().staking().payout_stakers(validator_stash, era);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let value: ::core::primitive::u128 = 128;
    let payload = runtime::tx().staking().rebond(value);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let stash: ::subxt::utils::AccountId32 = ::subxt::utils::AccountId32([8; 32usize]);
    let num_slashing_spans: ::core::primitive::u32 = 32;
    let payload = runtime::tx().staking().reap_stash(stash, num_slashing_spans);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let who: ::std::vec::Vec<
        ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
    > = vec![
        ::subxt::utils::MultiAddress::Id(::subxt::utils::AccountId32([8; 32usize],),),
        ::subxt::utils::MultiAddress::Id(::subxt::utils::AccountId32([8; 32usize],),)
    ];
    let payload = runtime::tx().staking().kick(who);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let min_nominator_bond: runtime_types::pallet_staking::pallet::pallet::ConfigOp<
        ::core::primitive::u128,
    > = runtime_types::pallet_staking::pallet::pallet::ConfigOp::Noop;
    let min_validator_bond: runtime_types::pallet_staking::pallet::pallet::ConfigOp<
        ::core::primitive::u128,
    > = runtime_types::pallet_staking::pallet::pallet::ConfigOp::Noop;
    let max_nominator_count: runtime_types::pallet_staking::pallet::pallet::ConfigOp<
        ::core::primitive::u32,
    > = runtime_types::pallet_staking::pallet::pallet::ConfigOp::Noop;
    let max_validator_count: runtime_types::pallet_staking::pallet::pallet::ConfigOp<
        ::core::primitive::u32,
    > = runtime_types::pallet_staking::pallet::pallet::ConfigOp::Noop;
    let chill_threshold: runtime_types::pallet_staking::pallet::pallet::ConfigOp<
        runtime_types::sp_arithmetic::per_things::Percent,
    > = runtime_types::pallet_staking::pallet::pallet::ConfigOp::Noop;
    let min_commission: runtime_types::pallet_staking::pallet::pallet::ConfigOp<
        runtime_types::sp_arithmetic::per_things::Perbill,
    > = runtime_types::pallet_staking::pallet::pallet::ConfigOp::Noop;
    let payload = runtime::tx()
        .staking()
        .set_staking_configs(
            min_nominator_bond,
            min_validator_bond,
            max_nominator_count,
            max_validator_count,
            chill_threshold,
            min_commission,
        );
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let controller: ::subxt::utils::AccountId32 = ::subxt::utils::AccountId32(
        [8; 32usize],
    );
    let payload = runtime::tx().staking().chill_other(controller);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let validator_stash: ::subxt::utils::AccountId32 = ::subxt::utils::AccountId32(
        [8; 32usize],
    );
    let payload = runtime::tx().staking().force_apply_min_commission(validator_stash);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let new: runtime_types::sp_arithmetic::per_things::Perbill = runtime_types::sp_arithmetic::per_things::Perbill(
        32,
    );
    let payload = runtime::tx().staking().set_min_commission(new);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let other_signatories: ::std::vec::Vec<::subxt::utils::AccountId32> = vec![
        ::subxt::utils::AccountId32([8; 32usize],), ::subxt::utils::AccountId32([8;
        32usize],)
    ];
    let call: ::core::primitive::bool = false;
    let payload = runtime::tx().multisig().as_multi_threshold_1(other_signatories, call);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let threshold: ::core::primitive::u16 = 16;
    let other_signatories: ::std::vec::Vec<::subxt::utils::AccountId32> = vec![
        ::subxt::utils::AccountId32([8; 32usize],), ::subxt::utils::AccountId32([8;
        32usize],)
    ];
    let maybe_timepoint: ::core::option::Option<
        runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
    > = ::core::option::Option::None;
    let call: ::core::primitive::bool = false;
    let max_weight: runtime_types::sp_weights::weight_v2::Weight = runtime_types::sp_weights::weight_v2::Weight {
        ref_time: 64,
        proof_size: 64,
    };
    let payload = runtime::tx()
        .multisig()
        .as_multi(threshold, other_signatories, maybe_timepoint, call, max_weight);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let threshold: ::core::primitive::u16 = 16;
    let other_signatories: ::std::vec::Vec<::subxt::utils::AccountId32> = vec![
        ::subxt::utils::AccountId32([8; 32usize],), ::subxt::utils::AccountId32([8;
        32usize],)
    ];
    let maybe_timepoint: ::core::option::Option<
        runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
    > = ::core::option::Option::None;
    let call_hash: [::core::primitive::u8; 32usize] = [8; 32usize];
    let max_weight: runtime_types::sp_weights::weight_v2::Weight = runtime_types::sp_weights::weight_v2::Weight {
        ref_time: 64,
        proof_size: 64,
    };
    let payload = runtime::tx()
        .multisig()
        .approve_as_multi(
            threshold,
            other_signatories,
            maybe_timepoint,
            call_hash,
            max_weight,
        );
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let threshold: ::core::primitive::u16 = 16;
    let other_signatories: ::std::vec::Vec<::subxt::utils::AccountId32> = vec![
        ::subxt::utils::AccountId32([8; 32usize],), ::subxt::utils::AccountId32([8;
        32usize],)
    ];
    let timepoint: runtime_types::pallet_multisig::Timepoint<::core::primitive::u32> = runtime_types::pallet_multisig::Timepoint {
        height: 32,
        index: 32,
    };
    let call_hash: [::core::primitive::u8; 32usize] = [8; 32usize];
    let payload = runtime::tx()
        .multisig()
        .cancel_as_multi(threshold, other_signatories, timepoint, call_hash);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let data: runtime_types::polkadot_primitives::v5::InherentData<
        runtime_types::sp_runtime::generic::header::Header<
            ::core::primitive::u32,
            runtime_types::sp_runtime::traits::BlakeTwo256,
        >,
    > = runtime_types::polkadot_primitives::v5::InherentData {
        bitfields: vec![
            runtime_types::polkadot_primitives::v5::signed::UncheckedSigned { payload :
            runtime_types::polkadot_primitives::v5::AvailabilityBitfield(subxt::utils::bits::DecodedBits::from_iter([true,
            false, false]),), validator_index :
            runtime_types::polkadot_primitives::v5::ValidatorIndex(32,), signature :
            runtime_types::polkadot_primitives::v5::validator_app::Signature(runtime_types::sp_core::sr25519::Signature([8;
            64usize],),), __subxt_unused_type_params : ::core::marker::PhantomData },
            runtime_types::polkadot_primitives::v5::signed::UncheckedSigned { payload :
            runtime_types::polkadot_primitives::v5::AvailabilityBitfield(subxt::utils::bits::DecodedBits::from_iter([true,
            false, false]),), validator_index :
            runtime_types::polkadot_primitives::v5::ValidatorIndex(32,), signature :
            runtime_types::polkadot_primitives::v5::validator_app::Signature(runtime_types::sp_core::sr25519::Signature([8;
            64usize],),), __subxt_unused_type_params : ::core::marker::PhantomData }
        ],
        backed_candidates: vec![
            runtime_types::polkadot_primitives::v5::BackedCandidate { candidate :
            runtime_types::polkadot_primitives::v5::CommittedCandidateReceipt {
            descriptor : runtime_types::polkadot_primitives::v5::CandidateDescriptor {
            para_id : runtime_types::polkadot_parachain::primitives::Id(32,),
            relay_parent : ::subxt::utils::H256([8; 32usize],), collator :
            runtime_types::polkadot_primitives::v5::collator_app::Public(runtime_types::sp_core::sr25519::Public([8;
            32usize],),), persisted_validation_data_hash : ::subxt::utils::H256([8;
            32usize],), pov_hash : ::subxt::utils::H256([8; 32usize],), erasure_root :
            ::subxt::utils::H256([8; 32usize],), signature :
            runtime_types::polkadot_primitives::v5::collator_app::Signature(runtime_types::sp_core::sr25519::Signature([8;
            64usize],),), para_head : ::subxt::utils::H256([8; 32usize],),
            validation_code_hash :
            runtime_types::polkadot_parachain::primitives::ValidationCodeHash(::subxt::utils::H256([8;
            32usize],),), }, commitments :
            runtime_types::polkadot_primitives::v5::CandidateCommitments {
            upward_messages :
            runtime_types::bounded_collections::bounded_vec::BoundedVec(vec![vec![8, 8],
            vec![8, 8]],), horizontal_messages :
            runtime_types::bounded_collections::bounded_vec::BoundedVec(vec![runtime_types::polkadot_core_primitives::OutboundHrmpMessage
            { recipient : runtime_types::polkadot_parachain::primitives::Id(32,), data :
            vec![8, 8], }, runtime_types::polkadot_core_primitives::OutboundHrmpMessage {
            recipient : runtime_types::polkadot_parachain::primitives::Id(32,), data :
            vec![8, 8], }],), new_validation_code : ::core::option::Option::None,
            head_data : runtime_types::polkadot_parachain::primitives::HeadData(vec![8,
            8],), processed_downward_messages : 32, hrmp_watermark : 32, }, },
            validity_votes :
            vec![runtime_types::polkadot_primitives::v5::ValidityAttestation::Implicit(runtime_types::polkadot_primitives::v5::validator_app::Signature(runtime_types::sp_core::sr25519::Signature([8;
            64usize],),),),
            runtime_types::polkadot_primitives::v5::ValidityAttestation::Implicit(runtime_types::polkadot_primitives::v5::validator_app::Signature(runtime_types::sp_core::sr25519::Signature([8;
            64usize],),),)], validator_indices :
            subxt::utils::bits::DecodedBits::from_iter([true, false, false]), },
            runtime_types::polkadot_primitives::v5::BackedCandidate { candidate :
            runtime_types::polkadot_primitives::v5::CommittedCandidateReceipt {
            descriptor : runtime_types::polkadot_primitives::v5::CandidateDescriptor {
            para_id : runtime_types::polkadot_parachain::primitives::Id(32,),
            relay_parent : ::subxt::utils::H256([8; 32usize],), collator :
            runtime_types::polkadot_primitives::v5::collator_app::Public(runtime_types::sp_core::sr25519::Public([8;
            32usize],),), persisted_validation_data_hash : ::subxt::utils::H256([8;
            32usize],), pov_hash : ::subxt::utils::H256([8; 32usize],), erasure_root :
            ::subxt::utils::H256([8; 32usize],), signature :
            runtime_types::polkadot_primitives::v5::collator_app::Signature(runtime_types::sp_core::sr25519::Signature([8;
            64usize],),), para_head : ::subxt::utils::H256([8; 32usize],),
            validation_code_hash :
            runtime_types::polkadot_parachain::primitives::ValidationCodeHash(::subxt::utils::H256([8;
            32usize],),), }, commitments :
            runtime_types::polkadot_primitives::v5::CandidateCommitments {
            upward_messages :
            runtime_types::bounded_collections::bounded_vec::BoundedVec(vec![vec![8, 8],
            vec![8, 8]],), horizontal_messages :
            runtime_types::bounded_collections::bounded_vec::BoundedVec(vec![runtime_types::polkadot_core_primitives::OutboundHrmpMessage
            { recipient : runtime_types::polkadot_parachain::primitives::Id(32,), data :
            vec![8, 8], }, runtime_types::polkadot_core_primitives::OutboundHrmpMessage {
            recipient : runtime_types::polkadot_parachain::primitives::Id(32,), data :
            vec![8, 8], }],), new_validation_code : ::core::option::Option::None,
            head_data : runtime_types::polkadot_parachain::primitives::HeadData(vec![8,
            8],), processed_downward_messages : 32, hrmp_watermark : 32, }, },
            validity_votes :
            vec![runtime_types::polkadot_primitives::v5::ValidityAttestation::Implicit(runtime_types::polkadot_primitives::v5::validator_app::Signature(runtime_types::sp_core::sr25519::Signature([8;
            64usize],),),),
            runtime_types::polkadot_primitives::v5::ValidityAttestation::Implicit(runtime_types::polkadot_primitives::v5::validator_app::Signature(runtime_types::sp_core::sr25519::Signature([8;
            64usize],),),)], validator_indices :
            subxt::utils::bits::DecodedBits::from_iter([true, false, false]), }
        ],
        disputes: vec![
            runtime_types::polkadot_primitives::v5::DisputeStatementSet { candidate_hash
            :
            runtime_types::polkadot_core_primitives::CandidateHash(::subxt::utils::H256([8;
            32usize],),), session : 32, statements :
            vec![(runtime_types::polkadot_primitives::v5::DisputeStatement::Valid(runtime_types::polkadot_primitives::v5::ValidDisputeStatementKind::Explicit,),
            runtime_types::polkadot_primitives::v5::ValidatorIndex(32,),
            runtime_types::polkadot_primitives::v5::validator_app::Signature(runtime_types::sp_core::sr25519::Signature([8;
            64usize],),)),
            (runtime_types::polkadot_primitives::v5::DisputeStatement::Valid(runtime_types::polkadot_primitives::v5::ValidDisputeStatementKind::Explicit,),
            runtime_types::polkadot_primitives::v5::ValidatorIndex(32,),
            runtime_types::polkadot_primitives::v5::validator_app::Signature(runtime_types::sp_core::sr25519::Signature([8;
            64usize],),))], },
            runtime_types::polkadot_primitives::v5::DisputeStatementSet { candidate_hash
            :
            runtime_types::polkadot_core_primitives::CandidateHash(::subxt::utils::H256([8;
            32usize],),), session : 32, statements :
            vec![(runtime_types::polkadot_primitives::v5::DisputeStatement::Valid(runtime_types::polkadot_primitives::v5::ValidDisputeStatementKind::Explicit,),
            runtime_types::polkadot_primitives::v5::ValidatorIndex(32,),
            runtime_types::polkadot_primitives::v5::validator_app::Signature(runtime_types::sp_core::sr25519::Signature([8;
            64usize],),)),
            (runtime_types::polkadot_primitives::v5::DisputeStatement::Valid(runtime_types::polkadot_primitives::v5::ValidDisputeStatementKind::Explicit,),
            runtime_types::polkadot_primitives::v5::ValidatorIndex(32,),
            runtime_types::polkadot_primitives::v5::validator_app::Signature(runtime_types::sp_core::sr25519::Signature([8;
            64usize],),))], }
        ],
        parent_header: runtime_types::sp_runtime::generic::header::Header {
            parent_hash: ::subxt::utils::H256([8; 32usize]),
            number: 32,
            state_root: ::subxt::utils::H256([8; 32usize]),
            extrinsics_root: ::subxt::utils::H256([8; 32usize]),
            digest: runtime_types::sp_runtime::generic::digest::Digest {
                logs: vec![
                    runtime_types::sp_runtime::generic::digest::DigestItem::PreRuntime([8;
                    4usize], vec![8, 8],),
                    runtime_types::sp_runtime::generic::digest::DigestItem::PreRuntime([8;
                    4usize], vec![8, 8],)
                ],
            },
            __subxt_unused_type_params: ::core::marker::PhantomData,
        },
    };
    let payload = runtime::tx().para_inherent().enter(data);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let key_0: ::subxt::utils::AccountId32 = ::subxt::utils::AccountId32([8; 32usize]);
    let storage_query = runtime::storage().system().account(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::frame_system::AccountInfo<
            ::core::primitive::u32,
            runtime_types::pallet_balances::types::AccountData<::core::primitive::u128>,
        >,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let storage_query = runtime::storage().system().extrinsic_count();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = runtime::storage().system().block_weight();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::frame_support::dispatch::PerDispatchClass<
            runtime_types::sp_weights::weight_v2::Weight,
        >,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let storage_query = runtime::storage().system().all_extrinsics_len();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: ::core::primitive::u32 = 32;
    let storage_query = runtime::storage().system().block_hash(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::subxt::utils::H256> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: ::core::primitive::u32 = 32;
    let storage_query = runtime::storage().system().extrinsic_data(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::std::vec::Vec<::core::primitive::u8>> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = runtime::storage().system().number();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = runtime::storage().system().parent_hash();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::subxt::utils::H256> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = runtime::storage().system().digest();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<runtime_types::sp_runtime::generic::digest::Digest> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = runtime::storage().system().events();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        ::std::vec::Vec<
            runtime_types::frame_system::EventRecord<
                runtime_types::polkadot_runtime::RuntimeEvent,
                ::subxt::utils::H256,
            >,
        >,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let storage_query = runtime::storage().system().event_count();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: ::subxt::utils::H256 = ::subxt::utils::H256([8; 32usize]);
    let storage_query = runtime::storage().system().event_topics(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        ::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32)>,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let storage_query = runtime::storage().system().last_runtime_upgrade();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<runtime_types::frame_system::LastRuntimeUpgradeInfo> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = runtime::storage().system().upgraded_to_u32_ref_count();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::bool> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = runtime::storage().system().upgraded_to_triple_ref_count();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::bool> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = runtime::storage().system().execution_phase();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<runtime_types::frame_system::Phase> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = runtime::storage().timestamp().now();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u64> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = runtime::storage().timestamp().did_update();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::bool> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = runtime::storage().balances().total_issuance();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u128> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = runtime::storage().balances().inactive_issuance();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u128> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: ::subxt::utils::AccountId32 = ::subxt::utils::AccountId32([8; 32usize]);
    let storage_query = runtime::storage().balances().account(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::pallet_balances::types::AccountData<::core::primitive::u128>,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let key_0: ::subxt::utils::AccountId32 = ::subxt::utils::AccountId32([8; 32usize]);
    let storage_query = runtime::storage().balances().locks(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
            runtime_types::pallet_balances::types::BalanceLock<::core::primitive::u128>,
        >,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let key_0: ::subxt::utils::AccountId32 = ::subxt::utils::AccountId32([8; 32usize]);
    let storage_query = runtime::storage().balances().reserves(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::bounded_collections::bounded_vec::BoundedVec<
            runtime_types::pallet_balances::types::ReserveData<
                [::core::primitive::u8; 8usize],
                ::core::primitive::u128,
            >,
        >,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let key_0: ::subxt::utils::AccountId32 = ::subxt::utils::AccountId32([8; 32usize]);
    let storage_query = runtime::storage().balances().holds(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::bounded_collections::bounded_vec::BoundedVec<
            runtime_types::pallet_balances::types::IdAmount<
                runtime_types::polkadot_runtime::RuntimeHoldReason,
                ::core::primitive::u128,
            >,
        >,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let key_0: ::subxt::utils::AccountId32 = ::subxt::utils::AccountId32([8; 32usize]);
    let storage_query = runtime::storage().balances().freezes(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::bounded_collections::bounded_vec::BoundedVec<
            runtime_types::pallet_balances::types::IdAmount<(), ::core::primitive::u128>,
        >,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let storage_query = runtime::storage().staking().validator_count();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = runtime::storage().staking().minimum_validator_count();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = runtime::storage().staking().invulnerables();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::std::vec::Vec<::subxt::utils::AccountId32>> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: ::subxt::utils::AccountId32 = ::subxt::utils::AccountId32([8; 32usize]);
    let storage_query = runtime::storage().staking().bonded(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::subxt::utils::AccountId32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = runtime::storage().staking().min_nominator_bond();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u128> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = runtime::storage().staking().min_validator_bond();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u128> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = runtime::storage().staking().minimum_active_stake();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u128> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = runtime::storage().staking().min_commission();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<runtime_types::sp_arithmetic::per_things::Perbill> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: ::subxt::utils::AccountId32 = ::subxt::utils::AccountId32([8; 32usize]);
    let storage_query = runtime::storage().staking().ledger(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<runtime_types::pallet_staking::StakingLedger> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: ::subxt::utils::AccountId32 = ::subxt::utils::AccountId32([8; 32usize]);
    let storage_query = runtime::storage().staking().payee(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::pallet_staking::RewardDestination<::subxt::utils::AccountId32>,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let key_0: ::subxt::utils::AccountId32 = ::subxt::utils::AccountId32([8; 32usize]);
    let storage_query = runtime::storage().staking().validators(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<runtime_types::pallet_staking::ValidatorPrefs> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = runtime::storage().staking().counter_for_validators();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = runtime::storage().staking().max_validators_count();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: ::subxt::utils::AccountId32 = ::subxt::utils::AccountId32([8; 32usize]);
    let storage_query = runtime::storage().staking().nominators(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<runtime_types::pallet_staking::Nominations> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = runtime::storage().staking().counter_for_nominators();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = runtime::storage().staking().max_nominators_count();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = runtime::storage().staking().current_era();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = runtime::storage().staking().active_era();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<runtime_types::pallet_staking::ActiveEraInfo> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: ::core::primitive::u32 = 32;
    let storage_query = runtime::storage().staking().eras_start_session_index(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: ::core::primitive::u32 = 32;
    let key_1: ::subxt::utils::AccountId32 = ::subxt::utils::AccountId32([8; 32usize]);
    let storage_query = runtime::storage().staking().eras_stakers(key_0, key_1);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::pallet_staking::Exposure<
            ::subxt::utils::AccountId32,
            ::core::primitive::u128,
        >,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let key_0: ::core::primitive::u32 = 32;
    let key_1: ::subxt::utils::AccountId32 = ::subxt::utils::AccountId32([8; 32usize]);
    let storage_query = runtime::storage().staking().eras_stakers_clipped(key_0, key_1);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::pallet_staking::Exposure<
            ::subxt::utils::AccountId32,
            ::core::primitive::u128,
        >,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let key_0: ::core::primitive::u32 = 32;
    let key_1: ::subxt::utils::AccountId32 = ::subxt::utils::AccountId32([8; 32usize]);
    let storage_query = runtime::storage().staking().eras_validator_prefs(key_0, key_1);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<runtime_types::pallet_staking::ValidatorPrefs> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: ::core::primitive::u32 = 32;
    let storage_query = runtime::storage().staking().eras_validator_reward(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u128> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: ::core::primitive::u32 = 32;
    let storage_query = runtime::storage().staking().eras_reward_points(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::pallet_staking::EraRewardPoints<::subxt::utils::AccountId32>,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let key_0: ::core::primitive::u32 = 32;
    let storage_query = runtime::storage().staking().eras_total_stake(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u128> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = runtime::storage().staking().force_era();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<runtime_types::pallet_staking::Forcing> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = runtime::storage().staking().slash_reward_fraction();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<runtime_types::sp_arithmetic::per_things::Perbill> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = runtime::storage().staking().canceled_slash_payout();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u128> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: ::core::primitive::u32 = 32;
    let storage_query = runtime::storage().staking().unapplied_slashes(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        ::std::vec::Vec<
            runtime_types::pallet_staking::UnappliedSlash<
                ::subxt::utils::AccountId32,
                ::core::primitive::u128,
            >,
        >,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let storage_query = runtime::storage().staking().bonded_eras();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        ::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32)>,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let key_0: ::core::primitive::u32 = 32;
    let key_1: ::subxt::utils::AccountId32 = ::subxt::utils::AccountId32([8; 32usize]);
    let storage_query = runtime::storage()
        .staking()
        .validator_slash_in_era(key_0, key_1);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        (runtime_types::sp_arithmetic::per_things::Perbill, ::core::primitive::u128),
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let key_0: ::core::primitive::u32 = 32;
    let key_1: ::subxt::utils::AccountId32 = ::subxt::utils::AccountId32([8; 32usize]);
    let storage_query = runtime::storage()
        .staking()
        .nominator_slash_in_era(key_0, key_1);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u128> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: ::subxt::utils::AccountId32 = ::subxt::utils::AccountId32([8; 32usize]);
    let storage_query = runtime::storage().staking().slashing_spans(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<runtime_types::pallet_staking::slashing::SlashingSpans> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: ::subxt::utils::AccountId32 = ::subxt::utils::AccountId32([8; 32usize]);
    let key_1: ::core::primitive::u32 = 32;
    let storage_query = runtime::storage().staking().span_slash(key_0, key_1);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::pallet_staking::slashing::SpanRecord<::core::primitive::u128>,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let storage_query = runtime::storage().staking().current_planned_session();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = runtime::storage().staking().offending_validators();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        ::std::vec::Vec<(::core::primitive::u32, ::core::primitive::bool)>,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let storage_query = runtime::storage().staking().chill_threshold();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<runtime_types::sp_arithmetic::per_things::Percent> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: ::subxt::utils::AccountId32 = ::subxt::utils::AccountId32([8; 32usize]);
    let key_1: [::core::primitive::u8; 32usize] = [8; 32usize];
    let storage_query = runtime::storage().multisig().multisigs(key_0, key_1);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::pallet_multisig::Multisig<
            ::core::primitive::u32,
            ::core::primitive::u128,
            ::subxt::utils::AccountId32,
        >,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let storage_query = runtime::storage().para_inherent().included();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<()> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = runtime::storage().para_inherent().on_chain_votes();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::polkadot_primitives::v5::ScrapedOnChainVotes<::subxt::utils::H256>,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let constant_query = runtime::constants().system().block_weights();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: runtime_types::frame_system::limits::BlockWeights = api
        .constants()
        .at(&constant_query)?;
    let constant_query = runtime::constants().system().block_length();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: runtime_types::frame_system::limits::BlockLength = api
        .constants()
        .at(&constant_query)?;
    let constant_query = runtime::constants().system().block_hash_count();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    let constant_query = runtime::constants().system().db_weight();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: runtime_types::sp_weights::RuntimeDbWeight = api
        .constants()
        .at(&constant_query)?;
    let constant_query = runtime::constants().system().version();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: runtime_types::sp_version::RuntimeVersion = api
        .constants()
        .at(&constant_query)?;
    let constant_query = runtime::constants().system().ss58_prefix();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u16 = api.constants().at(&constant_query)?;
    let constant_query = runtime::constants().timestamp().minimum_period();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u64 = api.constants().at(&constant_query)?;
    let constant_query = runtime::constants().balances().existential_deposit();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u128 = api.constants().at(&constant_query)?;
    let constant_query = runtime::constants().balances().max_locks();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    let constant_query = runtime::constants().balances().max_reserves();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    let constant_query = runtime::constants().balances().max_holds();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    let constant_query = runtime::constants().balances().max_freezes();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    let constant_query = runtime::constants().staking().max_nominations();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    let constant_query = runtime::constants().staking().history_depth();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    let constant_query = runtime::constants().staking().sessions_per_era();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    let constant_query = runtime::constants().staking().bonding_duration();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    let constant_query = runtime::constants().staking().slash_defer_duration();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    let constant_query = runtime::constants()
        .staking()
        .max_nominator_rewarded_per_validator();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    let constant_query = runtime::constants().staking().max_unlocking_chunks();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    let constant_query = runtime::constants().multisig().deposit_base();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u128 = api.constants().at(&constant_query)?;
    let constant_query = runtime::constants().multisig().deposit_factor();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u128 = api.constants().at(&constant_query)?;
    let constant_query = runtime::constants().multisig().max_signatories();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    Ok(())
}
