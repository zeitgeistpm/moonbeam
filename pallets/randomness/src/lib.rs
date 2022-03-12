// Copyright 2019-2022 PureStake Inc.
// This file is part of Moonbeam.

// Moonbeam is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Moonbeam is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Moonbeam.  If not, see <http://www.gnu.org/licenses/>.

//! Randomness pallet

#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::pallet;

pub use pallet::*;

// pub mod weights;
// use weights::WeightInfo;
// #[cfg(any(test, feature = "runtime-benchmarks"))]
// mod benchmarks;
// #[cfg(test)]
// mod mock;
// #[cfg(test)]
// mod tests;

#[pallet]
pub mod pallet {
	// use crate::WeightInfo;
	use frame_support::pallet_prelude::*;
	use frame_support::traits::{Currency, ReservableCurrency};
	use frame_system::pallet_prelude::*;

	pub type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(PhantomData<T>);

	/// Request identifier, unique per request for randomness
	pub type RequestId = u64;

	/// Configuration trait of this pallet.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Overarching event type
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		/// Currency in which the security deposit will be taken.
		type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
		/// The amount that should be taken as a security deposit when requesting randomness.
		type Deposit: Get<BalanceOf<Self>>;
		// /// Weight information for extrinsics in this pallet.
		// type WeightInfo: WeightInfo;
	}

	#[pallet::error]
	pub enum Error<T> {
		/// The counter for request IDs overflowed
		RequestCounterOverflowed,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(crate) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Randomness requested and request put in storage
		RandomnessRequested {
			refund_address: T::AccountId,
			contract_address: T::AccountId,
			fulfillment_fee: BalanceOf<T>,
			request_id: RequestId,
		},
	}

	#[pallet::storage]
	#[pallet::getter(fn request)]
	/// Randomness requests not yet fulfilled or purged
	pub type Request<T: Config> =
		StorageMap<_, Blake2_128Concat, RequestId, RequestId, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn request_count)]
	/// Number of randomness requests made so far, used to generate the next request's uid
	pub type RequestCount<T: Config> = StorageValue<_, RequestId, ValueQuery>;

	// #[pallet::genesis_config]
	// /// Genesis config for author mapping pallet
	// pub struct GenesisConfig<T: Config> {
	// 	/// The associations that should exist at chain genesis
	// 	pub mappings: Vec<(NimbusId, T::AccountId)>,
	// }

	// #[cfg(feature = "std")]
	// impl<T: Config> Default for GenesisConfig<T> {
	// 	fn default() -> Self {
	// 		Self { mappings: vec![] }
	// 	}
	// }

	// #[pallet::genesis_build]
	// impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
	// 	fn build(&self) {
	// 		for (author_id, account_id) in &self.mappings {
	// 			if let Err(e) = Pallet::<T>::enact_registration(&author_id, &account_id) {
	// 				log::warn!("Error with genesis author mapping registration: {:?}", e);
	// 			}
	// 		}
	// 	}
	// }

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Make randomness request
		#[pallet::weight(0)]
		pub fn request_randomness(
			origin: OriginFor<T>,
			source_id: u32, // what type here
			refund_address: T::AccountId,
			contract_address: T::AccountId,
			salt: [u8; 32],
			fulfillment_fee: BalanceOf<T>,
		) -> DispatchResult {
			ensure_signed(origin)?;
			// TODO: just emit error if this overflows instead of saturating?
			let new_id = <RequestCount<T>>::get()
				.checked_add(1u64)
				.ok_or(Error::<T>::RequestCounterOverflowed)?;
			<RequestCount<T>>::put(new_id);
			Self::deposit_event(Event::RandomnessRequested {
				refund_address,
				contract_address,
				fulfillment_fee,
				request_id: new_id,
			});
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		/// A helper function to lookup the account id associated with the given author id. This is
		/// the primary lookup that this pallet is responsible for.
		pub fn todo() {
			todo!()
		}
	}
}
