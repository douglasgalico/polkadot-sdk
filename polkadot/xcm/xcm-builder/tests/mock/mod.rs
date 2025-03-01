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

use frame_support::{
	construct_runtime, parameter_types,
	traits::{ConstU32, Everything, Nothing},
	weights::Weight,
};
use frame_system::EnsureRoot;
use parity_scale_codec::Encode;
use primitive_types::H256;
use sp_runtime::{traits::IdentityLookup, AccountId32, BuildStorage};
use sp_std::cell::RefCell;

use polkadot_parachain_primitives::primitives::Id as ParaId;
use polkadot_runtime_parachains::{configuration, origin, shared};
use xcm::latest::{opaque, prelude::*};
use xcm_executor::XcmExecutor;

use staging_xcm_builder as xcm_builder;

use xcm_builder::{
	AccountId32Aliases, AllowTopLevelPaidExecutionFrom, AllowUnpaidExecutionFrom,
	ChildParachainAsNative, ChildParachainConvertsVia, ChildSystemParachainAsSuperuser,
	CurrencyAdapter as XcmCurrencyAdapter, FixedRateOfFungible, FixedWeightBounds,
	IsChildSystemParachain, IsConcrete, MintLocation, RespectSuspension, SignedAccountId32AsNative,
	SignedToAccountId32, SovereignSignedViaLocation, TakeWeightCredit,
};

pub type AccountId = AccountId32;
pub type Balance = u128;

thread_local! {
	pub static SENT_XCM: RefCell<Vec<(MultiLocation, opaque::Xcm, XcmHash)>> = RefCell::new(Vec::new());
}
pub fn sent_xcm() -> Vec<(MultiLocation, opaque::Xcm, XcmHash)> {
	SENT_XCM.with(|q| (*q.borrow()).clone())
}
pub struct TestSendXcm;
impl SendXcm for TestSendXcm {
	type Ticket = (MultiLocation, Xcm<()>, XcmHash);
	fn validate(
		dest: &mut Option<MultiLocation>,
		msg: &mut Option<Xcm<()>>,
	) -> SendResult<(MultiLocation, Xcm<()>, XcmHash)> {
		let msg = msg.take().unwrap();
		let hash = fake_message_hash(&msg);
		let triplet = (dest.take().unwrap(), msg, hash);
		Ok((triplet, MultiAssets::new()))
	}
	fn deliver(triplet: (MultiLocation, Xcm<()>, XcmHash)) -> Result<XcmHash, SendError> {
		let hash = triplet.2;
		SENT_XCM.with(|q| q.borrow_mut().push(triplet));
		Ok(hash)
	}
}

// copied from kusama constants
pub const UNITS: Balance = 1_000_000_000_000;
pub const CENTS: Balance = UNITS / 30_000;

parameter_types! {
	pub const BlockHashCount: u64 = 250;
}

