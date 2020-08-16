// This file is part of Frontier.

// Copyright (C) 2019-2020 Parity Technologies (UK) Ltd.
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

use sp_core::{Pair, Public, ecdsa, H160, U256, H512, H256};
use moonbeam_runtime::{
	AccountId, AuraConfig, BalancesConfig, EVMConfig, EthereumConfig, GenesisConfig, GrandpaConfig,
	SudoConfig, SystemConfig, WASM_BINARY, Signature
};
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{Verify, IdentifyAccount};
use sc_service::ChainType;
use std::collections::BTreeMap;
use std::str::FromStr;
use hex_literal::hex;
use sha3::{Digest, Keccak256};

// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate an Aura authority key.
pub fn authority_keys_from_seed(s: &str) -> (AuraId, GrandpaId) {
	(
		get_from_seed::<AuraId>(s),
		get_from_seed::<GrandpaId>(s),
	)
}

pub fn development_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or("Development wasm binary not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Development",
		// ID
		"dev",
		ChainType::Development,
		move || testnet_genesis(
			wasm_binary,
			// Initial PoA authorities
			vec![
				authority_keys_from_seed("Alice"),
			],
			// Sudo account
			get_account_id_from_seed::<ecdsa::Public>("Alice"),
			// Pre-funded accounts
			vec![
				get_account_id_from_seed::<ecdsa::Public>("Alice"),
				get_account_id_from_seed::<ecdsa::Public>("Bob"),
				get_account_id_from_seed::<ecdsa::Public>("Alice//stash"),
				get_account_id_from_seed::<ecdsa::Public>("Bob//stash"),
			],
			true,
		),
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Properties
		None,
		// Extensions
		None,
	))
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or("Development wasm binary not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Local Testnet",
		// ID
		"local_testnet",
		ChainType::Local,
		move || testnet_genesis(
			wasm_binary,
			// Initial PoA authorities
			vec![
				authority_keys_from_seed("Alice"),
				authority_keys_from_seed("Bob"),
			],
			// Sudo account
			get_account_id_from_seed::<ecdsa::Public>("Alice"),
			// Pre-funded accounts
			vec![
				get_account_id_from_seed::<ecdsa::Public>("Alice"),
				get_account_id_from_seed::<ecdsa::Public>("Bob"),
				get_account_id_from_seed::<ecdsa::Public>("Charlie"),
				get_account_id_from_seed::<ecdsa::Public>("Dave"),
				get_account_id_from_seed::<ecdsa::Public>("Eve"),
				get_account_id_from_seed::<ecdsa::Public>("Ferdie"),
				get_account_id_from_seed::<ecdsa::Public>("Alice//stash"),
				get_account_id_from_seed::<ecdsa::Public>("Bob//stash"),
				get_account_id_from_seed::<ecdsa::Public>("Charlie//stash"),
				get_account_id_from_seed::<ecdsa::Public>("Dave//stash"),
				get_account_id_from_seed::<ecdsa::Public>("Eve//stash"),
				get_account_id_from_seed::<ecdsa::Public>("Ferdie//stash"),
			],
			true,
		),
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Properties
		None,
		// Extensions
		None,
	))
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(AuraId, GrandpaId)>,
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	_enable_println: bool,
) -> GenesisConfig {
	let alice_evm_account_id = H160::from_str("6Be02d1d3665660d22FF9624b7BE0551ee1Ac91b").unwrap();
	let mut evm_accounts = BTreeMap::new();
	evm_accounts.insert(
		alice_evm_account_id,
		evm::GenesisAccount {
			nonce: 0.into(),
			balance: U256::from(123456_123_000_000_000_000_000u128),
			storage: BTreeMap::new(),
			code: vec![],
		},
	);
	for ac in endowed_accounts.clone() {
		log::info!("Using account: {:?}", ac);
	}

	let sec = hex!("99B3C12287537E38C90A9219D4CB074A89A16E9CDB20BF85728EBD97C343E342");
	let secret = secp256k1::SecretKey::parse_slice(&sec).unwrap();
	let public = secp256k1::PublicKey::from_secret_key(&secret);

	log::info!("Using priv: {:?}", H256::from_slice(&secret.serialize()[0..32]));
	log::info!("Using pub: {:?}", H512::from_slice(&public.serialize()[1..65]));
	log::info!("Using compressed: {:?}", H256::from_slice(&public.serialize_compressed()[1..33]));
	log::info!("Using address: {:?}", H160::from(H256::from_slice(Keccak256::digest(&public.serialize()[1..65]).as_slice())));
	log::info!("Using gerald: {:?}", ecdsa::Pair::from_seed_slice(&sec).unwrap().public());

	GenesisConfig {
		system: Some(SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
			changes_trie_config: Default::default(),
		}),
		balances: Some(BalancesConfig {
			// Configure endowed accounts with initial balance of 1 << 60.
			balances: endowed_accounts.iter().cloned().map(|k|(k, 1 << 60)).collect(),
		}),
		aura: Some(AuraConfig {
			authorities: initial_authorities.iter().map(|x| (x.0.clone())).collect(),
		}),
		grandpa: Some(GrandpaConfig {
			authorities: initial_authorities.iter().map(|x| (x.1.clone(), 1)).collect(),
		}),
		sudo: Some(SudoConfig {
			// Assign network admin rights.
			key: root_key,
		}),
		evm: Some(EVMConfig {
			accounts: evm_accounts,
		}),
		ethereum: Some(EthereumConfig {}),
	}
}
