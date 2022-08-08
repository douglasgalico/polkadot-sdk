// Copyright 2017-2021 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for `pallet_xcm_benchmarks::generic`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-12-01, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("westend-dev"), DB CACHE: 128

// Executed Command:
// target/release/polkadot
// benchmark
// --chain=westend-dev
// --steps=50
// --repeat=20
// --pallet=pallet_xcm_benchmarks::generic
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --header=./file_header.txt
// --template=./xcm/pallet-xcm-benchmarks/template.hbs
// --output=./runtime/westend/src/weights/xcm/pallet_xcm_benchmarks_generic.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weights for `pallet_xcm_benchmarks::generic`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo<T> {
	// Storage: XcmPallet SupportedVersion (r:1 w:0)
	// Storage: XcmPallet VersionDiscoveryQueue (r:1 w:1)
	// Storage: XcmPallet SafeXcmVersion (r:1 w:0)
	// Storage: Dmp DownwardMessageQueueHeads (r:1 w:1)
	// Storage: Dmp DownwardMessageQueues (r:1 w:1)
	pub(crate) fn query_holding() -> Weight {
		(39_278_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	pub(crate) fn buy_execution() -> Weight {
		(5_922_000 as Weight)
	}
	// Storage: XcmPallet Queries (r:1 w:0)
	pub(crate) fn query_response() -> Weight {
		(20_625_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
	}
	pub(crate) fn transact() -> Weight {
		(22_198_000 as Weight)
	}
	pub(crate) fn refund_surplus() -> Weight {
		(6_122_000 as Weight)
	}
	pub(crate) fn set_error_handler() -> Weight {
		(5_758_000 as Weight)
	}
	pub(crate) fn set_appendix() -> Weight {
		(5_764_000 as Weight)
	}
	pub(crate) fn clear_error() -> Weight {
		(5_679_000 as Weight)
	}
	pub(crate) fn descend_origin() -> Weight {
		(7_206_000 as Weight)
	}
	pub(crate) fn clear_origin() -> Weight {
		(5_738_000 as Weight)
	}
	// Storage: XcmPallet SupportedVersion (r:1 w:0)
	// Storage: XcmPallet VersionDiscoveryQueue (r:1 w:1)
	// Storage: XcmPallet SafeXcmVersion (r:1 w:0)
	// Storage: Dmp DownwardMessageQueueHeads (r:1 w:1)
	// Storage: Dmp DownwardMessageQueues (r:1 w:1)
	pub(crate) fn report_error() -> Weight {
		(31_512_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: XcmPallet AssetTraps (r:1 w:1)
	pub(crate) fn claim_asset() -> Weight {
		(13_594_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	pub(crate) fn trap() -> Weight {
		(5_745_000 as Weight)
	}
	// Storage: XcmPallet VersionNotifyTargets (r:1 w:1)
	// Storage: XcmPallet SupportedVersion (r:1 w:0)
	// Storage: XcmPallet VersionDiscoveryQueue (r:1 w:1)
	// Storage: XcmPallet SafeXcmVersion (r:1 w:0)
	// Storage: Dmp DownwardMessageQueueHeads (r:1 w:1)
	// Storage: Dmp DownwardMessageQueues (r:1 w:1)
	pub(crate) fn subscribe_version() -> Weight {
		(38_138_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: XcmPallet VersionNotifyTargets (r:0 w:1)
	pub(crate) fn unsubscribe_version() -> Weight {
		(9_127_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: XcmPallet SupportedVersion (r:1 w:0)
	// Storage: XcmPallet VersionDiscoveryQueue (r:1 w:1)
	// Storage: XcmPallet SafeXcmVersion (r:1 w:0)
	// Storage: Dmp DownwardMessageQueueHeads (r:1 w:1)
	// Storage: Dmp DownwardMessageQueues (r:1 w:1)
	pub(crate) fn initiate_reserve_withdraw() -> Weight {
		(41_443_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
}