impl frame_system::Config for Runtime {
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Nonce = u64;
	type Hash = H256;
	type Hashing = ::sp_runtime::traits::BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Block = Block;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = BlockHashCount;
	type BlockWeights = ();
	type BlockLength = ();
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type DbWeight = ();
	type BaseCallFilter = Everything;
	type SystemWeightInfo = ();
	type SS58Prefix = ();
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

parameter_types! {
	pub ExistentialDeposit: Balance = 1 * CENTS;
	pub const MaxLocks: u32 = 50;
	pub const MaxReserves: u32 = 50;
}

impl pallet_balances::Config for Runtime {
	type MaxLocks = MaxLocks;
	type Balance = Balance;
	type RuntimeEvent = RuntimeEvent;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
	type MaxReserves = MaxReserves;
	type ReserveIdentifier = [u8; 8];
	type RuntimeHoldReason = RuntimeHoldReason;
	type RuntimeFreezeReason = RuntimeFreezeReason;
	type FreezeIdentifier = ();
	type MaxHolds = ConstU32<0>;
	type MaxFreezes = ConstU32<0>;
}

impl shared::Config for Runtime {}

impl configuration::Config for Runtime {
	type WeightInfo = configuration::TestWeightInfo;
}

// aims to closely emulate the Kusama XcmConfig
parameter_types! {
	pub const KsmLocation: MultiLocation = MultiLocation::here();
	pub const KusamaNetwork: NetworkId = NetworkId::Kusama;
	pub UniversalLocation: InteriorMultiLocation = Here;
	pub CheckAccount: (AccountId, MintLocation) = (XcmPallet::check_account(), MintLocation::Local);
}

pub type SovereignAccountOf =
	(ChildParachainConvertsVia<ParaId, AccountId>, AccountId32Aliases<KusamaNetwork, AccountId>);

pub type LocalCurrencyAdapter = XcmCurrencyAdapter<
	Balances,
	IsConcrete<KsmLocation>,
	SovereignAccountOf,
	AccountId,
	CheckAccount,
>;

pub type LocalAssetTransactor = (LocalCurrencyAdapter,);

type LocalOriginConverter = (
	SovereignSignedViaLocation<SovereignAccountOf, RuntimeOrigin>,
	ChildParachainAsNative<origin::Origin, RuntimeOrigin>,
	SignedAccountId32AsNative<KusamaNetwork, RuntimeOrigin>,
	ChildSystemParachainAsSuperuser<ParaId, RuntimeOrigin>,
);

parameter_types! {
	pub const BaseXcmWeight: Weight = Weight::from_parts(1_000_000_000, 1024);
	pub KsmPerSecondPerByte: (AssetId, u128, u128) = (KsmLocation::get().into(), 1, 1);
}

pub type Barrier = (
	TakeWeightCredit,
	AllowTopLevelPaidExecutionFrom<Everything>,
	// Unused/Untested
	AllowUnpaidExecutionFrom<IsChildSystemParachain<ParaId>>,
);

parameter_types! {
	pub KusamaForAssetHub: (MultiAssetFilter, MultiLocation) =
		(Wild(AllOf { id: Concrete(Here.into()), fun: WildFungible }), Parachain(1000).into());
	pub const MaxInstructions: u32 = 100;
	pub const MaxAssetsIntoHolding: u32 = 4;
}

pub type TrustedTeleporters = (xcm_builder::Case<KusamaForAssetHub>,);

pub struct XcmConfig;
impl xcm_executor::Config for XcmConfig {
	type RuntimeCall = RuntimeCall;
	type XcmSender = TestSendXcm;
	type AssetTransactor = LocalAssetTransactor;
	type OriginConverter = LocalOriginConverter;
	type IsReserve = ();
	type IsTeleporter = TrustedTeleporters;
	type UniversalLocation = UniversalLocation;
	type Barrier = RespectSuspension<Barrier, XcmPallet>;
	type Weigher = FixedWeightBounds<BaseXcmWeight, RuntimeCall, MaxInstructions>;
	type Trader = FixedRateOfFungible<KsmPerSecondPerByte, ()>;
	type ResponseHandler = XcmPallet;
	type AssetTrap = XcmPallet;
	type AssetLocker = ();
	type AssetExchanger = ();
	type AssetClaims = XcmPallet;
	type SubscriptionService = XcmPallet;
	type PalletInstancesInfo = AllPalletsWithSystem;
	type MaxAssetsIntoHolding = MaxAssetsIntoHolding;
	type FeeManager = ();
	type MessageExporter = ();
	type UniversalAliases = Nothing;
	type CallDispatcher = RuntimeCall;
	type SafeCallFilter = Everything;
	type Aliasers = Nothing;
}

pub type LocalOriginToLocation = SignedToAccountId32<RuntimeOrigin, AccountId, KusamaNetwork>;

impl pallet_xcm::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type UniversalLocation = UniversalLocation;
	type SendXcmOrigin = xcm_builder::EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
	type XcmRouter = TestSendXcm;
	// Anyone can execute XCM messages locally...
	type ExecuteXcmOrigin = xcm_builder::EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
	type XcmExecuteFilter = Nothing;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type XcmTeleportFilter = Everything;
	type XcmReserveTransferFilter = Everything;
	type Weigher = FixedWeightBounds<BaseXcmWeight, RuntimeCall, MaxInstructions>;
	type RuntimeCall = RuntimeCall;
	type RuntimeOrigin = RuntimeOrigin;
	const VERSION_DISCOVERY_QUEUE_SIZE: u32 = 100;
	type AdvertisedXcmVersion = pallet_xcm::CurrentXcmVersion;
	type TrustedLockers = ();
	type SovereignAccountOf = ();
	type Currency = Balances;
	type CurrencyMatcher = IsConcrete<KsmLocation>;
	type MaxLockers = frame_support::traits::ConstU32<8>;
	type MaxRemoteLockConsumers = frame_support::traits::ConstU32<0>;
	type RemoteLockConsumerIdentifier = ();
	type WeightInfo = pallet_xcm::TestWeightInfo;
	type AdminOrigin = EnsureRoot<AccountId>;
}

impl origin::Config for Runtime {}

type Block = frame_system::mocking::MockBlock<Runtime>;

construct_runtime!(
	pub enum Runtime
	{
		System: frame_system::{Pallet, Call, Storage, Config<T>, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
		ParasOrigin: origin::{Pallet, Origin},
		XcmPallet: pallet_xcm::{Pallet, Call, Storage, Event<T>, Origin},
	}
);

pub fn kusama_like_with_balances(balances: Vec<(AccountId, Balance)>) -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::<Runtime>::default().build_storage().unwrap();

	pallet_balances::GenesisConfig::<Runtime> { balances }
		.assimilate_storage(&mut t)
		.unwrap();

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(1));
	ext
}

pub fn fake_message_hash<T>(message: &Xcm<T>) -> XcmHash {
	message.using_encoded(sp_io::hashing::blake2_256)
}
