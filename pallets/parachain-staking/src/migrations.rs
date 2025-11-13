// Copyright 2019-2025 PureStake Inc.
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

use frame_support::{traits::OnRuntimeUpgrade, weights::Weight};

use crate::*;
use frame_support::pallet_prelude::*;
use frame_support::storage::{generator::StorageValue, storage_prefix, unhashed};
use frame_system::pallet_prelude::*;
use parity_scale_codec::{Decode, Encode};
use sp_runtime::Saturating;
use sp_std::vec::Vec;

// For parachains using asynchronous backing, the round length is doubled
// See more details here: https://github.com/moonbeam-foundation/moonbeam/blob/6b2f75c9b29e3b3483940bb69ff40edf9f91eff6/runtime/moonbase/src/migrations.rs#L33
// Multiply round length by 2
pub struct MultiplyRoundLenBy2<T: Config>(core::marker::PhantomData<T>);

const ROUND_LENGTH_MIGRATION_KEY: &[u8] = b"RoundLenX2Applied";

fn round_length_migration_storage_key() -> [u8; 32] {
	storage_prefix(b"ParachainStaking", ROUND_LENGTH_MIGRATION_KEY)
}

fn has_round_length_been_multiplied() -> bool {
	let key = round_length_migration_storage_key();
	unhashed::get::<bool>(&key).unwrap_or(false)
}

fn mark_round_length_as_multiplied() {
	let key = round_length_migration_storage_key();
	unhashed::put(&key, &true);
}

impl<T> OnRuntimeUpgrade for MultiplyRoundLenBy2<T>
where
	T: Config,
	BlockNumberFor<T>: From<u32> + Into<u64>,
{
	fn on_runtime_upgrade() -> frame_support::pallet_prelude::Weight {
		let db_weight = T::DbWeight::get();

		if has_round_length_been_multiplied() {
			log::info!("MultiplyRoundLenBy2 already executed; skipping.");
			return db_weight.reads(1);
		}

		let mut round = crate::Round::<T>::get();
		let old_length = round.length;

		// Multiply round length by 2
		round.length = round.length.saturating_mul(2);

		crate::Round::<T>::put(round);
		// Mark as completed so subsequent executions (e.g., try-runtime re-run) become no-ops.
		// This keeps the migration idempotent even when executed multiple times.
		if round.length != old_length {
			log::info!(
				"MultiplyRoundLenBy2 doubled round length from {} to {}.",
				old_length,
				round.length
			);
		} else {
			log::warn!(
				"MultiplyRoundLenBy2 left round length unchanged at {} (old value: {}).",
				round.length,
				old_length
			);
		}
		mark_round_length_as_multiplied();

		db_weight.reads_writes(2, 2)
	}

	#[cfg(feature = "try-runtime")]
	fn pre_upgrade() -> Result<Vec<u8>, sp_runtime::TryRuntimeError> {
		let round = crate::Round::<T>::get();
		let already_run = has_round_length_been_multiplied();

		Ok((round.length, already_run).encode())
	}

	#[cfg(feature = "try-runtime")]
	fn post_upgrade(state: Vec<u8>) -> Result<(), sp_runtime::TryRuntimeError> {
		let (old_length, already_run): (u32, bool) =
			<(u32, bool)>::decode(&mut &state[..]).map_err(|_| {
				sp_runtime::TryRuntimeError::Other(
					"MultiplyRoundLenBy2: failed to decode pre-upgrade state".into(),
				)
			})?;

		let round = crate::Round::<T>::get();
		let flag_after = has_round_length_been_multiplied();

		if already_run {
			ensure!(
				round.length == old_length,
				"Round length changed even though migration was previously applied"
			);
		} else {
			ensure!(
				round.length == old_length.saturating_mul(2),
				"Round length was not doubled during migration"
			);
		}

		ensure!(flag_after, "Round length multiplication flag missing after migration");

		Ok(())
	}
}

#[derive(
	Clone,
	PartialEq,
	Eq,
	parity_scale_codec::Decode,
	parity_scale_codec::Encode,
	sp_runtime::RuntimeDebug,
)]
/// Reserve information { account, percent_of_inflation }
pub struct OldParachainBondConfig<AccountId> {
	/// Account which receives funds intended for parachain bond
	pub account: AccountId,
	/// Percent of inflation set aside for parachain bond account
	pub percent: sp_runtime::Percent,
}

pub struct MigrateParachainBondConfig<T>(sp_std::marker::PhantomData<T>);
impl<T: Config> OnRuntimeUpgrade for MigrateParachainBondConfig<T> {
	fn on_runtime_upgrade() -> Weight {
		let (account, percent) = if let Some(config) =
			frame_support::storage::migration::get_storage_value::<
				OldParachainBondConfig<T::AccountId>,
			>(b"ParachainStaking", b"ParachainBondInfo", &[])
		{
			(config.account, config.percent)
		} else {
			return Weight::default();
		};

		let pbr = InflationDistributionAccount { account, percent };
		let treasury = InflationDistributionAccount::<T::AccountId>::default();
		let configs: InflationDistributionConfig<T::AccountId> = [pbr, treasury].into();

		//***** Start mutate storage *****//

		InflationDistributionInfo::<T>::put(configs);

		// Remove storage value ParachainStaking::ParachainBondInfo
		frame_support::storage::unhashed::kill(&frame_support::storage::storage_prefix(
			b"ParachainStaking",
			b"ParachainBondInfo",
		));

		log::info!("MigrateParachainBondConfig migration done.");

		Weight::default()
	}

