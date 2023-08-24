
#[subxt::subxt(runtime_metadata_path = "polkadot.scale")]
pub mod polkadot {}
use polkadot::runtime_types;
use subxt::{OnlineClient, PolkadotConfig};
use subxt_signer::sr25519::dev;
pub fn main() {}
async fn wrapper() -> Result<(), Box<dyn std::error::Error>> {
    let remark: ::std::vec::Vec<::core::primitive::u8> = vec![8, 8];
    let payload = polkadot::tx().system().remark(remark);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let pages: ::core::primitive::u64 = 64;
    let payload = polkadot::tx().system().set_heap_pages(pages);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let code: ::std::vec::Vec<::core::primitive::u8> = vec![8, 8];
    let payload = polkadot::tx().system().set_code(code);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let code: ::std::vec::Vec<::core::primitive::u8> = vec![8, 8];
    let payload = polkadot::tx().system().set_code_without_checks(code);
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
    let payload = polkadot::tx().system().set_storage(items);
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
    let payload = polkadot::tx().system().kill_storage(keys);
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
    let payload = polkadot::tx().system().kill_prefix(prefix, subkeys);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let remark: ::std::vec::Vec<::core::primitive::u8> = vec![8, 8];
    let payload = polkadot::tx().system().remark_with_event(remark);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let when: ::core::primitive::u32 = 32;
    let maybe_periodic: ::core::option::Option<
        (::core::primitive::u32, ::core::primitive::u32),
    > = ::core::option::Option::None;
    let priority: ::core::primitive::u8 = 8;
    let call: runtime_types::polkadot_runtime::RuntimeCall = runtime_types::polkadot_runtime::RuntimeCall::System(runtime_types::frame_system::pallet::Call::remark {
        remark: vec![8, 8],
    });
    let payload = polkadot::tx()
        .scheduler()
        .schedule(when, maybe_periodic, priority, call);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let when: ::core::primitive::u32 = 32;
    let index: ::core::primitive::u32 = 32;
    let payload = polkadot::tx().scheduler().cancel(when, index);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let id: [::core::primitive::u8; 32usize] = [8; 32usize];
    let when: ::core::primitive::u32 = 32;
    let maybe_periodic: ::core::option::Option<
        (::core::primitive::u32, ::core::primitive::u32),
    > = ::core::option::Option::None;
    let priority: ::core::primitive::u8 = 8;
    let call: runtime_types::polkadot_runtime::RuntimeCall = runtime_types::polkadot_runtime::RuntimeCall::System(runtime_types::frame_system::pallet::Call::remark {
        remark: vec![8, 8],
    });
    let payload = polkadot::tx()
        .scheduler()
        .schedule_named(id, when, maybe_periodic, priority, call);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let id: [::core::primitive::u8; 32usize] = [8; 32usize];
    let payload = polkadot::tx().scheduler().cancel_named(id);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let after: ::core::primitive::u32 = 32;
    let maybe_periodic: ::core::option::Option<
        (::core::primitive::u32, ::core::primitive::u32),
    > = ::core::option::Option::None;
    let priority: ::core::primitive::u8 = 8;
    let call: runtime_types::polkadot_runtime::RuntimeCall = runtime_types::polkadot_runtime::RuntimeCall::System(runtime_types::frame_system::pallet::Call::remark {
        remark: vec![8, 8],
    });
    let payload = polkadot::tx()
        .scheduler()
        .schedule_after(after, maybe_periodic, priority, call);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let id: [::core::primitive::u8; 32usize] = [8; 32usize];
    let after: ::core::primitive::u32 = 32;
    let maybe_periodic: ::core::option::Option<
        (::core::primitive::u32, ::core::primitive::u32),
    > = ::core::option::Option::None;
    let priority: ::core::primitive::u8 = 8;
    let call: runtime_types::polkadot_runtime::RuntimeCall = runtime_types::polkadot_runtime::RuntimeCall::System(runtime_types::frame_system::pallet::Call::remark {
        remark: vec![8, 8],
    });
    let payload = polkadot::tx()
        .scheduler()
        .schedule_named_after(id, after, maybe_periodic, priority, call);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let bytes: ::std::vec::Vec<::core::primitive::u8> = vec![8, 8];
    let payload = polkadot::tx().preimage().note_preimage(bytes);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let hash: ::subxt::utils::H256 = ::subxt::utils::H256([8; 32usize]);
    let payload = polkadot::tx().preimage().unnote_preimage(hash);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let hash: ::subxt::utils::H256 = ::subxt::utils::H256([8; 32usize]);
    let payload = polkadot::tx().preimage().request_preimage(hash);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let hash: ::subxt::utils::H256 = ::subxt::utils::H256([8; 32usize]);
    let payload = polkadot::tx().preimage().unrequest_preimage(hash);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let equivocation_proof: runtime_types::sp_consensus_slots::EquivocationProof<
        runtime_types::sp_runtime::generic::header::Header<
            ::core::primitive::u32,
            runtime_types::sp_runtime::traits::BlakeTwo256,
        >,
        runtime_types::sp_consensus_babe::app::Public,
    > = runtime_types::sp_consensus_slots::EquivocationProof {
        offender: runtime_types::sp_consensus_babe::app::Public(
            runtime_types::sp_core::sr25519::Public([8; 32usize]),
        ),
        slot: runtime_types::sp_consensus_slots::Slot(64),
        first_header: runtime_types::sp_runtime::generic::header::Header {
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
        second_header: runtime_types::sp_runtime::generic::header::Header {
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
    let key_owner_proof: runtime_types::sp_session::MembershipProof = runtime_types::sp_session::MembershipProof {
        session: 32,
        trie_nodes: vec![vec![8, 8], vec![8, 8]],
        validator_count: 32,
    };
    let payload = polkadot::tx()
        .babe()
        .report_equivocation(equivocation_proof, key_owner_proof);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let equivocation_proof: runtime_types::sp_consensus_slots::EquivocationProof<
        runtime_types::sp_runtime::generic::header::Header<
            ::core::primitive::u32,
            runtime_types::sp_runtime::traits::BlakeTwo256,
        >,
        runtime_types::sp_consensus_babe::app::Public,
    > = runtime_types::sp_consensus_slots::EquivocationProof {
        offender: runtime_types::sp_consensus_babe::app::Public(
            runtime_types::sp_core::sr25519::Public([8; 32usize]),
        ),
        slot: runtime_types::sp_consensus_slots::Slot(64),
        first_header: runtime_types::sp_runtime::generic::header::Header {
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
        second_header: runtime_types::sp_runtime::generic::header::Header {
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
    let key_owner_proof: runtime_types::sp_session::MembershipProof = runtime_types::sp_session::MembershipProof {
        session: 32,
        trie_nodes: vec![vec![8, 8], vec![8, 8]],
        validator_count: 32,
    };
    let payload = polkadot::tx()
        .babe()
        .report_equivocation_unsigned(equivocation_proof, key_owner_proof);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let config: runtime_types::sp_consensus_babe::digests::NextConfigDescriptor = runtime_types::sp_consensus_babe::digests::NextConfigDescriptor::V1 {
        c: (64, 64),
        allowed_slots: runtime_types::sp_consensus_babe::AllowedSlots::PrimarySlots,
    };
    let payload = polkadot::tx().babe().plan_config_change(config);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let now: ::core::primitive::u64 = 64;
    let payload = polkadot::tx().timestamp().set(now);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let index: ::core::primitive::u32 = 32;
    let payload = polkadot::tx().indices().claim(index);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let new: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()> = ::subxt::utils::MultiAddress::Id(
        ::subxt::utils::AccountId32([8; 32usize]),
    );
    let index: ::core::primitive::u32 = 32;
    let payload = polkadot::tx().indices().transfer(new, index);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let index: ::core::primitive::u32 = 32;
    let payload = polkadot::tx().indices().free(index);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let new: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()> = ::subxt::utils::MultiAddress::Id(
        ::subxt::utils::AccountId32([8; 32usize]),
    );
    let index: ::core::primitive::u32 = 32;
    let freeze: ::core::primitive::bool = false;
    let payload = polkadot::tx().indices().force_transfer(new, index, freeze);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let index: ::core::primitive::u32 = 32;
    let payload = polkadot::tx().indices().freeze(index);
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
    let payload = polkadot::tx().balances().transfer_allow_death(dest, value);
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
    let payload = polkadot::tx()
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
    let payload = polkadot::tx().balances().force_transfer(source, dest, value);
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
    let payload = polkadot::tx().balances().transfer_keep_alive(dest, value);
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
    let payload = polkadot::tx().balances().transfer_all(dest, keep_alive);
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
    let payload = polkadot::tx().balances().force_unreserve(who, amount);
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
    let payload = polkadot::tx().balances().upgrade_accounts(who);
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
    let payload = polkadot::tx().balances().transfer(dest, value);
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
    let payload = polkadot::tx().balances().force_set_balance(who, new_free);
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
    let payload = polkadot::tx().staking().bond(value, payee);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let max_additional: ::core::primitive::u128 = 128;
    let payload = polkadot::tx().staking().bond_extra(max_additional);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let value: ::core::primitive::u128 = 128;
    let payload = polkadot::tx().staking().unbond(value);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let num_slashing_spans: ::core::primitive::u32 = 32;
    let payload = polkadot::tx().staking().withdraw_unbonded(num_slashing_spans);
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
    let payload = polkadot::tx().staking().validate(prefs);
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
    let payload = polkadot::tx().staking().nominate(targets);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let payload = polkadot::tx().staking().chill();
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
    let payload = polkadot::tx().staking().set_payee(payee);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let payload = polkadot::tx().staking().set_controller();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let new: ::core::primitive::u32 = 32;
    let payload = polkadot::tx().staking().set_validator_count(new);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let additional: ::core::primitive::u32 = 32;
    let payload = polkadot::tx().staking().increase_validator_count(additional);
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
    let payload = polkadot::tx().staking().scale_validator_count(factor);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let payload = polkadot::tx().staking().force_no_eras();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let payload = polkadot::tx().staking().force_new_era();
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
    let payload = polkadot::tx().staking().set_invulnerables(invulnerables);
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
    let payload = polkadot::tx().staking().force_unstake(stash, num_slashing_spans);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let payload = polkadot::tx().staking().force_new_era_always();
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
    let payload = polkadot::tx().staking().cancel_deferred_slash(era, slash_indices);
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
    let payload = polkadot::tx().staking().payout_stakers(validator_stash, era);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let value: ::core::primitive::u128 = 128;
    let payload = polkadot::tx().staking().rebond(value);
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
    let payload = polkadot::tx().staking().reap_stash(stash, num_slashing_spans);
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
    let payload = polkadot::tx().staking().kick(who);
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
    let payload = polkadot::tx()
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
    let payload = polkadot::tx().staking().chill_other(controller);
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
    let payload = polkadot::tx().staking().force_apply_min_commission(validator_stash);
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
    let payload = polkadot::tx().staking().set_min_commission(new);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let keys: runtime_types::polkadot_runtime::SessionKeys = runtime_types::polkadot_runtime::SessionKeys {
        grandpa: runtime_types::sp_consensus_grandpa::app::Public(
            runtime_types::sp_core::ed25519::Public([8; 32usize]),
        ),
        babe: runtime_types::sp_consensus_babe::app::Public(
            runtime_types::sp_core::sr25519::Public([8; 32usize]),
        ),
        im_online: runtime_types::pallet_im_online::sr25519::app_sr25519::Public(
            runtime_types::sp_core::sr25519::Public([8; 32usize]),
        ),
        para_validator: runtime_types::polkadot_primitives::v5::validator_app::Public(
            runtime_types::sp_core::sr25519::Public([8; 32usize]),
        ),
        para_assignment: runtime_types::polkadot_primitives::v5::assignment_app::Public(
            runtime_types::sp_core::sr25519::Public([8; 32usize]),
        ),
        authority_discovery: runtime_types::sp_authority_discovery::app::Public(
            runtime_types::sp_core::sr25519::Public([8; 32usize]),
        ),
    };
    let proof: ::std::vec::Vec<::core::primitive::u8> = vec![8, 8];
    let payload = polkadot::tx().session().set_keys(keys, proof);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let payload = polkadot::tx().session().purge_keys();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let equivocation_proof: runtime_types::sp_consensus_grandpa::EquivocationProof<
        ::subxt::utils::H256,
        ::core::primitive::u32,
    > = runtime_types::sp_consensus_grandpa::EquivocationProof {
        set_id: 64,
        equivocation: runtime_types::sp_consensus_grandpa::Equivocation::Prevote(runtime_types::finality_grandpa::Equivocation {
            round_number: 64,
            identity: runtime_types::sp_consensus_grandpa::app::Public(
                runtime_types::sp_core::ed25519::Public([8; 32usize]),
            ),
            first: (
                runtime_types::finality_grandpa::Prevote {
                    target_hash: ::subxt::utils::H256([8; 32usize]),
                    target_number: 32,
                },
                runtime_types::sp_consensus_grandpa::app::Signature(
                    runtime_types::sp_core::ed25519::Signature([8; 64usize]),
                ),
            ),
            second: (
                runtime_types::finality_grandpa::Prevote {
                    target_hash: ::subxt::utils::H256([8; 32usize]),
                    target_number: 32,
                },
                runtime_types::sp_consensus_grandpa::app::Signature(
                    runtime_types::sp_core::ed25519::Signature([8; 64usize]),
                ),
            ),
        }),
    };
    let key_owner_proof: runtime_types::sp_session::MembershipProof = runtime_types::sp_session::MembershipProof {
        session: 32,
        trie_nodes: vec![vec![8, 8], vec![8, 8]],
        validator_count: 32,
    };
    let payload = polkadot::tx()
        .grandpa()
        .report_equivocation(equivocation_proof, key_owner_proof);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let equivocation_proof: runtime_types::sp_consensus_grandpa::EquivocationProof<
        ::subxt::utils::H256,
        ::core::primitive::u32,
    > = runtime_types::sp_consensus_grandpa::EquivocationProof {
        set_id: 64,
        equivocation: runtime_types::sp_consensus_grandpa::Equivocation::Prevote(runtime_types::finality_grandpa::Equivocation {
            round_number: 64,
            identity: runtime_types::sp_consensus_grandpa::app::Public(
                runtime_types::sp_core::ed25519::Public([8; 32usize]),
            ),
            first: (
                runtime_types::finality_grandpa::Prevote {
                    target_hash: ::subxt::utils::H256([8; 32usize]),
                    target_number: 32,
                },
                runtime_types::sp_consensus_grandpa::app::Signature(
                    runtime_types::sp_core::ed25519::Signature([8; 64usize]),
                ),
            ),
            second: (
                runtime_types::finality_grandpa::Prevote {
                    target_hash: ::subxt::utils::H256([8; 32usize]),
                    target_number: 32,
                },
                runtime_types::sp_consensus_grandpa::app::Signature(
                    runtime_types::sp_core::ed25519::Signature([8; 64usize]),
                ),
            ),
        }),
    };
    let key_owner_proof: runtime_types::sp_session::MembershipProof = runtime_types::sp_session::MembershipProof {
        session: 32,
        trie_nodes: vec![vec![8, 8], vec![8, 8]],
        validator_count: 32,
    };
    let payload = polkadot::tx()
        .grandpa()
        .report_equivocation_unsigned(equivocation_proof, key_owner_proof);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let delay: ::core::primitive::u32 = 32;
    let best_finalized_block_number: ::core::primitive::u32 = 32;
    let payload = polkadot::tx()
        .grandpa()
        .note_stalled(delay, best_finalized_block_number);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let heartbeat: runtime_types::pallet_im_online::Heartbeat<::core::primitive::u32> = runtime_types::pallet_im_online::Heartbeat {
        block_number: 32,
        session_index: 32,
        authority_index: 32,
        validators_len: 32,
    };
    let signature: runtime_types::pallet_im_online::sr25519::app_sr25519::Signature = runtime_types::pallet_im_online::sr25519::app_sr25519::Signature(
        runtime_types::sp_core::sr25519::Signature([8; 64usize]),
    );
    let payload = polkadot::tx().im_online().heartbeat(heartbeat, signature);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let proposal: runtime_types::frame_support::traits::preimages::Bounded<
        runtime_types::polkadot_runtime::RuntimeCall,
    > = runtime_types::frame_support::traits::preimages::Bounded::Legacy {
        hash: ::subxt::utils::H256([8; 32usize]),
    };
    let value: ::core::primitive::u128 = 128;
    let payload = polkadot::tx().democracy().propose(proposal, value);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let proposal: ::core::primitive::u32 = 32;
    let payload = polkadot::tx().democracy().second(proposal);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let ref_index: ::core::primitive::u32 = 32;
    let vote: runtime_types::pallet_democracy::vote::AccountVote<
        ::core::primitive::u128,
    > = runtime_types::pallet_democracy::vote::AccountVote::Standard {
        vote: runtime_types::pallet_democracy::vote::Vote(8),
        balance: 128,
    };
    let payload = polkadot::tx().democracy().vote(ref_index, vote);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let ref_index: ::core::primitive::u32 = 32;
    let payload = polkadot::tx().democracy().emergency_cancel(ref_index);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let proposal: runtime_types::frame_support::traits::preimages::Bounded<
        runtime_types::polkadot_runtime::RuntimeCall,
    > = runtime_types::frame_support::traits::preimages::Bounded::Legacy {
        hash: ::subxt::utils::H256([8; 32usize]),
    };
    let payload = polkadot::tx().democracy().external_propose(proposal);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let proposal: runtime_types::frame_support::traits::preimages::Bounded<
        runtime_types::polkadot_runtime::RuntimeCall,
    > = runtime_types::frame_support::traits::preimages::Bounded::Legacy {
        hash: ::subxt::utils::H256([8; 32usize]),
    };
    let payload = polkadot::tx().democracy().external_propose_majority(proposal);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let proposal: runtime_types::frame_support::traits::preimages::Bounded<
        runtime_types::polkadot_runtime::RuntimeCall,
    > = runtime_types::frame_support::traits::preimages::Bounded::Legacy {
        hash: ::subxt::utils::H256([8; 32usize]),
    };
    let payload = polkadot::tx().democracy().external_propose_default(proposal);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let proposal_hash: ::subxt::utils::H256 = ::subxt::utils::H256([8; 32usize]);
    let voting_period: ::core::primitive::u32 = 32;
    let delay: ::core::primitive::u32 = 32;
    let payload = polkadot::tx()
        .democracy()
        .fast_track(proposal_hash, voting_period, delay);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let proposal_hash: ::subxt::utils::H256 = ::subxt::utils::H256([8; 32usize]);
    let payload = polkadot::tx().democracy().veto_external(proposal_hash);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let ref_index: ::core::primitive::u32 = 32;
    let payload = polkadot::tx().democracy().cancel_referendum(ref_index);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let to: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()> = ::subxt::utils::MultiAddress::Id(
        ::subxt::utils::AccountId32([8; 32usize]),
    );
    let conviction: runtime_types::pallet_democracy::conviction::Conviction = runtime_types::pallet_democracy::conviction::Conviction::None;
    let balance: ::core::primitive::u128 = 128;
    let payload = polkadot::tx().democracy().delegate(to, conviction, balance);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let payload = polkadot::tx().democracy().undelegate();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let payload = polkadot::tx().democracy().clear_public_proposals();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let target: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()> = ::subxt::utils::MultiAddress::Id(
        ::subxt::utils::AccountId32([8; 32usize]),
    );
    let payload = polkadot::tx().democracy().unlock(target);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let index: ::core::primitive::u32 = 32;
    let payload = polkadot::tx().democracy().remove_vote(index);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let target: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()> = ::subxt::utils::MultiAddress::Id(
        ::subxt::utils::AccountId32([8; 32usize]),
    );
    let index: ::core::primitive::u32 = 32;
    let payload = polkadot::tx().democracy().remove_other_vote(target, index);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let proposal_hash: ::subxt::utils::H256 = ::subxt::utils::H256([8; 32usize]);
    let maybe_ref_index: ::core::option::Option<::core::primitive::u32> = ::core::option::Option::None;
    let payload = polkadot::tx().democracy().blacklist(proposal_hash, maybe_ref_index);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let prop_index: ::core::primitive::u32 = 32;
    let payload = polkadot::tx().democracy().cancel_proposal(prop_index);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let owner: runtime_types::pallet_democracy::types::MetadataOwner = runtime_types::pallet_democracy::types::MetadataOwner::External;
    let maybe_hash: ::core::option::Option<::subxt::utils::H256> = ::core::option::Option::None;
    let payload = polkadot::tx().democracy().set_metadata(owner, maybe_hash);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let new_members: ::std::vec::Vec<::subxt::utils::AccountId32> = vec![
        ::subxt::utils::AccountId32([8; 32usize],), ::subxt::utils::AccountId32([8;
        32usize],)
    ];
    let prime: ::core::option::Option<::subxt::utils::AccountId32> = ::core::option::Option::None;
    let old_count: ::core::primitive::u32 = 32;
    let payload = polkadot::tx().council().set_members(new_members, prime, old_count);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let proposal: runtime_types::polkadot_runtime::RuntimeCall = runtime_types::polkadot_runtime::RuntimeCall::System(runtime_types::frame_system::pallet::Call::remark {
        remark: vec![8, 8],
    });
    let length_bound: ::core::primitive::u32 = 32;
    let payload = polkadot::tx().council().execute(proposal, length_bound);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let threshold: ::core::primitive::u32 = 32;
    let proposal: runtime_types::polkadot_runtime::RuntimeCall = runtime_types::polkadot_runtime::RuntimeCall::System(runtime_types::frame_system::pallet::Call::remark {
        remark: vec![8, 8],
    });
    let length_bound: ::core::primitive::u32 = 32;
    let payload = polkadot::tx().council().propose(threshold, proposal, length_bound);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let proposal: ::subxt::utils::H256 = ::subxt::utils::H256([8; 32usize]);
    let index: ::core::primitive::u32 = 32;
    let approve: ::core::primitive::bool = false;
    let payload = polkadot::tx().council().vote(proposal, index, approve);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let proposal_hash: ::subxt::utils::H256 = ::subxt::utils::H256([8; 32usize]);
    let payload = polkadot::tx().council().disapprove_proposal(proposal_hash);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let proposal_hash: ::subxt::utils::H256 = ::subxt::utils::H256([8; 32usize]);
    let index: ::core::primitive::u32 = 32;
    let proposal_weight_bound: runtime_types::sp_weights::weight_v2::Weight = runtime_types::sp_weights::weight_v2::Weight {
        ref_time: 64,
        proof_size: 64,
    };
    let length_bound: ::core::primitive::u32 = 32;
    let payload = polkadot::tx()
        .council()
        .close(proposal_hash, index, proposal_weight_bound, length_bound);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let new_members: ::std::vec::Vec<::subxt::utils::AccountId32> = vec![
        ::subxt::utils::AccountId32([8; 32usize],), ::subxt::utils::AccountId32([8;
        32usize],)
    ];
    let prime: ::core::option::Option<::subxt::utils::AccountId32> = ::core::option::Option::None;
    let old_count: ::core::primitive::u32 = 32;
    let payload = polkadot::tx()
        .technical_committee()
        .set_members(new_members, prime, old_count);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let proposal: runtime_types::polkadot_runtime::RuntimeCall = runtime_types::polkadot_runtime::RuntimeCall::System(runtime_types::frame_system::pallet::Call::remark {
        remark: vec![8, 8],
    });
    let length_bound: ::core::primitive::u32 = 32;
    let payload = polkadot::tx().technical_committee().execute(proposal, length_bound);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let threshold: ::core::primitive::u32 = 32;
    let proposal: runtime_types::polkadot_runtime::RuntimeCall = runtime_types::polkadot_runtime::RuntimeCall::System(runtime_types::frame_system::pallet::Call::remark {
        remark: vec![8, 8],
    });
    let length_bound: ::core::primitive::u32 = 32;
    let payload = polkadot::tx()
        .technical_committee()
        .propose(threshold, proposal, length_bound);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let proposal: ::subxt::utils::H256 = ::subxt::utils::H256([8; 32usize]);
    let index: ::core::primitive::u32 = 32;
    let approve: ::core::primitive::bool = false;
    let payload = polkadot::tx().technical_committee().vote(proposal, index, approve);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let proposal_hash: ::subxt::utils::H256 = ::subxt::utils::H256([8; 32usize]);
    let payload = polkadot::tx()
        .technical_committee()
        .disapprove_proposal(proposal_hash);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let proposal_hash: ::subxt::utils::H256 = ::subxt::utils::H256([8; 32usize]);
    let index: ::core::primitive::u32 = 32;
    let proposal_weight_bound: runtime_types::sp_weights::weight_v2::Weight = runtime_types::sp_weights::weight_v2::Weight {
        ref_time: 64,
        proof_size: 64,
    };
    let length_bound: ::core::primitive::u32 = 32;
    let payload = polkadot::tx()
        .technical_committee()
        .close(proposal_hash, index, proposal_weight_bound, length_bound);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let votes: ::std::vec::Vec<::subxt::utils::AccountId32> = vec![
        ::subxt::utils::AccountId32([8; 32usize],), ::subxt::utils::AccountId32([8;
        32usize],)
    ];
    let value: ::core::primitive::u128 = 128;
    let payload = polkadot::tx().phragmen_election().vote(votes, value);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let payload = polkadot::tx().phragmen_election().remove_voter();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let candidate_count: ::core::primitive::u32 = 32;
    let payload = polkadot::tx().phragmen_election().submit_candidacy(candidate_count);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let renouncing: runtime_types::pallet_elections_phragmen::Renouncing = runtime_types::pallet_elections_phragmen::Renouncing::Member;
    let payload = polkadot::tx().phragmen_election().renounce_candidacy(renouncing);
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
    let slash_bond: ::core::primitive::bool = false;
    let rerun_election: ::core::primitive::bool = false;
    let payload = polkadot::tx()
        .phragmen_election()
        .remove_member(who, slash_bond, rerun_election);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let num_voters: ::core::primitive::u32 = 32;
    let num_defunct: ::core::primitive::u32 = 32;
    let payload = polkadot::tx()
        .phragmen_election()
        .clean_defunct_voters(num_voters, num_defunct);
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
    let payload = polkadot::tx().technical_membership().add_member(who);
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
    let payload = polkadot::tx().technical_membership().remove_member(who);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let remove: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()> = ::subxt::utils::MultiAddress::Id(
        ::subxt::utils::AccountId32([8; 32usize]),
    );
    let add: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()> = ::subxt::utils::MultiAddress::Id(
        ::subxt::utils::AccountId32([8; 32usize]),
    );
    let payload = polkadot::tx().technical_membership().swap_member(remove, add);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let members: ::std::vec::Vec<::subxt::utils::AccountId32> = vec![
        ::subxt::utils::AccountId32([8; 32usize],), ::subxt::utils::AccountId32([8;
        32usize],)
    ];
    let payload = polkadot::tx().technical_membership().reset_members(members);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let new: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()> = ::subxt::utils::MultiAddress::Id(
        ::subxt::utils::AccountId32([8; 32usize]),
    );
    let payload = polkadot::tx().technical_membership().change_key(new);
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
    let payload = polkadot::tx().technical_membership().set_prime(who);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let payload = polkadot::tx().technical_membership().clear_prime();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let value: ::core::primitive::u128 = 128;
    let beneficiary: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()> = ::subxt::utils::MultiAddress::Id(
        ::subxt::utils::AccountId32([8; 32usize]),
    );
    let payload = polkadot::tx().treasury().propose_spend(value, beneficiary);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let proposal_id: ::core::primitive::u32 = 32;
    let payload = polkadot::tx().treasury().reject_proposal(proposal_id);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let proposal_id: ::core::primitive::u32 = 32;
    let payload = polkadot::tx().treasury().approve_proposal(proposal_id);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let amount: ::core::primitive::u128 = 128;
    let beneficiary: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()> = ::subxt::utils::MultiAddress::Id(
        ::subxt::utils::AccountId32([8; 32usize]),
    );
    let payload = polkadot::tx().treasury().spend(amount, beneficiary);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let proposal_id: ::core::primitive::u32 = 32;
    let payload = polkadot::tx().treasury().remove_approval(proposal_id);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let poll_index: ::core::primitive::u32 = 32;
    let vote: runtime_types::pallet_conviction_voting::vote::AccountVote<
        ::core::primitive::u128,
    > = runtime_types::pallet_conviction_voting::vote::AccountVote::Standard {
        vote: runtime_types::pallet_conviction_voting::vote::Vote(8),
        balance: 128,
    };
    let payload = polkadot::tx().conviction_voting().vote(poll_index, vote);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let class: ::core::primitive::u16 = 16;
    let to: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()> = ::subxt::utils::MultiAddress::Id(
        ::subxt::utils::AccountId32([8; 32usize]),
    );
    let conviction: runtime_types::pallet_conviction_voting::conviction::Conviction = runtime_types::pallet_conviction_voting::conviction::Conviction::None;
    let balance: ::core::primitive::u128 = 128;
    let payload = polkadot::tx()
        .conviction_voting()
        .delegate(class, to, conviction, balance);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let class: ::core::primitive::u16 = 16;
    let payload = polkadot::tx().conviction_voting().undelegate(class);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let class: ::core::primitive::u16 = 16;
    let target: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()> = ::subxt::utils::MultiAddress::Id(
        ::subxt::utils::AccountId32([8; 32usize]),
    );
    let payload = polkadot::tx().conviction_voting().unlock(class, target);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let class: ::core::option::Option<::core::primitive::u16> = ::core::option::Option::None;
    let index: ::core::primitive::u32 = 32;
    let payload = polkadot::tx().conviction_voting().remove_vote(class, index);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let target: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()> = ::subxt::utils::MultiAddress::Id(
        ::subxt::utils::AccountId32([8; 32usize]),
    );
    let class: ::core::primitive::u16 = 16;
    let index: ::core::primitive::u32 = 32;
    let payload = polkadot::tx()
        .conviction_voting()
        .remove_other_vote(target, class, index);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let proposal_origin: runtime_types::polkadot_runtime::OriginCaller = runtime_types::polkadot_runtime::OriginCaller::system(
        runtime_types::frame_support::dispatch::RawOrigin::Root,
    );
    let proposal: runtime_types::frame_support::traits::preimages::Bounded<
        runtime_types::polkadot_runtime::RuntimeCall,
    > = runtime_types::frame_support::traits::preimages::Bounded::Legacy {
        hash: ::subxt::utils::H256([8; 32usize]),
    };
    let enactment_moment: runtime_types::frame_support::traits::schedule::DispatchTime<
        ::core::primitive::u32,
    > = runtime_types::frame_support::traits::schedule::DispatchTime::At(32);
    let payload = polkadot::tx()
        .referenda()
        .submit(proposal_origin, proposal, enactment_moment);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let index: ::core::primitive::u32 = 32;
    let payload = polkadot::tx().referenda().place_decision_deposit(index);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let index: ::core::primitive::u32 = 32;
    let payload = polkadot::tx().referenda().refund_decision_deposit(index);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let index: ::core::primitive::u32 = 32;
    let payload = polkadot::tx().referenda().cancel(index);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let index: ::core::primitive::u32 = 32;
    let payload = polkadot::tx().referenda().kill(index);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let index: ::core::primitive::u32 = 32;
    let payload = polkadot::tx().referenda().nudge_referendum(index);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let track: ::core::primitive::u16 = 16;
    let payload = polkadot::tx().referenda().one_fewer_deciding(track);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let index: ::core::primitive::u32 = 32;
    let payload = polkadot::tx().referenda().refund_submission_deposit(index);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let index: ::core::primitive::u32 = 32;
    let maybe_hash: ::core::option::Option<::subxt::utils::H256> = ::core::option::Option::None;
    let payload = polkadot::tx().referenda().set_metadata(index, maybe_hash);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let call_hash: ::subxt::utils::H256 = ::subxt::utils::H256([8; 32usize]);
    let payload = polkadot::tx().whitelist().whitelist_call(call_hash);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let call_hash: ::subxt::utils::H256 = ::subxt::utils::H256([8; 32usize]);
    let payload = polkadot::tx().whitelist().remove_whitelisted_call(call_hash);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let call_hash: ::subxt::utils::H256 = ::subxt::utils::H256([8; 32usize]);
    let call_encoded_len: ::core::primitive::u32 = 32;
    let call_weight_witness: runtime_types::sp_weights::weight_v2::Weight = runtime_types::sp_weights::weight_v2::Weight {
        ref_time: 64,
        proof_size: 64,
    };
    let payload = polkadot::tx()
        .whitelist()
        .dispatch_whitelisted_call(call_hash, call_encoded_len, call_weight_witness);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let call: runtime_types::polkadot_runtime::RuntimeCall = runtime_types::polkadot_runtime::RuntimeCall::System(runtime_types::frame_system::pallet::Call::remark {
        remark: vec![8, 8],
    });
    let payload = polkadot::tx()
        .whitelist()
        .dispatch_whitelisted_call_with_preimage(call);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let dest: ::subxt::utils::AccountId32 = ::subxt::utils::AccountId32([8; 32usize]);
    let ethereum_signature: runtime_types::polkadot_runtime_common::claims::EcdsaSignature = runtime_types::polkadot_runtime_common::claims::EcdsaSignature(
        [8; 65usize],
    );
    let payload = polkadot::tx().claims().claim(dest, ethereum_signature);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let who: runtime_types::polkadot_runtime_common::claims::EthereumAddress = runtime_types::polkadot_runtime_common::claims::EthereumAddress(
        [8; 20usize],
    );
    let value: ::core::primitive::u128 = 128;
    let vesting_schedule: ::core::option::Option<
        (::core::primitive::u128, ::core::primitive::u128, ::core::primitive::u32),
    > = ::core::option::Option::None;
    let statement: ::core::option::Option<
        runtime_types::polkadot_runtime_common::claims::StatementKind,
    > = ::core::option::Option::None;
    let payload = polkadot::tx()
        .claims()
        .mint_claim(who, value, vesting_schedule, statement);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let dest: ::subxt::utils::AccountId32 = ::subxt::utils::AccountId32([8; 32usize]);
    let ethereum_signature: runtime_types::polkadot_runtime_common::claims::EcdsaSignature = runtime_types::polkadot_runtime_common::claims::EcdsaSignature(
        [8; 65usize],
    );
    let statement: ::std::vec::Vec<::core::primitive::u8> = vec![8, 8];
    let payload = polkadot::tx()
        .claims()
        .claim_attest(dest, ethereum_signature, statement);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let statement: ::std::vec::Vec<::core::primitive::u8> = vec![8, 8];
    let payload = polkadot::tx().claims().attest(statement);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let old: runtime_types::polkadot_runtime_common::claims::EthereumAddress = runtime_types::polkadot_runtime_common::claims::EthereumAddress(
        [8; 20usize],
    );
    let new: runtime_types::polkadot_runtime_common::claims::EthereumAddress = runtime_types::polkadot_runtime_common::claims::EthereumAddress(
        [8; 20usize],
    );
    let maybe_preclaim: ::core::option::Option<::subxt::utils::AccountId32> = ::core::option::Option::None;
    let payload = polkadot::tx().claims().move_claim(old, new, maybe_preclaim);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let payload = polkadot::tx().vesting().vest();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let target: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()> = ::subxt::utils::MultiAddress::Id(
        ::subxt::utils::AccountId32([8; 32usize]),
    );
    let payload = polkadot::tx().vesting().vest_other(target);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let target: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()> = ::subxt::utils::MultiAddress::Id(
        ::subxt::utils::AccountId32([8; 32usize]),
    );
    let schedule: runtime_types::pallet_vesting::vesting_info::VestingInfo<
        ::core::primitive::u128,
        ::core::primitive::u32,
    > = runtime_types::pallet_vesting::vesting_info::VestingInfo {
        locked: 128,
        per_block: 128,
        starting_block: 32,
    };
    let payload = polkadot::tx().vesting().vested_transfer(target, schedule);
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
    let target: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()> = ::subxt::utils::MultiAddress::Id(
        ::subxt::utils::AccountId32([8; 32usize]),
    );
    let schedule: runtime_types::pallet_vesting::vesting_info::VestingInfo<
        ::core::primitive::u128,
        ::core::primitive::u32,
    > = runtime_types::pallet_vesting::vesting_info::VestingInfo {
        locked: 128,
        per_block: 128,
        starting_block: 32,
    };
    let payload = polkadot::tx()
        .vesting()
        .force_vested_transfer(source, target, schedule);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let schedule1_index: ::core::primitive::u32 = 32;
    let schedule2_index: ::core::primitive::u32 = 32;
    let payload = polkadot::tx()
        .vesting()
        .merge_schedules(schedule1_index, schedule2_index);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let calls: ::std::vec::Vec<runtime_types::polkadot_runtime::RuntimeCall> = vec![
        runtime_types::polkadot_runtime::RuntimeCall::System(runtime_types::frame_system::pallet::Call::remark
        { remark : vec![8, 8], },),
        runtime_types::polkadot_runtime::RuntimeCall::System(runtime_types::frame_system::pallet::Call::remark
        { remark : vec![8, 8], },)
    ];
    let payload = polkadot::tx().utility().batch(calls);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let index: ::core::primitive::u16 = 16;
    let call: runtime_types::polkadot_runtime::RuntimeCall = runtime_types::polkadot_runtime::RuntimeCall::System(runtime_types::frame_system::pallet::Call::remark {
        remark: vec![8, 8],
    });
    let payload = polkadot::tx().utility().as_derivative(index, call);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let calls: ::std::vec::Vec<runtime_types::polkadot_runtime::RuntimeCall> = vec![
        runtime_types::polkadot_runtime::RuntimeCall::System(runtime_types::frame_system::pallet::Call::remark
        { remark : vec![8, 8], },),
        runtime_types::polkadot_runtime::RuntimeCall::System(runtime_types::frame_system::pallet::Call::remark
        { remark : vec![8, 8], },)
    ];
    let payload = polkadot::tx().utility().batch_all(calls);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let as_origin: runtime_types::polkadot_runtime::OriginCaller = runtime_types::polkadot_runtime::OriginCaller::system(
        runtime_types::frame_support::dispatch::RawOrigin::Root,
    );
    let call: runtime_types::polkadot_runtime::RuntimeCall = runtime_types::polkadot_runtime::RuntimeCall::System(runtime_types::frame_system::pallet::Call::remark {
        remark: vec![8, 8],
    });
    let payload = polkadot::tx().utility().dispatch_as(as_origin, call);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let calls: ::std::vec::Vec<runtime_types::polkadot_runtime::RuntimeCall> = vec![
        runtime_types::polkadot_runtime::RuntimeCall::System(runtime_types::frame_system::pallet::Call::remark
        { remark : vec![8, 8], },),
        runtime_types::polkadot_runtime::RuntimeCall::System(runtime_types::frame_system::pallet::Call::remark
        { remark : vec![8, 8], },)
    ];
    let payload = polkadot::tx().utility().force_batch(calls);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let call: runtime_types::polkadot_runtime::RuntimeCall = runtime_types::polkadot_runtime::RuntimeCall::System(runtime_types::frame_system::pallet::Call::remark {
        remark: vec![8, 8],
    });
    let weight: runtime_types::sp_weights::weight_v2::Weight = runtime_types::sp_weights::weight_v2::Weight {
        ref_time: 64,
        proof_size: 64,
    };
    let payload = polkadot::tx().utility().with_weight(call, weight);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let account: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()> = ::subxt::utils::MultiAddress::Id(
        ::subxt::utils::AccountId32([8; 32usize]),
    );
    let payload = polkadot::tx().identity().add_registrar(account);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let info: runtime_types::pallet_identity::types::IdentityInfo = runtime_types::pallet_identity::types::IdentityInfo {
        additional: runtime_types::bounded_collections::bounded_vec::BoundedVec(
            vec![
                (runtime_types::pallet_identity::types::Data::None,
                runtime_types::pallet_identity::types::Data::None),
                (runtime_types::pallet_identity::types::Data::None,
                runtime_types::pallet_identity::types::Data::None)
            ],
        ),
        display: runtime_types::pallet_identity::types::Data::None,
        legal: runtime_types::pallet_identity::types::Data::None,
        web: runtime_types::pallet_identity::types::Data::None,
        riot: runtime_types::pallet_identity::types::Data::None,
        email: runtime_types::pallet_identity::types::Data::None,
        pgp_fingerprint: ::core::option::Option::None,
        image: runtime_types::pallet_identity::types::Data::None,
        twitter: runtime_types::pallet_identity::types::Data::None,
    };
    let payload = polkadot::tx().identity().set_identity(info);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let subs: ::std::vec::Vec<
        (::subxt::utils::AccountId32, runtime_types::pallet_identity::types::Data),
    > = vec![
        (::subxt::utils::AccountId32([8; 32usize],),
        runtime_types::pallet_identity::types::Data::None),
        (::subxt::utils::AccountId32([8; 32usize],),
        runtime_types::pallet_identity::types::Data::None)
    ];
    let payload = polkadot::tx().identity().set_subs(subs);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let payload = polkadot::tx().identity().clear_identity();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let reg_index: ::core::primitive::u32 = 32;
    let max_fee: ::core::primitive::u128 = 128;
    let payload = polkadot::tx().identity().request_judgement(reg_index, max_fee);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let reg_index: ::core::primitive::u32 = 32;
    let payload = polkadot::tx().identity().cancel_request(reg_index);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let index: ::core::primitive::u32 = 32;
    let fee: ::core::primitive::u128 = 128;
    let payload = polkadot::tx().identity().set_fee(index, fee);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let index: ::core::primitive::u32 = 32;
    let new: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()> = ::subxt::utils::MultiAddress::Id(
        ::subxt::utils::AccountId32([8; 32usize]),
    );
    let payload = polkadot::tx().identity().set_account_id(index, new);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let index: ::core::primitive::u32 = 32;
    let fields: runtime_types::pallet_identity::types::BitFlags<
        runtime_types::pallet_identity::types::IdentityField,
    > = runtime_types::pallet_identity::types::BitFlags(64, ::core::marker::PhantomData);
    let payload = polkadot::tx().identity().set_fields(index, fields);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let reg_index: ::core::primitive::u32 = 32;
    let target: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()> = ::subxt::utils::MultiAddress::Id(
        ::subxt::utils::AccountId32([8; 32usize]),
    );
    let judgement: runtime_types::pallet_identity::types::Judgement<
        ::core::primitive::u128,
    > = runtime_types::pallet_identity::types::Judgement::Unknown;
    let identity: ::subxt::utils::H256 = ::subxt::utils::H256([8; 32usize]);
    let payload = polkadot::tx()
        .identity()
        .provide_judgement(reg_index, target, judgement, identity);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let target: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()> = ::subxt::utils::MultiAddress::Id(
        ::subxt::utils::AccountId32([8; 32usize]),
    );
    let payload = polkadot::tx().identity().kill_identity(target);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let sub: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()> = ::subxt::utils::MultiAddress::Id(
        ::subxt::utils::AccountId32([8; 32usize]),
    );
    let data: runtime_types::pallet_identity::types::Data = runtime_types::pallet_identity::types::Data::None;
    let payload = polkadot::tx().identity().add_sub(sub, data);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let sub: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()> = ::subxt::utils::MultiAddress::Id(
        ::subxt::utils::AccountId32([8; 32usize]),
    );
    let data: runtime_types::pallet_identity::types::Data = runtime_types::pallet_identity::types::Data::None;
    let payload = polkadot::tx().identity().rename_sub(sub, data);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let sub: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()> = ::subxt::utils::MultiAddress::Id(
        ::subxt::utils::AccountId32([8; 32usize]),
    );
    let payload = polkadot::tx().identity().remove_sub(sub);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let payload = polkadot::tx().identity().quit_sub();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let real: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()> = ::subxt::utils::MultiAddress::Id(
        ::subxt::utils::AccountId32([8; 32usize]),
    );
    let force_proxy_type: ::core::option::Option<
        runtime_types::polkadot_runtime::ProxyType,
    > = ::core::option::Option::None;
    let call: runtime_types::polkadot_runtime::RuntimeCall = runtime_types::polkadot_runtime::RuntimeCall::System(runtime_types::frame_system::pallet::Call::remark {
        remark: vec![8, 8],
    });
    let payload = polkadot::tx().proxy().proxy(real, force_proxy_type, call);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let delegate: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()> = ::subxt::utils::MultiAddress::Id(
        ::subxt::utils::AccountId32([8; 32usize]),
    );
    let proxy_type: runtime_types::polkadot_runtime::ProxyType = runtime_types::polkadot_runtime::ProxyType::Any;
    let delay: ::core::primitive::u32 = 32;
    let payload = polkadot::tx().proxy().add_proxy(delegate, proxy_type, delay);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let delegate: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()> = ::subxt::utils::MultiAddress::Id(
        ::subxt::utils::AccountId32([8; 32usize]),
    );
    let proxy_type: runtime_types::polkadot_runtime::ProxyType = runtime_types::polkadot_runtime::ProxyType::Any;
    let delay: ::core::primitive::u32 = 32;
    let payload = polkadot::tx().proxy().remove_proxy(delegate, proxy_type, delay);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let payload = polkadot::tx().proxy().remove_proxies();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let proxy_type: runtime_types::polkadot_runtime::ProxyType = runtime_types::polkadot_runtime::ProxyType::Any;
    let delay: ::core::primitive::u32 = 32;
    let index: ::core::primitive::u16 = 16;
    let payload = polkadot::tx().proxy().create_pure(proxy_type, delay, index);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let spawner: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()> = ::subxt::utils::MultiAddress::Id(
        ::subxt::utils::AccountId32([8; 32usize]),
    );
    let proxy_type: runtime_types::polkadot_runtime::ProxyType = runtime_types::polkadot_runtime::ProxyType::Any;
    let index: ::core::primitive::u16 = 16;
    let height: ::core::primitive::u32 = 32;
    let ext_index: ::core::primitive::u32 = 32;
    let payload = polkadot::tx()
        .proxy()
        .kill_pure(spawner, proxy_type, index, height, ext_index);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let real: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()> = ::subxt::utils::MultiAddress::Id(
        ::subxt::utils::AccountId32([8; 32usize]),
    );
    let call_hash: ::subxt::utils::H256 = ::subxt::utils::H256([8; 32usize]);
    let payload = polkadot::tx().proxy().announce(real, call_hash);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let real: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()> = ::subxt::utils::MultiAddress::Id(
        ::subxt::utils::AccountId32([8; 32usize]),
    );
    let call_hash: ::subxt::utils::H256 = ::subxt::utils::H256([8; 32usize]);
    let payload = polkadot::tx().proxy().remove_announcement(real, call_hash);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let delegate: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()> = ::subxt::utils::MultiAddress::Id(
        ::subxt::utils::AccountId32([8; 32usize]),
    );
    let call_hash: ::subxt::utils::H256 = ::subxt::utils::H256([8; 32usize]);
    let payload = polkadot::tx().proxy().reject_announcement(delegate, call_hash);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let delegate: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()> = ::subxt::utils::MultiAddress::Id(
        ::subxt::utils::AccountId32([8; 32usize]),
    );
    let real: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()> = ::subxt::utils::MultiAddress::Id(
        ::subxt::utils::AccountId32([8; 32usize]),
    );
    let force_proxy_type: ::core::option::Option<
        runtime_types::polkadot_runtime::ProxyType,
    > = ::core::option::Option::None;
    let call: runtime_types::polkadot_runtime::RuntimeCall = runtime_types::polkadot_runtime::RuntimeCall::System(runtime_types::frame_system::pallet::Call::remark {
        remark: vec![8, 8],
    });
    let payload = polkadot::tx()
        .proxy()
        .proxy_announced(delegate, real, force_proxy_type, call);
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
    let call: runtime_types::polkadot_runtime::RuntimeCall = runtime_types::polkadot_runtime::RuntimeCall::System(runtime_types::frame_system::pallet::Call::remark {
        remark: vec![8, 8],
    });
    let payload = polkadot::tx()
        .multisig()
        .as_multi_threshold_1(other_signatories, call);
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
    let call: runtime_types::polkadot_runtime::RuntimeCall = runtime_types::polkadot_runtime::RuntimeCall::System(runtime_types::frame_system::pallet::Call::remark {
        remark: vec![8, 8],
    });
    let max_weight: runtime_types::sp_weights::weight_v2::Weight = runtime_types::sp_weights::weight_v2::Weight {
        ref_time: 64,
        proof_size: 64,
    };
    let payload = polkadot::tx()
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
    let payload = polkadot::tx()
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
    let payload = polkadot::tx()
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
    let value: ::core::primitive::u128 = 128;
    let description: ::std::vec::Vec<::core::primitive::u8> = vec![8, 8];
    let payload = polkadot::tx().bounties().propose_bounty(value, description);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let bounty_id: ::core::primitive::u32 = 32;
    let payload = polkadot::tx().bounties().approve_bounty(bounty_id);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let bounty_id: ::core::primitive::u32 = 32;
    let curator: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()> = ::subxt::utils::MultiAddress::Id(
        ::subxt::utils::AccountId32([8; 32usize]),
    );
    let fee: ::core::primitive::u128 = 128;
    let payload = polkadot::tx().bounties().propose_curator(bounty_id, curator, fee);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let bounty_id: ::core::primitive::u32 = 32;
    let payload = polkadot::tx().bounties().unassign_curator(bounty_id);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let bounty_id: ::core::primitive::u32 = 32;
    let payload = polkadot::tx().bounties().accept_curator(bounty_id);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let bounty_id: ::core::primitive::u32 = 32;
    let beneficiary: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()> = ::subxt::utils::MultiAddress::Id(
        ::subxt::utils::AccountId32([8; 32usize]),
    );
    let payload = polkadot::tx().bounties().award_bounty(bounty_id, beneficiary);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let bounty_id: ::core::primitive::u32 = 32;
    let payload = polkadot::tx().bounties().claim_bounty(bounty_id);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let bounty_id: ::core::primitive::u32 = 32;
    let payload = polkadot::tx().bounties().close_bounty(bounty_id);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let bounty_id: ::core::primitive::u32 = 32;
    let remark: ::std::vec::Vec<::core::primitive::u8> = vec![8, 8];
    let payload = polkadot::tx().bounties().extend_bounty_expiry(bounty_id, remark);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let parent_bounty_id: ::core::primitive::u32 = 32;
    let value: ::core::primitive::u128 = 128;
    let description: ::std::vec::Vec<::core::primitive::u8> = vec![8, 8];
    let payload = polkadot::tx()
        .child_bounties()
        .add_child_bounty(parent_bounty_id, value, description);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let parent_bounty_id: ::core::primitive::u32 = 32;
    let child_bounty_id: ::core::primitive::u32 = 32;
    let curator: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()> = ::subxt::utils::MultiAddress::Id(
        ::subxt::utils::AccountId32([8; 32usize]),
    );
    let fee: ::core::primitive::u128 = 128;
    let payload = polkadot::tx()
        .child_bounties()
        .propose_curator(parent_bounty_id, child_bounty_id, curator, fee);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let parent_bounty_id: ::core::primitive::u32 = 32;
    let child_bounty_id: ::core::primitive::u32 = 32;
    let payload = polkadot::tx()
        .child_bounties()
        .accept_curator(parent_bounty_id, child_bounty_id);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let parent_bounty_id: ::core::primitive::u32 = 32;
    let child_bounty_id: ::core::primitive::u32 = 32;
    let payload = polkadot::tx()
        .child_bounties()
        .unassign_curator(parent_bounty_id, child_bounty_id);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let parent_bounty_id: ::core::primitive::u32 = 32;
    let child_bounty_id: ::core::primitive::u32 = 32;
    let beneficiary: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()> = ::subxt::utils::MultiAddress::Id(
        ::subxt::utils::AccountId32([8; 32usize]),
    );
    let payload = polkadot::tx()
        .child_bounties()
        .award_child_bounty(parent_bounty_id, child_bounty_id, beneficiary);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let parent_bounty_id: ::core::primitive::u32 = 32;
    let child_bounty_id: ::core::primitive::u32 = 32;
    let payload = polkadot::tx()
        .child_bounties()
        .claim_child_bounty(parent_bounty_id, child_bounty_id);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let parent_bounty_id: ::core::primitive::u32 = 32;
    let child_bounty_id: ::core::primitive::u32 = 32;
    let payload = polkadot::tx()
        .child_bounties()
        .close_child_bounty(parent_bounty_id, child_bounty_id);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let reason: ::std::vec::Vec<::core::primitive::u8> = vec![8, 8];
    let who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()> = ::subxt::utils::MultiAddress::Id(
        ::subxt::utils::AccountId32([8; 32usize]),
    );
    let payload = polkadot::tx().tips().report_awesome(reason, who);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let hash: ::subxt::utils::H256 = ::subxt::utils::H256([8; 32usize]);
    let payload = polkadot::tx().tips().retract_tip(hash);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let reason: ::std::vec::Vec<::core::primitive::u8> = vec![8, 8];
    let who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()> = ::subxt::utils::MultiAddress::Id(
        ::subxt::utils::AccountId32([8; 32usize]),
    );
    let tip_value: ::core::primitive::u128 = 128;
    let payload = polkadot::tx().tips().tip_new(reason, who, tip_value);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let hash: ::subxt::utils::H256 = ::subxt::utils::H256([8; 32usize]);
    let tip_value: ::core::primitive::u128 = 128;
    let payload = polkadot::tx().tips().tip(hash, tip_value);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let hash: ::subxt::utils::H256 = ::subxt::utils::H256([8; 32usize]);
    let payload = polkadot::tx().tips().close_tip(hash);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let hash: ::subxt::utils::H256 = ::subxt::utils::H256([8; 32usize]);
    let payload = polkadot::tx().tips().slash_tip(hash);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let raw_solution: runtime_types::pallet_election_provider_multi_phase::RawSolution<
        runtime_types::polkadot_runtime::NposCompactSolution16,
    > = runtime_types::pallet_election_provider_multi_phase::RawSolution {
        solution: runtime_types::polkadot_runtime::NposCompactSolution16 {
            votes1: vec![
                (::subxt::ext::codec::Compact(32), ::subxt::ext::codec::Compact(16)),
                (::subxt::ext::codec::Compact(32), ::subxt::ext::codec::Compact(16))
            ],
            votes2: vec![
                (::subxt::ext::codec::Compact(32), (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                ::subxt::ext::codec::Compact(16)), (::subxt::ext::codec::Compact(32),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                ::subxt::ext::codec::Compact(16))
            ],
            votes3: vec![
                (::subxt::ext::codec::Compact(32), [(::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,)))],
                ::subxt::ext::codec::Compact(16)), (::subxt::ext::codec::Compact(32),
                [(::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,)))],
                ::subxt::ext::codec::Compact(16))
            ],
            votes4: vec![
                (::subxt::ext::codec::Compact(32), [(::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,)))],
                ::subxt::ext::codec::Compact(16)), (::subxt::ext::codec::Compact(32),
                [(::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,)))],
                ::subxt::ext::codec::Compact(16))
            ],
            votes5: vec![
                (::subxt::ext::codec::Compact(32), [(::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,)))],
                ::subxt::ext::codec::Compact(16)), (::subxt::ext::codec::Compact(32),
                [(::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,)))],
                ::subxt::ext::codec::Compact(16))
            ],
            votes6: vec![
                (::subxt::ext::codec::Compact(32), [(::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,)))],
                ::subxt::ext::codec::Compact(16)), (::subxt::ext::codec::Compact(32),
                [(::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,)))],
                ::subxt::ext::codec::Compact(16))
            ],
            votes7: vec![
                (::subxt::ext::codec::Compact(32), [(::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,)))],
                ::subxt::ext::codec::Compact(16)), (::subxt::ext::codec::Compact(32),
                [(::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,)))],
                ::subxt::ext::codec::Compact(16))
            ],
            votes8: vec![
                (::subxt::ext::codec::Compact(32), [(::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,)))],
                ::subxt::ext::codec::Compact(16)), (::subxt::ext::codec::Compact(32),
                [(::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,)))],
                ::subxt::ext::codec::Compact(16))
            ],
            votes9: vec![
                (::subxt::ext::codec::Compact(32), [(::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,)))],
                ::subxt::ext::codec::Compact(16)), (::subxt::ext::codec::Compact(32),
                [(::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,)))],
                ::subxt::ext::codec::Compact(16))
            ],
            votes10: vec![
                (::subxt::ext::codec::Compact(32), [(::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,)))],
                ::subxt::ext::codec::Compact(16)), (::subxt::ext::codec::Compact(32),
                [(::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,)))],
                ::subxt::ext::codec::Compact(16))
            ],
            votes11: vec![
                (::subxt::ext::codec::Compact(32), [(::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,)))],
                ::subxt::ext::codec::Compact(16)), (::subxt::ext::codec::Compact(32),
                [(::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,)))],
                ::subxt::ext::codec::Compact(16))
            ],
            votes12: vec![
                (::subxt::ext::codec::Compact(32), [(::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,)))],
                ::subxt::ext::codec::Compact(16)), (::subxt::ext::codec::Compact(32),
                [(::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,)))],
                ::subxt::ext::codec::Compact(16))
            ],
            votes13: vec![
                (::subxt::ext::codec::Compact(32), [(::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,)))],
                ::subxt::ext::codec::Compact(16)), (::subxt::ext::codec::Compact(32),
                [(::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,)))],
                ::subxt::ext::codec::Compact(16))
            ],
            votes14: vec![
                (::subxt::ext::codec::Compact(32), [(::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,)))],
                ::subxt::ext::codec::Compact(16)), (::subxt::ext::codec::Compact(32),
                [(::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,)))],
                ::subxt::ext::codec::Compact(16))
            ],
            votes15: vec![
                (::subxt::ext::codec::Compact(32), [(::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,)))],
                ::subxt::ext::codec::Compact(16)), (::subxt::ext::codec::Compact(32),
                [(::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,)))],
                ::subxt::ext::codec::Compact(16))
            ],
            votes16: vec![
                (::subxt::ext::codec::Compact(32), [(::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,)))],
                ::subxt::ext::codec::Compact(16)), (::subxt::ext::codec::Compact(32),
                [(::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,)))],
                ::subxt::ext::codec::Compact(16))
            ],
        },
        score: runtime_types::sp_npos_elections::ElectionScore {
            minimal_stake: 128,
            sum_stake: 128,
            sum_stake_squared: 128,
        },
        round: 32,
    };
    let witness: runtime_types::pallet_election_provider_multi_phase::SolutionOrSnapshotSize = runtime_types::pallet_election_provider_multi_phase::SolutionOrSnapshotSize {
        voters: 32,
        targets: 32,
    };
    let payload = polkadot::tx()
        .election_provider_multi_phase()
        .submit_unsigned(raw_solution, witness);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let maybe_next_score: ::core::option::Option<
        runtime_types::sp_npos_elections::ElectionScore,
    > = ::core::option::Option::None;
    let payload = polkadot::tx()
        .election_provider_multi_phase()
        .set_minimum_untrusted_score(maybe_next_score);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let supports: ::std::vec::Vec<
        (
            ::subxt::utils::AccountId32,
            runtime_types::sp_npos_elections::Support<::subxt::utils::AccountId32>,
        ),
    > = vec![
        (::subxt::utils::AccountId32([8; 32usize],),
        runtime_types::sp_npos_elections::Support { total : 128, voters :
        vec![(::subxt::utils::AccountId32([8; 32usize],), 128),
        (::subxt::utils::AccountId32([8; 32usize],), 128)], }),
        (::subxt::utils::AccountId32([8; 32usize],),
        runtime_types::sp_npos_elections::Support { total : 128, voters :
        vec![(::subxt::utils::AccountId32([8; 32usize],), 128),
        (::subxt::utils::AccountId32([8; 32usize],), 128)], })
    ];
    let payload = polkadot::tx()
        .election_provider_multi_phase()
        .set_emergency_election_result(supports);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let raw_solution: runtime_types::pallet_election_provider_multi_phase::RawSolution<
        runtime_types::polkadot_runtime::NposCompactSolution16,
    > = runtime_types::pallet_election_provider_multi_phase::RawSolution {
        solution: runtime_types::polkadot_runtime::NposCompactSolution16 {
            votes1: vec![
                (::subxt::ext::codec::Compact(32), ::subxt::ext::codec::Compact(16)),
                (::subxt::ext::codec::Compact(32), ::subxt::ext::codec::Compact(16))
            ],
            votes2: vec![
                (::subxt::ext::codec::Compact(32), (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                ::subxt::ext::codec::Compact(16)), (::subxt::ext::codec::Compact(32),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                ::subxt::ext::codec::Compact(16))
            ],
            votes3: vec![
                (::subxt::ext::codec::Compact(32), [(::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,)))],
                ::subxt::ext::codec::Compact(16)), (::subxt::ext::codec::Compact(32),
                [(::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,)))],
                ::subxt::ext::codec::Compact(16))
            ],
            votes4: vec![
                (::subxt::ext::codec::Compact(32), [(::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,)))],
                ::subxt::ext::codec::Compact(16)), (::subxt::ext::codec::Compact(32),
                [(::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,)))],
                ::subxt::ext::codec::Compact(16))
            ],
            votes5: vec![
                (::subxt::ext::codec::Compact(32), [(::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,)))],
                ::subxt::ext::codec::Compact(16)), (::subxt::ext::codec::Compact(32),
                [(::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,)))],
                ::subxt::ext::codec::Compact(16))
            ],
            votes6: vec![
                (::subxt::ext::codec::Compact(32), [(::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,)))],
                ::subxt::ext::codec::Compact(16)), (::subxt::ext::codec::Compact(32),
                [(::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,)))],
                ::subxt::ext::codec::Compact(16))
            ],
            votes7: vec![
                (::subxt::ext::codec::Compact(32), [(::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,)))],
                ::subxt::ext::codec::Compact(16)), (::subxt::ext::codec::Compact(32),
                [(::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,)))],
                ::subxt::ext::codec::Compact(16))
            ],
            votes8: vec![
                (::subxt::ext::codec::Compact(32), [(::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,)))],
                ::subxt::ext::codec::Compact(16)), (::subxt::ext::codec::Compact(32),
                [(::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,)))],
                ::subxt::ext::codec::Compact(16))
            ],
            votes9: vec![
                (::subxt::ext::codec::Compact(32), [(::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,)))],
                ::subxt::ext::codec::Compact(16)), (::subxt::ext::codec::Compact(32),
                [(::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,)))],
                ::subxt::ext::codec::Compact(16))
            ],
            votes10: vec![
                (::subxt::ext::codec::Compact(32), [(::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,)))],
                ::subxt::ext::codec::Compact(16)), (::subxt::ext::codec::Compact(32),
                [(::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,)))],
                ::subxt::ext::codec::Compact(16))
            ],
            votes11: vec![
                (::subxt::ext::codec::Compact(32), [(::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,)))],
                ::subxt::ext::codec::Compact(16)), (::subxt::ext::codec::Compact(32),
                [(::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,)))],
                ::subxt::ext::codec::Compact(16))
            ],
            votes12: vec![
                (::subxt::ext::codec::Compact(32), [(::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,)))],
                ::subxt::ext::codec::Compact(16)), (::subxt::ext::codec::Compact(32),
                [(::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,)))],
                ::subxt::ext::codec::Compact(16))
            ],
            votes13: vec![
                (::subxt::ext::codec::Compact(32), [(::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,)))],
                ::subxt::ext::codec::Compact(16)), (::subxt::ext::codec::Compact(32),
                [(::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,)))],
                ::subxt::ext::codec::Compact(16))
            ],
            votes14: vec![
                (::subxt::ext::codec::Compact(32), [(::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,)))],
                ::subxt::ext::codec::Compact(16)), (::subxt::ext::codec::Compact(32),
                [(::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,)))],
                ::subxt::ext::codec::Compact(16))
            ],
            votes15: vec![
                (::subxt::ext::codec::Compact(32), [(::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,)))],
                ::subxt::ext::codec::Compact(16)), (::subxt::ext::codec::Compact(32),
                [(::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,)))],
                ::subxt::ext::codec::Compact(16))
            ],
            votes16: vec![
                (::subxt::ext::codec::Compact(32), [(::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,)))],
                ::subxt::ext::codec::Compact(16)), (::subxt::ext::codec::Compact(32),
                [(::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,))),
                (::subxt::ext::codec::Compact(16),
                ::subxt::ext::codec::Compact(runtime_types::sp_arithmetic::per_things::PerU16(16,)))],
                ::subxt::ext::codec::Compact(16))
            ],
        },
        score: runtime_types::sp_npos_elections::ElectionScore {
            minimal_stake: 128,
            sum_stake: 128,
            sum_stake_squared: 128,
        },
        round: 32,
    };
    let payload = polkadot::tx().election_provider_multi_phase().submit(raw_solution);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let maybe_max_voters: ::core::option::Option<::core::primitive::u32> = ::core::option::Option::None;
    let maybe_max_targets: ::core::option::Option<::core::primitive::u32> = ::core::option::Option::None;
    let payload = polkadot::tx()
        .election_provider_multi_phase()
        .governance_fallback(maybe_max_voters, maybe_max_targets);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let dislocated: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()> = ::subxt::utils::MultiAddress::Id(
        ::subxt::utils::AccountId32([8; 32usize]),
    );
    let payload = polkadot::tx().voter_list().rebag(dislocated);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let lighter: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()> = ::subxt::utils::MultiAddress::Id(
        ::subxt::utils::AccountId32([8; 32usize]),
    );
    let payload = polkadot::tx().voter_list().put_in_front_of(lighter);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let amount: ::core::primitive::u128 = 128;
    let pool_id: ::core::primitive::u32 = 32;
    let payload = polkadot::tx().nomination_pools().join(amount, pool_id);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let extra: runtime_types::pallet_nomination_pools::BondExtra<
        ::core::primitive::u128,
    > = runtime_types::pallet_nomination_pools::BondExtra::FreeBalance(128);
    let payload = polkadot::tx().nomination_pools().bond_extra(extra);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let payload = polkadot::tx().nomination_pools().claim_payout();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let member_account: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()> = ::subxt::utils::MultiAddress::Id(
        ::subxt::utils::AccountId32([8; 32usize]),
    );
    let unbonding_points: ::core::primitive::u128 = 128;
    let payload = polkadot::tx()
        .nomination_pools()
        .unbond(member_account, unbonding_points);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let pool_id: ::core::primitive::u32 = 32;
    let num_slashing_spans: ::core::primitive::u32 = 32;
    let payload = polkadot::tx()
        .nomination_pools()
        .pool_withdraw_unbonded(pool_id, num_slashing_spans);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let member_account: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()> = ::subxt::utils::MultiAddress::Id(
        ::subxt::utils::AccountId32([8; 32usize]),
    );
    let num_slashing_spans: ::core::primitive::u32 = 32;
    let payload = polkadot::tx()
        .nomination_pools()
        .withdraw_unbonded(member_account, num_slashing_spans);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let amount: ::core::primitive::u128 = 128;
    let root: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()> = ::subxt::utils::MultiAddress::Id(
        ::subxt::utils::AccountId32([8; 32usize]),
    );
    let nominator: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()> = ::subxt::utils::MultiAddress::Id(
        ::subxt::utils::AccountId32([8; 32usize]),
    );
    let bouncer: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()> = ::subxt::utils::MultiAddress::Id(
        ::subxt::utils::AccountId32([8; 32usize]),
    );
    let payload = polkadot::tx()
        .nomination_pools()
        .create(amount, root, nominator, bouncer);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let amount: ::core::primitive::u128 = 128;
    let root: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()> = ::subxt::utils::MultiAddress::Id(
        ::subxt::utils::AccountId32([8; 32usize]),
    );
    let nominator: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()> = ::subxt::utils::MultiAddress::Id(
        ::subxt::utils::AccountId32([8; 32usize]),
    );
    let bouncer: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()> = ::subxt::utils::MultiAddress::Id(
        ::subxt::utils::AccountId32([8; 32usize]),
    );
    let pool_id: ::core::primitive::u32 = 32;
    let payload = polkadot::tx()
        .nomination_pools()
        .create_with_pool_id(amount, root, nominator, bouncer, pool_id);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let pool_id: ::core::primitive::u32 = 32;
    let validators: ::std::vec::Vec<::subxt::utils::AccountId32> = vec![
        ::subxt::utils::AccountId32([8; 32usize],), ::subxt::utils::AccountId32([8;
        32usize],)
    ];
    let payload = polkadot::tx().nomination_pools().nominate(pool_id, validators);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let pool_id: ::core::primitive::u32 = 32;
    let state: runtime_types::pallet_nomination_pools::PoolState = runtime_types::pallet_nomination_pools::PoolState::Open;
    let payload = polkadot::tx().nomination_pools().set_state(pool_id, state);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let pool_id: ::core::primitive::u32 = 32;
    let metadata: ::std::vec::Vec<::core::primitive::u8> = vec![8, 8];
    let payload = polkadot::tx().nomination_pools().set_metadata(pool_id, metadata);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let min_join_bond: runtime_types::pallet_nomination_pools::ConfigOp<
        ::core::primitive::u128,
    > = runtime_types::pallet_nomination_pools::ConfigOp::Noop;
    let min_create_bond: runtime_types::pallet_nomination_pools::ConfigOp<
        ::core::primitive::u128,
    > = runtime_types::pallet_nomination_pools::ConfigOp::Noop;
    let max_pools: runtime_types::pallet_nomination_pools::ConfigOp<
        ::core::primitive::u32,
    > = runtime_types::pallet_nomination_pools::ConfigOp::Noop;
    let max_members: runtime_types::pallet_nomination_pools::ConfigOp<
        ::core::primitive::u32,
    > = runtime_types::pallet_nomination_pools::ConfigOp::Noop;
    let max_members_per_pool: runtime_types::pallet_nomination_pools::ConfigOp<
        ::core::primitive::u32,
    > = runtime_types::pallet_nomination_pools::ConfigOp::Noop;
    let global_max_commission: runtime_types::pallet_nomination_pools::ConfigOp<
        runtime_types::sp_arithmetic::per_things::Perbill,
    > = runtime_types::pallet_nomination_pools::ConfigOp::Noop;
    let payload = polkadot::tx()
        .nomination_pools()
        .set_configs(
            min_join_bond,
            min_create_bond,
            max_pools,
            max_members,
            max_members_per_pool,
            global_max_commission,
        );
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let pool_id: ::core::primitive::u32 = 32;
    let new_root: runtime_types::pallet_nomination_pools::ConfigOp<
        ::subxt::utils::AccountId32,
    > = runtime_types::pallet_nomination_pools::ConfigOp::Noop;
    let new_nominator: runtime_types::pallet_nomination_pools::ConfigOp<
        ::subxt::utils::AccountId32,
    > = runtime_types::pallet_nomination_pools::ConfigOp::Noop;
    let new_bouncer: runtime_types::pallet_nomination_pools::ConfigOp<
        ::subxt::utils::AccountId32,
    > = runtime_types::pallet_nomination_pools::ConfigOp::Noop;
    let payload = polkadot::tx()
        .nomination_pools()
        .update_roles(pool_id, new_root, new_nominator, new_bouncer);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let pool_id: ::core::primitive::u32 = 32;
    let payload = polkadot::tx().nomination_pools().chill(pool_id);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let member: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()> = ::subxt::utils::MultiAddress::Id(
        ::subxt::utils::AccountId32([8; 32usize]),
    );
    let extra: runtime_types::pallet_nomination_pools::BondExtra<
        ::core::primitive::u128,
    > = runtime_types::pallet_nomination_pools::BondExtra::FreeBalance(128);
    let payload = polkadot::tx().nomination_pools().bond_extra_other(member, extra);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let permission: runtime_types::pallet_nomination_pools::ClaimPermission = runtime_types::pallet_nomination_pools::ClaimPermission::Permissioned;
    let payload = polkadot::tx().nomination_pools().set_claim_permission(permission);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let other: ::subxt::utils::AccountId32 = ::subxt::utils::AccountId32([8; 32usize]);
    let payload = polkadot::tx().nomination_pools().claim_payout_other(other);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let pool_id: ::core::primitive::u32 = 32;
    let new_commission: ::core::option::Option<
        (runtime_types::sp_arithmetic::per_things::Perbill, ::subxt::utils::AccountId32),
    > = ::core::option::Option::None;
    let payload = polkadot::tx()
        .nomination_pools()
        .set_commission(pool_id, new_commission);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let pool_id: ::core::primitive::u32 = 32;
    let max_commission: runtime_types::sp_arithmetic::per_things::Perbill = runtime_types::sp_arithmetic::per_things::Perbill(
        32,
    );
    let payload = polkadot::tx()
        .nomination_pools()
        .set_commission_max(pool_id, max_commission);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let pool_id: ::core::primitive::u32 = 32;
    let change_rate: runtime_types::pallet_nomination_pools::CommissionChangeRate<
        ::core::primitive::u32,
    > = runtime_types::pallet_nomination_pools::CommissionChangeRate {
        max_increase: runtime_types::sp_arithmetic::per_things::Perbill(32),
        min_delay: 32,
    };
    let payload = polkadot::tx()
        .nomination_pools()
        .set_commission_change_rate(pool_id, change_rate);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let pool_id: ::core::primitive::u32 = 32;
    let payload = polkadot::tx().nomination_pools().claim_commission(pool_id);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let payload = polkadot::tx().fast_unstake().register_fast_unstake();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let payload = polkadot::tx().fast_unstake().deregister();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let eras_to_check: ::core::primitive::u32 = 32;
    let payload = polkadot::tx().fast_unstake().control(eras_to_check);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let new: ::core::primitive::u32 = 32;
    let payload = polkadot::tx().configuration().set_validation_upgrade_cooldown(new);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let new: ::core::primitive::u32 = 32;
    let payload = polkadot::tx().configuration().set_validation_upgrade_delay(new);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let new: ::core::primitive::u32 = 32;
    let payload = polkadot::tx().configuration().set_code_retention_period(new);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let new: ::core::primitive::u32 = 32;
    let payload = polkadot::tx().configuration().set_max_code_size(new);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let new: ::core::primitive::u32 = 32;
    let payload = polkadot::tx().configuration().set_max_pov_size(new);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let new: ::core::primitive::u32 = 32;
    let payload = polkadot::tx().configuration().set_max_head_data_size(new);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let new: ::core::primitive::u32 = 32;
    let payload = polkadot::tx().configuration().set_parathread_cores(new);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let new: ::core::primitive::u32 = 32;
    let payload = polkadot::tx().configuration().set_parathread_retries(new);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let new: ::core::primitive::u32 = 32;
    let payload = polkadot::tx().configuration().set_group_rotation_frequency(new);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let new: ::core::primitive::u32 = 32;
    let payload = polkadot::tx().configuration().set_chain_availability_period(new);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let new: ::core::primitive::u32 = 32;
    let payload = polkadot::tx().configuration().set_thread_availability_period(new);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let new: ::core::primitive::u32 = 32;
    let payload = polkadot::tx().configuration().set_scheduling_lookahead(new);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let new: ::core::option::Option<::core::primitive::u32> = ::core::option::Option::None;
    let payload = polkadot::tx().configuration().set_max_validators_per_core(new);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let new: ::core::option::Option<::core::primitive::u32> = ::core::option::Option::None;
    let payload = polkadot::tx().configuration().set_max_validators(new);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let new: ::core::primitive::u32 = 32;
    let payload = polkadot::tx().configuration().set_dispute_period(new);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let new: ::core::primitive::u32 = 32;
    let payload = polkadot::tx()
        .configuration()
        .set_dispute_post_conclusion_acceptance_period(new);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let new: ::core::primitive::u32 = 32;
    let payload = polkadot::tx().configuration().set_no_show_slots(new);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let new: ::core::primitive::u32 = 32;
    let payload = polkadot::tx().configuration().set_n_delay_tranches(new);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let new: ::core::primitive::u32 = 32;
    let payload = polkadot::tx().configuration().set_zeroth_delay_tranche_width(new);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let new: ::core::primitive::u32 = 32;
    let payload = polkadot::tx().configuration().set_needed_approvals(new);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let new: ::core::primitive::u32 = 32;
    let payload = polkadot::tx().configuration().set_relay_vrf_modulo_samples(new);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let new: ::core::primitive::u32 = 32;
    let payload = polkadot::tx().configuration().set_max_upward_queue_count(new);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let new: ::core::primitive::u32 = 32;
    let payload = polkadot::tx().configuration().set_max_upward_queue_size(new);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let new: ::core::primitive::u32 = 32;
    let payload = polkadot::tx().configuration().set_max_downward_message_size(new);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let new: ::core::primitive::u32 = 32;
    let payload = polkadot::tx().configuration().set_max_upward_message_size(new);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let new: ::core::primitive::u32 = 32;
    let payload = polkadot::tx()
        .configuration()
        .set_max_upward_message_num_per_candidate(new);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let new: ::core::primitive::u32 = 32;
    let payload = polkadot::tx().configuration().set_hrmp_open_request_ttl(new);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let new: ::core::primitive::u128 = 128;
    let payload = polkadot::tx().configuration().set_hrmp_sender_deposit(new);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let new: ::core::primitive::u128 = 128;
    let payload = polkadot::tx().configuration().set_hrmp_recipient_deposit(new);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let new: ::core::primitive::u32 = 32;
    let payload = polkadot::tx().configuration().set_hrmp_channel_max_capacity(new);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let new: ::core::primitive::u32 = 32;
    let payload = polkadot::tx().configuration().set_hrmp_channel_max_total_size(new);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let new: ::core::primitive::u32 = 32;
    let payload = polkadot::tx()
        .configuration()
        .set_hrmp_max_parachain_inbound_channels(new);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let new: ::core::primitive::u32 = 32;
    let payload = polkadot::tx()
        .configuration()
        .set_hrmp_max_parathread_inbound_channels(new);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let new: ::core::primitive::u32 = 32;
    let payload = polkadot::tx().configuration().set_hrmp_channel_max_message_size(new);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let new: ::core::primitive::u32 = 32;
    let payload = polkadot::tx()
        .configuration()
        .set_hrmp_max_parachain_outbound_channels(new);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let new: ::core::primitive::u32 = 32;
    let payload = polkadot::tx()
        .configuration()
        .set_hrmp_max_parathread_outbound_channels(new);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let new: ::core::primitive::u32 = 32;
    let payload = polkadot::tx()
        .configuration()
        .set_hrmp_max_message_num_per_candidate(new);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let new: ::core::primitive::bool = false;
    let payload = polkadot::tx().configuration().set_pvf_checking_enabled(new);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let new: ::core::primitive::u32 = 32;
    let payload = polkadot::tx().configuration().set_pvf_voting_ttl(new);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let new: ::core::primitive::u32 = 32;
    let payload = polkadot::tx()
        .configuration()
        .set_minimum_validation_upgrade_delay(new);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let new: ::core::primitive::bool = false;
    let payload = polkadot::tx().configuration().set_bypass_consistency_check(new);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let new: runtime_types::polkadot_primitives::vstaging::AsyncBackingParams = runtime_types::polkadot_primitives::vstaging::AsyncBackingParams {
        max_candidate_depth: 32,
        allowed_ancestry_len: 32,
    };
    let payload = polkadot::tx().configuration().set_async_backing_params(new);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let new: runtime_types::polkadot_primitives::v5::executor_params::ExecutorParams = runtime_types::polkadot_primitives::v5::executor_params::ExecutorParams(
        vec![
            runtime_types::polkadot_primitives::v5::executor_params::ExecutorParam::MaxMemoryPages(32,),
            runtime_types::polkadot_primitives::v5::executor_params::ExecutorParam::MaxMemoryPages(32,)
        ],
    );
    let payload = polkadot::tx().configuration().set_executor_params(new);
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
    let payload = polkadot::tx().para_inherent().enter(data);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let para: runtime_types::polkadot_parachain::primitives::Id = runtime_types::polkadot_parachain::primitives::Id(
        32,
    );
    let new_code: runtime_types::polkadot_parachain::primitives::ValidationCode = runtime_types::polkadot_parachain::primitives::ValidationCode(
        vec![8, 8],
    );
    let payload = polkadot::tx().paras().force_set_current_code(para, new_code);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let para: runtime_types::polkadot_parachain::primitives::Id = runtime_types::polkadot_parachain::primitives::Id(
        32,
    );
    let new_head: runtime_types::polkadot_parachain::primitives::HeadData = runtime_types::polkadot_parachain::primitives::HeadData(
        vec![8, 8],
    );
    let payload = polkadot::tx().paras().force_set_current_head(para, new_head);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let para: runtime_types::polkadot_parachain::primitives::Id = runtime_types::polkadot_parachain::primitives::Id(
        32,
    );
    let new_code: runtime_types::polkadot_parachain::primitives::ValidationCode = runtime_types::polkadot_parachain::primitives::ValidationCode(
        vec![8, 8],
    );
    let relay_parent_number: ::core::primitive::u32 = 32;
    let payload = polkadot::tx()
        .paras()
        .force_schedule_code_upgrade(para, new_code, relay_parent_number);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let para: runtime_types::polkadot_parachain::primitives::Id = runtime_types::polkadot_parachain::primitives::Id(
        32,
    );
    let new_head: runtime_types::polkadot_parachain::primitives::HeadData = runtime_types::polkadot_parachain::primitives::HeadData(
        vec![8, 8],
    );
    let payload = polkadot::tx().paras().force_note_new_head(para, new_head);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let para: runtime_types::polkadot_parachain::primitives::Id = runtime_types::polkadot_parachain::primitives::Id(
        32,
    );
    let payload = polkadot::tx().paras().force_queue_action(para);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let validation_code: runtime_types::polkadot_parachain::primitives::ValidationCode = runtime_types::polkadot_parachain::primitives::ValidationCode(
        vec![8, 8],
    );
    let payload = polkadot::tx().paras().add_trusted_validation_code(validation_code);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let validation_code_hash: runtime_types::polkadot_parachain::primitives::ValidationCodeHash = runtime_types::polkadot_parachain::primitives::ValidationCodeHash(
        ::subxt::utils::H256([8; 32usize]),
    );
    let payload = polkadot::tx()
        .paras()
        .poke_unused_validation_code(validation_code_hash);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let stmt: runtime_types::polkadot_primitives::v5::PvfCheckStatement = runtime_types::polkadot_primitives::v5::PvfCheckStatement {
        accept: false,
        subject: runtime_types::polkadot_parachain::primitives::ValidationCodeHash(
            ::subxt::utils::H256([8; 32usize]),
        ),
        session_index: 32,
        validator_index: runtime_types::polkadot_primitives::v5::ValidatorIndex(32),
    };
    let signature: runtime_types::polkadot_primitives::v5::validator_app::Signature = runtime_types::polkadot_primitives::v5::validator_app::Signature(
        runtime_types::sp_core::sr25519::Signature([8; 64usize]),
    );
    let payload = polkadot::tx().paras().include_pvf_check_statement(stmt, signature);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let up_to: ::core::primitive::u32 = 32;
    let payload = polkadot::tx().initializer().force_approve(up_to);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let recipient: runtime_types::polkadot_parachain::primitives::Id = runtime_types::polkadot_parachain::primitives::Id(
        32,
    );
    let proposed_max_capacity: ::core::primitive::u32 = 32;
    let proposed_max_message_size: ::core::primitive::u32 = 32;
    let payload = polkadot::tx()
        .hrmp()
        .hrmp_init_open_channel(
            recipient,
            proposed_max_capacity,
            proposed_max_message_size,
        );
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let sender: runtime_types::polkadot_parachain::primitives::Id = runtime_types::polkadot_parachain::primitives::Id(
        32,
    );
    let payload = polkadot::tx().hrmp().hrmp_accept_open_channel(sender);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let channel_id: runtime_types::polkadot_parachain::primitives::HrmpChannelId = runtime_types::polkadot_parachain::primitives::HrmpChannelId {
        sender: runtime_types::polkadot_parachain::primitives::Id(32),
        recipient: runtime_types::polkadot_parachain::primitives::Id(32),
    };
    let payload = polkadot::tx().hrmp().hrmp_close_channel(channel_id);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let para: runtime_types::polkadot_parachain::primitives::Id = runtime_types::polkadot_parachain::primitives::Id(
        32,
    );
    let inbound: ::core::primitive::u32 = 32;
    let outbound: ::core::primitive::u32 = 32;
    let payload = polkadot::tx().hrmp().force_clean_hrmp(para, inbound, outbound);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let channels: ::core::primitive::u32 = 32;
    let payload = polkadot::tx().hrmp().force_process_hrmp_open(channels);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let channels: ::core::primitive::u32 = 32;
    let payload = polkadot::tx().hrmp().force_process_hrmp_close(channels);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let channel_id: runtime_types::polkadot_parachain::primitives::HrmpChannelId = runtime_types::polkadot_parachain::primitives::HrmpChannelId {
        sender: runtime_types::polkadot_parachain::primitives::Id(32),
        recipient: runtime_types::polkadot_parachain::primitives::Id(32),
    };
    let open_requests: ::core::primitive::u32 = 32;
    let payload = polkadot::tx()
        .hrmp()
        .hrmp_cancel_open_request(channel_id, open_requests);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let sender: runtime_types::polkadot_parachain::primitives::Id = runtime_types::polkadot_parachain::primitives::Id(
        32,
    );
    let recipient: runtime_types::polkadot_parachain::primitives::Id = runtime_types::polkadot_parachain::primitives::Id(
        32,
    );
    let max_capacity: ::core::primitive::u32 = 32;
    let max_message_size: ::core::primitive::u32 = 32;
    let payload = polkadot::tx()
        .hrmp()
        .force_open_hrmp_channel(sender, recipient, max_capacity, max_message_size);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let payload = polkadot::tx().paras_disputes().force_unfreeze();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let dispute_proof: runtime_types::polkadot_primitives::v5::slashing::DisputeProof = runtime_types::polkadot_primitives::v5::slashing::DisputeProof {
        time_slot: runtime_types::polkadot_primitives::v5::slashing::DisputesTimeSlot {
            session_index: 32,
            candidate_hash: runtime_types::polkadot_core_primitives::CandidateHash(
                ::subxt::utils::H256([8; 32usize]),
            ),
        },
        kind: runtime_types::polkadot_primitives::v5::slashing::SlashingOffenceKind::ForInvalid,
        validator_index: runtime_types::polkadot_primitives::v5::ValidatorIndex(32),
        validator_id: runtime_types::polkadot_primitives::v5::validator_app::Public(
            runtime_types::sp_core::sr25519::Public([8; 32usize]),
        ),
    };
    let key_owner_proof: runtime_types::sp_session::MembershipProof = runtime_types::sp_session::MembershipProof {
        session: 32,
        trie_nodes: vec![vec![8, 8], vec![8, 8]],
        validator_count: 32,
    };
    let payload = polkadot::tx()
        .paras_slashing()
        .report_dispute_lost_unsigned(dispute_proof, key_owner_proof);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let id: runtime_types::polkadot_parachain::primitives::Id = runtime_types::polkadot_parachain::primitives::Id(
        32,
    );
    let genesis_head: runtime_types::polkadot_parachain::primitives::HeadData = runtime_types::polkadot_parachain::primitives::HeadData(
        vec![8, 8],
    );
    let validation_code: runtime_types::polkadot_parachain::primitives::ValidationCode = runtime_types::polkadot_parachain::primitives::ValidationCode(
        vec![8, 8],
    );
    let payload = polkadot::tx().registrar().register(id, genesis_head, validation_code);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let who: ::subxt::utils::AccountId32 = ::subxt::utils::AccountId32([8; 32usize]);
    let deposit: ::core::primitive::u128 = 128;
    let id: runtime_types::polkadot_parachain::primitives::Id = runtime_types::polkadot_parachain::primitives::Id(
        32,
    );
    let genesis_head: runtime_types::polkadot_parachain::primitives::HeadData = runtime_types::polkadot_parachain::primitives::HeadData(
        vec![8, 8],
    );
    let validation_code: runtime_types::polkadot_parachain::primitives::ValidationCode = runtime_types::polkadot_parachain::primitives::ValidationCode(
        vec![8, 8],
    );
    let payload = polkadot::tx()
        .registrar()
        .force_register(who, deposit, id, genesis_head, validation_code);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let id: runtime_types::polkadot_parachain::primitives::Id = runtime_types::polkadot_parachain::primitives::Id(
        32,
    );
    let payload = polkadot::tx().registrar().deregister(id);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let id: runtime_types::polkadot_parachain::primitives::Id = runtime_types::polkadot_parachain::primitives::Id(
        32,
    );
    let other: runtime_types::polkadot_parachain::primitives::Id = runtime_types::polkadot_parachain::primitives::Id(
        32,
    );
    let payload = polkadot::tx().registrar().swap(id, other);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let para: runtime_types::polkadot_parachain::primitives::Id = runtime_types::polkadot_parachain::primitives::Id(
        32,
    );
    let payload = polkadot::tx().registrar().remove_lock(para);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let payload = polkadot::tx().registrar().reserve();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let para: runtime_types::polkadot_parachain::primitives::Id = runtime_types::polkadot_parachain::primitives::Id(
        32,
    );
    let payload = polkadot::tx().registrar().add_lock(para);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let para: runtime_types::polkadot_parachain::primitives::Id = runtime_types::polkadot_parachain::primitives::Id(
        32,
    );
    let new_code: runtime_types::polkadot_parachain::primitives::ValidationCode = runtime_types::polkadot_parachain::primitives::ValidationCode(
        vec![8, 8],
    );
    let payload = polkadot::tx().registrar().schedule_code_upgrade(para, new_code);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let para: runtime_types::polkadot_parachain::primitives::Id = runtime_types::polkadot_parachain::primitives::Id(
        32,
    );
    let new_head: runtime_types::polkadot_parachain::primitives::HeadData = runtime_types::polkadot_parachain::primitives::HeadData(
        vec![8, 8],
    );
    let payload = polkadot::tx().registrar().set_current_head(para, new_head);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let para: runtime_types::polkadot_parachain::primitives::Id = runtime_types::polkadot_parachain::primitives::Id(
        32,
    );
    let leaser: ::subxt::utils::AccountId32 = ::subxt::utils::AccountId32([8; 32usize]);
    let amount: ::core::primitive::u128 = 128;
    let period_begin: ::core::primitive::u32 = 32;
    let period_count: ::core::primitive::u32 = 32;
    let payload = polkadot::tx()
        .slots()
        .force_lease(para, leaser, amount, period_begin, period_count);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let para: runtime_types::polkadot_parachain::primitives::Id = runtime_types::polkadot_parachain::primitives::Id(
        32,
    );
    let payload = polkadot::tx().slots().clear_all_leases(para);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let para: runtime_types::polkadot_parachain::primitives::Id = runtime_types::polkadot_parachain::primitives::Id(
        32,
    );
    let payload = polkadot::tx().slots().trigger_onboard(para);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let duration: ::core::primitive::u32 = 32;
    let lease_period_index: ::core::primitive::u32 = 32;
    let payload = polkadot::tx().auctions().new_auction(duration, lease_period_index);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let para: runtime_types::polkadot_parachain::primitives::Id = runtime_types::polkadot_parachain::primitives::Id(
        32,
    );
    let auction_index: ::core::primitive::u32 = 32;
    let first_slot: ::core::primitive::u32 = 32;
    let last_slot: ::core::primitive::u32 = 32;
    let amount: ::core::primitive::u128 = 128;
    let payload = polkadot::tx()
        .auctions()
        .bid(para, auction_index, first_slot, last_slot, amount);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let payload = polkadot::tx().auctions().cancel_auction();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let index: runtime_types::polkadot_parachain::primitives::Id = runtime_types::polkadot_parachain::primitives::Id(
        32,
    );
    let cap: ::core::primitive::u128 = 128;
    let first_period: ::core::primitive::u32 = 32;
    let last_period: ::core::primitive::u32 = 32;
    let end: ::core::primitive::u32 = 32;
    let verifier: ::core::option::Option<runtime_types::sp_runtime::MultiSigner> = ::core::option::Option::None;
    let payload = polkadot::tx()
        .crowdloan()
        .create(index, cap, first_period, last_period, end, verifier);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let index: runtime_types::polkadot_parachain::primitives::Id = runtime_types::polkadot_parachain::primitives::Id(
        32,
    );
    let value: ::core::primitive::u128 = 128;
    let signature: ::core::option::Option<runtime_types::sp_runtime::MultiSignature> = ::core::option::Option::None;
    let payload = polkadot::tx().crowdloan().contribute(index, value, signature);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let who: ::subxt::utils::AccountId32 = ::subxt::utils::AccountId32([8; 32usize]);
    let index: runtime_types::polkadot_parachain::primitives::Id = runtime_types::polkadot_parachain::primitives::Id(
        32,
    );
    let payload = polkadot::tx().crowdloan().withdraw(who, index);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let index: runtime_types::polkadot_parachain::primitives::Id = runtime_types::polkadot_parachain::primitives::Id(
        32,
    );
    let payload = polkadot::tx().crowdloan().refund(index);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let index: runtime_types::polkadot_parachain::primitives::Id = runtime_types::polkadot_parachain::primitives::Id(
        32,
    );
    let payload = polkadot::tx().crowdloan().dissolve(index);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let index: runtime_types::polkadot_parachain::primitives::Id = runtime_types::polkadot_parachain::primitives::Id(
        32,
    );
    let cap: ::core::primitive::u128 = 128;
    let first_period: ::core::primitive::u32 = 32;
    let last_period: ::core::primitive::u32 = 32;
    let end: ::core::primitive::u32 = 32;
    let verifier: ::core::option::Option<runtime_types::sp_runtime::MultiSigner> = ::core::option::Option::None;
    let payload = polkadot::tx()
        .crowdloan()
        .edit(index, cap, first_period, last_period, end, verifier);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let index: runtime_types::polkadot_parachain::primitives::Id = runtime_types::polkadot_parachain::primitives::Id(
        32,
    );
    let memo: ::std::vec::Vec<::core::primitive::u8> = vec![8, 8];
    let payload = polkadot::tx().crowdloan().add_memo(index, memo);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let index: runtime_types::polkadot_parachain::primitives::Id = runtime_types::polkadot_parachain::primitives::Id(
        32,
    );
    let payload = polkadot::tx().crowdloan().poke(index);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let index: runtime_types::polkadot_parachain::primitives::Id = runtime_types::polkadot_parachain::primitives::Id(
        32,
    );
    let signature: ::core::option::Option<runtime_types::sp_runtime::MultiSignature> = ::core::option::Option::None;
    let payload = polkadot::tx().crowdloan().contribute_all(index, signature);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let dest: runtime_types::xcm::VersionedMultiLocation = runtime_types::xcm::VersionedMultiLocation::V2(runtime_types::xcm::v2::multilocation::MultiLocation {
        parents: 8,
        interior: runtime_types::xcm::v2::multilocation::Junctions::Here,
    });
    let message: runtime_types::xcm::VersionedXcm = runtime_types::xcm::VersionedXcm::V2(
        runtime_types::xcm::v2::Xcm(
            vec![
                runtime_types::xcm::v2::Instruction::WithdrawAsset(runtime_types::xcm::v2::multiasset::MultiAssets(vec![runtime_types::xcm::v2::multiasset::MultiAsset
                { id :
                runtime_types::xcm::v2::multiasset::AssetId::Concrete(runtime_types::xcm::v2::multilocation::MultiLocation
                { parents : 8, interior :
                runtime_types::xcm::v2::multilocation::Junctions::Here, },), fun :
                runtime_types::xcm::v2::multiasset::Fungibility::Fungible(128,), },
                runtime_types::xcm::v2::multiasset::MultiAsset { id :
                runtime_types::xcm::v2::multiasset::AssetId::Concrete(runtime_types::xcm::v2::multilocation::MultiLocation
                { parents : 8, interior :
                runtime_types::xcm::v2::multilocation::Junctions::Here, },), fun :
                runtime_types::xcm::v2::multiasset::Fungibility::Fungible(128,), }],),),
                runtime_types::xcm::v2::Instruction::WithdrawAsset(runtime_types::xcm::v2::multiasset::MultiAssets(vec![runtime_types::xcm::v2::multiasset::MultiAsset
                { id :
                runtime_types::xcm::v2::multiasset::AssetId::Concrete(runtime_types::xcm::v2::multilocation::MultiLocation
                { parents : 8, interior :
                runtime_types::xcm::v2::multilocation::Junctions::Here, },), fun :
                runtime_types::xcm::v2::multiasset::Fungibility::Fungible(128,), },
                runtime_types::xcm::v2::multiasset::MultiAsset { id :
                runtime_types::xcm::v2::multiasset::AssetId::Concrete(runtime_types::xcm::v2::multilocation::MultiLocation
                { parents : 8, interior :
                runtime_types::xcm::v2::multilocation::Junctions::Here, },), fun :
                runtime_types::xcm::v2::multiasset::Fungibility::Fungible(128,), }],),)
            ],
        ),
    );
    let payload = polkadot::tx().xcm_pallet().send(dest, message);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let dest: runtime_types::xcm::VersionedMultiLocation = runtime_types::xcm::VersionedMultiLocation::V2(runtime_types::xcm::v2::multilocation::MultiLocation {
        parents: 8,
        interior: runtime_types::xcm::v2::multilocation::Junctions::Here,
    });
    let beneficiary: runtime_types::xcm::VersionedMultiLocation = runtime_types::xcm::VersionedMultiLocation::V2(runtime_types::xcm::v2::multilocation::MultiLocation {
        parents: 8,
        interior: runtime_types::xcm::v2::multilocation::Junctions::Here,
    });
    let assets: runtime_types::xcm::VersionedMultiAssets = runtime_types::xcm::VersionedMultiAssets::V2(
        runtime_types::xcm::v2::multiasset::MultiAssets(
            vec![
                runtime_types::xcm::v2::multiasset::MultiAsset { id :
                runtime_types::xcm::v2::multiasset::AssetId::Concrete(runtime_types::xcm::v2::multilocation::MultiLocation
                { parents : 8, interior :
                runtime_types::xcm::v2::multilocation::Junctions::Here, },), fun :
                runtime_types::xcm::v2::multiasset::Fungibility::Fungible(128,), },
                runtime_types::xcm::v2::multiasset::MultiAsset { id :
                runtime_types::xcm::v2::multiasset::AssetId::Concrete(runtime_types::xcm::v2::multilocation::MultiLocation
                { parents : 8, interior :
                runtime_types::xcm::v2::multilocation::Junctions::Here, },), fun :
                runtime_types::xcm::v2::multiasset::Fungibility::Fungible(128,), }
            ],
        ),
    );
    let fee_asset_item: ::core::primitive::u32 = 32;
    let payload = polkadot::tx()
        .xcm_pallet()
        .teleport_assets(dest, beneficiary, assets, fee_asset_item);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let dest: runtime_types::xcm::VersionedMultiLocation = runtime_types::xcm::VersionedMultiLocation::V2(runtime_types::xcm::v2::multilocation::MultiLocation {
        parents: 8,
        interior: runtime_types::xcm::v2::multilocation::Junctions::Here,
    });
    let beneficiary: runtime_types::xcm::VersionedMultiLocation = runtime_types::xcm::VersionedMultiLocation::V2(runtime_types::xcm::v2::multilocation::MultiLocation {
        parents: 8,
        interior: runtime_types::xcm::v2::multilocation::Junctions::Here,
    });
    let assets: runtime_types::xcm::VersionedMultiAssets = runtime_types::xcm::VersionedMultiAssets::V2(
        runtime_types::xcm::v2::multiasset::MultiAssets(
            vec![
                runtime_types::xcm::v2::multiasset::MultiAsset { id :
                runtime_types::xcm::v2::multiasset::AssetId::Concrete(runtime_types::xcm::v2::multilocation::MultiLocation
                { parents : 8, interior :
                runtime_types::xcm::v2::multilocation::Junctions::Here, },), fun :
                runtime_types::xcm::v2::multiasset::Fungibility::Fungible(128,), },
                runtime_types::xcm::v2::multiasset::MultiAsset { id :
                runtime_types::xcm::v2::multiasset::AssetId::Concrete(runtime_types::xcm::v2::multilocation::MultiLocation
                { parents : 8, interior :
                runtime_types::xcm::v2::multilocation::Junctions::Here, },), fun :
                runtime_types::xcm::v2::multiasset::Fungibility::Fungible(128,), }
            ],
        ),
    );
    let fee_asset_item: ::core::primitive::u32 = 32;
    let payload = polkadot::tx()
        .xcm_pallet()
        .reserve_transfer_assets(dest, beneficiary, assets, fee_asset_item);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let message: runtime_types::xcm::VersionedXcm = runtime_types::xcm::VersionedXcm::V2(
        runtime_types::xcm::v2::Xcm(
            vec![
                runtime_types::xcm::v2::Instruction::WithdrawAsset(runtime_types::xcm::v2::multiasset::MultiAssets(vec![runtime_types::xcm::v2::multiasset::MultiAsset
                { id :
                runtime_types::xcm::v2::multiasset::AssetId::Concrete(runtime_types::xcm::v2::multilocation::MultiLocation
                { parents : 8, interior :
                runtime_types::xcm::v2::multilocation::Junctions::Here, },), fun :
                runtime_types::xcm::v2::multiasset::Fungibility::Fungible(128,), },
                runtime_types::xcm::v2::multiasset::MultiAsset { id :
                runtime_types::xcm::v2::multiasset::AssetId::Concrete(runtime_types::xcm::v2::multilocation::MultiLocation
                { parents : 8, interior :
                runtime_types::xcm::v2::multilocation::Junctions::Here, },), fun :
                runtime_types::xcm::v2::multiasset::Fungibility::Fungible(128,), }],),),
                runtime_types::xcm::v2::Instruction::WithdrawAsset(runtime_types::xcm::v2::multiasset::MultiAssets(vec![runtime_types::xcm::v2::multiasset::MultiAsset
                { id :
                runtime_types::xcm::v2::multiasset::AssetId::Concrete(runtime_types::xcm::v2::multilocation::MultiLocation
                { parents : 8, interior :
                runtime_types::xcm::v2::multilocation::Junctions::Here, },), fun :
                runtime_types::xcm::v2::multiasset::Fungibility::Fungible(128,), },
                runtime_types::xcm::v2::multiasset::MultiAsset { id :
                runtime_types::xcm::v2::multiasset::AssetId::Concrete(runtime_types::xcm::v2::multilocation::MultiLocation
                { parents : 8, interior :
                runtime_types::xcm::v2::multilocation::Junctions::Here, },), fun :
                runtime_types::xcm::v2::multiasset::Fungibility::Fungible(128,), }],),)
            ],
        ),
    );
    let max_weight: runtime_types::sp_weights::weight_v2::Weight = runtime_types::sp_weights::weight_v2::Weight {
        ref_time: 64,
        proof_size: 64,
    };
    let payload = polkadot::tx().xcm_pallet().execute(message, max_weight);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let location: runtime_types::xcm::v3::multilocation::MultiLocation = runtime_types::xcm::v3::multilocation::MultiLocation {
        parents: 8,
        interior: runtime_types::xcm::v3::junctions::Junctions::Here,
    };
    let version: ::core::primitive::u32 = 32;
    let payload = polkadot::tx().xcm_pallet().force_xcm_version(location, version);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let maybe_xcm_version: ::core::option::Option<::core::primitive::u32> = ::core::option::Option::None;
    let payload = polkadot::tx()
        .xcm_pallet()
        .force_default_xcm_version(maybe_xcm_version);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let location: runtime_types::xcm::VersionedMultiLocation = runtime_types::xcm::VersionedMultiLocation::V2(runtime_types::xcm::v2::multilocation::MultiLocation {
        parents: 8,
        interior: runtime_types::xcm::v2::multilocation::Junctions::Here,
    });
    let payload = polkadot::tx().xcm_pallet().force_subscribe_version_notify(location);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let location: runtime_types::xcm::VersionedMultiLocation = runtime_types::xcm::VersionedMultiLocation::V2(runtime_types::xcm::v2::multilocation::MultiLocation {
        parents: 8,
        interior: runtime_types::xcm::v2::multilocation::Junctions::Here,
    });
    let payload = polkadot::tx().xcm_pallet().force_unsubscribe_version_notify(location);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let dest: runtime_types::xcm::VersionedMultiLocation = runtime_types::xcm::VersionedMultiLocation::V2(runtime_types::xcm::v2::multilocation::MultiLocation {
        parents: 8,
        interior: runtime_types::xcm::v2::multilocation::Junctions::Here,
    });
    let beneficiary: runtime_types::xcm::VersionedMultiLocation = runtime_types::xcm::VersionedMultiLocation::V2(runtime_types::xcm::v2::multilocation::MultiLocation {
        parents: 8,
        interior: runtime_types::xcm::v2::multilocation::Junctions::Here,
    });
    let assets: runtime_types::xcm::VersionedMultiAssets = runtime_types::xcm::VersionedMultiAssets::V2(
        runtime_types::xcm::v2::multiasset::MultiAssets(
            vec![
                runtime_types::xcm::v2::multiasset::MultiAsset { id :
                runtime_types::xcm::v2::multiasset::AssetId::Concrete(runtime_types::xcm::v2::multilocation::MultiLocation
                { parents : 8, interior :
                runtime_types::xcm::v2::multilocation::Junctions::Here, },), fun :
                runtime_types::xcm::v2::multiasset::Fungibility::Fungible(128,), },
                runtime_types::xcm::v2::multiasset::MultiAsset { id :
                runtime_types::xcm::v2::multiasset::AssetId::Concrete(runtime_types::xcm::v2::multilocation::MultiLocation
                { parents : 8, interior :
                runtime_types::xcm::v2::multilocation::Junctions::Here, },), fun :
                runtime_types::xcm::v2::multiasset::Fungibility::Fungible(128,), }
            ],
        ),
    );
    let fee_asset_item: ::core::primitive::u32 = 32;
    let weight_limit: runtime_types::xcm::v3::WeightLimit = runtime_types::xcm::v3::WeightLimit::Unlimited;
    let payload = polkadot::tx()
        .xcm_pallet()
        .limited_reserve_transfer_assets(
            dest,
            beneficiary,
            assets,
            fee_asset_item,
            weight_limit,
        );
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let dest: runtime_types::xcm::VersionedMultiLocation = runtime_types::xcm::VersionedMultiLocation::V2(runtime_types::xcm::v2::multilocation::MultiLocation {
        parents: 8,
        interior: runtime_types::xcm::v2::multilocation::Junctions::Here,
    });
    let beneficiary: runtime_types::xcm::VersionedMultiLocation = runtime_types::xcm::VersionedMultiLocation::V2(runtime_types::xcm::v2::multilocation::MultiLocation {
        parents: 8,
        interior: runtime_types::xcm::v2::multilocation::Junctions::Here,
    });
    let assets: runtime_types::xcm::VersionedMultiAssets = runtime_types::xcm::VersionedMultiAssets::V2(
        runtime_types::xcm::v2::multiasset::MultiAssets(
            vec![
                runtime_types::xcm::v2::multiasset::MultiAsset { id :
                runtime_types::xcm::v2::multiasset::AssetId::Concrete(runtime_types::xcm::v2::multilocation::MultiLocation
                { parents : 8, interior :
                runtime_types::xcm::v2::multilocation::Junctions::Here, },), fun :
                runtime_types::xcm::v2::multiasset::Fungibility::Fungible(128,), },
                runtime_types::xcm::v2::multiasset::MultiAsset { id :
                runtime_types::xcm::v2::multiasset::AssetId::Concrete(runtime_types::xcm::v2::multilocation::MultiLocation
                { parents : 8, interior :
                runtime_types::xcm::v2::multilocation::Junctions::Here, },), fun :
                runtime_types::xcm::v2::multiasset::Fungibility::Fungible(128,), }
            ],
        ),
    );
    let fee_asset_item: ::core::primitive::u32 = 32;
    let weight_limit: runtime_types::xcm::v3::WeightLimit = runtime_types::xcm::v3::WeightLimit::Unlimited;
    let payload = polkadot::tx()
        .xcm_pallet()
        .limited_teleport_assets(
            dest,
            beneficiary,
            assets,
            fee_asset_item,
            weight_limit,
        );
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let suspended: ::core::primitive::bool = false;
    let payload = polkadot::tx().xcm_pallet().force_suspension(suspended);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let message_origin: runtime_types::polkadot_runtime_parachains::inclusion::AggregateMessageOrigin = runtime_types::polkadot_runtime_parachains::inclusion::AggregateMessageOrigin::Ump(
        runtime_types::polkadot_runtime_parachains::inclusion::UmpQueueId::Para(
            runtime_types::polkadot_parachain::primitives::Id(32),
        ),
    );
    let page_index: ::core::primitive::u32 = 32;
    let payload = polkadot::tx().message_queue().reap_page(message_origin, page_index);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let from = dev::alice();
    let events = api
        .tx()
        .sign_and_submit_then_watch_default(&payload, &from)
        .await?
        .wait_for_finalized_success()
        .await?;
    let message_origin: runtime_types::polkadot_runtime_parachains::inclusion::AggregateMessageOrigin = runtime_types::polkadot_runtime_parachains::inclusion::AggregateMessageOrigin::Ump(
        runtime_types::polkadot_runtime_parachains::inclusion::UmpQueueId::Para(
            runtime_types::polkadot_parachain::primitives::Id(32),
        ),
    );
    let page: ::core::primitive::u32 = 32;
    let index: ::core::primitive::u32 = 32;
    let weight_limit: runtime_types::sp_weights::weight_v2::Weight = runtime_types::sp_weights::weight_v2::Weight {
        ref_time: 64,
        proof_size: 64,
    };
    let payload = polkadot::tx()
        .message_queue()
        .execute_overweight(message_origin, page, index, weight_limit);
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
