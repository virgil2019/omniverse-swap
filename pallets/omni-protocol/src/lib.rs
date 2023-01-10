#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub mod types;
pub use types::*;

pub mod functions;

pub mod traits;

#[frame_support::pallet]
pub mod pallet {
	use super::types::{OmniverseTokenProtocol, VerifyError, VerifyResult, EvilTxData, OmniverseTx};
	use codec::{Decode, Encode};
	use frame_support::{pallet_prelude::*, traits::UnixTime, weights::constants};
	use frame_system::pallet_prelude::*;
	use sp_std::vec::Vec;
	use core::time::Duration;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		#[pallet::constant]
		type ChainId: Get<u8>;
		type Timestamp: UnixTime;
	}

	#[pallet::type_value]
	pub fn GetDefaultValue() -> u128 {
		0
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn transaction_recorder)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	pub type TransactionRecorder<T: Config> =
		StorageDoubleMap<_, Blake2_128Concat, [u8; 64], Blake2_128Concat, u128, OmniverseTx>;

	#[pallet::storage]
	#[pallet::getter(fn transaction_count)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	pub type TransactionCount<T: Config> =
		StorageMap<_, Blake2_128Concat, [u8; 64], u128, ValueQuery, GetDefaultValue>;

	#[pallet::storage]
	#[pallet::getter(fn evil_recorder)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	pub type EvilRecorder<T: Config> = StorageMap<_, Blake2_128Concat, [u8; 64], Vec<EvilTxData>>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		TransactionSent([u8; 64], u128),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {}
}
