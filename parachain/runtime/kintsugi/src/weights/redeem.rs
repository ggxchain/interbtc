
//! Autogenerated weights for redeem
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-08-07, STEPS: `50`, REPEAT: `10`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `interlay-rust-runner-2mz2v-kcxvd`, CPU: `AMD EPYC 7502P 32-Core Processor`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("kintsugi-dev"), DB CACHE: 1024

// Executed Command:
// target/release/interbtc-parachain
// benchmark
// pallet
// --pallet
// *
// --extrinsic
// *
// --chain
// kintsugi-dev
// --execution=wasm
// --wasm-execution=compiled
// --steps
// 50
// --repeat
// 10
// --output
// parachain/runtime/kintsugi/src/weights/
// --template
// .deploy/runtime-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weights for redeem using the Substrate node and recommended hardware.
pub struct WeightInfo<T>(PhantomData<T>);

impl<T: frame_system::Config> redeem::WeightInfo for WeightInfo<T> {

	/// Storage: Tokens Accounts (r:2 w:1)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: Fee RedeemFee (r:1 w:0)
	/// Proof: Fee RedeemFee (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: Redeem RedeemTransactionSize (r:1 w:0)
	/// Proof: Redeem RedeemTransactionSize (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Oracle Aggregate (r:2 w:0)
	/// Proof: Oracle Aggregate (max_values: None, max_size: Some(44), added: 2519, mode: MaxEncodedLen)
	/// Storage: VaultRegistry Vaults (r:1 w:1)
	/// Proof: VaultRegistry Vaults (max_values: None, max_size: Some(260), added: 2735, mode: MaxEncodedLen)
	/// Storage: Redeem RedeemBtcDustValue (r:1 w:0)
	/// Proof: Redeem RedeemBtcDustValue (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: Security Nonce (r:1 w:1)
	/// Proof: Security Nonce (max_values: Some(1), max_size: Some(32), added: 527, mode: MaxEncodedLen)
	/// Storage: System ParentHash (r:1 w:0)
	/// Proof: System ParentHash (max_values: Some(1), max_size: Some(32), added: 527, mode: MaxEncodedLen)
	/// Storage: VaultRegistry PremiumRedeemThreshold (r:1 w:0)
	/// Proof: VaultRegistry PremiumRedeemThreshold (max_values: None, max_size: Some(54), added: 2529, mode: MaxEncodedLen)
	/// Storage: VaultStaking Nonce (r:1 w:0)
	/// Proof: VaultStaking Nonce (max_values: None, max_size: Some(74), added: 2549, mode: MaxEncodedLen)
	/// Storage: VaultStaking TotalCurrentStake (r:1 w:0)
	/// Proof: VaultStaking TotalCurrentStake (max_values: None, max_size: Some(106), added: 2581, mode: MaxEncodedLen)
	/// Storage: Loans UnderlyingAssetId (r:1 w:0)
	/// Proof: Loans UnderlyingAssetId (max_values: None, max_size: Some(38), added: 2513, mode: MaxEncodedLen)
	/// Storage: Loans Markets (r:2 w:0)
	/// Proof: Loans Markets (max_values: None, max_size: Some(160), added: 2635, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Loans LastAccruedInterestTime (r:1 w:1)
	/// Proof: Loans LastAccruedInterestTime (max_values: None, max_size: Some(35), added: 2510, mode: MaxEncodedLen)
	/// Storage: Tokens TotalIssuance (r:1 w:0)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(35), added: 2510, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Loans TotalBorrows (r:1 w:0)
	/// Proof: Loans TotalBorrows (max_values: None, max_size: Some(43), added: 2518, mode: MaxEncodedLen)
	/// Storage: Loans TotalReserves (r:1 w:0)
	/// Proof: Loans TotalReserves (max_values: None, max_size: Some(43), added: 2518, mode: MaxEncodedLen)
	/// Storage: Loans MinExchangeRate (r:1 w:0)
	/// Proof: Loans MinExchangeRate (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: Loans MaxExchangeRate (r:1 w:0)
	/// Proof: Loans MaxExchangeRate (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: Fee PremiumRedeemFee (r:1 w:0)
	/// Proof: Fee PremiumRedeemFee (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: Security ActiveBlockCount (r:1 w:0)
	/// Proof: Security ActiveBlockCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Redeem RedeemPeriod (r:1 w:0)
	/// Proof: Redeem RedeemPeriod (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: BTCRelay BestBlockHeight (r:1 w:0)
	/// Proof: BTCRelay BestBlockHeight (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Redeem RedeemRequests (r:0 w:1)
	/// Proof: Redeem RedeemRequests (max_values: None, max_size: Some(245), added: 2720, mode: MaxEncodedLen)
	fn request_redeem	() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3189`
		//  Estimated: `6260`
		// Minimum execution time: 311_614_000 picoseconds.
		Weight::from_parts(319_349_000, 6260)
			.saturating_add(T::DbWeight::get().reads(28_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	/// Storage: Tokens Accounts (r:3 w:3)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: Tokens TotalIssuance (r:1 w:1)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(35), added: 2510, mode: MaxEncodedLen)
	/// Storage: VaultRegistry LiquidationVault (r:1 w:1)
	/// Proof: VaultRegistry LiquidationVault (max_values: None, max_size: Some(124), added: 2599, mode: MaxEncodedLen)
	/// Storage: VaultRegistry TotalUserVaultCollateral (r:1 w:1)
	/// Proof: VaultRegistry TotalUserVaultCollateral (max_values: None, max_size: Some(54), added: 2529, mode: MaxEncodedLen)
	/// Storage: Loans UnderlyingAssetId (r:1 w:0)
	/// Proof: Loans UnderlyingAssetId (max_values: None, max_size: Some(38), added: 2513, mode: MaxEncodedLen)
	/// Storage: Loans RewardSupplyState (r:1 w:1)
	/// Proof: Loans RewardSupplyState (max_values: None, max_size: Some(47), added: 2522, mode: MaxEncodedLen)
	/// Storage: Loans RewardSupplySpeed (r:1 w:0)
	/// Proof: Loans RewardSupplySpeed (max_values: None, max_size: Some(43), added: 2518, mode: MaxEncodedLen)
	/// Storage: Loans RewardSupplierIndex (r:2 w:2)
	/// Proof: Loans RewardSupplierIndex (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	/// Storage: Loans Markets (r:1 w:0)
	/// Proof: Loans Markets (max_values: None, max_size: Some(160), added: 2635, mode: MaxEncodedLen)
	/// Storage: Loans RewardAccrued (r:2 w:2)
	/// Proof: Loans RewardAccrued (max_values: None, max_size: Some(64), added: 2539, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Loans AccountDeposits (r:1 w:0)
	/// Proof: Loans AccountDeposits (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	fn liquidation_redeem	() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2173`
		//  Estimated: `8760`
		// Minimum execution time: 287_687_000 picoseconds.
		Weight::from_parts(292_145_000, 8760)
			.saturating_add(T::DbWeight::get().reads(16_u64))
			.saturating_add(T::DbWeight::get().writes(11_u64))
	}
	/// Storage: Redeem RedeemRequests (r:1 w:1)
	/// Proof: Redeem RedeemRequests (max_values: None, max_size: Some(245), added: 2720, mode: MaxEncodedLen)
	/// Storage: BTCRelay DisableInclusionCheck (r:1 w:0)
	/// Proof: BTCRelay DisableInclusionCheck (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	/// Storage: BTCRelay BestBlockHeight (r:1 w:0)
	/// Proof: BTCRelay BestBlockHeight (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: BTCRelay Chains (r:1 w:0)
	/// Proof: BTCRelay Chains (max_values: None, max_size: Some(24), added: 2499, mode: MaxEncodedLen)
	/// Storage: BTCRelay BlockHeaders (r:1 w:0)
	/// Proof: BTCRelay BlockHeaders (max_values: None, max_size: Some(200), added: 2675, mode: MaxEncodedLen)
	/// Storage: BTCRelay StableBitcoinConfirmations (r:1 w:0)
	/// Proof: BTCRelay StableBitcoinConfirmations (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Security ActiveBlockCount (r:1 w:0)
	/// Proof: Security ActiveBlockCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: BTCRelay StableParachainConfirmations (r:1 w:0)
	/// Proof: BTCRelay StableParachainConfirmations (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:1 w:1)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: Tokens TotalIssuance (r:1 w:1)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(35), added: 2510, mode: MaxEncodedLen)
	/// Storage: VaultRegistry Vaults (r:1 w:1)
	/// Proof: VaultRegistry Vaults (max_values: None, max_size: Some(260), added: 2735, mode: MaxEncodedLen)
	/// The range of component `h` is `[2, 10]`.
	/// The range of component `i` is `[1, 10]`.
	/// The range of component `o` is `[2, 3]`.
	/// The range of component `b` is `[541, 2048]`.
	fn execute_redeem	(h: u32, i: u32, o: u32, b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2295 + o * (1 ±0)`
		//  Estimated: `3725`
		// Minimum execution time: 184_800_000 picoseconds.
		Weight::from_parts(149_034_719, 3725)
			// Standard Error: 137_585
			.saturating_add(Weight::from_parts(3_833_551, 0).saturating_mul(h.into()))
			// Standard Error: 124_035
			.saturating_add(Weight::from_parts(1_176_171, 0).saturating_mul(i.into()))
			// Standard Error: 751_913
			.saturating_add(Weight::from_parts(1_964_806, 0).saturating_mul(o.into()))
			// Standard Error: 757
			.saturating_add(Weight::from_parts(5_490, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(11_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: Redeem RedeemRequests (r:1 w:1)
	/// Proof: Redeem RedeemRequests (max_values: None, max_size: Some(245), added: 2720, mode: MaxEncodedLen)
	/// Storage: Redeem RedeemPeriod (r:1 w:0)
	/// Proof: Redeem RedeemPeriod (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Security ActiveBlockCount (r:1 w:0)
	/// Proof: Security ActiveBlockCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: BTCRelay BestBlockHeight (r:1 w:0)
	/// Proof: BTCRelay BestBlockHeight (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: VaultRegistry Vaults (r:1 w:1)
	/// Proof: VaultRegistry Vaults (max_values: None, max_size: Some(260), added: 2735, mode: MaxEncodedLen)
	/// Storage: Loans UnderlyingAssetId (r:1 w:0)
	/// Proof: Loans UnderlyingAssetId (max_values: None, max_size: Some(38), added: 2513, mode: MaxEncodedLen)
	/// Storage: Oracle Aggregate (r:1 w:0)
	/// Proof: Oracle Aggregate (max_values: None, max_size: Some(44), added: 2519, mode: MaxEncodedLen)
	/// Storage: Loans Markets (r:2 w:0)
	/// Proof: Loans Markets (max_values: None, max_size: Some(160), added: 2635, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Loans LastAccruedInterestTime (r:1 w:1)
	/// Proof: Loans LastAccruedInterestTime (max_values: None, max_size: Some(35), added: 2510, mode: MaxEncodedLen)
	/// Storage: Tokens TotalIssuance (r:2 w:1)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(35), added: 2510, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:4 w:3)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Loans TotalBorrows (r:1 w:0)
	/// Proof: Loans TotalBorrows (max_values: None, max_size: Some(43), added: 2518, mode: MaxEncodedLen)
	/// Storage: Loans TotalReserves (r:1 w:0)
	/// Proof: Loans TotalReserves (max_values: None, max_size: Some(43), added: 2518, mode: MaxEncodedLen)
	/// Storage: Loans MinExchangeRate (r:1 w:0)
	/// Proof: Loans MinExchangeRate (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: Loans MaxExchangeRate (r:1 w:0)
	/// Proof: Loans MaxExchangeRate (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: Fee PunishmentFee (r:1 w:0)
	/// Proof: Fee PunishmentFee (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: VaultStaking Nonce (r:1 w:0)
	/// Proof: VaultStaking Nonce (max_values: None, max_size: Some(74), added: 2549, mode: MaxEncodedLen)
	/// Storage: VaultStaking TotalCurrentStake (r:1 w:1)
	/// Proof: VaultStaking TotalCurrentStake (max_values: None, max_size: Some(106), added: 2581, mode: MaxEncodedLen)
	/// Storage: VaultRegistry TotalUserVaultCollateral (r:1 w:1)
	/// Proof: VaultRegistry TotalUserVaultCollateral (max_values: None, max_size: Some(54), added: 2529, mode: MaxEncodedLen)
	/// Storage: VaultCapacity Stake (r:1 w:1)
	/// Proof: VaultCapacity Stake (max_values: None, max_size: Some(43), added: 2518, mode: MaxEncodedLen)
	/// Storage: VaultCapacity RewardPerToken (r:2 w:0)
	/// Proof: VaultCapacity RewardPerToken (max_values: None, max_size: Some(59), added: 2534, mode: MaxEncodedLen)
	/// Storage: VaultCapacity RewardTally (r:2 w:2)
	/// Proof: VaultCapacity RewardTally (max_values: None, max_size: Some(70), added: 2545, mode: MaxEncodedLen)
	/// Storage: VaultCapacity TotalRewards (r:2 w:2)
	/// Proof: VaultCapacity TotalRewards (max_values: None, max_size: Some(43), added: 2518, mode: MaxEncodedLen)
	/// Storage: VaultRewards Stake (r:1 w:1)
	/// Proof: VaultRewards Stake (max_values: None, max_size: Some(97), added: 2572, mode: MaxEncodedLen)
	/// Storage: VaultRewards RewardPerToken (r:2 w:0)
	/// Proof: VaultRewards RewardPerToken (max_values: None, max_size: Some(70), added: 2545, mode: MaxEncodedLen)
	/// Storage: VaultRewards RewardTally (r:2 w:2)
	/// Proof: VaultRewards RewardTally (max_values: None, max_size: Some(124), added: 2599, mode: MaxEncodedLen)
	/// Storage: VaultRewards TotalRewards (r:2 w:2)
	/// Proof: VaultRewards TotalRewards (max_values: None, max_size: Some(43), added: 2518, mode: MaxEncodedLen)
	/// Storage: Fee Commission (r:1 w:0)
	/// Proof: Fee Commission (max_values: None, max_size: Some(86), added: 2561, mode: MaxEncodedLen)
	/// Storage: VaultStaking RewardPerToken (r:2 w:2)
	/// Proof: VaultStaking RewardPerToken (max_values: None, max_size: Some(117), added: 2592, mode: MaxEncodedLen)
	/// Storage: VaultStaking TotalStake (r:1 w:0)
	/// Proof: VaultStaking TotalStake (max_values: None, max_size: Some(106), added: 2581, mode: MaxEncodedLen)
	/// Storage: VaultStaking SlashPerToken (r:1 w:1)
	/// Proof: VaultStaking SlashPerToken (max_values: None, max_size: Some(106), added: 2581, mode: MaxEncodedLen)
	/// Storage: VaultRegistry SecureCollateralThreshold (r:1 w:0)
	/// Proof: VaultRegistry SecureCollateralThreshold (max_values: None, max_size: Some(54), added: 2529, mode: MaxEncodedLen)
	/// Storage: VaultRewards TotalStake (r:1 w:1)
	/// Proof: VaultRewards TotalStake (max_values: None, max_size: Some(43), added: 2518, mode: MaxEncodedLen)
	/// Storage: VaultRewards RewardCurrencies (r:1 w:0)
	/// Proof: VaultRewards RewardCurrencies (max_values: None, max_size: Some(50), added: 2525, mode: MaxEncodedLen)
	/// Storage: VaultCapacity TotalStake (r:1 w:1)
	/// Proof: VaultCapacity TotalStake (max_values: None, max_size: Some(32), added: 2507, mode: MaxEncodedLen)
	/// Storage: VaultCapacity RewardCurrencies (r:1 w:0)
	/// Proof: VaultCapacity RewardCurrencies (max_values: None, max_size: Some(39), added: 2514, mode: MaxEncodedLen)
	/// Storage: Loans RewardSupplyState (r:1 w:1)
	/// Proof: Loans RewardSupplyState (max_values: None, max_size: Some(47), added: 2522, mode: MaxEncodedLen)
	/// Storage: Loans RewardSupplySpeed (r:1 w:0)
	/// Proof: Loans RewardSupplySpeed (max_values: None, max_size: Some(43), added: 2518, mode: MaxEncodedLen)
	/// Storage: Loans RewardSupplierIndex (r:2 w:2)
	/// Proof: Loans RewardSupplierIndex (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	/// Storage: Loans RewardAccrued (r:2 w:2)
	/// Proof: Loans RewardAccrued (max_values: None, max_size: Some(64), added: 2539, mode: MaxEncodedLen)
	/// Storage: Loans AccountDeposits (r:1 w:0)
	/// Proof: Loans AccountDeposits (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	/// Storage: VaultRegistry PunishmentDelay (r:1 w:0)
	/// Proof: VaultRegistry PunishmentDelay (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn cancel_redeem_reimburse	() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `5430`
		//  Estimated: `11350`
		// Minimum execution time: 850_042_000 picoseconds.
		Weight::from_parts(865_223_000, 11350)
			.saturating_add(T::DbWeight::get().reads(59_u64))
			.saturating_add(T::DbWeight::get().writes(29_u64))
	}
	/// Storage: Redeem RedeemRequests (r:1 w:1)
	/// Proof: Redeem RedeemRequests (max_values: None, max_size: Some(245), added: 2720, mode: MaxEncodedLen)
	/// Storage: Redeem RedeemPeriod (r:1 w:0)
	/// Proof: Redeem RedeemPeriod (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Security ActiveBlockCount (r:1 w:0)
	/// Proof: Security ActiveBlockCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: BTCRelay BestBlockHeight (r:1 w:0)
	/// Proof: BTCRelay BestBlockHeight (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: VaultRegistry Vaults (r:1 w:1)
	/// Proof: VaultRegistry Vaults (max_values: None, max_size: Some(260), added: 2735, mode: MaxEncodedLen)
	/// Storage: Loans UnderlyingAssetId (r:1 w:0)
	/// Proof: Loans UnderlyingAssetId (max_values: None, max_size: Some(38), added: 2513, mode: MaxEncodedLen)
	/// Storage: Oracle Aggregate (r:1 w:0)
	/// Proof: Oracle Aggregate (max_values: None, max_size: Some(44), added: 2519, mode: MaxEncodedLen)
	/// Storage: Loans Markets (r:2 w:0)
	/// Proof: Loans Markets (max_values: None, max_size: Some(160), added: 2635, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Loans LastAccruedInterestTime (r:1 w:1)
	/// Proof: Loans LastAccruedInterestTime (max_values: None, max_size: Some(35), added: 2510, mode: MaxEncodedLen)
	/// Storage: Tokens TotalIssuance (r:1 w:0)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(35), added: 2510, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:4 w:3)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Loans TotalBorrows (r:1 w:0)
	/// Proof: Loans TotalBorrows (max_values: None, max_size: Some(43), added: 2518, mode: MaxEncodedLen)
	/// Storage: Loans TotalReserves (r:1 w:0)
	/// Proof: Loans TotalReserves (max_values: None, max_size: Some(43), added: 2518, mode: MaxEncodedLen)
	/// Storage: Loans MinExchangeRate (r:1 w:0)
	/// Proof: Loans MinExchangeRate (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: Loans MaxExchangeRate (r:1 w:0)
	/// Proof: Loans MaxExchangeRate (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: Fee PunishmentFee (r:1 w:0)
	/// Proof: Fee PunishmentFee (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: VaultStaking Nonce (r:1 w:0)
	/// Proof: VaultStaking Nonce (max_values: None, max_size: Some(74), added: 2549, mode: MaxEncodedLen)
	/// Storage: VaultStaking TotalCurrentStake (r:1 w:1)
	/// Proof: VaultStaking TotalCurrentStake (max_values: None, max_size: Some(106), added: 2581, mode: MaxEncodedLen)
	/// Storage: VaultRegistry TotalUserVaultCollateral (r:1 w:1)
	/// Proof: VaultRegistry TotalUserVaultCollateral (max_values: None, max_size: Some(54), added: 2529, mode: MaxEncodedLen)
	/// Storage: VaultCapacity Stake (r:1 w:1)
	/// Proof: VaultCapacity Stake (max_values: None, max_size: Some(43), added: 2518, mode: MaxEncodedLen)
	/// Storage: VaultCapacity RewardPerToken (r:2 w:0)
	/// Proof: VaultCapacity RewardPerToken (max_values: None, max_size: Some(59), added: 2534, mode: MaxEncodedLen)
	/// Storage: VaultCapacity RewardTally (r:2 w:2)
	/// Proof: VaultCapacity RewardTally (max_values: None, max_size: Some(70), added: 2545, mode: MaxEncodedLen)
	/// Storage: VaultCapacity TotalRewards (r:2 w:2)
	/// Proof: VaultCapacity TotalRewards (max_values: None, max_size: Some(43), added: 2518, mode: MaxEncodedLen)
	/// Storage: VaultRewards Stake (r:1 w:1)
	/// Proof: VaultRewards Stake (max_values: None, max_size: Some(97), added: 2572, mode: MaxEncodedLen)
	/// Storage: VaultRewards RewardPerToken (r:2 w:0)
	/// Proof: VaultRewards RewardPerToken (max_values: None, max_size: Some(70), added: 2545, mode: MaxEncodedLen)
	/// Storage: VaultRewards RewardTally (r:2 w:2)
	/// Proof: VaultRewards RewardTally (max_values: None, max_size: Some(124), added: 2599, mode: MaxEncodedLen)
	/// Storage: VaultRewards TotalRewards (r:2 w:2)
	/// Proof: VaultRewards TotalRewards (max_values: None, max_size: Some(43), added: 2518, mode: MaxEncodedLen)
	/// Storage: Fee Commission (r:1 w:0)
	/// Proof: Fee Commission (max_values: None, max_size: Some(86), added: 2561, mode: MaxEncodedLen)
	/// Storage: VaultStaking RewardPerToken (r:2 w:2)
	/// Proof: VaultStaking RewardPerToken (max_values: None, max_size: Some(117), added: 2592, mode: MaxEncodedLen)
	/// Storage: VaultStaking TotalStake (r:1 w:0)
	/// Proof: VaultStaking TotalStake (max_values: None, max_size: Some(106), added: 2581, mode: MaxEncodedLen)
	/// Storage: VaultStaking SlashPerToken (r:1 w:1)
	/// Proof: VaultStaking SlashPerToken (max_values: None, max_size: Some(106), added: 2581, mode: MaxEncodedLen)
	/// Storage: VaultRegistry SecureCollateralThreshold (r:1 w:0)
	/// Proof: VaultRegistry SecureCollateralThreshold (max_values: None, max_size: Some(54), added: 2529, mode: MaxEncodedLen)
	/// Storage: VaultRewards TotalStake (r:1 w:1)
	/// Proof: VaultRewards TotalStake (max_values: None, max_size: Some(43), added: 2518, mode: MaxEncodedLen)
	/// Storage: VaultRewards RewardCurrencies (r:1 w:0)
	/// Proof: VaultRewards RewardCurrencies (max_values: None, max_size: Some(50), added: 2525, mode: MaxEncodedLen)
	/// Storage: VaultCapacity TotalStake (r:1 w:1)
	/// Proof: VaultCapacity TotalStake (max_values: None, max_size: Some(32), added: 2507, mode: MaxEncodedLen)
	/// Storage: VaultCapacity RewardCurrencies (r:1 w:0)
	/// Proof: VaultCapacity RewardCurrencies (max_values: None, max_size: Some(39), added: 2514, mode: MaxEncodedLen)
	/// Storage: Loans RewardSupplyState (r:1 w:1)
	/// Proof: Loans RewardSupplyState (max_values: None, max_size: Some(47), added: 2522, mode: MaxEncodedLen)
	/// Storage: Loans RewardSupplySpeed (r:1 w:0)
	/// Proof: Loans RewardSupplySpeed (max_values: None, max_size: Some(43), added: 2518, mode: MaxEncodedLen)
	/// Storage: Loans RewardSupplierIndex (r:2 w:2)
	/// Proof: Loans RewardSupplierIndex (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	/// Storage: Loans RewardAccrued (r:2 w:2)
	/// Proof: Loans RewardAccrued (max_values: None, max_size: Some(64), added: 2539, mode: MaxEncodedLen)
	/// Storage: Loans AccountDeposits (r:1 w:0)
	/// Proof: Loans AccountDeposits (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	/// Storage: VaultRegistry PunishmentDelay (r:1 w:0)
	/// Proof: VaultRegistry PunishmentDelay (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn cancel_redeem_retry	() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `5430`
		//  Estimated: `11350`
		// Minimum execution time: 773_779_000 picoseconds.
		Weight::from_parts(789_120_000, 11350)
			.saturating_add(T::DbWeight::get().reads(58_u64))
			.saturating_add(T::DbWeight::get().writes(28_u64))
	}
	/// Storage: Redeem RedeemPeriod (r:0 w:1)
	/// Proof: Redeem RedeemPeriod (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn set_redeem_period	() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 18_106_000 picoseconds.
		Weight::from_parts(18_667_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: VaultRegistry Vaults (r:1 w:1)
	/// Proof: VaultRegistry Vaults (max_values: None, max_size: Some(260), added: 2735, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:1 w:1)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(115), added: 2590, mode: MaxEncodedLen)
	/// Storage: Tokens TotalIssuance (r:1 w:1)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(35), added: 2510, mode: MaxEncodedLen)
	fn self_redeem	() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1427`
		//  Estimated: `3725`
		// Minimum execution time: 150_121_000 picoseconds.
		Weight::from_parts(151_013_000, 3725)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
}