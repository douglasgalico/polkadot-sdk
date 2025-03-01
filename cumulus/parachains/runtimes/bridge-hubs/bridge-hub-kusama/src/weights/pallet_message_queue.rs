// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for `pallet_message_queue`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-03-24, STEPS: `2`, REPEAT: `1`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `i9`, CPU: `13th Gen Intel(R) Core(TM) i9-13900K`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("bridge-hub-kusama-dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/polkadot-parachain
// benchmark
// pallet
// --chain
// bridge-hub-kusama-dev
// --pallet
// pallet_message_queue
// --extrinsic
// *
// --execution
// wasm
// --wasm-execution
// compiled
// --output
// parachains/runtimes/bridge-hubs/bridge-hub-kusama/src/weights

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_message_queue`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_message_queue::WeightInfo for WeightInfo<T> {
	/// Storage: MessageQueue ServiceHead (r:1 w:0)
	/// Proof: MessageQueue ServiceHead (max_values: Some(1), max_size: Some(5), added: 500, mode: MaxEncodedLen)
	/// Storage: MessageQueue BookStateFor (r:2 w:2)
	/// Proof: MessageQueue BookStateFor (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	fn ready_ring_knit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `189`
		//  Estimated: `7534`
		// Minimum execution time: 11_692_000 picoseconds.
		Weight::from_parts(11_692_000, 0)
			.saturating_add(Weight::from_parts(0, 7534))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: MessageQueue BookStateFor (r:2 w:2)
	/// Proof: MessageQueue BookStateFor (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	/// Storage: MessageQueue ServiceHead (r:1 w:1)
	/// Proof: MessageQueue ServiceHead (max_values: Some(1), max_size: Some(5), added: 500, mode: MaxEncodedLen)
	fn ready_ring_unknit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `184`
		//  Estimated: `7534`
		// Minimum execution time: 10_614_000 picoseconds.
		Weight::from_parts(10_614_000, 0)
			.saturating_add(Weight::from_parts(0, 7534))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: MessageQueue BookStateFor (r:1 w:1)
	/// Proof: MessageQueue BookStateFor (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	fn service_queue_base() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6`
		//  Estimated: `3517`
		// Minimum execution time: 7_085_000 picoseconds.
		Weight::from_parts(7_085_000, 0)
			.saturating_add(Weight::from_parts(0, 3517))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: MessageQueue Pages (r:1 w:1)
	/// Proof: MessageQueue Pages (max_values: None, max_size: Some(65585), added: 68060, mode: MaxEncodedLen)
	fn service_page_base_completion() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `72`
		//  Estimated: `69050`
		// Minimum execution time: 5_813_000 picoseconds.
		Weight::from_parts(5_813_000, 0)
			.saturating_add(Weight::from_parts(0, 69050))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: MessageQueue Pages (r:1 w:1)
	/// Proof: MessageQueue Pages (max_values: None, max_size: Some(65585), added: 68060, mode: MaxEncodedLen)
	fn service_page_base_no_completion() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `72`
		//  Estimated: `69050`
		// Minimum execution time: 6_090_000 picoseconds.
		Weight::from_parts(6_090_000, 0)
			.saturating_add(Weight::from_parts(0, 69050))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	fn service_page_item() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 58_905_000 picoseconds.
		Weight::from_parts(58_905_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	/// Storage: MessageQueue ServiceHead (r:1 w:1)
	/// Proof: MessageQueue ServiceHead (max_values: Some(1), max_size: Some(5), added: 500, mode: MaxEncodedLen)
	/// Storage: MessageQueue BookStateFor (r:1 w:0)
	/// Proof: MessageQueue BookStateFor (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	fn bump_service_head() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `99`
		//  Estimated: `5007`
		// Minimum execution time: 6_501_000 picoseconds.
		Weight::from_parts(6_501_000, 0)
			.saturating_add(Weight::from_parts(0, 5007))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: MessageQueue BookStateFor (r:1 w:1)
	/// Proof: MessageQueue BookStateFor (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	/// Storage: MessageQueue Pages (r:1 w:1)
	/// Proof: MessageQueue Pages (max_values: None, max_size: Some(65585), added: 68060, mode: MaxEncodedLen)
	fn reap_page() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `65667`
		//  Estimated: `72567`
		// Minimum execution time: 39_695_000 picoseconds.
		Weight::from_parts(39_695_000, 0)
			.saturating_add(Weight::from_parts(0, 72567))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: MessageQueue BookStateFor (r:1 w:1)
	/// Proof: MessageQueue BookStateFor (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	/// Storage: MessageQueue Pages (r:1 w:1)
	/// Proof: MessageQueue Pages (max_values: None, max_size: Some(65585), added: 68060, mode: MaxEncodedLen)
	fn execute_overweight_page_removed() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `65667`
		//  Estimated: `72567`
		// Minimum execution time: 50_543_000 picoseconds.
		Weight::from_parts(50_543_000, 0)
			.saturating_add(Weight::from_parts(0, 72567))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: MessageQueue BookStateFor (r:1 w:1)
	/// Proof: MessageQueue BookStateFor (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	/// Storage: MessageQueue Pages (r:1 w:1)
	/// Proof: MessageQueue Pages (max_values: None, max_size: Some(65585), added: 68060, mode: MaxEncodedLen)
	fn execute_overweight_page_updated() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `65667`
		//  Estimated: `72567`
		// Minimum execution time: 69_294_000 picoseconds.
		Weight::from_parts(69_294_000, 0)
			.saturating_add(Weight::from_parts(0, 72567))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}
