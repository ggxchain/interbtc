
//! Autogenerated weights for dex_general
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-06-05, STEPS: `50`, REPEAT: `10`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `dev-benchmark`, CPU: `Intel(R) Xeon(R) CPU @ 2.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("interlay-dev"), DB CACHE: 1024

// Executed Command:
// target/release/interbtc-parachain
// benchmark
// pallet
// --pallet
// *
// --extrinsic
// *
// --chain
// interlay-dev
// --execution=wasm
// --wasm-execution=compiled
// --steps
// 50
// --repeat
// 10
// --output
// ../tmp-weight/
// --template
// .deploy/runtime-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weights for dex_general using the Substrate node and recommended hardware.
pub struct WeightInfo<T>(PhantomData<T>);

impl<T: frame_system::Config> dex_general::WeightInfo for WeightInfo<T> {

	/// Storage: DexGeneral FeeMeta (r:1 w:1)
	/// Proof: DexGeneral FeeMeta (max_values: Some(1), max_size: Some(34), added: 529, mode: MaxEncodedLen)
	fn set_fee_receiver	() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3`
		//  Estimated: `1519`
		// Minimum execution time: 15_919_000 picoseconds.
		Weight::from_parts(16_728_000, 1519)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: DexGeneral FeeMeta (r:1 w:1)
	/// Proof: DexGeneral FeeMeta (max_values: Some(1), max_size: Some(34), added: 529, mode: MaxEncodedLen)
	fn set_fee_point	() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3`
		//  Estimated: `1519`
		// Minimum execution time: 26_414_000 picoseconds.
		Weight::from_parts(26_871_000, 1519)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: DexGeneral PairStatuses (r:1 w:1)
	/// Proof: DexGeneral PairStatuses (max_values: None, max_size: Some(163), added: 2638, mode: MaxEncodedLen)
	/// Storage: DexGeneral LiquidityPairs (r:0 w:1)
	/// Proof: DexGeneral LiquidityPairs (max_values: None, max_size: Some(50), added: 2525, mode: MaxEncodedLen)
	fn create_pair	() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3`
		//  Estimated: `3628`
		// Minimum execution time: 39_508_000 picoseconds.
		Weight::from_parts(41_610_000, 3628)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: DexGeneral PairStatuses (r:1 w:1)
	/// Proof: DexGeneral PairStatuses (max_values: None, max_size: Some(163), added: 2638, mode: MaxEncodedLen)
	/// Storage: DexGeneral BootstrapLimits (r:0 w:1)
	/// Proof: DexGeneral BootstrapLimits (max_values: None, max_size: Some(27032), added: 29507, mode: MaxEncodedLen)
	/// Storage: DexGeneral BootstrapRewards (r:0 w:1)
	/// Proof: DexGeneral BootstrapRewards (max_values: None, max_size: Some(27032), added: 29507, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 10]`.
	/// The range of component `l` is `[1, 10]`.
	fn bootstrap_create	(r: u32, l: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3`
		//  Estimated: `3628`
		// Minimum execution time: 46_807_000 picoseconds.
		Weight::from_parts(49_800_031, 3628)
			// Standard Error: 55_147
			.saturating_add(Weight::from_parts(120_739, 0).saturating_mul(r.into()))
			// Standard Error: 55_147
			.saturating_add(Weight::from_parts(229_186, 0).saturating_mul(l.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: DexGeneral BootstrapLimits (r:1 w:0)
	/// Proof: DexGeneral BootstrapLimits (max_values: None, max_size: Some(27032), added: 29507, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:4 w:4)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: DexGeneral PairStatuses (r:1 w:1)
	/// Proof: DexGeneral PairStatuses (max_values: None, max_size: Some(163), added: 2638, mode: MaxEncodedLen)
	/// Storage: DexGeneral BootstrapPersonalSupply (r:1 w:1)
	/// Proof: DexGeneral BootstrapPersonalSupply (max_values: None, max_size: Some(102), added: 2577, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn bootstrap_contribute	() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `671`
		//  Estimated: `52635`
		// Minimum execution time: 185_711_000 picoseconds.
		Weight::from_parts(199_156_000, 52635)
			.saturating_add(T::DbWeight::get().reads(8_u64))
			.saturating_add(T::DbWeight::get().writes(7_u64))
	}
	/// Storage: DexGeneral PairStatuses (r:1 w:0)
	/// Proof: DexGeneral PairStatuses (max_values: None, max_size: Some(163), added: 2638, mode: MaxEncodedLen)
	/// Storage: DexGeneral BootstrapPersonalSupply (r:1 w:1)
	/// Proof: DexGeneral BootstrapPersonalSupply (max_values: None, max_size: Some(102), added: 2577, mode: MaxEncodedLen)
	/// Storage: DexGeneral BootstrapEndStatus (r:1 w:0)
	/// Proof: DexGeneral BootstrapEndStatus (max_values: None, max_size: Some(163), added: 2638, mode: MaxEncodedLen)
	/// Storage: DexGeneral LiquidityPairs (r:1 w:0)
	/// Proof: DexGeneral LiquidityPairs (max_values: None, max_size: Some(50), added: 2525, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: DexGeneral BootstrapRewards (r:1 w:0)
	/// Proof: DexGeneral BootstrapRewards (max_values: None, max_size: Some(27032), added: 29507, mode: MaxEncodedLen)
	fn bootstrap_claim	() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1231`
		//  Estimated: `54598`
		// Minimum execution time: 169_864_000 picoseconds.
		Weight::from_parts(181_561_000, 54598)
			.saturating_add(T::DbWeight::get().reads(8_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: DexGeneral PairStatuses (r:1 w:1)
	/// Proof: DexGeneral PairStatuses (max_values: None, max_size: Some(163), added: 2638, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:5 w:5)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Tokens TotalIssuance (r:1 w:1)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(35), added: 2510, mode: MaxEncodedLen)
	/// Storage: DexGeneral LiquidityPairs (r:0 w:1)
	/// Proof: DexGeneral LiquidityPairs (max_values: None, max_size: Some(50), added: 2525, mode: MaxEncodedLen)
	/// Storage: DexGeneral BootstrapEndStatus (r:0 w:1)
	/// Proof: DexGeneral BootstrapEndStatus (max_values: None, max_size: Some(163), added: 2638, mode: MaxEncodedLen)
	fn bootstrap_end	() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `893`
		//  Estimated: `27264`
		// Minimum execution time: 240_023_000 picoseconds.
		Weight::from_parts(253_923_000, 27264)
			.saturating_add(T::DbWeight::get().reads(9_u64))
			.saturating_add(T::DbWeight::get().writes(10_u64))
	}
	/// Storage: DexGeneral PairStatuses (r:1 w:1)
	/// Proof: DexGeneral PairStatuses (max_values: None, max_size: Some(163), added: 2638, mode: MaxEncodedLen)
	/// Storage: DexGeneral BootstrapRewards (r:1 w:1)
	/// Proof: DexGeneral BootstrapRewards (max_values: None, max_size: Some(27032), added: 29507, mode: MaxEncodedLen)
	/// Storage: DexGeneral BootstrapLimits (r:0 w:1)
	/// Proof: DexGeneral BootstrapLimits (max_values: None, max_size: Some(27032), added: 29507, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 10]`.
	/// The range of component `l` is `[1, 10]`.
	fn bootstrap_update	(r: u32, l: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `259 + r * (21 ±0)`
		//  Estimated: `34125`
		// Minimum execution time: 57_936_000 picoseconds.
		Weight::from_parts(57_288_965, 34125)
			// Standard Error: 53_814
			.saturating_add(Weight::from_parts(572_652, 0).saturating_mul(r.into()))
			// Standard Error: 53_814
			.saturating_add(Weight::from_parts(371_921, 0).saturating_mul(l.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: DexGeneral PairStatuses (r:1 w:1)
	/// Proof: DexGeneral PairStatuses (max_values: None, max_size: Some(163), added: 2638, mode: MaxEncodedLen)
	/// Storage: DexGeneral BootstrapPersonalSupply (r:1 w:1)
	/// Proof: DexGeneral BootstrapPersonalSupply (max_values: None, max_size: Some(102), added: 2577, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:4 w:4)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn bootstrap_refund	() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1046`
		//  Estimated: `22138`
		// Minimum execution time: 152_230_000 picoseconds.
		Weight::from_parts(168_048_000, 22138)
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
	/// Storage: DexGeneral PairStatuses (r:1 w:1)
	/// Proof: DexGeneral PairStatuses (max_values: None, max_size: Some(163), added: 2638, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:5 w:5)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: DexGeneral LiquidityPairs (r:1 w:0)
	/// Proof: DexGeneral LiquidityPairs (max_values: None, max_size: Some(50), added: 2525, mode: MaxEncodedLen)
	/// Storage: DexGeneral KLast (r:1 w:1)
	/// Proof: DexGeneral KLast (max_values: None, max_size: Some(62), added: 2537, mode: MaxEncodedLen)
	/// Storage: DexGeneral FeeMeta (r:1 w:0)
	/// Proof: DexGeneral FeeMeta (max_values: Some(1), max_size: Some(34), added: 529, mode: MaxEncodedLen)
	/// Storage: Tokens TotalIssuance (r:1 w:1)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(35), added: 2510, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn add_liquidity	() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `704`
		//  Estimated: `33222`
		// Minimum execution time: 265_466_000 picoseconds.
		Weight::from_parts(294_876_000, 33222)
			.saturating_add(T::DbWeight::get().reads(11_u64))
			.saturating_add(T::DbWeight::get().writes(9_u64))
	}
	/// Storage: DexGeneral PairStatuses (r:1 w:1)
	/// Proof: DexGeneral PairStatuses (max_values: None, max_size: Some(163), added: 2638, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:5 w:5)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: DexGeneral LiquidityPairs (r:1 w:0)
	/// Proof: DexGeneral LiquidityPairs (max_values: None, max_size: Some(50), added: 2525, mode: MaxEncodedLen)
	/// Storage: DexGeneral KLast (r:1 w:1)
	/// Proof: DexGeneral KLast (max_values: None, max_size: Some(62), added: 2537, mode: MaxEncodedLen)
	/// Storage: DexGeneral FeeMeta (r:1 w:0)
	/// Proof: DexGeneral FeeMeta (max_values: Some(1), max_size: Some(34), added: 529, mode: MaxEncodedLen)
	/// Storage: Tokens TotalIssuance (r:1 w:1)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(35), added: 2510, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn remove_liquidity	() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1188`
		//  Estimated: `33222`
		// Minimum execution time: 224_704_000 picoseconds.
		Weight::from_parts(228_516_000, 33222)
			.saturating_add(T::DbWeight::get().reads(11_u64))
			.saturating_add(T::DbWeight::get().writes(8_u64))
	}
	/// Storage: Tokens Accounts (r:20 w:20)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: DexGeneral PairStatuses (r:9 w:0)
	/// Proof: DexGeneral PairStatuses (max_values: None, max_size: Some(163), added: 2638, mode: MaxEncodedLen)
	/// Storage: System Account (r:9 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `a` is `[2, 10]`.
	fn swap_exact_assets_for_assets	(a: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0 + a * (435 ±0)`
		//  Estimated: `8211 + a * (8825 ±21)`
		// Minimum execution time: 161_917_000 picoseconds.
		Weight::from_parts(12_273_386, 8211)
			// Standard Error: 283_655
			.saturating_add(Weight::from_parts(78_807_907, 0).saturating_mul(a.into()))
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().reads((3_u64).saturating_mul(a.into())))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(a.into())))
			.saturating_add(Weight::from_parts(0, 8825).saturating_mul(a.into()))
	}
	/// Storage: Tokens Accounts (r:20 w:20)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: DexGeneral PairStatuses (r:9 w:0)
	/// Proof: DexGeneral PairStatuses (max_values: None, max_size: Some(163), added: 2638, mode: MaxEncodedLen)
	/// Storage: System Account (r:9 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `a` is `[2, 10]`.
	fn swap_assets_for_exact_assets	(a: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0 + a * (435 ±0)`
		//  Estimated: `8211 + a * (8825 ±0)`
		// Minimum execution time: 163_640_000 picoseconds.
		Weight::from_parts(14_064_501, 8211)
			// Standard Error: 299_174
			.saturating_add(Weight::from_parts(79_120_950, 0).saturating_mul(a.into()))
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().reads((3_u64).saturating_mul(a.into())))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(a.into())))
			.saturating_add(Weight::from_parts(0, 8825).saturating_mul(a.into()))
	}
	/// Storage: DexGeneral BootstrapRewards (r:1 w:1)
	/// Proof: DexGeneral BootstrapRewards (max_values: None, max_size: Some(27032), added: 29507, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:20 w:20)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 10]`.
	fn bootstrap_charge_reward	(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `376 + r * (88 ±0)`
		//  Estimated: `35080 + r * (5180 ±0)`
		// Minimum execution time: 108_140_000 picoseconds.
		Weight::from_parts(68_067_108, 35080)
			// Standard Error: 227_743
			.saturating_add(Weight::from_parts(53_817_168, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(r.into())))
			.saturating_add(T::DbWeight::get().writes(2_u64))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(r.into())))
			.saturating_add(Weight::from_parts(0, 5180).saturating_mul(r.into()))
	}
	/// Storage: DexGeneral BootstrapRewards (r:1 w:1)
	/// Proof: DexGeneral BootstrapRewards (max_values: None, max_size: Some(27032), added: 29507, mode: MaxEncodedLen)
	fn bootstrap_withdraw_reward	() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `135`
		//  Estimated: `30497`
		// Minimum execution time: 37_512_000 picoseconds.
		Weight::from_parts(44_039_000, 30497)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}