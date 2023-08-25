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
    let message: runtime_types::xcm::VersionedXcm2 = runtime_types::xcm::VersionedXcm2::V2(
        runtime_types::xcm::v2::Xcm2(
            vec![
                runtime_types::xcm::v2::Instruction2::WithdrawAsset(runtime_types::xcm::v2::multiasset::MultiAssets(vec![runtime_types::xcm::v2::multiasset::MultiAsset
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
                runtime_types::xcm::v2::Instruction2::WithdrawAsset(runtime_types::xcm::v2::multiasset::MultiAssets(vec![runtime_types::xcm::v2::multiasset::MultiAsset
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
    let key_0: ::subxt::utils::AccountId32 = ::subxt::utils::AccountId32([8; 32usize]);
    let storage_query = polkadot::storage().system().account(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::frame_system::AccountInfo<
            ::core::primitive::u32,
            runtime_types::pallet_balances::types::AccountData<::core::primitive::u128>,
        >,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let storage_query = polkadot::storage().system().extrinsic_count();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().system().block_weight();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::frame_support::dispatch::PerDispatchClass<
            runtime_types::sp_weights::weight_v2::Weight,
        >,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let storage_query = polkadot::storage().system().all_extrinsics_len();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: ::core::primitive::u32 = 32;
    let storage_query = polkadot::storage().system().block_hash(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::subxt::utils::H256> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: ::core::primitive::u32 = 32;
    let storage_query = polkadot::storage().system().extrinsic_data(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::std::vec::Vec<::core::primitive::u8>> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().system().number();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().system().parent_hash();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::subxt::utils::H256> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().system().digest();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<runtime_types::sp_runtime::generic::digest::Digest> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().system().events();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        ::std::vec::Vec<
            runtime_types::frame_system::EventRecord<
                runtime_types::polkadot_runtime::RuntimeEvent,
                ::subxt::utils::H256,
            >,
        >,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let storage_query = polkadot::storage().system().event_count();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: ::subxt::utils::H256 = ::subxt::utils::H256([8; 32usize]);
    let storage_query = polkadot::storage().system().event_topics(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        ::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32)>,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let storage_query = polkadot::storage().system().last_runtime_upgrade();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<runtime_types::frame_system::LastRuntimeUpgradeInfo> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().system().upgraded_to_u32_ref_count();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::bool> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().system().upgraded_to_triple_ref_count();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::bool> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().system().execution_phase();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<runtime_types::frame_system::Phase> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().scheduler().incomplete_since();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: ::core::primitive::u32 = 32;
    let storage_query = polkadot::storage().scheduler().agenda(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::bounded_collections::bounded_vec::BoundedVec<
            ::core::option::Option<
                runtime_types::pallet_scheduler::Scheduled<
                    [::core::primitive::u8; 32usize],
                    runtime_types::frame_support::traits::preimages::Bounded<
                        runtime_types::polkadot_runtime::RuntimeCall,
                    >,
                    ::core::primitive::u32,
                    runtime_types::polkadot_runtime::OriginCaller,
                    ::subxt::utils::AccountId32,
                >,
            >,
        >,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let key_0: [::core::primitive::u8; 32usize] = [8; 32usize];
    let storage_query = polkadot::storage().scheduler().lookup(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<(::core::primitive::u32, ::core::primitive::u32)> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: ::subxt::utils::H256 = ::subxt::utils::H256([8; 32usize]);
    let storage_query = polkadot::storage().preimage().status_for(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::pallet_preimage::RequestStatus<
            ::subxt::utils::AccountId32,
            ::core::primitive::u128,
        >,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let key_0: ::subxt::utils::H256 = ::subxt::utils::H256([8; 32usize]);
    let key_1: ::core::primitive::u32 = 32;
    let storage_query = polkadot::storage().preimage().preimage_for(key_0, key_1);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::bounded_collections::bounded_vec::BoundedVec<
            ::core::primitive::u8,
        >,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let storage_query = polkadot::storage().babe().epoch_index();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u64> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().babe().authorities();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
            (runtime_types::sp_consensus_babe::app::Public, ::core::primitive::u64),
        >,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let storage_query = polkadot::storage().babe().genesis_slot();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<runtime_types::sp_consensus_slots::Slot> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().babe().current_slot();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<runtime_types::sp_consensus_slots::Slot> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().babe().randomness();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<[::core::primitive::u8; 32usize]> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().babe().pending_epoch_config_change();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::sp_consensus_babe::digests::NextConfigDescriptor,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let storage_query = polkadot::storage().babe().next_randomness();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<[::core::primitive::u8; 32usize]> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().babe().next_authorities();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
            (runtime_types::sp_consensus_babe::app::Public, ::core::primitive::u64),
        >,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let storage_query = polkadot::storage().babe().segment_index();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: ::core::primitive::u32 = 32;
    let storage_query = polkadot::storage().babe().under_construction(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::bounded_collections::bounded_vec::BoundedVec<
            [::core::primitive::u8; 32usize],
        >,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let storage_query = polkadot::storage().babe().initialized();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        ::core::option::Option<runtime_types::sp_consensus_babe::digests::PreDigest>,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let storage_query = polkadot::storage().babe().author_vrf_randomness();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::option::Option<[::core::primitive::u8; 32usize]>> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().babe().epoch_start();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<(::core::primitive::u32, ::core::primitive::u32)> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().babe().lateness();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().babe().epoch_config();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<runtime_types::sp_consensus_babe::BabeEpochConfiguration> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().babe().next_epoch_config();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<runtime_types::sp_consensus_babe::BabeEpochConfiguration> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().babe().skipped_epochs();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::bounded_collections::bounded_vec::BoundedVec<
            (::core::primitive::u64, ::core::primitive::u32),
        >,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let storage_query = polkadot::storage().timestamp().now();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u64> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().timestamp().did_update();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::bool> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: ::core::primitive::u32 = 32;
    let storage_query = polkadot::storage().indices().accounts(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        (::subxt::utils::AccountId32, ::core::primitive::u128, ::core::primitive::bool),
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let storage_query = polkadot::storage().balances().total_issuance();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u128> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().balances().inactive_issuance();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u128> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: ::subxt::utils::AccountId32 = ::subxt::utils::AccountId32([8; 32usize]);
    let storage_query = polkadot::storage().balances().account(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::pallet_balances::types::AccountData<::core::primitive::u128>,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let key_0: ::subxt::utils::AccountId32 = ::subxt::utils::AccountId32([8; 32usize]);
    let storage_query = polkadot::storage().balances().locks(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
            runtime_types::pallet_balances::types::BalanceLock<::core::primitive::u128>,
        >,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let key_0: ::subxt::utils::AccountId32 = ::subxt::utils::AccountId32([8; 32usize]);
    let storage_query = polkadot::storage().balances().reserves(key_0);
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
    let storage_query = polkadot::storage().balances().holds(key_0);
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
    let storage_query = polkadot::storage().balances().freezes(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::bounded_collections::bounded_vec::BoundedVec<
            runtime_types::pallet_balances::types::IdAmount<(), ::core::primitive::u128>,
        >,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let storage_query = polkadot::storage().transaction_payment().next_fee_multiplier();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<runtime_types::sp_arithmetic::fixed_point::FixedU128> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().transaction_payment().storage_version();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<runtime_types::pallet_transaction_payment::Releases> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().authorship().author();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::subxt::utils::AccountId32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().staking().validator_count();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().staking().minimum_validator_count();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().staking().invulnerables();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::std::vec::Vec<::subxt::utils::AccountId32>> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: ::subxt::utils::AccountId32 = ::subxt::utils::AccountId32([8; 32usize]);
    let storage_query = polkadot::storage().staking().bonded(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::subxt::utils::AccountId32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().staking().min_nominator_bond();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u128> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().staking().min_validator_bond();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u128> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().staking().minimum_active_stake();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u128> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().staking().min_commission();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<runtime_types::sp_arithmetic::per_things::Perbill> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: ::subxt::utils::AccountId32 = ::subxt::utils::AccountId32([8; 32usize]);
    let storage_query = polkadot::storage().staking().ledger(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<runtime_types::pallet_staking::StakingLedger> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: ::subxt::utils::AccountId32 = ::subxt::utils::AccountId32([8; 32usize]);
    let storage_query = polkadot::storage().staking().payee(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::pallet_staking::RewardDestination<::subxt::utils::AccountId32>,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let key_0: ::subxt::utils::AccountId32 = ::subxt::utils::AccountId32([8; 32usize]);
    let storage_query = polkadot::storage().staking().validators(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<runtime_types::pallet_staking::ValidatorPrefs> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().staking().counter_for_validators();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().staking().max_validators_count();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: ::subxt::utils::AccountId32 = ::subxt::utils::AccountId32([8; 32usize]);
    let storage_query = polkadot::storage().staking().nominators(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<runtime_types::pallet_staking::Nominations> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().staking().counter_for_nominators();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().staking().max_nominators_count();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().staking().current_era();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().staking().active_era();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<runtime_types::pallet_staking::ActiveEraInfo> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: ::core::primitive::u32 = 32;
    let storage_query = polkadot::storage().staking().eras_start_session_index(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: ::core::primitive::u32 = 32;
    let key_1: ::subxt::utils::AccountId32 = ::subxt::utils::AccountId32([8; 32usize]);
    let storage_query = polkadot::storage().staking().eras_stakers(key_0, key_1);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::pallet_staking::Exposure<
            ::subxt::utils::AccountId32,
            ::core::primitive::u128,
        >,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let key_0: ::core::primitive::u32 = 32;
    let key_1: ::subxt::utils::AccountId32 = ::subxt::utils::AccountId32([8; 32usize]);
    let storage_query = polkadot::storage().staking().eras_stakers_clipped(key_0, key_1);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::pallet_staking::Exposure<
            ::subxt::utils::AccountId32,
            ::core::primitive::u128,
        >,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let key_0: ::core::primitive::u32 = 32;
    let key_1: ::subxt::utils::AccountId32 = ::subxt::utils::AccountId32([8; 32usize]);
    let storage_query = polkadot::storage().staking().eras_validator_prefs(key_0, key_1);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<runtime_types::pallet_staking::ValidatorPrefs> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: ::core::primitive::u32 = 32;
    let storage_query = polkadot::storage().staking().eras_validator_reward(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u128> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: ::core::primitive::u32 = 32;
    let storage_query = polkadot::storage().staking().eras_reward_points(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::pallet_staking::EraRewardPoints<::subxt::utils::AccountId32>,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let key_0: ::core::primitive::u32 = 32;
    let storage_query = polkadot::storage().staking().eras_total_stake(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u128> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().staking().force_era();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<runtime_types::pallet_staking::Forcing> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().staking().slash_reward_fraction();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<runtime_types::sp_arithmetic::per_things::Perbill> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().staking().canceled_slash_payout();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u128> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: ::core::primitive::u32 = 32;
    let storage_query = polkadot::storage().staking().unapplied_slashes(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        ::std::vec::Vec<
            runtime_types::pallet_staking::UnappliedSlash<
                ::subxt::utils::AccountId32,
                ::core::primitive::u128,
            >,
        >,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let storage_query = polkadot::storage().staking().bonded_eras();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        ::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32)>,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let key_0: ::core::primitive::u32 = 32;
    let key_1: ::subxt::utils::AccountId32 = ::subxt::utils::AccountId32([8; 32usize]);
    let storage_query = polkadot::storage()
        .staking()
        .validator_slash_in_era(key_0, key_1);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        (runtime_types::sp_arithmetic::per_things::Perbill, ::core::primitive::u128),
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let key_0: ::core::primitive::u32 = 32;
    let key_1: ::subxt::utils::AccountId32 = ::subxt::utils::AccountId32([8; 32usize]);
    let storage_query = polkadot::storage()
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
    let storage_query = polkadot::storage().staking().slashing_spans(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<runtime_types::pallet_staking::slashing::SlashingSpans> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: ::subxt::utils::AccountId32 = ::subxt::utils::AccountId32([8; 32usize]);
    let key_1: ::core::primitive::u32 = 32;
    let storage_query = polkadot::storage().staking().span_slash(key_0, key_1);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::pallet_staking::slashing::SpanRecord<::core::primitive::u128>,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let storage_query = polkadot::storage().staking().current_planned_session();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().staking().offending_validators();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        ::std::vec::Vec<(::core::primitive::u32, ::core::primitive::bool)>,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let storage_query = polkadot::storage().staking().chill_threshold();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<runtime_types::sp_arithmetic::per_things::Percent> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: ::subxt::utils::H256 = ::subxt::utils::H256([8; 32usize]);
    let storage_query = polkadot::storage().offences().reports(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::sp_staking::offence::OffenceDetails<
            ::subxt::utils::AccountId32,
            (
                ::subxt::utils::AccountId32,
                runtime_types::pallet_staking::Exposure<
                    ::subxt::utils::AccountId32,
                    ::core::primitive::u128,
                >,
            ),
        >,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let key_0: [::core::primitive::u8; 16usize] = [8; 16usize];
    let key_1: ::std::vec::Vec<::core::primitive::u8> = vec![8, 8];
    let storage_query = polkadot::storage()
        .offences()
        .concurrent_reports_index(key_0, key_1);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::std::vec::Vec<::subxt::utils::H256>> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().session().validators();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::std::vec::Vec<::subxt::utils::AccountId32>> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().session().current_index();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().session().queued_changed();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::bool> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().session().queued_keys();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        ::std::vec::Vec<
            (::subxt::utils::AccountId32, runtime_types::polkadot_runtime::SessionKeys),
        >,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let storage_query = polkadot::storage().session().disabled_validators();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::std::vec::Vec<::core::primitive::u32>> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: ::subxt::utils::AccountId32 = ::subxt::utils::AccountId32([8; 32usize]);
    let storage_query = polkadot::storage().session().next_keys(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<runtime_types::polkadot_runtime::SessionKeys> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: runtime_types::sp_core::crypto::KeyTypeId = runtime_types::sp_core::crypto::KeyTypeId(
        [8; 4usize],
    );
    let key_1: ::std::vec::Vec<::core::primitive::u8> = vec![8, 8];
    let storage_query = polkadot::storage().session().key_owner(key_0, key_1);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::subxt::utils::AccountId32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().grandpa().state();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::pallet_grandpa::StoredState<::core::primitive::u32>,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let storage_query = polkadot::storage().grandpa().pending_change();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::pallet_grandpa::StoredPendingChange<::core::primitive::u32>,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let storage_query = polkadot::storage().grandpa().next_forced();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().grandpa().stalled();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<(::core::primitive::u32, ::core::primitive::u32)> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().grandpa().current_set_id();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u64> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: ::core::primitive::u64 = 64;
    let storage_query = polkadot::storage().grandpa().set_id_session(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().im_online().heartbeat_after();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().im_online().keys();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
            runtime_types::pallet_im_online::sr25519::app_sr25519::Public,
        >,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let key_0: ::core::primitive::u32 = 32;
    let key_1: ::core::primitive::u32 = 32;
    let storage_query = polkadot::storage()
        .im_online()
        .received_heartbeats(key_0, key_1);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::bool> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: ::core::primitive::u32 = 32;
    let key_1: ::subxt::utils::AccountId32 = ::subxt::utils::AccountId32([8; 32usize]);
    let storage_query = polkadot::storage().im_online().authored_blocks(key_0, key_1);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().democracy().public_prop_count();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().democracy().public_props();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::bounded_collections::bounded_vec::BoundedVec<
            (
                ::core::primitive::u32,
                runtime_types::frame_support::traits::preimages::Bounded<
                    runtime_types::polkadot_runtime::RuntimeCall,
                >,
                ::subxt::utils::AccountId32,
            ),
        >,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let key_0: ::core::primitive::u32 = 32;
    let storage_query = polkadot::storage().democracy().deposit_of(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        (
            runtime_types::bounded_collections::bounded_vec::BoundedVec<
                ::subxt::utils::AccountId32,
            >,
            ::core::primitive::u128,
        ),
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let storage_query = polkadot::storage().democracy().referendum_count();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().democracy().lowest_unbaked();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: ::core::primitive::u32 = 32;
    let storage_query = polkadot::storage().democracy().referendum_info_of(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::pallet_democracy::types::ReferendumInfo<
            ::core::primitive::u32,
            runtime_types::frame_support::traits::preimages::Bounded<
                runtime_types::polkadot_runtime::RuntimeCall,
            >,
            ::core::primitive::u128,
        >,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let key_0: ::subxt::utils::AccountId32 = ::subxt::utils::AccountId32([8; 32usize]);
    let storage_query = polkadot::storage().democracy().voting_of(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::pallet_democracy::vote::Voting<
            ::core::primitive::u128,
            ::subxt::utils::AccountId32,
            ::core::primitive::u32,
        >,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let storage_query = polkadot::storage().democracy().last_tabled_was_external();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::bool> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().democracy().next_external();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        (
            runtime_types::frame_support::traits::preimages::Bounded<
                runtime_types::polkadot_runtime::RuntimeCall,
            >,
            runtime_types::pallet_democracy::vote_threshold::VoteThreshold,
        ),
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let key_0: ::subxt::utils::H256 = ::subxt::utils::H256([8; 32usize]);
    let storage_query = polkadot::storage().democracy().blacklist(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        (
            ::core::primitive::u32,
            runtime_types::bounded_collections::bounded_vec::BoundedVec<
                ::subxt::utils::AccountId32,
            >,
        ),
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let key_0: ::subxt::utils::H256 = ::subxt::utils::H256([8; 32usize]);
    let storage_query = polkadot::storage().democracy().cancellations(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::bool> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: runtime_types::pallet_democracy::types::MetadataOwner = runtime_types::pallet_democracy::types::MetadataOwner::External;
    let storage_query = polkadot::storage().democracy().metadata_of(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::subxt::utils::H256> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().council().proposals();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::bounded_collections::bounded_vec::BoundedVec<::subxt::utils::H256>,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let key_0: ::subxt::utils::H256 = ::subxt::utils::H256([8; 32usize]);
    let storage_query = polkadot::storage().council().proposal_of(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<runtime_types::polkadot_runtime::RuntimeCall> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: ::subxt::utils::H256 = ::subxt::utils::H256([8; 32usize]);
    let storage_query = polkadot::storage().council().voting(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::pallet_collective::Votes<
            ::subxt::utils::AccountId32,
            ::core::primitive::u32,
        >,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let storage_query = polkadot::storage().council().proposal_count();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().council().members();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::std::vec::Vec<::subxt::utils::AccountId32>> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().council().prime();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::subxt::utils::AccountId32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().technical_committee().proposals();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::bounded_collections::bounded_vec::BoundedVec<::subxt::utils::H256>,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let key_0: ::subxt::utils::H256 = ::subxt::utils::H256([8; 32usize]);
    let storage_query = polkadot::storage().technical_committee().proposal_of(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<runtime_types::polkadot_runtime::RuntimeCall> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: ::subxt::utils::H256 = ::subxt::utils::H256([8; 32usize]);
    let storage_query = polkadot::storage().technical_committee().voting(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::pallet_collective::Votes<
            ::subxt::utils::AccountId32,
            ::core::primitive::u32,
        >,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let storage_query = polkadot::storage().technical_committee().proposal_count();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().technical_committee().members();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::std::vec::Vec<::subxt::utils::AccountId32>> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().technical_committee().prime();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::subxt::utils::AccountId32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().phragmen_election().members();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        ::std::vec::Vec<
            runtime_types::pallet_elections_phragmen::SeatHolder<
                ::subxt::utils::AccountId32,
                ::core::primitive::u128,
            >,
        >,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let storage_query = polkadot::storage().phragmen_election().runners_up();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        ::std::vec::Vec<
            runtime_types::pallet_elections_phragmen::SeatHolder<
                ::subxt::utils::AccountId32,
                ::core::primitive::u128,
            >,
        >,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let storage_query = polkadot::storage().phragmen_election().candidates();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        ::std::vec::Vec<(::subxt::utils::AccountId32, ::core::primitive::u128)>,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let storage_query = polkadot::storage().phragmen_election().election_rounds();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: ::subxt::utils::AccountId32 = ::subxt::utils::AccountId32([8; 32usize]);
    let storage_query = polkadot::storage().phragmen_election().voting(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::pallet_elections_phragmen::Voter<
            ::subxt::utils::AccountId32,
            ::core::primitive::u128,
        >,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let storage_query = polkadot::storage().technical_membership().members();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::bounded_collections::bounded_vec::BoundedVec<
            ::subxt::utils::AccountId32,
        >,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let storage_query = polkadot::storage().technical_membership().prime();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::subxt::utils::AccountId32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().treasury().proposal_count();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: ::core::primitive::u32 = 32;
    let storage_query = polkadot::storage().treasury().proposals(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::pallet_treasury::Proposal<
            ::subxt::utils::AccountId32,
            ::core::primitive::u128,
        >,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let storage_query = polkadot::storage().treasury().deactivated();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u128> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().treasury().approvals();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::bounded_collections::bounded_vec::BoundedVec<
            ::core::primitive::u32,
        >,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let key_0: ::subxt::utils::AccountId32 = ::subxt::utils::AccountId32([8; 32usize]);
    let key_1: ::core::primitive::u16 = 16;
    let storage_query = polkadot::storage().conviction_voting().voting_for(key_0, key_1);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::pallet_conviction_voting::vote::Voting<
            ::core::primitive::u128,
            ::subxt::utils::AccountId32,
            ::core::primitive::u32,
            ::core::primitive::u32,
        >,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let key_0: ::subxt::utils::AccountId32 = ::subxt::utils::AccountId32([8; 32usize]);
    let storage_query = polkadot::storage().conviction_voting().class_locks_for(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::bounded_collections::bounded_vec::BoundedVec<
            (::core::primitive::u16, ::core::primitive::u128),
        >,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let storage_query = polkadot::storage().referenda().referendum_count();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: ::core::primitive::u32 = 32;
    let storage_query = polkadot::storage().referenda().referendum_info_for(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::pallet_referenda::types::ReferendumInfo<
            ::core::primitive::u16,
            runtime_types::polkadot_runtime::OriginCaller,
            ::core::primitive::u32,
            runtime_types::frame_support::traits::preimages::Bounded<
                runtime_types::polkadot_runtime::RuntimeCall,
            >,
            ::core::primitive::u128,
            runtime_types::pallet_conviction_voting::types::Tally<
                ::core::primitive::u128,
            >,
            ::subxt::utils::AccountId32,
            (::core::primitive::u32, ::core::primitive::u32),
        >,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let key_0: ::core::primitive::u16 = 16;
    let storage_query = polkadot::storage().referenda().track_queue(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::bounded_collections::bounded_vec::BoundedVec<
            (::core::primitive::u32, ::core::primitive::u128),
        >,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let key_0: ::core::primitive::u16 = 16;
    let storage_query = polkadot::storage().referenda().deciding_count(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: ::core::primitive::u32 = 32;
    let storage_query = polkadot::storage().referenda().metadata_of(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::subxt::utils::H256> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: ::subxt::utils::H256 = ::subxt::utils::H256([8; 32usize]);
    let storage_query = polkadot::storage().whitelist().whitelisted_call(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<()> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: runtime_types::polkadot_runtime_common::claims::EthereumAddress = runtime_types::polkadot_runtime_common::claims::EthereumAddress(
        [8; 20usize],
    );
    let storage_query = polkadot::storage().claims().claims(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u128> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().claims().total();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u128> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: runtime_types::polkadot_runtime_common::claims::EthereumAddress = runtime_types::polkadot_runtime_common::claims::EthereumAddress(
        [8; 20usize],
    );
    let storage_query = polkadot::storage().claims().vesting(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        (::core::primitive::u128, ::core::primitive::u128, ::core::primitive::u32),
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let key_0: runtime_types::polkadot_runtime_common::claims::EthereumAddress = runtime_types::polkadot_runtime_common::claims::EthereumAddress(
        [8; 20usize],
    );
    let storage_query = polkadot::storage().claims().signing(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<runtime_types::polkadot_runtime_common::claims::StatementKind> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: ::subxt::utils::AccountId32 = ::subxt::utils::AccountId32([8; 32usize]);
    let storage_query = polkadot::storage().claims().preclaims(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::polkadot_runtime_common::claims::EthereumAddress,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let key_0: ::subxt::utils::AccountId32 = ::subxt::utils::AccountId32([8; 32usize]);
    let storage_query = polkadot::storage().vesting().vesting(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::bounded_collections::bounded_vec::BoundedVec<
            runtime_types::pallet_vesting::vesting_info::VestingInfo<
                ::core::primitive::u128,
                ::core::primitive::u32,
            >,
        >,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let storage_query = polkadot::storage().vesting().storage_version();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<runtime_types::pallet_vesting::Releases> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: ::subxt::utils::AccountId32 = ::subxt::utils::AccountId32([8; 32usize]);
    let storage_query = polkadot::storage().identity().identity_of(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::pallet_identity::types::Registration<::core::primitive::u128>,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let key_0: ::subxt::utils::AccountId32 = ::subxt::utils::AccountId32([8; 32usize]);
    let storage_query = polkadot::storage().identity().super_of(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        (::subxt::utils::AccountId32, runtime_types::pallet_identity::types::Data),
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let key_0: ::subxt::utils::AccountId32 = ::subxt::utils::AccountId32([8; 32usize]);
    let storage_query = polkadot::storage().identity().subs_of(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        (
            ::core::primitive::u128,
            runtime_types::bounded_collections::bounded_vec::BoundedVec<
                ::subxt::utils::AccountId32,
            >,
        ),
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let storage_query = polkadot::storage().identity().registrars();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::bounded_collections::bounded_vec::BoundedVec<
            ::core::option::Option<
                runtime_types::pallet_identity::types::RegistrarInfo<
                    ::core::primitive::u128,
                    ::subxt::utils::AccountId32,
                >,
            >,
        >,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let key_0: ::subxt::utils::AccountId32 = ::subxt::utils::AccountId32([8; 32usize]);
    let storage_query = polkadot::storage().proxy().proxies(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        (
            runtime_types::bounded_collections::bounded_vec::BoundedVec<
                runtime_types::pallet_proxy::ProxyDefinition<
                    ::subxt::utils::AccountId32,
                    runtime_types::polkadot_runtime::ProxyType,
                    ::core::primitive::u32,
                >,
            >,
            ::core::primitive::u128,
        ),
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let key_0: ::subxt::utils::AccountId32 = ::subxt::utils::AccountId32([8; 32usize]);
    let storage_query = polkadot::storage().proxy().announcements(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        (
            runtime_types::bounded_collections::bounded_vec::BoundedVec<
                runtime_types::pallet_proxy::Announcement<
                    ::subxt::utils::AccountId32,
                    ::subxt::utils::H256,
                    ::core::primitive::u32,
                >,
            >,
            ::core::primitive::u128,
        ),
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let key_0: ::subxt::utils::AccountId32 = ::subxt::utils::AccountId32([8; 32usize]);
    let key_1: [::core::primitive::u8; 32usize] = [8; 32usize];
    let storage_query = polkadot::storage().multisig().multisigs(key_0, key_1);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::pallet_multisig::Multisig<
            ::core::primitive::u32,
            ::core::primitive::u128,
            ::subxt::utils::AccountId32,
        >,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let storage_query = polkadot::storage().bounties().bounty_count();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: ::core::primitive::u32 = 32;
    let storage_query = polkadot::storage().bounties().bounties(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::pallet_bounties::Bounty<
            ::subxt::utils::AccountId32,
            ::core::primitive::u128,
            ::core::primitive::u32,
        >,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let key_0: ::core::primitive::u32 = 32;
    let storage_query = polkadot::storage().bounties().bounty_descriptions(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::bounded_collections::bounded_vec::BoundedVec<
            ::core::primitive::u8,
        >,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let storage_query = polkadot::storage().bounties().bounty_approvals();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::bounded_collections::bounded_vec::BoundedVec<
            ::core::primitive::u32,
        >,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let storage_query = polkadot::storage().child_bounties().child_bounty_count();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: ::core::primitive::u32 = 32;
    let storage_query = polkadot::storage()
        .child_bounties()
        .parent_child_bounties(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: ::core::primitive::u32 = 32;
    let key_1: ::core::primitive::u32 = 32;
    let storage_query = polkadot::storage()
        .child_bounties()
        .child_bounties(key_0, key_1);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::pallet_child_bounties::ChildBounty<
            ::subxt::utils::AccountId32,
            ::core::primitive::u128,
            ::core::primitive::u32,
        >,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let key_0: ::core::primitive::u32 = 32;
    let storage_query = polkadot::storage()
        .child_bounties()
        .child_bounty_descriptions(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::bounded_collections::bounded_vec::BoundedVec<
            ::core::primitive::u8,
        >,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let key_0: ::core::primitive::u32 = 32;
    let storage_query = polkadot::storage()
        .child_bounties()
        .children_curator_fees(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u128> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: ::subxt::utils::H256 = ::subxt::utils::H256([8; 32usize]);
    let storage_query = polkadot::storage().tips().tips(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::pallet_tips::OpenTip<
            ::subxt::utils::AccountId32,
            ::core::primitive::u128,
            ::core::primitive::u32,
            ::subxt::utils::H256,
        >,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let key_0: ::subxt::utils::H256 = ::subxt::utils::H256([8; 32usize]);
    let storage_query = polkadot::storage().tips().reasons(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::std::vec::Vec<::core::primitive::u8>> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().election_provider_multi_phase().round();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage()
        .election_provider_multi_phase()
        .current_phase();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::pallet_election_provider_multi_phase::Phase<
            ::core::primitive::u32,
        >,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let storage_query = polkadot::storage()
        .election_provider_multi_phase()
        .queued_solution();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::pallet_election_provider_multi_phase::ReadySolution,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let storage_query = polkadot::storage().election_provider_multi_phase().snapshot();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::pallet_election_provider_multi_phase::RoundSnapshot<
            ::subxt::utils::AccountId32,
            (
                ::subxt::utils::AccountId32,
                ::core::primitive::u64,
                runtime_types::bounded_collections::bounded_vec::BoundedVec<
                    ::subxt::utils::AccountId32,
                >,
            ),
        >,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let storage_query = polkadot::storage()
        .election_provider_multi_phase()
        .desired_targets();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage()
        .election_provider_multi_phase()
        .snapshot_metadata();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::pallet_election_provider_multi_phase::SolutionOrSnapshotSize,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let storage_query = polkadot::storage()
        .election_provider_multi_phase()
        .signed_submission_next_index();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage()
        .election_provider_multi_phase()
        .signed_submission_indices();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::bounded_collections::bounded_vec::BoundedVec<
            (
                runtime_types::sp_npos_elections::ElectionScore,
                ::core::primitive::u32,
                ::core::primitive::u32,
            ),
        >,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let key_0: ::core::primitive::u32 = 32;
    let storage_query = polkadot::storage()
        .election_provider_multi_phase()
        .signed_submissions_map(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::pallet_election_provider_multi_phase::signed::SignedSubmission<
            ::subxt::utils::AccountId32,
            ::core::primitive::u128,
            runtime_types::polkadot_runtime::NposCompactSolution16,
        >,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let storage_query = polkadot::storage()
        .election_provider_multi_phase()
        .minimum_untrusted_score();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<runtime_types::sp_npos_elections::ElectionScore> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: ::subxt::utils::AccountId32 = ::subxt::utils::AccountId32([8; 32usize]);
    let storage_query = polkadot::storage().voter_list().list_nodes(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<runtime_types::pallet_bags_list::list::Node> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().voter_list().counter_for_list_nodes();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: ::core::primitive::u64 = 64;
    let storage_query = polkadot::storage().voter_list().list_bags(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<runtime_types::pallet_bags_list::list::Bag> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().nomination_pools().min_join_bond();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u128> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().nomination_pools().min_create_bond();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u128> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().nomination_pools().max_pools();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().nomination_pools().max_pool_members();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage()
        .nomination_pools()
        .max_pool_members_per_pool();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().nomination_pools().global_max_commission();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<runtime_types::sp_arithmetic::per_things::Perbill> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: ::subxt::utils::AccountId32 = ::subxt::utils::AccountId32([8; 32usize]);
    let storage_query = polkadot::storage().nomination_pools().pool_members(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<runtime_types::pallet_nomination_pools::PoolMember> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage()
        .nomination_pools()
        .counter_for_pool_members();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: ::core::primitive::u32 = 32;
    let storage_query = polkadot::storage().nomination_pools().bonded_pools(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<runtime_types::pallet_nomination_pools::BondedPoolInner> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage()
        .nomination_pools()
        .counter_for_bonded_pools();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: ::core::primitive::u32 = 32;
    let storage_query = polkadot::storage().nomination_pools().reward_pools(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<runtime_types::pallet_nomination_pools::RewardPool> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage()
        .nomination_pools()
        .counter_for_reward_pools();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: ::core::primitive::u32 = 32;
    let storage_query = polkadot::storage().nomination_pools().sub_pools_storage(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<runtime_types::pallet_nomination_pools::SubPools> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage()
        .nomination_pools()
        .counter_for_sub_pools_storage();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: ::core::primitive::u32 = 32;
    let storage_query = polkadot::storage().nomination_pools().metadata(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::bounded_collections::bounded_vec::BoundedVec<
            ::core::primitive::u8,
        >,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let storage_query = polkadot::storage().nomination_pools().counter_for_metadata();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().nomination_pools().last_pool_id();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: ::subxt::utils::AccountId32 = ::subxt::utils::AccountId32([8; 32usize]);
    let storage_query = polkadot::storage()
        .nomination_pools()
        .reverse_pool_id_lookup(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage()
        .nomination_pools()
        .counter_for_reverse_pool_id_lookup();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: ::subxt::utils::AccountId32 = ::subxt::utils::AccountId32([8; 32usize]);
    let storage_query = polkadot::storage().nomination_pools().claim_permissions(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<runtime_types::pallet_nomination_pools::ClaimPermission> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().fast_unstake().head();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<runtime_types::pallet_fast_unstake::types::UnstakeRequest> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: ::subxt::utils::AccountId32 = ::subxt::utils::AccountId32([8; 32usize]);
    let storage_query = polkadot::storage().fast_unstake().queue(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u128> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().fast_unstake().counter_for_queue();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().fast_unstake().eras_to_check_per_block();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().configuration().active_config();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::polkadot_runtime_parachains::configuration::HostConfiguration<
            ::core::primitive::u32,
        >,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let storage_query = polkadot::storage().configuration().pending_configs();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        ::std::vec::Vec<
            (
                ::core::primitive::u32,
                runtime_types::polkadot_runtime_parachains::configuration::HostConfiguration<
                    ::core::primitive::u32,
                >,
            ),
        >,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let storage_query = polkadot::storage().configuration().bypass_consistency_check();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::bool> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().paras_shared().current_session_index();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().paras_shared().active_validator_indices();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        ::std::vec::Vec<runtime_types::polkadot_primitives::v5::ValidatorIndex>,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let storage_query = polkadot::storage().paras_shared().active_validator_keys();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        ::std::vec::Vec<runtime_types::polkadot_primitives::v5::validator_app::Public>,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let key_0: runtime_types::polkadot_primitives::v5::ValidatorIndex = runtime_types::polkadot_primitives::v5::ValidatorIndex(
        32,
    );
    let storage_query = polkadot::storage()
        .para_inclusion()
        .availability_bitfields(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::polkadot_runtime_parachains::inclusion::AvailabilityBitfieldRecord<
            ::core::primitive::u32,
        >,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let key_0: runtime_types::polkadot_parachain::primitives::Id = runtime_types::polkadot_parachain::primitives::Id(
        32,
    );
    let storage_query = polkadot::storage().para_inclusion().pending_availability(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::polkadot_runtime_parachains::inclusion::CandidatePendingAvailability<
            ::subxt::utils::H256,
            ::core::primitive::u32,
        >,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let key_0: runtime_types::polkadot_parachain::primitives::Id = runtime_types::polkadot_parachain::primitives::Id(
        32,
    );
    let storage_query = polkadot::storage()
        .para_inclusion()
        .pending_availability_commitments(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::polkadot_primitives::v5::CandidateCommitments<
            ::core::primitive::u32,
        >,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let storage_query = polkadot::storage().para_inherent().included();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<()> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().para_inherent().on_chain_votes();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::polkadot_primitives::v5::ScrapedOnChainVotes<::subxt::utils::H256>,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let storage_query = polkadot::storage().para_scheduler().validator_groups();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        ::std::vec::Vec<
            ::std::vec::Vec<runtime_types::polkadot_primitives::v5::ValidatorIndex>,
        >,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let storage_query = polkadot::storage().para_scheduler().parathread_queue();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::polkadot_runtime_parachains::scheduler::ParathreadClaimQueue,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let storage_query = polkadot::storage().para_scheduler().availability_cores();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        ::std::vec::Vec<
            ::core::option::Option<runtime_types::polkadot_primitives::v5::CoreOccupied>,
        >,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let storage_query = polkadot::storage().para_scheduler().parathread_claim_index();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        ::std::vec::Vec<runtime_types::polkadot_parachain::primitives::Id>,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let storage_query = polkadot::storage().para_scheduler().session_start_block();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().para_scheduler().scheduled();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        ::std::vec::Vec<
            runtime_types::polkadot_runtime_parachains::scheduler::CoreAssignment,
        >,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let key_0: runtime_types::polkadot_parachain::primitives::ValidationCodeHash = runtime_types::polkadot_parachain::primitives::ValidationCodeHash(
        ::subxt::utils::H256([8; 32usize]),
    );
    let storage_query = polkadot::storage().paras().pvf_active_vote_map(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::polkadot_runtime_parachains::paras::PvfCheckActiveVoteState<
            ::core::primitive::u32,
        >,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let storage_query = polkadot::storage().paras().pvf_active_vote_list();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        ::std::vec::Vec<
            runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
        >,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let storage_query = polkadot::storage().paras().parachains();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        ::std::vec::Vec<runtime_types::polkadot_parachain::primitives::Id>,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let key_0: runtime_types::polkadot_parachain::primitives::Id = runtime_types::polkadot_parachain::primitives::Id(
        32,
    );
    let storage_query = polkadot::storage().paras().para_lifecycles(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::polkadot_runtime_parachains::paras::ParaLifecycle,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let key_0: runtime_types::polkadot_parachain::primitives::Id = runtime_types::polkadot_parachain::primitives::Id(
        32,
    );
    let storage_query = polkadot::storage().paras().heads(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<runtime_types::polkadot_parachain::primitives::HeadData> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: runtime_types::polkadot_parachain::primitives::Id = runtime_types::polkadot_parachain::primitives::Id(
        32,
    );
    let storage_query = polkadot::storage().paras().current_code_hash(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let key_0: runtime_types::polkadot_parachain::primitives::Id = runtime_types::polkadot_parachain::primitives::Id(
        32,
    );
    let key_1: ::core::primitive::u32 = 32;
    let storage_query = polkadot::storage().paras().past_code_hash(key_0, key_1);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let key_0: runtime_types::polkadot_parachain::primitives::Id = runtime_types::polkadot_parachain::primitives::Id(
        32,
    );
    let storage_query = polkadot::storage().paras().past_code_meta(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::polkadot_runtime_parachains::paras::ParaPastCodeMeta<
            ::core::primitive::u32,
        >,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let storage_query = polkadot::storage().paras().past_code_pruning();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        ::std::vec::Vec<
            (runtime_types::polkadot_parachain::primitives::Id, ::core::primitive::u32),
        >,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let key_0: runtime_types::polkadot_parachain::primitives::Id = runtime_types::polkadot_parachain::primitives::Id(
        32,
    );
    let storage_query = polkadot::storage().paras().future_code_upgrades(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: runtime_types::polkadot_parachain::primitives::Id = runtime_types::polkadot_parachain::primitives::Id(
        32,
    );
    let storage_query = polkadot::storage().paras().future_code_hash(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let key_0: runtime_types::polkadot_parachain::primitives::Id = runtime_types::polkadot_parachain::primitives::Id(
        32,
    );
    let storage_query = polkadot::storage().paras().upgrade_go_ahead_signal(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<runtime_types::polkadot_primitives::v5::UpgradeGoAhead> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: runtime_types::polkadot_parachain::primitives::Id = runtime_types::polkadot_parachain::primitives::Id(
        32,
    );
    let storage_query = polkadot::storage().paras().upgrade_restriction_signal(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<runtime_types::polkadot_primitives::v5::UpgradeRestriction> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().paras().upgrade_cooldowns();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        ::std::vec::Vec<
            (runtime_types::polkadot_parachain::primitives::Id, ::core::primitive::u32),
        >,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let storage_query = polkadot::storage().paras().upcoming_upgrades();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        ::std::vec::Vec<
            (runtime_types::polkadot_parachain::primitives::Id, ::core::primitive::u32),
        >,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let key_0: ::core::primitive::u32 = 32;
    let storage_query = polkadot::storage().paras().actions_queue(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        ::std::vec::Vec<runtime_types::polkadot_parachain::primitives::Id>,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let key_0: runtime_types::polkadot_parachain::primitives::Id = runtime_types::polkadot_parachain::primitives::Id(
        32,
    );
    let storage_query = polkadot::storage().paras().upcoming_paras_genesis(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::polkadot_runtime_parachains::paras::ParaGenesisArgs,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let key_0: runtime_types::polkadot_parachain::primitives::ValidationCodeHash = runtime_types::polkadot_parachain::primitives::ValidationCodeHash(
        ::subxt::utils::H256([8; 32usize]),
    );
    let storage_query = polkadot::storage().paras().code_by_hash_refs(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: runtime_types::polkadot_parachain::primitives::ValidationCodeHash = runtime_types::polkadot_parachain::primitives::ValidationCodeHash(
        ::subxt::utils::H256([8; 32usize]),
    );
    let storage_query = polkadot::storage().paras().code_by_hash(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<runtime_types::polkadot_parachain::primitives::ValidationCode> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().initializer().has_initialized();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<()> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().initializer().buffered_session_changes();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        ::std::vec::Vec<
            runtime_types::polkadot_runtime_parachains::initializer::BufferedSessionChange,
        >,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let key_0: runtime_types::polkadot_parachain::primitives::Id = runtime_types::polkadot_parachain::primitives::Id(
        32,
    );
    let storage_query = polkadot::storage().dmp().downward_message_queues(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        ::std::vec::Vec<
            runtime_types::polkadot_core_primitives::InboundDownwardMessage<
                ::core::primitive::u32,
            >,
        >,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let key_0: runtime_types::polkadot_parachain::primitives::Id = runtime_types::polkadot_parachain::primitives::Id(
        32,
    );
    let storage_query = polkadot::storage().dmp().downward_message_queue_heads(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::subxt::utils::H256> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: runtime_types::polkadot_parachain::primitives::Id = runtime_types::polkadot_parachain::primitives::Id(
        32,
    );
    let storage_query = polkadot::storage().dmp().delivery_fee_factor(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<runtime_types::sp_arithmetic::fixed_point::FixedU128> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: runtime_types::polkadot_parachain::primitives::HrmpChannelId = runtime_types::polkadot_parachain::primitives::HrmpChannelId {
        sender: runtime_types::polkadot_parachain::primitives::Id(32),
        recipient: runtime_types::polkadot_parachain::primitives::Id(32),
    };
    let storage_query = polkadot::storage().hrmp().hrmp_open_channel_requests(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::polkadot_runtime_parachains::hrmp::HrmpOpenChannelRequest,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let storage_query = polkadot::storage().hrmp().hrmp_open_channel_requests_list();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        ::std::vec::Vec<runtime_types::polkadot_parachain::primitives::HrmpChannelId>,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let key_0: runtime_types::polkadot_parachain::primitives::Id = runtime_types::polkadot_parachain::primitives::Id(
        32,
    );
    let storage_query = polkadot::storage()
        .hrmp()
        .hrmp_open_channel_request_count(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: runtime_types::polkadot_parachain::primitives::Id = runtime_types::polkadot_parachain::primitives::Id(
        32,
    );
    let storage_query = polkadot::storage()
        .hrmp()
        .hrmp_accepted_channel_request_count(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: runtime_types::polkadot_parachain::primitives::HrmpChannelId = runtime_types::polkadot_parachain::primitives::HrmpChannelId {
        sender: runtime_types::polkadot_parachain::primitives::Id(32),
        recipient: runtime_types::polkadot_parachain::primitives::Id(32),
    };
    let storage_query = polkadot::storage().hrmp().hrmp_close_channel_requests(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<()> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().hrmp().hrmp_close_channel_requests_list();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        ::std::vec::Vec<runtime_types::polkadot_parachain::primitives::HrmpChannelId>,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let key_0: runtime_types::polkadot_parachain::primitives::Id = runtime_types::polkadot_parachain::primitives::Id(
        32,
    );
    let storage_query = polkadot::storage().hrmp().hrmp_watermarks(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: runtime_types::polkadot_parachain::primitives::HrmpChannelId = runtime_types::polkadot_parachain::primitives::HrmpChannelId {
        sender: runtime_types::polkadot_parachain::primitives::Id(32),
        recipient: runtime_types::polkadot_parachain::primitives::Id(32),
    };
    let storage_query = polkadot::storage().hrmp().hrmp_channels(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<runtime_types::polkadot_runtime_parachains::hrmp::HrmpChannel> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: runtime_types::polkadot_parachain::primitives::Id = runtime_types::polkadot_parachain::primitives::Id(
        32,
    );
    let storage_query = polkadot::storage().hrmp().hrmp_ingress_channels_index(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        ::std::vec::Vec<runtime_types::polkadot_parachain::primitives::Id>,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let key_0: runtime_types::polkadot_parachain::primitives::Id = runtime_types::polkadot_parachain::primitives::Id(
        32,
    );
    let storage_query = polkadot::storage().hrmp().hrmp_egress_channels_index(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        ::std::vec::Vec<runtime_types::polkadot_parachain::primitives::Id>,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let key_0: runtime_types::polkadot_parachain::primitives::HrmpChannelId = runtime_types::polkadot_parachain::primitives::HrmpChannelId {
        sender: runtime_types::polkadot_parachain::primitives::Id(32),
        recipient: runtime_types::polkadot_parachain::primitives::Id(32),
    };
    let storage_query = polkadot::storage().hrmp().hrmp_channel_contents(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        ::std::vec::Vec<
            runtime_types::polkadot_core_primitives::InboundHrmpMessage<
                ::core::primitive::u32,
            >,
        >,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let key_0: runtime_types::polkadot_parachain::primitives::Id = runtime_types::polkadot_parachain::primitives::Id(
        32,
    );
    let storage_query = polkadot::storage().hrmp().hrmp_channel_digests(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        ::std::vec::Vec<
            (
                ::core::primitive::u32,
                ::std::vec::Vec<runtime_types::polkadot_parachain::primitives::Id>,
            ),
        >,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let storage_query = polkadot::storage().para_session_info().assignment_keys_unsafe();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        ::std::vec::Vec<runtime_types::polkadot_primitives::v5::assignment_app::Public>,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let storage_query = polkadot::storage()
        .para_session_info()
        .earliest_stored_session();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: ::core::primitive::u32 = 32;
    let storage_query = polkadot::storage().para_session_info().sessions(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<runtime_types::polkadot_primitives::v5::SessionInfo> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: ::core::primitive::u32 = 32;
    let storage_query = polkadot::storage().para_session_info().account_keys(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::std::vec::Vec<::subxt::utils::AccountId32>> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: ::core::primitive::u32 = 32;
    let storage_query = polkadot::storage()
        .para_session_info()
        .session_executor_params(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::polkadot_primitives::v5::executor_params::ExecutorParams,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let storage_query = polkadot::storage().paras_disputes().last_pruned_session();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: ::core::primitive::u32 = 32;
    let key_1: runtime_types::polkadot_core_primitives::CandidateHash = runtime_types::polkadot_core_primitives::CandidateHash(
        ::subxt::utils::H256([8; 32usize]),
    );
    let storage_query = polkadot::storage().paras_disputes().disputes(key_0, key_1);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::polkadot_primitives::v5::DisputeState<::core::primitive::u32>,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let key_0: ::core::primitive::u32 = 32;
    let key_1: runtime_types::polkadot_core_primitives::CandidateHash = runtime_types::polkadot_core_primitives::CandidateHash(
        ::subxt::utils::H256([8; 32usize]),
    );
    let storage_query = polkadot::storage()
        .paras_disputes()
        .backers_on_disputes(key_0, key_1);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        ::std::vec::Vec<runtime_types::polkadot_primitives::v5::ValidatorIndex>,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let key_0: ::core::primitive::u32 = 32;
    let key_1: runtime_types::polkadot_core_primitives::CandidateHash = runtime_types::polkadot_core_primitives::CandidateHash(
        ::subxt::utils::H256([8; 32usize]),
    );
    let storage_query = polkadot::storage().paras_disputes().included(key_0, key_1);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().paras_disputes().frozen();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::option::Option<::core::primitive::u32>> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: ::core::primitive::u32 = 32;
    let key_1: runtime_types::polkadot_core_primitives::CandidateHash = runtime_types::polkadot_core_primitives::CandidateHash(
        ::subxt::utils::H256([8; 32usize]),
    );
    let storage_query = polkadot::storage()
        .paras_slashing()
        .unapplied_slashes(key_0, key_1);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::polkadot_primitives::v5::slashing::PendingSlashes,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let key_0: ::core::primitive::u32 = 32;
    let storage_query = polkadot::storage().paras_slashing().validator_set_counts(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: runtime_types::polkadot_parachain::primitives::Id = runtime_types::polkadot_parachain::primitives::Id(
        32,
    );
    let storage_query = polkadot::storage().registrar().pending_swap(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<runtime_types::polkadot_parachain::primitives::Id> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: runtime_types::polkadot_parachain::primitives::Id = runtime_types::polkadot_parachain::primitives::Id(
        32,
    );
    let storage_query = polkadot::storage().registrar().paras(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::polkadot_runtime_common::paras_registrar::ParaInfo<
            ::subxt::utils::AccountId32,
            ::core::primitive::u128,
        >,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let storage_query = polkadot::storage().registrar().next_free_para_id();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<runtime_types::polkadot_parachain::primitives::Id> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: runtime_types::polkadot_parachain::primitives::Id = runtime_types::polkadot_parachain::primitives::Id(
        32,
    );
    let storage_query = polkadot::storage().slots().leases(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        ::std::vec::Vec<
            ::core::option::Option<
                (::subxt::utils::AccountId32, ::core::primitive::u128),
            >,
        >,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let storage_query = polkadot::storage().auctions().auction_counter();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().auctions().auction_info();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<(::core::primitive::u32, ::core::primitive::u32)> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: ::subxt::utils::AccountId32 = ::subxt::utils::AccountId32([8; 32usize]);
    let key_1: runtime_types::polkadot_parachain::primitives::Id = runtime_types::polkadot_parachain::primitives::Id(
        32,
    );
    let storage_query = polkadot::storage().auctions().reserved_amounts(key_0, key_1);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u128> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: ::core::primitive::u32 = 32;
    let storage_query = polkadot::storage().auctions().winning(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        [::core::option::Option<
            (
                ::subxt::utils::AccountId32,
                runtime_types::polkadot_parachain::primitives::Id,
                ::core::primitive::u128,
            ),
        >; 36usize],
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let key_0: runtime_types::polkadot_parachain::primitives::Id = runtime_types::polkadot_parachain::primitives::Id(
        32,
    );
    let storage_query = polkadot::storage().crowdloan().funds(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::polkadot_runtime_common::crowdloan::FundInfo<
            ::subxt::utils::AccountId32,
            ::core::primitive::u128,
            ::core::primitive::u32,
            ::core::primitive::u32,
        >,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let storage_query = polkadot::storage().crowdloan().new_raise();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        ::std::vec::Vec<runtime_types::polkadot_parachain::primitives::Id>,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let storage_query = polkadot::storage().crowdloan().endings_count();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().crowdloan().next_fund_index();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().xcm_pallet().query_counter();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u64> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: ::core::primitive::u64 = 64;
    let storage_query = polkadot::storage().xcm_pallet().queries(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::pallet_xcm::pallet::QueryStatus<::core::primitive::u32>,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let key_0: ::subxt::utils::H256 = ::subxt::utils::H256([8; 32usize]);
    let storage_query = polkadot::storage().xcm_pallet().asset_traps(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let storage_query = polkadot::storage().xcm_pallet().safe_xcm_version();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: ::core::primitive::u32 = 32;
    let key_1: runtime_types::xcm::VersionedMultiLocation = runtime_types::xcm::VersionedMultiLocation::V2(runtime_types::xcm::v2::multilocation::MultiLocation {
        parents: 8,
        interior: runtime_types::xcm::v2::multilocation::Junctions::Here,
    });
    let storage_query = polkadot::storage().xcm_pallet().supported_version(key_0, key_1);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u32> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: ::core::primitive::u32 = 32;
    let key_1: runtime_types::xcm::VersionedMultiLocation = runtime_types::xcm::VersionedMultiLocation::V2(runtime_types::xcm::v2::multilocation::MultiLocation {
        parents: 8,
        interior: runtime_types::xcm::v2::multilocation::Junctions::Here,
    });
    let storage_query = polkadot::storage().xcm_pallet().version_notifiers(key_0, key_1);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::u64> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: ::core::primitive::u32 = 32;
    let key_1: runtime_types::xcm::VersionedMultiLocation = runtime_types::xcm::VersionedMultiLocation::V2(runtime_types::xcm::v2::multilocation::MultiLocation {
        parents: 8,
        interior: runtime_types::xcm::v2::multilocation::Junctions::Here,
    });
    let storage_query = polkadot::storage()
        .xcm_pallet()
        .version_notify_targets(key_0, key_1);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        (
            ::core::primitive::u64,
            runtime_types::sp_weights::weight_v2::Weight,
            ::core::primitive::u32,
        ),
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let storage_query = polkadot::storage().xcm_pallet().version_discovery_queue();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::bounded_collections::bounded_vec::BoundedVec<
            (runtime_types::xcm::VersionedMultiLocation, ::core::primitive::u32),
        >,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let storage_query = polkadot::storage().xcm_pallet().current_migration();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<runtime_types::pallet_xcm::pallet::VersionMigrationStage> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: ::core::primitive::u32 = 32;
    let key_1: ::subxt::utils::AccountId32 = ::subxt::utils::AccountId32([8; 32usize]);
    let key_2: runtime_types::xcm::VersionedAssetId = runtime_types::xcm::VersionedAssetId::V3(
        runtime_types::xcm::v3::multiasset::AssetId::Concrete(runtime_types::xcm::v3::multilocation::MultiLocation {
            parents: 8,
            interior: runtime_types::xcm::v3::junctions::Junctions::Here,
        }),
    );
    let storage_query = polkadot::storage()
        .xcm_pallet()
        .remote_locked_fungibles(key_0, key_1, key_2);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::pallet_xcm::pallet::RemoteLockedFungibleRecord<()>,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let key_0: ::subxt::utils::AccountId32 = ::subxt::utils::AccountId32([8; 32usize]);
    let storage_query = polkadot::storage().xcm_pallet().locked_fungibles(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::bounded_collections::bounded_vec::BoundedVec<
            (::core::primitive::u128, runtime_types::xcm::VersionedMultiLocation),
        >,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let storage_query = polkadot::storage().xcm_pallet().xcm_execution_suspended();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<::core::primitive::bool> = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;
    let key_0: runtime_types::polkadot_runtime_parachains::inclusion::AggregateMessageOrigin = runtime_types::polkadot_runtime_parachains::inclusion::AggregateMessageOrigin::Ump(
        runtime_types::polkadot_runtime_parachains::inclusion::UmpQueueId::Para(
            runtime_types::polkadot_parachain::primitives::Id(32),
        ),
    );
    let storage_query = polkadot::storage().message_queue().book_state_for(key_0);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::pallet_message_queue::BookState<
            runtime_types::polkadot_runtime_parachains::inclusion::AggregateMessageOrigin,
        >,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let storage_query = polkadot::storage().message_queue().service_head();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::polkadot_runtime_parachains::inclusion::AggregateMessageOrigin,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let key_0: runtime_types::polkadot_runtime_parachains::inclusion::AggregateMessageOrigin = runtime_types::polkadot_runtime_parachains::inclusion::AggregateMessageOrigin::Ump(
        runtime_types::polkadot_runtime_parachains::inclusion::UmpQueueId::Para(
            runtime_types::polkadot_parachain::primitives::Id(32),
        ),
    );
    let key_1: ::core::primitive::u32 = 32;
    let storage_query = polkadot::storage().message_queue().pages(key_0, key_1);
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let result: Option<
        runtime_types::pallet_message_queue::Page<::core::primitive::u32>,
    > = api.storage().at_latest().await?.fetch(&storage_query).await?;
    let constant_query = polkadot::constants().system().block_weights();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: runtime_types::frame_system::limits::BlockWeights = api
        .constants()
        .at(&constant_query)?;
    let constant_query = polkadot::constants().system().block_length();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: runtime_types::frame_system::limits::BlockLength = api
        .constants()
        .at(&constant_query)?;
    let constant_query = polkadot::constants().system().block_hash_count();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().system().db_weight();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: runtime_types::sp_weights::RuntimeDbWeight = api
        .constants()
        .at(&constant_query)?;
    let constant_query = polkadot::constants().system().version();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: runtime_types::sp_version::RuntimeVersion = api
        .constants()
        .at(&constant_query)?;
    let constant_query = polkadot::constants().system().ss58_prefix();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u16 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().scheduler().maximum_weight();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: runtime_types::sp_weights::weight_v2::Weight = api
        .constants()
        .at(&constant_query)?;
    let constant_query = polkadot::constants().scheduler().max_scheduled_per_block();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().babe().epoch_duration();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u64 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().babe().expected_block_time();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u64 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().babe().max_authorities();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().timestamp().minimum_period();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u64 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().indices().deposit();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u128 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().balances().existential_deposit();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u128 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().balances().max_locks();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().balances().max_reserves();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().balances().max_holds();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().balances().max_freezes();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants()
        .transaction_payment()
        .operational_fee_multiplier();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u8 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().staking().max_nominations();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().staking().history_depth();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().staking().sessions_per_era();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().staking().bonding_duration();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().staking().slash_defer_duration();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants()
        .staking()
        .max_nominator_rewarded_per_validator();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().staking().max_unlocking_chunks();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().grandpa().max_authorities();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().grandpa().max_set_id_session_entries();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u64 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().im_online().unsigned_priority();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u64 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().democracy().enactment_period();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().democracy().launch_period();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().democracy().voting_period();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().democracy().vote_locking_period();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().democracy().minimum_deposit();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u128 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().democracy().instant_allowed();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::bool = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().democracy().fast_track_voting_period();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().democracy().cooloff_period();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().democracy().max_votes();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().democracy().max_proposals();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().democracy().max_deposits();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().democracy().max_blacklisted();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().council().max_proposal_weight();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: runtime_types::sp_weights::weight_v2::Weight = api
        .constants()
        .at(&constant_query)?;
    let constant_query = polkadot::constants()
        .technical_committee()
        .max_proposal_weight();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: runtime_types::sp_weights::weight_v2::Weight = api
        .constants()
        .at(&constant_query)?;
    let constant_query = polkadot::constants().phragmen_election().pallet_id();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: [::core::primitive::u8; 8usize] = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().phragmen_election().candidacy_bond();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u128 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().phragmen_election().voting_bond_base();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u128 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().phragmen_election().voting_bond_factor();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u128 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().phragmen_election().desired_members();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().phragmen_election().desired_runners_up();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().phragmen_election().term_duration();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().phragmen_election().max_candidates();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().phragmen_election().max_voters();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().phragmen_election().max_votes_per_voter();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().treasury().proposal_bond();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: runtime_types::sp_arithmetic::per_things::Permill = api
        .constants()
        .at(&constant_query)?;
    let constant_query = polkadot::constants().treasury().proposal_bond_minimum();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u128 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().treasury().proposal_bond_maximum();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::option::Option<::core::primitive::u128> = api
        .constants()
        .at(&constant_query)?;
    let constant_query = polkadot::constants().treasury().spend_period();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().treasury().burn();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: runtime_types::sp_arithmetic::per_things::Permill = api
        .constants()
        .at(&constant_query)?;
    let constant_query = polkadot::constants().treasury().pallet_id();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: runtime_types::frame_support::PalletId = api
        .constants()
        .at(&constant_query)?;
    let constant_query = polkadot::constants().treasury().max_approvals();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().conviction_voting().max_votes();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().conviction_voting().vote_locking_period();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().referenda().submission_deposit();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u128 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().referenda().max_queued();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().referenda().undeciding_timeout();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().referenda().alarm_interval();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().referenda().tracks();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::std::vec::Vec<
        (
            ::core::primitive::u16,
            runtime_types::pallet_referenda::types::TrackInfo<
                ::core::primitive::u128,
                ::core::primitive::u32,
            >,
        ),
    > = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().claims().prefix();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::std::vec::Vec<::core::primitive::u8> = api
        .constants()
        .at(&constant_query)?;
    let constant_query = polkadot::constants().vesting().min_vested_transfer();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u128 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().vesting().max_vesting_schedules();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().utility().batched_calls_limit();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().identity().basic_deposit();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u128 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().identity().field_deposit();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u128 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().identity().sub_account_deposit();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u128 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().identity().max_sub_accounts();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().identity().max_additional_fields();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().identity().max_registrars();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().proxy().proxy_deposit_base();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u128 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().proxy().proxy_deposit_factor();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u128 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().proxy().max_proxies();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().proxy().max_pending();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().proxy().announcement_deposit_base();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u128 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().proxy().announcement_deposit_factor();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u128 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().multisig().deposit_base();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u128 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().multisig().deposit_factor();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u128 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().multisig().max_signatories();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().bounties().bounty_deposit_base();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u128 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().bounties().bounty_deposit_payout_delay();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().bounties().bounty_update_period();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().bounties().curator_deposit_multiplier();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: runtime_types::sp_arithmetic::per_things::Permill = api
        .constants()
        .at(&constant_query)?;
    let constant_query = polkadot::constants().bounties().curator_deposit_max();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::option::Option<::core::primitive::u128> = api
        .constants()
        .at(&constant_query)?;
    let constant_query = polkadot::constants().bounties().curator_deposit_min();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::option::Option<::core::primitive::u128> = api
        .constants()
        .at(&constant_query)?;
    let constant_query = polkadot::constants().bounties().bounty_value_minimum();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u128 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().bounties().data_deposit_per_byte();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u128 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().bounties().maximum_reason_length();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants()
        .child_bounties()
        .max_active_child_bounty_count();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants()
        .child_bounties()
        .child_bounty_value_minimum();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u128 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().tips().maximum_reason_length();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().tips().data_deposit_per_byte();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u128 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().tips().tip_countdown();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().tips().tip_finders_fee();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: runtime_types::sp_arithmetic::per_things::Percent = api
        .constants()
        .at(&constant_query)?;
    let constant_query = polkadot::constants().tips().tip_report_deposit_base();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u128 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants()
        .election_provider_multi_phase()
        .unsigned_phase();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants()
        .election_provider_multi_phase()
        .signed_phase();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants()
        .election_provider_multi_phase()
        .better_signed_threshold();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: runtime_types::sp_arithmetic::per_things::Perbill = api
        .constants()
        .at(&constant_query)?;
    let constant_query = polkadot::constants()
        .election_provider_multi_phase()
        .better_unsigned_threshold();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: runtime_types::sp_arithmetic::per_things::Perbill = api
        .constants()
        .at(&constant_query)?;
    let constant_query = polkadot::constants()
        .election_provider_multi_phase()
        .offchain_repeat();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants()
        .election_provider_multi_phase()
        .miner_tx_priority();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u64 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants()
        .election_provider_multi_phase()
        .signed_max_submissions();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants()
        .election_provider_multi_phase()
        .signed_max_weight();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: runtime_types::sp_weights::weight_v2::Weight = api
        .constants()
        .at(&constant_query)?;
    let constant_query = polkadot::constants()
        .election_provider_multi_phase()
        .signed_max_refunds();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants()
        .election_provider_multi_phase()
        .signed_reward_base();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u128 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants()
        .election_provider_multi_phase()
        .signed_deposit_base();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u128 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants()
        .election_provider_multi_phase()
        .signed_deposit_byte();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u128 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants()
        .election_provider_multi_phase()
        .signed_deposit_weight();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u128 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants()
        .election_provider_multi_phase()
        .max_electing_voters();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants()
        .election_provider_multi_phase()
        .max_electable_targets();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u16 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants()
        .election_provider_multi_phase()
        .max_winners();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants()
        .election_provider_multi_phase()
        .miner_max_length();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants()
        .election_provider_multi_phase()
        .miner_max_weight();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: runtime_types::sp_weights::weight_v2::Weight = api
        .constants()
        .at(&constant_query)?;
    let constant_query = polkadot::constants()
        .election_provider_multi_phase()
        .miner_max_votes_per_voter();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants()
        .election_provider_multi_phase()
        .miner_max_winners();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().voter_list().bag_thresholds();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::std::vec::Vec<::core::primitive::u64> = api
        .constants()
        .at(&constant_query)?;
    let constant_query = polkadot::constants().nomination_pools().pallet_id();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: runtime_types::frame_support::PalletId = api
        .constants()
        .at(&constant_query)?;
    let constant_query = polkadot::constants()
        .nomination_pools()
        .max_points_to_balance();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u8 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().fast_unstake().deposit();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u128 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().paras().unsigned_priority();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u64 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().registrar().para_deposit();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u128 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().registrar().data_deposit_per_byte();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u128 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().slots().lease_period();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().slots().lease_offset();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().auctions().ending_period();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().auctions().sample_length();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().auctions().slot_range_count();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().auctions().lease_periods_per_slot();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().crowdloan().pallet_id();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: runtime_types::frame_support::PalletId = api
        .constants()
        .at(&constant_query)?;
    let constant_query = polkadot::constants().crowdloan().min_contribution();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u128 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().crowdloan().remove_keys_limit();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().message_queue().heap_size();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().message_queue().max_stale();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::primitive::u32 = api.constants().at(&constant_query)?;
    let constant_query = polkadot::constants().message_queue().service_weight();
    let api = OnlineClient::<PolkadotConfig>::new().await?;
    let value: ::core::option::Option<runtime_types::sp_weights::weight_v2::Weight> = api
        .constants()
        .at(&constant_query)?;
    Ok(())
}
