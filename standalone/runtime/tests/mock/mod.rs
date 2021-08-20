#![allow(dead_code)]

extern crate hex;

use bitcoin::types::TransactionInputSource;
pub use bitcoin::{
    formatter::{Formattable, TryFormattable},
    types::*,
};
pub use btc_relay::{BtcAddress, BtcPublicKey};
pub use currency::ParachainCurrency;
use frame_support::traits::GenesisBuild;
pub use frame_support::{assert_err, assert_noop, assert_ok, dispatch::DispatchResultWithPostInfo};
pub use interbtc_runtime_standalone::{
    AccountId, Balance, BlockNumber, Call, CurrencyId, Event, GetCollateralCurrencyId, GetWrappedCurrencyId, Runtime,
    DOT, INTERBTC,
};
pub use mocktopus::mocking::*;
pub use orml_tokens::CurrencyAdapter;
use redeem::RedeemRequestStatus;
pub use security::{ErrorCode, StatusCode};
pub use sp_arithmetic::{FixedI128, FixedPointNumber, FixedU128};
pub use sp_core::{H160, H256, U256};
pub use sp_runtime::traits::{Dispatchable, One, Zero};
pub use sp_std::convert::TryInto;
pub use vault_registry::CurrencySource;

pub use exchange_rate_oracle::OracleKey;
pub use issue::{IssueRequest, IssueRequestStatus};
pub use redeem::RedeemRequest;
pub use refund::RefundRequest;
pub use replace::ReplaceRequest;
pub use reward::Rewards;
pub use sp_runtime::AccountId32;
use std::collections::HashMap;
pub use std::convert::TryFrom;
pub use vault_registry::{Vault, VaultStatus};

use self::redeem_testing_utils::USER_BTC_ADDRESS;

pub mod issue_testing_utils;
pub mod nomination_testing_utils;
pub mod redeem_testing_utils;
pub mod reward_testing_utils;

pub const ALICE: [u8; 32] = [0u8; 32];
pub const BOB: [u8; 32] = [1u8; 32];
pub const CAROL: [u8; 32] = [2u8; 32];
pub const DAVE: [u8; 32] = [10u8; 32];
pub const EVE: [u8; 32] = [11u8; 32];
pub const FRANK: [u8; 32] = [12u8; 32];
pub const GRACE: [u8; 32] = [13u8; 32];

pub const FAUCET: [u8; 32] = [128u8; 32];
pub const DUMMY: [u8; 32] = [255u8; 32];

pub const INITIAL_BALANCE: u128 = 1_000_000_000_000;

pub const DEFAULT_USER_FREE_BALANCE: u128 = 1_000_000;
pub const DEFAULT_USER_LOCKED_BALANCE: u128 = 100_000;
pub const DEFAULT_USER_FREE_TOKENS: u128 = 10_000_000;
pub const DEFAULT_USER_LOCKED_TOKENS: u128 = 1000;

pub const DEFAULT_VAULT_TO_BE_ISSUED: u128 = 10_000;
pub const DEFAULT_VAULT_ISSUED: u128 = 100_000;
pub const DEFAULT_VAULT_TO_BE_REDEEMED: u128 = 20_000;
pub const DEFAULT_VAULT_BACKING_COLLATERAL: u128 = 1_000_000;
pub const DEFAULT_VAULT_GRIEFING_COLLATERAL: u128 = 30_000;
pub const DEFAULT_VAULT_FREE_BALANCE: u128 = 200_000;
pub const DEFAULT_VAULT_FREE_TOKENS: u128 = 0;
pub const DEFAULT_VAULT_REPLACE_COLLATERAL: u128 = 20_000;
pub const DEFAULT_VAULT_TO_BE_REPLACED: u128 = 40_000;

pub const DEFAULT_NOMINATION_TOTAL_NOMINATED_COLLATERAL: u128 = 0;
pub const DEFAULT_NOMINATION_COLLATERAL_TO_BE_WITHDRAWN: u128 = 0;

pub const CONFIRMATIONS: u32 = 6;

pub type BTCRelayCall = btc_relay::Call<Runtime>;
pub type BTCRelayPallet = btc_relay::Pallet<Runtime>;
pub type BTCRelayError = btc_relay::Error<Runtime>;
pub type BTCRelayEvent = btc_relay::Event<Runtime>;

pub type TokensError = orml_tokens::Error<Runtime>;

pub type CollateralPallet = CurrencyAdapter<Runtime, GetCollateralCurrencyId>;
pub type TreasuryPallet = CurrencyAdapter<Runtime, GetWrappedCurrencyId>;

pub type ExchangeRateOracleCall = exchange_rate_oracle::Call<Runtime>;
pub type ExchangeRateOraclePallet = exchange_rate_oracle::Pallet<Runtime>;

pub type FeeCall = fee::Call<Runtime>;
pub type FeeError = fee::Error<Runtime>;
pub type FeePallet = fee::Pallet<Runtime>;

pub type VaultRewardsPallet = reward::Pallet<Runtime>;
pub type VaultStakingPallet = staking::Pallet<Runtime>;

pub type IssueCall = issue::Call<Runtime>;
pub type IssuePallet = issue::Pallet<Runtime>;
pub type IssueEvent = issue::Event<Runtime>;
pub type IssueError = issue::Error<Runtime>;