	#[cfg(feature = "try-runtime")]
	fn pre_upgrade() -> Result<Vec<u8>, sp_runtime::DispatchError> {
		let state = frame_support::storage::migration::get_storage_value::<
			OldParachainBondConfig<T::AccountId>,
		>(b"ParachainStaking", b"ParachainBondInfo", &[]);

		if state.is_some() {
			log::info!("MigrateParachainBondConfig pre_upgrade: state found.");
			Ok(state.unwrap().encode())
		} else {
			log::warn!("MigrateParachainBondConfig pre_upgrade: state not found.");
			Ok(Vec::new())
		}
	}

	#[cfg(feature = "try-runtime")]
	fn post_upgrade(state: Vec<u8>) -> Result<(), sp_runtime::DispatchError> {
		if state.is_empty() {
			log::warn!("MigrateParachainBondConfig post_upgrade: no state to migrate.");
			return Ok(());
		}
		let old_state: OldParachainBondConfig<T::AccountId> =
			parity_scale_codec::Decode::decode(&mut &state[..])
				.map_err(|_| sp_runtime::DispatchError::Other("Failed to decode old state"))?;

		let new_state = InflationDistributionInfo::<T>::get();

		let pbr = InflationDistributionAccount {
			account: old_state.account,
			percent: old_state.percent,
		};
		let treasury = InflationDistributionAccount::<T::AccountId>::default();
		let expected_new_state: InflationDistributionConfig<T::AccountId> = [pbr, treasury].into();

		ensure!(new_state == expected_new_state, "State migration failed");

		Ok(())
	}
}

/// Migrates RoundInfo and add the field first_slot
pub struct MigrateRoundWithFirstSlot<T: Config>(core::marker::PhantomData<T>);

#[derive(Copy, Clone, PartialEq, Eq, Encode, Decode, RuntimeDebug, TypeInfo)]
struct OldRoundInfo<BlockNumber> {
	/// Current round index
	pub current: RoundIndex,
	/// The first block of the current round
	pub first: BlockNumber,
	/// The length of the current round in number of blocks
	pub length: u32,
}
impl<BlockNumber: From<u32>> From<OldRoundInfo<BlockNumber>> for RoundInfo<BlockNumber> {
	fn from(round: OldRoundInfo<BlockNumber>) -> Self {
		Self {
			current: round.current,
			first: round.first.into(),
			length: round.length,
			first_slot: 0,
		}
	}
}

impl<T> OnRuntimeUpgrade for MigrateRoundWithFirstSlot<T>
where
	T: Config,
	BlockNumberFor<T>: From<u32> + Into<u64>,
{
	#[cfg(feature = "try-runtime")]
	fn pre_upgrade() -> Result<Vec<u8>, sp_runtime::TryRuntimeError> {
		let raw_key = crate::Round::<T>::storage_value_final_key();
		let maybe_raw_value = unhashed::get_raw(&raw_key);
		let len = maybe_raw_value
			.expect("ParachainStaking.Round should exist!")
			.len();
		ensure!(
			len == 16 || len == 24,
			"ParachainStaking.Round should have 16 or 24 bytes (already applied) length!"
		);

		Ok(Vec::new())
	}

	fn on_runtime_upgrade() -> frame_support::pallet_prelude::Weight {
		let raw_key = crate::Round::<T>::storage_value_final_key();

		// Read old round info
		let mut round: RoundInfo<BlockNumberFor<T>> = if let Some(bytes) =
			unhashed::get_raw(&raw_key)
		{
			let len = bytes.len();
			match len {
				// Migration already done
				24 => {
					log::info!("MigrateRoundWithFirstSlot already applied.");
					return Default::default();
				}
				// Migrate from rt2700
				16 => match OldRoundInfo::<BlockNumberFor<T>>::decode(&mut &bytes[..]) {
					Ok(round) => round.into(),
					Err(e) => panic!("corrupted storage: fail to decode RoundInfoRt2700: {}", e),
				},
				// Storage corrupted
				x => panic!(
					"corrupted storage: parachainStaking.Round invalid length: {} bytes",
					x
				),
			}
		} else {
			panic!("corrupted storage: parachainStaking.Round don't exist");
		};

		// Compute new field `first_slot``
		round.first_slot = compute_theoretical_first_slot(
			<frame_system::Pallet<T>>::block_number(),
			round.first,
			u64::from(T::SlotProvider::get()),
			T::BlockTime::get(),
		);

		// Fill DelayedPayouts for rounds N and N-1
		if let Some(delayed_payout) =
			<crate::DelayedPayouts<T>>::get(round.current.saturating_sub(2))
		{
			<crate::DelayedPayouts<T>>::insert(
				round.current.saturating_sub(1),
				delayed_payout.clone(),
			);
			<crate::DelayedPayouts<T>>::insert(round.current, delayed_payout);
		}

		// Apply the migration (write new Round value)
		crate::Round::<T>::put(round);

		Default::default()
	}

	#[cfg(feature = "try-runtime")]
	fn post_upgrade(_state: Vec<u8>) -> Result<(), sp_runtime::TryRuntimeError> {
		let _round = crate::Round::<T>::get(); // Should panic if SCALE decode fail
		Ok(())
	}
}

fn compute_theoretical_first_slot<BlockNumber: Saturating + Into<u64>>(
	current_block: BlockNumber,
	first_block: BlockNumber,
	current_slot: u64,
	block_time: u64,
) -> u64 {
	let blocks_since_first: u64 = (current_block.saturating_sub(first_block)).into();
	let slots_since_first = match block_time {
		12_000 => blocks_since_first * 2,
		6_000 => blocks_since_first,
		_ => panic!("Unsupported BlockTime"),
	};
	current_slot.saturating_sub(slots_since_first)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_compute_theoretical_first_slot() {
		assert_eq!(
			compute_theoretical_first_slot::<u32>(10, 5, 100, 12_000),
			90,
		);
	}
}
