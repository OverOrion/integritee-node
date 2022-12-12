
//! Autogenerated weights for `pallet_multisig`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-11-11, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("integritee-solo-fresh"), DB CACHE: 128

// Executed Command:
// ./integritee-node
// benchmark
// --chain=integritee-solo-fresh
// --steps=50
// --repeat=20
// --pallet=pallet_multisig
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=runtime/src/weights/pallet_multisig.rs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for pallet_multisig.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_multisig::WeightInfo for WeightInfo<T> {
	fn as_multi_threshold_1(z: u32, ) -> Weight {
		Weight::from_ref_time(9_050_000)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(3_000)).saturating_mul(z.into())
	}
	// Storage: Multisig Multisigs (r:1 w:1)
	// Storage: unknown [0x3a65787472696e7369635f696e646578] (r:1 w:0)
	fn as_multi_create(s: u32, z: u32, ) -> Weight {
		Weight::from_ref_time(99_228_000)
			// Standard Error: 87_000
			.saturating_add(Weight::from_ref_time(793_000)).saturating_mul(s.into())
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(3_000)).saturating_mul(z.into())
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Multisig Multisigs (r:1 w:1)
	fn as_multi_approve(_s: u32, z: u32, ) -> Weight {
		Weight::from_ref_time(70_439_000)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(3_000)).saturating_mul(z.into())
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Multisig Multisigs (r:1 w:1)
	// Storage: Multisig Calls (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn as_multi_complete(s: u32, z: u32, ) -> Weight {
		Weight::from_ref_time(139_018_000)
			// Standard Error: 317_000
			.saturating_add(Weight::from_ref_time(629_000)).saturating_mul(s.into())
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(9_000)).saturating_mul(z.into())
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Multisig Multisigs (r:1 w:1)
	// Storage: unknown [0x3a65787472696e7369635f696e646578] (r:1 w:0)
	fn approve_as_multi_create(s: u32, ) -> Weight {
		Weight::from_ref_time(93_678_000)
			// Standard Error: 75_000
			.saturating_add(Weight::from_ref_time(1_063_000)).saturating_mul(s.into())
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Multisig Multisigs (r:1 w:1)
	// Storage: Multisig Calls (r:1 w:0)
	fn approve_as_multi_approve(_s: u32, ) -> Weight {
		Weight::from_ref_time(81_581_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Multisig Multisigs (r:1 w:1)
	// Storage: Multisig Calls (r:1 w:1)
	fn cancel_as_multi(_s: u32, ) -> Weight {
		Weight::from_ref_time(194_725_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}