pub type RefundCall = refund::Call<Runtime>;
pub type RefundPallet = refund::Pallet<Runtime>;
pub type RefundEvent = refund::Event<Runtime>;
pub type RefundError = refund::Error<Runtime>;

pub type RedeemCall = redeem::Call<Runtime>;
pub type RedeemPallet = redeem::Pallet<Runtime>;
pub type RedeemError = redeem::Error<Runtime>;
pub type RedeemEvent = redeem::Event<Runtime>;

pub type ReplaceCall = replace::Call<Runtime>;
pub type ReplaceEvent = replace::Event<Runtime>;
pub type ReplaceError = replace::Error<Runtime>;
pub type ReplacePallet = replace::Pallet<Runtime>;

pub type SecurityError = security::Error<Runtime>;
pub type SecurityPallet = security::Pallet<Runtime>;

pub type RelayCall = relay::Call<Runtime>;
pub type RelayPallet = relay::Pallet<Runtime>;

pub type SystemModule = frame_system::Pallet<Runtime>;

pub type VaultRegistryCall = vault_registry::Call<Runtime>;
pub type VaultRegistryError = vault_registry::Error<Runtime>;
pub type VaultRegistryPallet = vault_registry::Pallet<Runtime>;

pub type NominationCall = nomination::Call<Runtime>;
pub type NominationError = nomination::Error<Runtime>;
pub type NominationEvent = nomination::Event<Runtime>;
pub type NominationPallet = nomination::Pallet<Runtime>;

pub const DEFAULT_TESTING_CURRENCY: <Runtime as orml_tokens::Config>::CurrencyId =
    <Runtime as orml_tokens::Config>::CurrencyId::DOT;
pub const GRIEFING_CURRENCY: <Runtime as orml_tokens::Config>::CurrencyId =
    <Runtime as orml_tokens::Config>::CurrencyId::DOT;

pub fn default_user_state() -> UserData {
    let mut balances = HashMap::new();
    for currency_id in iter_collateral_currencies() {
        balances.insert(
            currency_id,
            AccountBalance {
                free: DEFAULT_USER_FREE_BALANCE,
                locked: DEFAULT_USER_LOCKED_BALANCE,
            },
        );
    }
    balances.insert(
        CurrencyId::INTERBTC,
        AccountBalance {
            free: DEFAULT_USER_FREE_TOKENS,
            locked: DEFAULT_USER_LOCKED_TOKENS,
        },
    );
    UserData { balances }
}

pub fn default_vault_state(currency_id: CurrencyId) -> CoreVaultData {
    CoreVaultData {
        to_be_issued: DEFAULT_VAULT_TO_BE_ISSUED,
        issued: DEFAULT_VAULT_ISSUED,
        to_be_redeemed: DEFAULT_VAULT_TO_BE_REDEEMED,
        backing_collateral: DEFAULT_VAULT_BACKING_COLLATERAL,
        griefing_collateral: DEFAULT_VAULT_GRIEFING_COLLATERAL,
        free_balance: iter_all_currencies().map(|x| (x, DEFAULT_VAULT_FREE_BALANCE)).collect(),
        liquidated_collateral: 0,
        replace_collateral: DEFAULT_VAULT_REPLACE_COLLATERAL,
        to_be_replaced: DEFAULT_VAULT_TO_BE_REPLACED,
        collateral_currency: currency_id,
    }
}

pub fn default_liquidation_vault_state() -> LiquidationVaultData {
    LiquidationVaultData {
        funds: iter_all_currencies().map(|currency| (currency, 0)).collect(),
        ..Default::default()
    }
}

pub fn premium_redeem_request(
    user_to_redeem: u128,
    vault: [u8; 32],
    user: [u8; 32],
) -> RedeemRequest<AccountId, BlockNumber, Balance> {
    let redeem_fee = FeePallet::get_redeem_fee(user_to_redeem).unwrap();
    let burned_tokens = user_to_redeem - redeem_fee;
    let inclusion_fee = RedeemPallet::get_current_inclusion_fee().unwrap();
    let premium_redeem_fee = FeePallet::get_premium_redeem_fee(burned_tokens - inclusion_fee).unwrap();

    RedeemRequest {
        premium: premium_redeem_fee,
        ..default_redeem_request(user_to_redeem, vault, user)
    }
}

pub fn default_redeem_request(
    user_to_redeem: u128,
    vault: [u8; 32],
    user: [u8; 32],
) -> RedeemRequest<AccountId, BlockNumber, Balance> {
    let redeem_fee = FeePallet::get_redeem_fee(user_to_redeem).unwrap();
    let burned_tokens = user_to_redeem - redeem_fee;
    let inclusion_fee = RedeemPallet::get_current_inclusion_fee().unwrap();
    let redeem_period = RedeemPallet::redeem_period();
    let btc_height = BTCRelayPallet::get_best_block_height();
    let opentime = SecurityPallet::active_block_number();

    RedeemRequest {
        vault: account_of(vault),
        opentime,
        fee: redeem_fee,
        transfer_fee_btc: inclusion_fee,
        amount_btc: burned_tokens - inclusion_fee,
        premium: 0,
        period: redeem_period,
        redeemer: account_of(user),
        btc_address: USER_BTC_ADDRESS,
        btc_height,
        status: RedeemRequestStatus::Pending,
    }
}

