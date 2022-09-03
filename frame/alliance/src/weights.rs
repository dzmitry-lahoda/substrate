// This file is part of Substrate.

// Copyright (C) 2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for pallet_alliance
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-09-01, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `bm3`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// /home/benchbot/cargo_target_dir/production/substrate
// benchmark
// pallet
// --steps=50
// --repeat=20
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --pallet=pallet_alliance
// --chain=dev
// --output=./frame/alliance/src/weights.rs
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_alliance.
pub trait WeightInfo {
	fn propose_proposed(b: u32, x: u32, y: u32, p: u32, ) -> Weight;
	fn vote(x: u32, y: u32, ) -> Weight;
	fn veto(p: u32, ) -> Weight;
	fn close_early_disapproved(x: u32, y: u32, p: u32, ) -> Weight;
	fn close_early_approved(b: u32, x: u32, y: u32, p: u32, ) -> Weight;
	fn close_disapproved(x: u32, y: u32, p: u32, ) -> Weight;
	fn close_approved(b: u32, x: u32, y: u32, p: u32, ) -> Weight;
	fn force_set_members(x: u32, y: u32, z: u32, p: u32, c: u32, m: u32, ) -> Weight;
	fn set_rule() -> Weight;
	fn announce() -> Weight;
	fn remove_announcement() -> Weight;
	fn join_alliance() -> Weight;
	fn nominate_ally() -> Weight;
	fn elevate_ally() -> Weight;
	fn give_retirement_notice() -> Weight;
	fn retire() -> Weight;
	fn kick_member() -> Weight;
	fn add_unscrupulous_items(n: u32, l: u32, ) -> Weight;
	fn remove_unscrupulous_items(n: u32, l: u32, ) -> Weight;
}

