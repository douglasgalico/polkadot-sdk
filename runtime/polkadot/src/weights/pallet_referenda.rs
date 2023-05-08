// Copyright (C) Parity Technologies (UK) Ltd.
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

//! Autogenerated weights for `pallet_referenda`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-04-28, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bm5`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("polkadot-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=polkadot-dev
// --steps=50
// --repeat=20
// --pallet=pallet_referenda
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/polkadot/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_referenda`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_referenda::WeightInfo for WeightInfo<T> {
	/// Storage: Referenda ReferendumCount (r:1 w:1)
	/// Proof: Referenda ReferendumCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	/// Storage: Referenda ReferendumInfoFor (r:0 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	fn submit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `258`
		//  Estimated: `42428`
		// Minimum execution time: 40_492_000 picoseconds.
		Weight::from_parts(41_242_000, 0)
			.saturating_add(Weight::from_parts(0, 42428))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:2 w:2)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	fn place_decision_deposit_preparing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `511`
		//  Estimated: `83866`
		// Minimum execution time: 52_228_000 picoseconds.
		Weight::from_parts(52_663_000, 0)
			.saturating_add(Weight::from_parts(0, 83866))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: Referenda DecidingCount (r:1 w:0)
	/// Proof: Referenda DecidingCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: Referenda TrackQueue (r:1 w:1)
	/// Proof: Referenda TrackQueue (max_values: None, max_size: Some(2012), added: 4487, mode: MaxEncodedLen)
	fn place_decision_deposit_queued() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3107`
		//  Estimated: `5477`
		// Minimum execution time: 47_524_000 picoseconds.
		Weight::from_parts(47_888_000, 0)
			.saturating_add(Weight::from_parts(0, 5477))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: Referenda DecidingCount (r:1 w:0)
	/// Proof: Referenda DecidingCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: Referenda TrackQueue (r:1 w:1)
	/// Proof: Referenda TrackQueue (max_values: None, max_size: Some(2012), added: 4487, mode: MaxEncodedLen)
	fn place_decision_deposit_not_queued() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3127`
		//  Estimated: `5477`
		// Minimum execution time: 47_223_000 picoseconds.
		Weight::from_parts(47_762_000, 0)
			.saturating_add(Weight::from_parts(0, 5477))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: Referenda DecidingCount (r:1 w:1)
	/// Proof: Referenda DecidingCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: Balances InactiveIssuance (r:1 w:0)
	/// Proof: Balances InactiveIssuance (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:2 w:2)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	fn place_decision_deposit_passing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `511`
		//  Estimated: `83866`
		// Minimum execution time: 62_475_000 picoseconds.
		Weight::from_parts(63_577_000, 0)
			.saturating_add(Weight::from_parts(0, 83866))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: Referenda DecidingCount (r:1 w:1)
	/// Proof: Referenda DecidingCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: Balances InactiveIssuance (r:1 w:0)
	/// Proof: Balances InactiveIssuance (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	fn place_decision_deposit_failing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `418`
		//  Estimated: `4401`
		// Minimum execution time: 41_823_000 picoseconds.
		Weight::from_parts(42_291_000, 0)
			.saturating_add(Weight::from_parts(0, 4401))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	fn refund_decision_deposit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `351`
		//  Estimated: `4401`
		// Minimum execution time: 31_300_000 picoseconds.
		Weight::from_parts(31_694_000, 0)
			.saturating_add(Weight::from_parts(0, 4401))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	fn refund_submission_deposit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `341`
		//  Estimated: `4401`
		// Minimum execution time: 30_829_000 picoseconds.
		Weight::from_parts(31_385_000, 0)
			.saturating_add(Weight::from_parts(0, 4401))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:2 w:2)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	fn cancel() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `419`
		//  Estimated: `83866`
		// Minimum execution time: 39_038_000 picoseconds.
		Weight::from_parts(39_493_000, 0)
			.saturating_add(Weight::from_parts(0, 83866))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:2 w:2)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	/// Storage: Referenda MetadataOf (r:1 w:0)
	/// Proof: Referenda MetadataOf (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	fn kill() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `660`
		//  Estimated: `83866`
		// Minimum execution time: 99_148_000 picoseconds.
		Weight::from_parts(99_945_000, 0)
			.saturating_add(Weight::from_parts(0, 83866))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Referenda TrackQueue (r:1 w:0)
	/// Proof: Referenda TrackQueue (max_values: None, max_size: Some(2012), added: 4487, mode: MaxEncodedLen)
	/// Storage: Referenda DecidingCount (r:1 w:1)
	/// Proof: Referenda DecidingCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	fn one_fewer_deciding_queue_empty() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `174`
		//  Estimated: `5477`
		// Minimum execution time: 9_857_000 picoseconds.
		Weight::from_parts(10_060_000, 0)
			.saturating_add(Weight::from_parts(0, 5477))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Referenda TrackQueue (r:1 w:1)
	/// Proof: Referenda TrackQueue (max_values: None, max_size: Some(2012), added: 4487, mode: MaxEncodedLen)
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: Balances InactiveIssuance (r:1 w:0)
	/// Proof: Balances InactiveIssuance (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:2 w:2)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	fn one_fewer_deciding_failing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3877`
		//  Estimated: `83866`
		// Minimum execution time: 108_983_000 picoseconds.
		Weight::from_parts(109_674_000, 0)
			.saturating_add(Weight::from_parts(0, 83866))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: Referenda TrackQueue (r:1 w:1)
	/// Proof: Referenda TrackQueue (max_values: None, max_size: Some(2012), added: 4487, mode: MaxEncodedLen)
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: Balances InactiveIssuance (r:1 w:0)
	/// Proof: Balances InactiveIssuance (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:2 w:2)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	fn one_fewer_deciding_passing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3877`
		//  Estimated: `83866`
		// Minimum execution time: 111_651_000 picoseconds.
		Weight::from_parts(112_150_000, 0)
			.saturating_add(Weight::from_parts(0, 83866))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: Referenda TrackQueue (r:1 w:1)
	/// Proof: Referenda TrackQueue (max_values: None, max_size: Some(2012), added: 4487, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:0)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	fn nudge_referendum_requeued_insertion() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3895`
		//  Estimated: `42428`
		// Minimum execution time: 58_706_000 picoseconds.
		Weight::from_parts(59_075_000, 0)
			.saturating_add(Weight::from_parts(0, 42428))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: Referenda TrackQueue (r:1 w:1)
	/// Proof: Referenda TrackQueue (max_values: None, max_size: Some(2012), added: 4487, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:0)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	fn nudge_referendum_requeued_slide() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3895`
		//  Estimated: `42428`
		// Minimum execution time: 58_178_000 picoseconds.
		Weight::from_parts(59_016_000, 0)
			.saturating_add(Weight::from_parts(0, 42428))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: Referenda DecidingCount (r:1 w:0)
	/// Proof: Referenda DecidingCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: Referenda TrackQueue (r:1 w:1)
	/// Proof: Referenda TrackQueue (max_values: None, max_size: Some(2012), added: 4487, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:0)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	fn nudge_referendum_queued() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3869`
		//  Estimated: `42428`
		// Minimum execution time: 60_717_000 picoseconds.
		Weight::from_parts(61_235_000, 0)
			.saturating_add(Weight::from_parts(0, 42428))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: Referenda DecidingCount (r:1 w:0)
	/// Proof: Referenda DecidingCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: Referenda TrackQueue (r:1 w:1)
	/// Proof: Referenda TrackQueue (max_values: None, max_size: Some(2012), added: 4487, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:0)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	fn nudge_referendum_not_queued() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3889`
		//  Estimated: `42428`
		// Minimum execution time: 60_100_000 picoseconds.
		Weight::from_parts(60_772_000, 0)
			.saturating_add(Weight::from_parts(0, 42428))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	fn nudge_referendum_no_deposit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `371`
		//  Estimated: `42428`
		// Minimum execution time: 26_329_000 picoseconds.
		Weight::from_parts(26_670_000, 0)
			.saturating_add(Weight::from_parts(0, 42428))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	fn nudge_referendum_preparing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `419`
		//  Estimated: `42428`
		// Minimum execution time: 26_902_000 picoseconds.
		Weight::from_parts(27_151_000, 0)
			.saturating_add(Weight::from_parts(0, 42428))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	fn nudge_referendum_timed_out() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `278`
		//  Estimated: `4401`
		// Minimum execution time: 18_461_000 picoseconds.
		Weight::from_parts(18_646_000, 0)
			.saturating_add(Weight::from_parts(0, 4401))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: Referenda DecidingCount (r:1 w:1)
	/// Proof: Referenda DecidingCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: Balances InactiveIssuance (r:1 w:0)
	/// Proof: Balances InactiveIssuance (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	fn nudge_referendum_begin_deciding_failing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `419`
		//  Estimated: `42428`
		// Minimum execution time: 36_446_000 picoseconds.
		Weight::from_parts(36_714_000, 0)
			.saturating_add(Weight::from_parts(0, 42428))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: Referenda DecidingCount (r:1 w:1)
	/// Proof: Referenda DecidingCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: Balances InactiveIssuance (r:1 w:0)
	/// Proof: Balances InactiveIssuance (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	fn nudge_referendum_begin_deciding_passing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `419`
		//  Estimated: `42428`
		// Minimum execution time: 38_361_000 picoseconds.
		Weight::from_parts(38_745_000, 0)
			.saturating_add(Weight::from_parts(0, 42428))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: Balances InactiveIssuance (r:1 w:0)
	/// Proof: Balances InactiveIssuance (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	fn nudge_referendum_begin_confirming() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `472`
		//  Estimated: `42428`
		// Minimum execution time: 31_495_000 picoseconds.
		Weight::from_parts(31_677_000, 0)
			.saturating_add(Weight::from_parts(0, 42428))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: Balances InactiveIssuance (r:1 w:0)
	/// Proof: Balances InactiveIssuance (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	fn nudge_referendum_end_confirming() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `455`
		//  Estimated: `42428`
		// Minimum execution time: 32_112_000 picoseconds.
		Weight::from_parts(32_616_000, 0)
			.saturating_add(Weight::from_parts(0, 42428))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: Balances InactiveIssuance (r:1 w:0)
	/// Proof: Balances InactiveIssuance (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	fn nudge_referendum_continue_not_confirming() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `472`
		//  Estimated: `42428`
		// Minimum execution time: 29_866_000 picoseconds.
		Weight::from_parts(30_155_000, 0)
			.saturating_add(Weight::from_parts(0, 42428))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: Balances InactiveIssuance (r:1 w:0)
	/// Proof: Balances InactiveIssuance (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	fn nudge_referendum_continue_confirming() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `476`
		//  Estimated: `42428`
		// Minimum execution time: 28_575_000 picoseconds.
		Weight::from_parts(28_967_000, 0)
			.saturating_add(Weight::from_parts(0, 42428))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: Balances InactiveIssuance (r:1 w:0)
	/// Proof: Balances InactiveIssuance (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:2 w:2)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	/// Storage: Scheduler Lookup (r:1 w:1)
	/// Proof: Scheduler Lookup (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	fn nudge_referendum_approved() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `476`
		//  Estimated: `83866`
		// Minimum execution time: 43_590_000 picoseconds.
		Weight::from_parts(43_932_000, 0)
			.saturating_add(Weight::from_parts(0, 83866))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: Balances InactiveIssuance (r:1 w:0)
	/// Proof: Balances InactiveIssuance (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	fn nudge_referendum_rejected() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `472`
		//  Estimated: `42428`
		// Minimum execution time: 31_730_000 picoseconds.
		Weight::from_parts(32_179_000, 0)
			.saturating_add(Weight::from_parts(0, 42428))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Referenda ReferendumInfoFor (r:1 w:0)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: Preimage StatusFor (r:1 w:0)
	/// Proof: Preimage StatusFor (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	/// Storage: Referenda MetadataOf (r:0 w:1)
	/// Proof: Referenda MetadataOf (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	fn set_some_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `422`
		//  Estimated: `4401`
		// Minimum execution time: 21_380_000 picoseconds.
		Weight::from_parts(21_710_000, 0)
			.saturating_add(Weight::from_parts(0, 4401))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Referenda ReferendumInfoFor (r:1 w:0)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: Referenda MetadataOf (r:1 w:1)
	/// Proof: Referenda MetadataOf (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	fn clear_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `355`
		//  Estimated: `4401`
		// Minimum execution time: 19_134_000 picoseconds.
		Weight::from_parts(19_371_000, 0)
			.saturating_add(Weight::from_parts(0, 4401))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}