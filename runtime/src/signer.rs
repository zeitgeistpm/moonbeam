

use sp_core::{H256, ecdsa};
use codec::{Decode, Encode};

use sp_std::convert::TryFrom;

#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[derive(Eq, PartialEq, Clone, Encode, Decode, sp_core::RuntimeDebug)]
pub struct EthereumSignature(ecdsa::Signature);

impl From<ecdsa::Signature> for EthereumSignature {
	fn from(x: ecdsa::Signature) -> Self {
		log::info!("From<ecdsa::Signature> ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ");
		EthereumSignature(x)
	}
}

// impl TryFrom<EthereumSignature> for ecdsa::Signature {
// 	type Error = ();
// 	fn try_from(es: EthereumSignature) -> Result<Self, Self::Error> {
// 		Ok(es)
// 	}
// }

// impl Default for EthereumSignature {
// 	fn default() -> Self {
// 		EthereumSignature(Default::default())
// 	}
// }

/// Public key for any known crypto algorithm.
#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Encode, Decode, sp_core::RuntimeDebug)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct EthereumSigner (ecdsa::Public);

// impl Default for EthereumSigner {
// 	fn default() -> Self {
// 		EthereumSigner(Default::default())
// 	}
// }

/// NOTE: This implementations is required by `SimpleAddressDeterminer`,
/// we convert the hash into some AccountId, it's fine to use any scheme.
impl<T: Into<H256> + sp_std::fmt::Debug> sp_core::crypto::UncheckedFrom<T> for EthereumSigner {
	fn unchecked_from(x: T) -> Self {
		log::info!("UncheckedFrom: {:?}", x);
		let mut value = [0u8; 33];
		value[0] = 4u8;
		value[1..33].copy_from_slice(&x.into()[0..32]);
		ecdsa::Public::unchecked_from(value.into()).into()
	}
}

impl sp_runtime::traits::IdentifyAccount for EthereumSigner {
	type AccountId = super::account::AccountId20;
	fn into_account(self) -> super::account::AccountId20 {
		log::info!("EthereumSigner::into_account() ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ");
		let mut value = [0u8; 20];
		value.copy_from_slice(&sp_io::hashing::blake2_256(&self.0.as_ref()[..])[(32 - 20)..]);
		value.into()
	}
}

impl From<ecdsa::Public> for EthereumSigner {
	fn from(x: ecdsa::Public) -> Self {
		EthereumSigner(x)
	}
}

// impl TryFrom<EthereumSigner> for ecdsa::Public {
// 	type Error = ();
// 	fn try_from(m: EthereumSigner) -> Result<Self, Self::Error> {
// 		Ok(m)
// 	}
// }

#[cfg(feature = "std")]
impl std::fmt::Display for EthereumSigner {
	fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(fmt, "ethereum signature: {}", self)
	}
}

impl sp_runtime::traits::Verify for EthereumSignature {
	type Signer = EthereumSigner;
	fn verify<L: sp_runtime::traits::Lazy<[u8]>>(&self, mut msg: L, signer: &super::account::AccountId20) -> bool {
		log::info!("impl sp_runtime::traits::Verify for EthereumSignature ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ");
		let m = sp_io::hashing::blake2_256(msg.get());
		match sp_io::crypto::secp256k1_ecdsa_recover_compressed(self.0.as_ref(), &m) {
			Ok(pubkey) => {
				let mut value = [0u8; 20];
				value.copy_from_slice(&sp_io::hashing::blake2_256(pubkey.as_ref())[(32 - 20)..]);
				&value == <dyn AsRef<[u8; 20]>>::as_ref(signer)
			},
			_ => false,
		}
	}
}