pub fn root() -> <Runtime as frame_system::Config>::Origin {
    <Runtime as frame_system::Config>::Origin::root()
}

pub fn origin_of(account_id: AccountId) -> <Runtime as frame_system::Config>::Origin {
    <Runtime as frame_system::Config>::Origin::signed(account_id)
}

pub fn account_of(address: [u8; 32]) -> AccountId {
    AccountId::from(address)
}

#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct AccountBalance {
    pub free: u128,
    pub locked: u128,
}

#[derive(Debug, PartialEq, Default, Clone)]
pub struct UserData {
    pub balances: HashMap<CurrencyId, AccountBalance>,
}

pub fn iter_collateral_currencies() -> impl Iterator<Item = CurrencyId> {
    vec![CurrencyId::DOT, CurrencyId::KSM].into_iter()
}

pub fn iter_all_currencies() -> impl Iterator<Item = CurrencyId> {
    iter_collateral_currencies().chain(vec![CurrencyId::INTERBTC].into_iter())
}

impl UserData {
    #[allow(dead_code)]
    pub fn get(id: [u8; 32]) -> Self {
        let account_id = account_of(id);
        let mut hash_map = HashMap::new();

        for currency_id in iter_all_currencies() {
            let free = currency::with_currency_id::get_free_balance::<Runtime>(currency_id, &account_id);
            let locked = currency::with_currency_id::get_reserved_balance::<Runtime>(currency_id, &account_id);
            hash_map.insert(currency_id, AccountBalance { free, locked });
        }

        Self { balances: hash_map }
    }
    #[allow(dead_code)]
    pub fn force_to(id: [u8; 32], new: Self) -> Self {
        let old = Self::get(id);
        let account_id = account_of(id);

        // Clear collateral currencies:
        for (currency_id, balance) in old.balances.iter() {
            currency::with_currency_id::transfer::<Runtime>(
                *currency_id,
                &account_id,
                &account_of(FAUCET),
                balance.free,
            )
            .unwrap();
            currency::with_currency_id::slash::<Runtime>(
                *currency_id,
                &account_id,
                &account_of(FAUCET),
                balance.locked,
            )
            .unwrap();
        }

        for (currency_id, balance) in new.balances.iter() {
            // set free balance:
            currency::with_currency_id::transfer::<Runtime>(
                *currency_id,
                &account_of(FAUCET),
                &account_id,
                balance.free,
            )
            .unwrap();

            // set locked balance:
            currency::with_currency_id::transfer_and_lock::<Runtime>(
                *currency_id,
                &account_of(FAUCET),
                &account_id,
                balance.locked,
            )
            .unwrap();
        }

        // sanity check:
        assert_eq!(Self::get(id), new);

        new
    }
}

#[derive(Debug, Default, Clone)]
pub struct FeePool {
    pub vault_rewards: u128,
}

fn abs_difference<T: std::ops::Sub<Output = T> + Ord>(x: T, y: T) -> T {
    if x < y {
        y - x
    } else {
        x - y
    }
}

impl PartialEq for FeePool {
    fn eq(&self, rhs: &Self) -> bool {
        abs_difference(self.vault_rewards, rhs.vault_rewards) <= 1
    }
}