/// Weights for pallet_alliance using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Alliance Members (r:1 w:0)
	// Storage: AllianceMotion ProposalOf (r:1 w:1)
	// Storage: AllianceMotion Proposals (r:1 w:1)
	// Storage: AllianceMotion ProposalCount (r:1 w:1)
	// Storage: AllianceMotion Voting (r:0 w:1)
	/// The range of component `b` is `[1, 1024]`.
	/// The range of component `x` is `[2, 10]`.
	/// The range of component `y` is `[0, 90]`.
	/// The range of component `p` is `[1, 100]`.
	fn propose_proposed(_b: u32, x: u32, y: u32, p: u32, ) -> Weight {
		Weight::from_ref_time(37_864_000 as u64)
			// Standard Error: 28_000
			.saturating_add(Weight::from_ref_time(69_000 as u64).saturating_mul(x as u64))
			// Standard Error: 2_000
			.saturating_add(Weight::from_ref_time(66_000 as u64).saturating_mul(y as u64))
			// Standard Error: 2_000
			.saturating_add(Weight::from_ref_time(192_000 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: Alliance Members (r:2 w:0)
	// Storage: AllianceMotion Voting (r:1 w:1)
	/// The range of component `x` is `[3, 10]`.
	/// The range of component `y` is `[2, 90]`.
	fn vote(_x: u32, y: u32, ) -> Weight {
		Weight::from_ref_time(46_813_000 as u64)
			// Standard Error: 2_000
			.saturating_add(Weight::from_ref_time(125_000 as u64).saturating_mul(y as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Alliance Members (r:1 w:0)
	// Storage: AllianceMotion ProposalOf (r:1 w:1)
	// Storage: AllianceMotion Proposals (r:1 w:1)
	// Storage: AllianceMotion Voting (r:0 w:1)
	/// The range of component `p` is `[1, 100]`.
	fn veto(p: u32, ) -> Weight {
		Weight::from_ref_time(35_316_000 as u64)
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(172_000 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Alliance Members (r:1 w:0)
	// Storage: AllianceMotion Voting (r:1 w:1)
	// Storage: AllianceMotion Members (r:1 w:0)
	// Storage: AllianceMotion Proposals (r:1 w:1)
	// Storage: AllianceMotion ProposalOf (r:0 w:1)
	/// The range of component `x` is `[2, 10]`.
	/// The range of component `y` is `[2, 90]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_early_disapproved(x: u32, y: u32, p: u32, ) -> Weight {
		Weight::from_ref_time(36_245_000 as u64)
			// Standard Error: 18_000
			.saturating_add(Weight::from_ref_time(336_000 as u64).saturating_mul(x as u64))
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(109_000 as u64).saturating_mul(y as u64))
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(178_000 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Alliance Members (r:1 w:0)
	// Storage: AllianceMotion Voting (r:1 w:1)
	// Storage: AllianceMotion Members (r:1 w:0)
	// Storage: AllianceMotion ProposalOf (r:1 w:1)
	// Storage: AllianceMotion Proposals (r:1 w:1)
	/// The range of component `b` is `[1, 1024]`.
	/// The range of component `x` is `[2, 10]`.
	/// The range of component `y` is `[2, 90]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_early_approved(b: u32, x: u32, y: u32, p: u32, ) -> Weight {
		Weight::from_ref_time(48_088_000 as u64)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(1_000 as u64).saturating_mul(b as u64))
			// Standard Error: 16_000
			.saturating_add(Weight::from_ref_time(194_000 as u64).saturating_mul(x as u64))
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(93_000 as u64).saturating_mul(y as u64))
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(201_000 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Alliance Members (r:1 w:0)
	// Storage: AllianceMotion Voting (r:1 w:1)
	// Storage: AllianceMotion Members (r:1 w:0)
	// Storage: AllianceMotion Prime (r:1 w:0)
	// Storage: AllianceMotion Proposals (r:1 w:1)
	// Storage: AllianceMotion ProposalOf (r:0 w:1)
	/// The range of component `x` is `[2, 10]`.
	/// The range of component `y` is `[2, 90]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_disapproved(_x: u32, y: u32, p: u32, ) -> Weight {
		Weight::from_ref_time(43_374_000 as u64)
			// Standard Error: 2_000
			.saturating_add(Weight::from_ref_time(101_000 as u64).saturating_mul(y as u64))
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(182_000 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Alliance Members (r:1 w:0)
	// Storage: AllianceMotion Voting (r:1 w:1)
	// Storage: AllianceMotion Members (r:1 w:0)
	// Storage: AllianceMotion Prime (r:1 w:0)
	// Storage: AllianceMotion Proposals (r:1 w:1)
	// Storage: AllianceMotion ProposalOf (r:0 w:1)
	/// The range of component `b` is `[1, 1024]`.
	/// The range of component `x` is `[2, 10]`.
	/// The range of component `y` is `[2, 90]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_approved(_b: u32, _x: u32, y: u32, p: u32, ) -> Weight {
		Weight::from_ref_time(42_798_000 as u64)
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(87_000 as u64).saturating_mul(y as u64))
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(192_000 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: AllianceMotion Proposals (r:1 w:1)
	// Storage: Alliance Members (r:3 w:3)
	// Storage: Alliance DepositOf (r:200 w:199)
	// Storage: System Account (r:199 w:199)
	// Storage: AllianceMotion Voting (r:0 w:100)
	// Storage: AllianceMotion Members (r:0 w:1)
	// Storage: AllianceMotion Prime (r:0 w:1)
	// Storage: AllianceMotion ProposalOf (r:0 w:100)
	/// The range of component `x` is `[1, 10]`.
	/// The range of component `y` is `[0, 90]`.
	/// The range of component `z` is `[0, 100]`.
	/// The range of component `p` is `[0, 100]`.
	/// The range of component `c` is `[0, 100]`.
	/// The range of component `m` is `[0, 100]`.
	fn force_set_members(x: u32, y: u32, z: u32, p: u32, c: u32, m: u32, ) -> Weight {
		Weight::from_ref_time(0 as u64)
			// Standard Error: 221_000
			.saturating_add(Weight::from_ref_time(1_294_000 as u64).saturating_mul(x as u64))
			// Standard Error: 23_000
			.saturating_add(Weight::from_ref_time(231_000 as u64).saturating_mul(y as u64))
			// Standard Error: 21_000
			.saturating_add(Weight::from_ref_time(115_000 as u64).saturating_mul(z as u64))
			// Standard Error: 21_000
			.saturating_add(Weight::from_ref_time(9_371_000 as u64).saturating_mul(p as u64))
			// Standard Error: 21_000
			.saturating_add(Weight::from_ref_time(11_673_000 as u64).saturating_mul(c as u64))
			// Standard Error: 21_000
			.saturating_add(Weight::from_ref_time(11_581_000 as u64).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().reads((2 as u64).saturating_mul(c as u64)))
			.saturating_add(T::DbWeight::get().reads((2 as u64).saturating_mul(m as u64)))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
			.saturating_add(T::DbWeight::get().writes((2 as u64).saturating_mul(p as u64)))
			.saturating_add(T::DbWeight::get().writes((2 as u64).saturating_mul(c as u64)))
			.saturating_add(T::DbWeight::get().writes((2 as u64).saturating_mul(m as u64)))
	}
	// Storage: Alliance Rule (r:0 w:1)
	fn set_rule() -> Weight {
		Weight::from_ref_time(18_721_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Alliance Announcements (r:1 w:1)
	fn announce() -> Weight {
		Weight::from_ref_time(21_887_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Alliance Announcements (r:1 w:1)
	fn remove_announcement() -> Weight {
		Weight::from_ref_time(23_052_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Alliance Members (r:4 w:1)
	// Storage: Alliance UnscrupulousAccounts (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: Alliance DepositOf (r:0 w:1)
	fn join_alliance() -> Weight {
		Weight::from_ref_time(54_504_000 as u64)
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Alliance Members (r:4 w:1)
	// Storage: Alliance UnscrupulousAccounts (r:1 w:0)
	fn nominate_ally() -> Weight {
		Weight::from_ref_time(42_601_000 as u64)
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Alliance Members (r:3 w:2)
	// Storage: AllianceMotion Proposals (r:1 w:0)
	// Storage: AllianceMotion Members (r:0 w:1)
	// Storage: AllianceMotion Prime (r:0 w:1)
	fn elevate_ally() -> Weight {
		Weight::from_ref_time(37_704_000 as u64)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: Alliance Members (r:4 w:2)
	// Storage: AllianceMotion Proposals (r:1 w:0)
	// Storage: AllianceMotion Members (r:0 w:1)
	// Storage: AllianceMotion Prime (r:0 w:1)
	// Storage: Alliance RetiringMembers (r:0 w:1)
	fn give_retirement_notice() -> Weight {
		Weight::from_ref_time(40_859_000 as u64)
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	// Storage: Alliance RetiringMembers (r:1 w:1)
	// Storage: Alliance Members (r:1 w:1)
	// Storage: Alliance DepositOf (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn retire() -> Weight {
		Weight::from_ref_time(43_447_000 as u64)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: Alliance Members (r:3 w:1)
	// Storage: AllianceMotion Proposals (r:1 w:0)
	// Storage: Alliance DepositOf (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: AllianceMotion Members (r:0 w:1)
	// Storage: AllianceMotion Prime (r:0 w:1)
	fn kick_member() -> Weight {
		Weight::from_ref_time(61_718_000 as u64)
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	// Storage: Alliance UnscrupulousAccounts (r:1 w:1)
	// Storage: Alliance UnscrupulousWebsites (r:1 w:1)
	/// The range of component `n` is `[1, 100]`.
	/// The range of component `l` is `[1, 255]`.
	fn add_unscrupulous_items(n: u32, l: u32, ) -> Weight {
		Weight::from_ref_time(359_000 as u64)
			// Standard Error: 2_000
			.saturating_add(Weight::from_ref_time(1_376_000 as u64).saturating_mul(n as u64))
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(112_000 as u64).saturating_mul(l as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Alliance UnscrupulousAccounts (r:1 w:1)
	// Storage: Alliance UnscrupulousWebsites (r:1 w:1)
	/// The range of component `n` is `[1, 100]`.
	/// The range of component `l` is `[1, 255]`.
	fn remove_unscrupulous_items(n: u32, l: u32, ) -> Weight {
		Weight::from_ref_time(0 as u64)
			// Standard Error: 145_000
			.saturating_add(Weight::from_ref_time(20_932_000 as u64).saturating_mul(n as u64))
			// Standard Error: 56_000
			.saturating_add(Weight::from_ref_time(3_649_000 as u64).saturating_mul(l as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Alliance Members (r:1 w:0)
	// Storage: AllianceMotion ProposalOf (r:1 w:1)
	// Storage: AllianceMotion Proposals (r:1 w:1)
	// Storage: AllianceMotion ProposalCount (r:1 w:1)
	// Storage: AllianceMotion Voting (r:0 w:1)
	/// The range of component `b` is `[1, 1024]`.
	/// The range of component `x` is `[2, 10]`.
	/// The range of component `y` is `[0, 90]`.
	/// The range of component `p` is `[1, 100]`.
	fn propose_proposed(_b: u32, x: u32, y: u32, p: u32, ) -> Weight {
		Weight::from_ref_time(37_864_000 as u64)
			// Standard Error: 28_000
			.saturating_add(Weight::from_ref_time(69_000 as u64).saturating_mul(x as u64))
			// Standard Error: 2_000
			.saturating_add(Weight::from_ref_time(66_000 as u64).saturating_mul(y as u64))
			// Standard Error: 2_000
			.saturating_add(Weight::from_ref_time(192_000 as u64).saturating_mul(p as u64))
			.saturating_add(RocksDbWeight::get().reads(4 as u64))
			.saturating_add(RocksDbWeight::get().writes(4 as u64))
	}
	// Storage: Alliance Members (r:2 w:0)
	// Storage: AllianceMotion Voting (r:1 w:1)
	/// The range of component `x` is `[3, 10]`.
	/// The range of component `y` is `[2, 90]`.
	fn vote(_x: u32, y: u32, ) -> Weight {
		Weight::from_ref_time(46_813_000 as u64)
			// Standard Error: 2_000
			.saturating_add(Weight::from_ref_time(125_000 as u64).saturating_mul(y as u64))
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Alliance Members (r:1 w:0)
	// Storage: AllianceMotion ProposalOf (r:1 w:1)
	// Storage: AllianceMotion Proposals (r:1 w:1)
	// Storage: AllianceMotion Voting (r:0 w:1)
	/// The range of component `p` is `[1, 100]`.
	fn veto(p: u32, ) -> Weight {
		Weight::from_ref_time(35_316_000 as u64)
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(172_000 as u64).saturating_mul(p as u64))
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
	// Storage: Alliance Members (r:1 w:0)
	// Storage: AllianceMotion Voting (r:1 w:1)
	// Storage: AllianceMotion Members (r:1 w:0)
	// Storage: AllianceMotion Proposals (r:1 w:1)
	// Storage: AllianceMotion ProposalOf (r:0 w:1)
	/// The range of component `x` is `[2, 10]`.
	/// The range of component `y` is `[2, 90]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_early_disapproved(x: u32, y: u32, p: u32, ) -> Weight {
		Weight::from_ref_time(36_245_000 as u64)
			// Standard Error: 18_000
			.saturating_add(Weight::from_ref_time(336_000 as u64).saturating_mul(x as u64))
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(109_000 as u64).saturating_mul(y as u64))
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(178_000 as u64).saturating_mul(p as u64))
			.saturating_add(RocksDbWeight::get().reads(4 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
	// Storage: Alliance Members (r:1 w:0)
	// Storage: AllianceMotion Voting (r:1 w:1)
	// Storage: AllianceMotion Members (r:1 w:0)
	// Storage: AllianceMotion ProposalOf (r:1 w:1)
	// Storage: AllianceMotion Proposals (r:1 w:1)
	/// The range of component `b` is `[1, 1024]`.
	/// The range of component `x` is `[2, 10]`.
	/// The range of component `y` is `[2, 90]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_early_approved(b: u32, x: u32, y: u32, p: u32, ) -> Weight {
		Weight::from_ref_time(48_088_000 as u64)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(1_000 as u64).saturating_mul(b as u64))
			// Standard Error: 16_000
			.saturating_add(Weight::from_ref_time(194_000 as u64).saturating_mul(x as u64))
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(93_000 as u64).saturating_mul(y as u64))
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(201_000 as u64).saturating_mul(p as u64))
			.saturating_add(RocksDbWeight::get().reads(5 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
	// Storage: Alliance Members (r:1 w:0)
	// Storage: AllianceMotion Voting (r:1 w:1)
	// Storage: AllianceMotion Members (r:1 w:0)
	// Storage: AllianceMotion Prime (r:1 w:0)
	// Storage: AllianceMotion Proposals (r:1 w:1)
	// Storage: AllianceMotion ProposalOf (r:0 w:1)
	/// The range of component `x` is `[2, 10]`.
	/// The range of component `y` is `[2, 90]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_disapproved(_x: u32, y: u32, p: u32, ) -> Weight {
		Weight::from_ref_time(43_374_000 as u64)
			// Standard Error: 2_000
			.saturating_add(Weight::from_ref_time(101_000 as u64).saturating_mul(y as u64))
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(182_000 as u64).saturating_mul(p as u64))
			.saturating_add(RocksDbWeight::get().reads(5 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
	// Storage: Alliance Members (r:1 w:0)
	// Storage: AllianceMotion Voting (r:1 w:1)
	// Storage: AllianceMotion Members (r:1 w:0)
	// Storage: AllianceMotion Prime (r:1 w:0)
	// Storage: AllianceMotion Proposals (r:1 w:1)
	// Storage: AllianceMotion ProposalOf (r:0 w:1)
	/// The range of component `b` is `[1, 1024]`.
	/// The range of component `x` is `[2, 10]`.
	/// The range of component `y` is `[2, 90]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_approved(_b: u32, _x: u32, y: u32, p: u32, ) -> Weight {
		Weight::from_ref_time(42_798_000 as u64)
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(87_000 as u64).saturating_mul(y as u64))
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(192_000 as u64).saturating_mul(p as u64))
			.saturating_add(RocksDbWeight::get().reads(5 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
	// Storage: AllianceMotion Proposals (r:1 w:1)
	// Storage: Alliance Members (r:3 w:3)
	// Storage: Alliance DepositOf (r:200 w:199)
	// Storage: System Account (r:199 w:199)
	// Storage: AllianceMotion Voting (r:0 w:100)
	// Storage: AllianceMotion Members (r:0 w:1)
	// Storage: AllianceMotion Prime (r:0 w:1)
	// Storage: AllianceMotion ProposalOf (r:0 w:100)
	/// The range of component `x` is `[1, 10]`.
	/// The range of component `y` is `[0, 90]`.
	/// The range of component `z` is `[0, 100]`.
	/// The range of component `p` is `[0, 100]`.
	/// The range of component `c` is `[0, 100]`.
	/// The range of component `m` is `[0, 100]`.
	fn force_set_members(x: u32, y: u32, z: u32, p: u32, c: u32, m: u32, ) -> Weight {
		Weight::from_ref_time(0 as u64)
			// Standard Error: 221_000
			.saturating_add(Weight::from_ref_time(1_294_000 as u64).saturating_mul(x as u64))
			// Standard Error: 23_000
			.saturating_add(Weight::from_ref_time(231_000 as u64).saturating_mul(y as u64))
			// Standard Error: 21_000
			.saturating_add(Weight::from_ref_time(115_000 as u64).saturating_mul(z as u64))
			// Standard Error: 21_000
			.saturating_add(Weight::from_ref_time(9_371_000 as u64).saturating_mul(p as u64))
			// Standard Error: 21_000
			.saturating_add(Weight::from_ref_time(11_673_000 as u64).saturating_mul(c as u64))
			// Standard Error: 21_000
			.saturating_add(Weight::from_ref_time(11_581_000 as u64).saturating_mul(m as u64))
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().reads((2 as u64).saturating_mul(c as u64)))
			.saturating_add(RocksDbWeight::get().reads((2 as u64).saturating_mul(m as u64)))
			.saturating_add(RocksDbWeight::get().writes(4 as u64))
			.saturating_add(RocksDbWeight::get().writes((2 as u64).saturating_mul(p as u64)))
			.saturating_add(RocksDbWeight::get().writes((2 as u64).saturating_mul(c as u64)))
			.saturating_add(RocksDbWeight::get().writes((2 as u64).saturating_mul(m as u64)))
	}
	// Storage: Alliance Rule (r:0 w:1)
	fn set_rule() -> Weight {
		Weight::from_ref_time(18_721_000 as u64)
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Alliance Announcements (r:1 w:1)
	fn announce() -> Weight {
		Weight::from_ref_time(21_887_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Alliance Announcements (r:1 w:1)
	fn remove_announcement() -> Weight {
		Weight::from_ref_time(23_052_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Alliance Members (r:4 w:1)
	// Storage: Alliance UnscrupulousAccounts (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: Alliance DepositOf (r:0 w:1)
	fn join_alliance() -> Weight {
		Weight::from_ref_time(54_504_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(6 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
	// Storage: Alliance Members (r:4 w:1)
	// Storage: Alliance UnscrupulousAccounts (r:1 w:0)
	fn nominate_ally() -> Weight {
		Weight::from_ref_time(42_601_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(5 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Alliance Members (r:3 w:2)
	// Storage: AllianceMotion Proposals (r:1 w:0)
	// Storage: AllianceMotion Members (r:0 w:1)
	// Storage: AllianceMotion Prime (r:0 w:1)
	fn elevate_ally() -> Weight {
		Weight::from_ref_time(37_704_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(4 as u64))
			.saturating_add(RocksDbWeight::get().writes(4 as u64))
	}
	// Storage: Alliance Members (r:4 w:2)
	// Storage: AllianceMotion Proposals (r:1 w:0)
	// Storage: AllianceMotion Members (r:0 w:1)
	// Storage: AllianceMotion Prime (r:0 w:1)
	// Storage: Alliance RetiringMembers (r:0 w:1)
	fn give_retirement_notice() -> Weight {
		Weight::from_ref_time(40_859_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(5 as u64))
			.saturating_add(RocksDbWeight::get().writes(5 as u64))
	}
	// Storage: Alliance RetiringMembers (r:1 w:1)
	// Storage: Alliance Members (r:1 w:1)
	// Storage: Alliance DepositOf (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn retire() -> Weight {
		Weight::from_ref_time(43_447_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(4 as u64))
			.saturating_add(RocksDbWeight::get().writes(4 as u64))
	}
	// Storage: Alliance Members (r:3 w:1)
	// Storage: AllianceMotion Proposals (r:1 w:0)
	// Storage: Alliance DepositOf (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: AllianceMotion Members (r:0 w:1)
	// Storage: AllianceMotion Prime (r:0 w:1)
	fn kick_member() -> Weight {
		Weight::from_ref_time(61_718_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(6 as u64))
			.saturating_add(RocksDbWeight::get().writes(5 as u64))
	}
	// Storage: Alliance UnscrupulousAccounts (r:1 w:1)
	// Storage: Alliance UnscrupulousWebsites (r:1 w:1)
	/// The range of component `n` is `[1, 100]`.
	/// The range of component `l` is `[1, 255]`.
	fn add_unscrupulous_items(n: u32, l: u32, ) -> Weight {
		Weight::from_ref_time(359_000 as u64)
			// Standard Error: 2_000
			.saturating_add(Weight::from_ref_time(1_376_000 as u64).saturating_mul(n as u64))
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(112_000 as u64).saturating_mul(l as u64))
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	// Storage: Alliance UnscrupulousAccounts (r:1 w:1)
	// Storage: Alliance UnscrupulousWebsites (r:1 w:1)
	/// The range of component `n` is `[1, 100]`.
	/// The range of component `l` is `[1, 255]`.
	fn remove_unscrupulous_items(n: u32, l: u32, ) -> Weight {
		Weight::from_ref_time(0 as u64)
			// Standard Error: 145_000
			.saturating_add(Weight::from_ref_time(20_932_000 as u64).saturating_mul(n as u64))
			// Standard Error: 56_000
			.saturating_add(Weight::from_ref_time(3_649_000 as u64).saturating_mul(l as u64))
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
}
