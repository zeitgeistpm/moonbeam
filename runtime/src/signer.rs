

use sp_core::{ H160, H256, U256, ecdsa};
use codec::{Decode, Encode};

#[cfg(feature = "std")]
use sp_std::convert::TryInto;
use sp_std::convert::TryFrom;

#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[derive(Eq, PartialEq, Clone, Encode, Decode, sp_core::RuntimeDebug)]
pub enum MultiSignature {
	/// An ECDSA/SECP256k1 signature.
	Ecdsa(ecdsa::Signature),
}

impl From<ecdsa::Signature> for MultiSignature {
	fn from(x: ecdsa::Signature) -> Self {
		log::info!("From<ecdsa::Signature> ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ");
		MultiSignature::Ecdsa(x)
	}
}

impl TryFrom<MultiSignature> for ecdsa::Signature {
	type Error = ();
	fn try_from(m: MultiSignature) -> Result<Self, Self::Error> {
		log::info!("TryFrom<MultiSignature> ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ");
		if let MultiSignature::Ecdsa(x) = m { Ok(x) } else { Err(()) }
	}
}

impl Default for MultiSignature {
	fn default() -> Self {
		log::info!("Default MultiSignature ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ");
		MultiSignature::Ecdsa(Default::default())
	}
}

/// Public key for any known crypto algorithm.
#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Encode, Decode, sp_core::RuntimeDebug)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub enum MultiSigner {
	/// An SECP256k1/ECDSA identity (actually, the Blake2 hash of the compressed pub key).
	Ecdsa(ecdsa::Public),
}

impl Default for MultiSigner {
	fn default() -> Self {
		log::info!("Default for MultiSigner ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ");
		MultiSigner::Ecdsa(Default::default())
	}
}

/// NOTE: This implementations is required by `SimpleAddressDeterminer`,
/// we convert the hash into some AccountId, it's fine to use any scheme.
impl<T: Into<H256> + sp_std::fmt::Debug> sp_core::crypto::UncheckedFrom<T> for MultiSigner {
	fn unchecked_from(x: T) -> Self {
		log::info!("UncheckedFrom: {:?}", x);
		let mut value = [0u8; 33];
		value[0] = 4u8;
		value[1..33].copy_from_slice(&x.into()[0..32]);
		ecdsa::Public::unchecked_from(value.into()).into()
	}
}

impl AsRef<[u8]> for MultiSigner {
	fn as_ref(&self) -> &[u8] {
		match *self {
			MultiSigner::Ecdsa(ref who) => who.as_ref(),
		}
	}
}

impl sp_runtime::traits::IdentifyAccount for MultiSigner {
	type AccountId = super::account::AccountId20;
	fn into_account(self) -> super::account::AccountId20 {
		match self {
			MultiSigner::Ecdsa(who) => {
				log::info!("MultiSigner::Ecdsa(who) ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ");
				let mut value = [0u8; 20];
				value.copy_from_slice(&sp_io::hashing::blake2_256(&who.as_ref()[..])[(32 - 20)..]);
				value.into()
			}
		}
	}
}

impl From<ecdsa::Public> for MultiSigner {
	fn from(x: ecdsa::Public) -> Self {
		log::info!("impl From<ecdsa::Public> for MultiSigner ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ");
		MultiSigner::Ecdsa(x)
	}
}

impl TryFrom<MultiSigner> for ecdsa::Public {
	type Error = ();
	fn try_from(m: MultiSigner) -> Result<Self, Self::Error> {
		log::info!("impl TryFrom<MultiSigner> for ecdsa::Public ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ");
		if let MultiSigner::Ecdsa(x) = m { Ok(x) } else { Err(()) }
	}
}

#[cfg(feature = "std")]
impl std::fmt::Display for MultiSigner {
	fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
		match *self {
			MultiSigner::Ecdsa(ref who) => write!(fmt, "ecdsa: {}", who),
		}
	}
}

impl sp_runtime::traits::Verify for MultiSignature {
	type Signer = MultiSigner;
	fn verify<L: sp_runtime::traits::Lazy<[u8]>>(&self, mut msg: L, signer: &super::account::AccountId20) -> bool {
		log::info!("impl sp_runtime::traits::Verify for MultiSignature ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ⬇ ");
		match (self, signer) {
			(MultiSignature::Ecdsa(ref sig), who) => {
				let m = sp_io::hashing::blake2_256(msg.get());
				match sp_io::crypto::secp256k1_ecdsa_recover_compressed(sig.as_ref(), &m) {
					Ok(pubkey) => {
						let mut value = [0u8; 20];
						value.copy_from_slice(&sp_io::hashing::blake2_256(pubkey.as_ref())[(32 - 20)..]);
						&value == <dyn AsRef<[u8; 20]>>::as_ref(who)
					},
					_ => false,
				}
			}
		}
	}
}