impl FeePool {
    pub fn get() -> Self {
        Self {
            vault_rewards: VaultRewardsPallet::get_total_rewards(INTERBTC).unwrap() as u128,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CoreVaultData {
    pub to_be_issued: u128,
    pub issued: u128,
    pub to_be_redeemed: u128,
    pub backing_collateral: u128,
    pub griefing_collateral: u128,
    pub liquidated_collateral: u128,
    pub free_balance: HashMap<CurrencyId, u128>,
    pub to_be_replaced: u128,
    pub replace_collateral: u128,
    pub collateral_currency: CurrencyId,
}

impl Default for CoreVaultData {
    fn default() -> Self {
        Self {
            to_be_issued: 0,
            issued: 0,
            to_be_redeemed: 0,
            backing_collateral: 0,
            griefing_collateral: 0,
            liquidated_collateral: 0,
            free_balance: iter_all_currencies().map(|x| (x, 0)).collect(),
            to_be_replaced: 0,
            replace_collateral: 0,
            collateral_currency: DOT,
        }
    }
}

impl CoreVaultData {
    #[allow(dead_code)]
    pub fn vault(vault: [u8; 32]) -> Self {
        let account_id = account_of(vault);
        let vault = VaultRegistryPallet::get_vault_from_id(&account_id).unwrap();
        Self {
            to_be_issued: vault.to_be_issued_tokens,
            issued: vault.issued_tokens,
            to_be_redeemed: vault.to_be_redeemed_tokens,
            backing_collateral: CurrencySource::<Runtime>::Collateral(account_id.clone())
                .current_balance(vault.currency_id)
                .unwrap(),
            griefing_collateral: CurrencySource::<Runtime>::Griefing(account_id.clone())
                .current_balance(<Runtime as vault_registry::Config>::GetGriefingCollateralCurrencyId::get())
                .unwrap(),
            liquidated_collateral: vault.liquidated_collateral,
            free_balance: iter_all_currencies()
                .map(|currency_id| {
                    (
                        currency_id,
                        CurrencySource::<Runtime>::FreeBalance(account_id.clone())
                            .current_balance(currency_id)
                            .unwrap(),
                    )
                })
                .collect(),
            to_be_replaced: vault.to_be_replaced_tokens,
            replace_collateral: vault.replace_collateral,
            collateral_currency: vault.currency_id,
        }
    }

    #[allow(dead_code)]
    pub fn force_to(vault: [u8; 32], state: CoreVaultData) {
        // replace collateral is part of griefing collateral, so it needs to smaller or equal
        assert!(state.griefing_collateral >= state.replace_collateral);
        assert!(state.to_be_replaced + state.to_be_redeemed <= state.issued);

        // register vault if not yet registered
        try_register_vault(state.collateral_currency, 100, vault);

        // todo: check that currency did not change
        let currency_id = VaultRegistryPallet::get_vault_from_id(&account_of(vault))
            .unwrap()
            .currency_id;

        // temporarily give vault a lot of backing collateral so we can set issued & to-be-issued to whatever we want
        VaultRegistryPallet::transfer_funds(
            currency_id,
            CurrencySource::FreeBalance(account_of(FAUCET)),
            CurrencySource::Collateral(account_of(vault)),
            currency::with_currency_id::get_free_balance::<Runtime>(currency_id, &account_of(FAUCET)),
        )
        .unwrap();

        let current = CoreVaultData::vault(vault);

        // set all token types to 0
        assert_ok!(VaultRegistryPallet::decrease_to_be_issued_tokens(
            &account_of(vault),
            current.to_be_issued
        ));
        assert_ok!(VaultRegistryPallet::decrease_to_be_redeemed_tokens(
            &account_of(vault),
            current.to_be_redeemed
        ));
        assert_ok!(VaultRegistryPallet::try_increase_to_be_redeemed_tokens(
            &account_of(vault),
            current.issued
        ));
        assert_ok!(VaultRegistryPallet::decrease_tokens(
            &account_of(vault),
            &account_of(DUMMY),
            current.issued,
        ));
        assert_ok!(VaultRegistryPallet::decrease_to_be_replaced_tokens(
            &account_of(vault),
            current.to_be_replaced,
        ));

        // set to-be-issued
        assert_ok!(VaultRegistryPallet::try_increase_to_be_issued_tokens(
            &account_of(vault),
            state.to_be_issued
        ));
        // set issued (2 steps)
        assert_ok!(VaultRegistryPallet::try_increase_to_be_issued_tokens(
            &account_of(vault),
            state.issued
        ));
        assert_ok!(VaultRegistryPallet::issue_tokens(&account_of(vault), state.issued));
        // set to-be-redeemed
        assert_ok!(VaultRegistryPallet::try_increase_to_be_redeemed_tokens(
            &account_of(vault),
            state.to_be_redeemed
        ));
        // set to-be-replaced:
        assert_ok!(VaultRegistryPallet::try_increase_to_be_replaced_tokens(
            &account_of(vault),
            state.to_be_replaced,
            state.replace_collateral
        ));

        // clear all balances
        for currency_id in iter_all_currencies() {
            VaultRegistryPallet::transfer_funds(
                currency_id,
                CurrencySource::FreeBalance(account_of(vault)),
                CurrencySource::FreeBalance(account_of(FAUCET)),
                CurrencySource::<Runtime>::FreeBalance(account_of(vault))
                    .current_balance(currency_id)
                    .unwrap(),
            )
            .unwrap();

            VaultRegistryPallet::transfer_funds(
                currency_id,
                CurrencySource::FreeBalance(account_of(FAUCET)),
                CurrencySource::FreeBalance(account_of(vault)),
                state.free_balance.get(&currency_id).copied().unwrap_or(0),
            )
            .unwrap();
        }
        VaultRegistryPallet::transfer_funds(
            <Runtime as vault_registry::Config>::GetGriefingCollateralCurrencyId::get(),
            CurrencySource::Griefing(account_of(vault)),
            CurrencySource::FreeBalance(account_of(FAUCET)),
            CurrencySource::<Runtime>::Griefing(account_of(vault))
                .current_balance(<Runtime as vault_registry::Config>::GetGriefingCollateralCurrencyId::get())
                .unwrap(),
        )
        .unwrap();

        // vault's backing collateral was temporarily increased - reset to 0
        VaultRegistryPallet::transfer_funds(
            currency_id,
            CurrencySource::Collateral(account_of(vault)),
            CurrencySource::FreeBalance(account_of(FAUCET)),
            CurrencySource::<Runtime>::Collateral(account_of(vault))
                .current_balance(currency_id)
                .unwrap(),
        )
        .unwrap();

        // set backing and griefing collateral
        VaultRegistryPallet::transfer_funds(
            currency_id,
            CurrencySource::FreeBalance(account_of(FAUCET)),
            CurrencySource::Collateral(account_of(vault)),
            state.backing_collateral,
        )
        .unwrap();
        VaultRegistryPallet::transfer_funds(
            <Runtime as vault_registry::Config>::GetGriefingCollateralCurrencyId::get(),
            CurrencySource::FreeBalance(account_of(FAUCET)),
            CurrencySource::Griefing(account_of(vault)),
            state.griefing_collateral,
        )
        .unwrap();

        // check that we achieved the desired state
        assert_eq!(CoreVaultData::vault(vault), state);
    }
}

#[derive(Debug, PartialEq, Default, Clone)]
pub struct LiquidationVaultData {
    pub to_be_issued: u128,
    pub issued: u128,
    pub to_be_redeemed: u128,
    pub funds: HashMap<CurrencyId, u128>,
}

impl LiquidationVaultData {
    #[allow(dead_code)]
    pub fn get() -> Self {
        let vault = VaultRegistryPallet::get_liquidation_vault();

        let mut ret = Self {
            to_be_issued: vault.to_be_issued_tokens,
            issued: vault.issued_tokens,
            to_be_redeemed: vault.to_be_redeemed_tokens,
            funds: Default::default(),
        };

        for currency_id in iter_all_currencies() {
            ret.funds.insert(
                currency_id,
                CurrencySource::<Runtime>::LiquidationVault
                    .current_balance(currency_id)
                    .unwrap(),
            );
        }

        ret
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CoreNominatorData {
    pub collateral_to_be_withdrawn: u128,
}

impl Default for CoreNominatorData {
    fn default() -> Self {
        CoreNominatorData {
            collateral_to_be_withdrawn: 0,
        }
    }
}

impl CoreNominatorData {}

#[derive(Debug, PartialEq, Clone)]
pub struct ParachainState {
    user: UserData,
    vault: CoreVaultData,
    liquidation_vault: LiquidationVaultData,
    fee_pool: FeePool,
}

impl ParachainState {
    pub fn get_default(vault_currency: CurrencyId) -> Self {
        Self {
            user: default_user_state(),
            vault: default_vault_state(vault_currency),
            liquidation_vault: LiquidationVaultData {
                funds: iter_all_currencies().map(|currency| (currency, 0)).collect(),
                ..Default::default()
            },
            fee_pool: Default::default(),
        }
    }
}

impl ParachainState {
    pub fn get() -> Self {
        Self {
            user: UserData::get(ALICE),
            vault: CoreVaultData::vault(BOB),
            liquidation_vault: LiquidationVaultData::get(),
            fee_pool: FeePool::get(),
        }
    }

    pub fn with_changes(
        &self,
        f: impl FnOnce(&mut UserData, &mut CoreVaultData, &mut LiquidationVaultData, &mut FeePool),
    ) -> Self {
        let mut state = self.clone();
        f(
            &mut state.user,
            &mut state.vault,
            &mut state.liquidation_vault,
            &mut state.fee_pool,
        );
        state
    }
}

// todo: merge with ParachainState
#[derive(Debug, PartialEq, Clone)]
pub struct ParachainTwoVaultState {
    pub vault1: CoreVaultData,
    pub vault2: CoreVaultData,
    pub liquidation_vault: LiquidationVaultData,
}

impl ParachainTwoVaultState {
    pub fn get_default(currency_id: CurrencyId) -> Self {
        Self {
            vault1: default_vault_state(currency_id),
            vault2: default_vault_state(currency_id),
            liquidation_vault: LiquidationVaultData {
                funds: iter_all_currencies().map(|currency| (currency, 0)).collect(),
                ..Default::default()
            },
        }
    }
}

impl ParachainTwoVaultState {
    pub fn get() -> Self {
        Self {
            vault1: CoreVaultData::vault(BOB),
            vault2: CoreVaultData::vault(CAROL),
            liquidation_vault: LiquidationVaultData::get(),
        }
    }

    pub fn with_changes(
        &self,
        f: impl FnOnce(&mut CoreVaultData, &mut CoreVaultData, &mut LiquidationVaultData),
    ) -> Self {
        let mut state = self.clone();
        f(&mut state.vault1, &mut state.vault2, &mut state.liquidation_vault);
        state
    }
}
#[allow(dead_code)]
pub fn liquidate_vault(currency_id: CurrencyId, vault: [u8; 32]) {
    assert_ok!(ExchangeRateOraclePallet::_set_exchange_rate(
        currency_id,
        FixedU128::checked_from_integer(10_000_000_000).unwrap()
    ));
    assert_ok!(VaultRegistryPallet::liquidate_vault(&account_of(vault)));
    assert_ok!(ExchangeRateOraclePallet::_set_exchange_rate(
        currency_id,
        FixedU128::checked_from_integer(1).unwrap()
    ));
}

#[allow(dead_code)]
pub fn set_default_thresholds() {
    let secure = FixedU128::checked_from_rational(150, 100).unwrap();
    let premium = FixedU128::checked_from_rational(135, 100).unwrap();
    let liquidation = FixedU128::checked_from_rational(110, 100).unwrap();

    for currency_id in iter_collateral_currencies() {
        VaultRegistryPallet::set_secure_collateral_threshold(currency_id, secure);
        VaultRegistryPallet::set_premium_redeem_threshold(currency_id, premium);
        VaultRegistryPallet::set_liquidation_collateral_threshold(currency_id, liquidation);
    }
}

pub fn dummy_public_key() -> BtcPublicKey {
    BtcPublicKey([
        2, 205, 114, 218, 156, 16, 235, 172, 106, 37, 18, 153, 202, 140, 176, 91, 207, 51, 187, 55, 18, 45, 222, 180,
        119, 54, 243, 97, 173, 150, 161, 169, 230,
    ])
}

#[allow(dead_code)]
pub fn try_register_vault(currency_id: CurrencyId, collateral: u128, vault: [u8; 32]) {
    if VaultRegistryPallet::get_vault_from_id(&account_of(vault)).is_err() {
        assert_ok!(Call::VaultRegistry(VaultRegistryCall::register_vault(
            collateral,
            dummy_public_key(),
            currency_id
        ))
        .dispatch(origin_of(account_of(vault))));
    };
}

#[allow(dead_code)]
pub fn try_register_operator(operator: [u8; 32]) {
    let _ = Call::Nomination(NominationCall::opt_in_to_nomination()).dispatch(origin_of(account_of(operator)));
}

#[allow(dead_code)]
pub fn force_issue_tokens(currency_id: CurrencyId, user: [u8; 32], vault: [u8; 32], collateral: u128, tokens: u128) {
    // register the vault
    assert_ok!(Call::VaultRegistry(VaultRegistryCall::register_vault(
        collateral,
        dummy_public_key(),
        currency_id
    ))
    .dispatch(origin_of(account_of(vault))));

    // increase to be issued tokens
    assert_ok!(VaultRegistryPallet::try_increase_to_be_issued_tokens(
        &account_of(vault),
        tokens
    ));

    // issue tokens
    assert_ok!(VaultRegistryPallet::issue_tokens(&account_of(vault), tokens));

    // mint tokens to the user
    assert_ok!(TreasuryPallet::mint(&user.into(), tokens));
}

#[allow(dead_code)]
pub fn required_collateral_for_issue(issued_tokens: u128, currency_id: CurrencyId) -> u128 {
    let fee_amount_btc = FeePallet::get_issue_fee(issued_tokens).unwrap();
    let total_amount_btc = issued_tokens + fee_amount_btc;
    VaultRegistryPallet::get_required_collateral_for_wrapped(total_amount_btc, currency_id).unwrap()
}

pub fn assert_store_main_chain_header_event(height: u32, hash: H256Le, relayer: AccountId) {
    let store_event = Event::BTCRelay(BTCRelayEvent::StoreMainChainHeader(height, hash, relayer));
    let events = SystemModule::events();

    // store only main chain header
    assert!(events.iter().any(|a| a.event == store_event));
}

pub fn mine_blocks(blocks: u32) {
    let start_height = BTCRelayPallet::get_best_block_height();
    TransactionGenerator::new().with_confirmations(blocks).mine();
    let end_height = BTCRelayPallet::get_best_block_height();
    // sanity check
    assert_eq!(end_height, start_height + blocks);
}

#[derive(Default, Clone, Debug)]
pub struct TransactionGenerator {
    address: BtcAddress,
    amount: u128,
    return_data: Option<H256>,
    script: Vec<u8>,
    confirmations: u32,
    relayer: Option<[u8; 32]>,
}

impl TransactionGenerator {
    pub fn new() -> Self {
        Self {
            relayer: None,
            confirmations: 7,
            amount: 100,
            script: vec![
                0, 71, 48, 68, 2, 32, 91, 128, 41, 150, 96, 53, 187, 63, 230, 129, 53, 234, 210, 186, 21, 187, 98, 38,
                255, 112, 30, 27, 228, 29, 132, 140, 155, 62, 123, 216, 232, 168, 2, 32, 72, 126, 179, 207, 142, 8, 99,
                8, 32, 78, 244, 166, 106, 160, 207, 227, 61, 210, 172, 234, 234, 93, 59, 159, 79, 12, 194, 240, 212, 3,
                120, 50, 1, 71, 81, 33, 3, 113, 209, 131, 177, 9, 29, 242, 229, 15, 217, 247, 165, 78, 111, 80, 79, 50,
                200, 117, 80, 30, 233, 210, 167, 133, 175, 62, 253, 134, 127, 212, 51, 33, 2, 128, 200, 184, 235, 148,
                25, 43, 34, 28, 173, 55, 54, 189, 164, 187, 243, 243, 152, 7, 84, 210, 85, 156, 238, 77, 97, 188, 240,
                162, 197, 105, 62, 82, 174,
            ],
            return_data: Some(H256::zero()),
            ..Default::default()
        }
    }
    pub fn with_address(&mut self, address: BtcAddress) -> &mut Self {
        self.address = address;
        self
    }

    pub fn with_amount(&mut self, amount: u128) -> &mut Self {
        self.amount = amount;
        self
    }

    pub fn with_op_return(&mut self, op_return: Option<H256>) -> &mut Self {
        self.return_data = op_return;
        self
    }
    pub fn with_script(&mut self, script: &[u8]) -> &mut Self {
        self.script = script.to_vec();
        self
    }
    pub fn with_confirmations(&mut self, confirmations: u32) -> &mut Self {
        self.confirmations = confirmations;
        self
    }
    pub fn with_relayer(&mut self, relayer: Option<[u8; 32]>) -> &mut Self {
        self.relayer = relayer;
        self
    }
    pub fn mine(&self) -> (H256Le, u32, Vec<u8>, Vec<u8>, Transaction) {
        let mut height = 1;
        let extra_confirmations = self.confirmations - 1;

        // initialize BTC Relay with one block
        let init_block = BlockBuilder::new()
            .with_version(4)
            .with_coinbase(&self.address, 50, 3)
            .with_timestamp(1588813835)
            .mine(U256::from(2).pow(254.into()))
            .unwrap();

        let raw_init_block_header = RawBlockHeader::from_bytes(&init_block.header.try_format().unwrap())
            .expect("could not serialize block header");
        let init_block_header = BTCRelayPallet::parse_raw_block_header(&raw_init_block_header).unwrap();

        match BTCRelayPallet::initialize(account_of(ALICE), init_block_header, height) {
            Ok(_) => {}
            Err(e) if e == BTCRelayError::AlreadyInitialized.into() => {}
            _ => panic!("Failed to initialize btc relay"),
        }

        height = BTCRelayPallet::get_best_block_height() + 1;

        let value = self.amount as i64;
        let mut transaction_builder = TransactionBuilder::new();
        transaction_builder.with_version(2);
        transaction_builder.add_input(
            TransactionInputBuilder::new()
                .with_script(&self.script)
                .with_source(TransactionInputSource::FromOutput(init_block.transactions[0].hash(), 0))
                .build(),
        );

        transaction_builder.add_output(TransactionOutput::payment(value, &self.address));
        if let Some(op_return_data) = self.return_data {
            transaction_builder.add_output(TransactionOutput::op_return(0, op_return_data.as_bytes()));
        }

        let transaction = transaction_builder.build();

        let prev_hash = BTCRelayPallet::get_best_block();
        let block = BlockBuilder::new()
            .with_previous_hash(prev_hash)
            .with_version(4)
            .with_coinbase(&self.address, 50, 3)
            .with_timestamp(1588814835)
            .add_transaction(transaction.clone())
            .mine(U256::from(2).pow(254.into()))
            .unwrap();

        let raw_block_header =
            RawBlockHeader::from_bytes(&block.header.try_format().unwrap()).expect("could not serialize block header");

        let tx_id = transaction.tx_id();
        let tx_block_height = height;
        let proof = block.merkle_proof(&[tx_id]).unwrap();
        let bytes_proof = proof.try_format().unwrap();
        let raw_tx = transaction.format_with(true);

        self.relay(height, &block, raw_block_header);

        // Mine six new blocks to get over required confirmations
        let mut prev_block_hash = block.header.hash;
        let mut timestamp = 1588814835;
        for _ in 0..extra_confirmations {
            height += 1;
            timestamp += 1000;
            let conf_block = BlockBuilder::new()
                .with_previous_hash(prev_block_hash)
                .with_version(4)
                .with_coinbase(&self.address, 50, 3)
                .with_timestamp(timestamp)
                .mine(U256::from(2).pow(254.into()))
                .unwrap();

            let raw_conf_block_header = RawBlockHeader::from_bytes(&conf_block.header.try_format().unwrap())
                .expect("could not serialize block header");
            self.relay(height, &conf_block, raw_conf_block_header);

            prev_block_hash = conf_block.header.hash;
        }

        (tx_id, tx_block_height, bytes_proof, raw_tx, transaction)
    }

    fn relay(&self, height: u32, block: &Block, raw_block_header: RawBlockHeader) {
        if let Some(relayer) = self.relayer {
            assert_ok!(
                Call::Relay(RelayCall::store_block_header(raw_block_header)).dispatch(origin_of(account_of(relayer)))
            );
            assert_store_main_chain_header_event(height, raw_block_header.hash(), account_of(relayer));
        } else {
            // bypass staked relayer module
            let block_header = BTCRelayPallet::parse_raw_block_header(&raw_block_header).unwrap();
            assert_ok!(BTCRelayPallet::store_block_header(&account_of(ALICE), block_header));
            assert_store_main_chain_header_event(height, block.header.hash, account_of(ALICE));
        }
    }
}

#[allow(dead_code)]
pub fn generate_transaction_and_mine(
    address: BtcAddress,
    amount: u128,
    return_data: Option<H256>,
) -> (H256Le, u32, Vec<u8>, Vec<u8>) {
    let (tx_id, height, proof, raw_tx, _) = TransactionGenerator::new()
        .with_address(address)
        .with_amount(amount)
        .with_op_return(return_data)
        .mine();
    (tx_id, height, proof, raw_tx)
}

pub struct ExtBuilder {
    test_externalities: sp_io::TestExternalities,
}

impl ExtBuilder {
    pub fn build() -> Self {
        let mut storage = frame_system::GenesisConfig::default()
            .build_storage::<Runtime>()
            .unwrap();

        let balances = vec![
            (account_of(ALICE), INITIAL_BALANCE),
            (account_of(BOB), INITIAL_BALANCE),
            (account_of(CAROL), INITIAL_BALANCE),
            (account_of(DAVE), INITIAL_BALANCE),
            (account_of(EVE), INITIAL_BALANCE),
            (account_of(FRANK), INITIAL_BALANCE),
            (account_of(GRACE), INITIAL_BALANCE),
            (account_of(FAUCET), 1 << 60),
        ];

        let balances = balances
            .into_iter()
            .flat_map(|(account, balance)| {
                vec![
                    (account.clone(), CurrencyId::DOT, balance),
                    (account.clone(), CurrencyId::KSM, balance),
                ]
                .into_iter()
            })
            .chain(vec![(account_of(FAUCET), CurrencyId::INTERBTC, 1 << 60)].into_iter())
            .collect();

        orml_tokens::GenesisConfig::<Runtime> { balances }
            .assimilate_storage(&mut storage)
            .unwrap();

        exchange_rate_oracle::GenesisConfig::<Runtime> {
            authorized_oracles: vec![(account_of(BOB), BOB.to_vec())],
            max_delay: 3600000, // one hour
        }
        .assimilate_storage(&mut storage)
        .unwrap();

        btc_relay::GenesisConfig::<Runtime> {
            bitcoin_confirmations: CONFIRMATIONS,
            parachain_confirmations: CONFIRMATIONS,
            disable_difficulty_check: false,
            disable_inclusion_check: false,
            disable_op_return_check: false,
        }
        .assimilate_storage(&mut storage)
        .unwrap();

        vault_registry::GenesisConfig::<Runtime> {
            minimum_collateral_vault: vec![(CurrencyId::DOT, 0), (CurrencyId::KSM, 0)],
            punishment_delay: 8,
            secure_collateral_threshold: vec![
                (CurrencyId::DOT, FixedU128::checked_from_rational(150, 100).unwrap()),
                (CurrencyId::KSM, FixedU128::checked_from_rational(150, 100).unwrap()),
            ],
            premium_redeem_threshold: vec![
                (CurrencyId::DOT, FixedU128::checked_from_rational(135, 100).unwrap()),
                (CurrencyId::KSM, FixedU128::checked_from_rational(135, 100).unwrap()),
            ],
            liquidation_collateral_threshold: vec![
                (CurrencyId::DOT, FixedU128::checked_from_rational(110, 100).unwrap()),
                (CurrencyId::KSM, FixedU128::checked_from_rational(110, 100).unwrap()),
            ],
        }
        .assimilate_storage(&mut storage)
        .unwrap();

        issue::GenesisConfig::<Runtime> {
            issue_period: 10,
            issue_btc_dust_value: 2,
        }
        .assimilate_storage(&mut storage)
        .unwrap();

        redeem::GenesisConfig::<Runtime> {
            redeem_transaction_size: 400,
            redeem_period: 10,
            redeem_btc_dust_value: 1,
        }
        .assimilate_storage(&mut storage)
        .unwrap();

        replace::GenesisConfig::<Runtime> {
            replace_period: 10,
            replace_btc_dust_value: 2,
        }
        .assimilate_storage(&mut storage)
        .unwrap();

        fee::GenesisConfig::<Runtime> {
            issue_fee: FixedU128::checked_from_rational(5, 1000).unwrap(), // 0.5%
            issue_griefing_collateral: FixedU128::checked_from_rational(5, 100000).unwrap(), // 0.005%
            refund_fee: FixedU128::checked_from_rational(5, 1000).unwrap(), // 0.5%
            redeem_fee: FixedU128::checked_from_rational(5, 1000).unwrap(), // 0.5%
            premium_redeem_fee: FixedU128::checked_from_rational(5, 100).unwrap(), // 5%
            punishment_fee: FixedU128::checked_from_rational(1, 10).unwrap(), // 10%
            replace_griefing_collateral: FixedU128::checked_from_rational(1, 10).unwrap(), // 10%
            theft_fee: FixedU128::checked_from_rational(5, 100).unwrap(),  // 5%
            theft_fee_max: 10000000,                                       // 0.1 BTC
        }
        .assimilate_storage(&mut storage)
        .unwrap();

        Self {
            test_externalities: sp_io::TestExternalities::from(storage),
        }
    }

    /// do setup common to all integration tests, then execute the callback
    pub fn execute_with<R>(self, execute: impl FnOnce() -> R) -> R {
        self.execute_without_relay_init(|| {
            // initialize btc relay
            let _ = TransactionGenerator::new().with_confirmations(7).mine();

            assert_ok!(
                Call::ExchangeRateOracle(ExchangeRateOracleCall::insert_authorized_oracle(
                    account_of(ALICE),
                    vec![]
                ))
                .dispatch(root())
            );
            assert_ok!(Call::ExchangeRateOracle(ExchangeRateOracleCall::feed_values(vec![
                (OracleKey::ExchangeRate(CurrencyId::DOT), FixedU128::from(1)),
                (OracleKey::FeeEstimation, FixedU128::from(3)),
            ]))
            .dispatch(origin_of(account_of(ALICE))));
            ExchangeRateOraclePallet::begin_block(0);

            execute()
        })
    }

    /// used for btc-relay test
    pub fn execute_without_relay_init<R>(mut self, execute: impl FnOnce() -> R) -> R {
        self.test_externalities.execute_with(|| {
            SystemModule::set_block_number(1); // required to be able to dispatch functions
            SecurityPallet::set_active_block_number(1);

            assert_ok!(ExchangeRateOraclePallet::_set_exchange_rate(
                DEFAULT_TESTING_CURRENCY,
                FixedU128::one()
            ));
            set_default_thresholds();

            execute()
        })
    }
}
