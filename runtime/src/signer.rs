

use sp_core::{ H160, H256, U256, ed25519, sr25519, ecdsa};
use codec::{Decode, Encode};

#[cfg(feature = "std")]
use sp_std::convert::TryInto;
use sp_std::convert::TryFrom;

#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[derive(Eq, PartialEq, Clone, Encode, Decode, sp_core::RuntimeDebug)]
pub enum MultiSignature {
	/// An Ed25519 signature.
	Ed25519(ed25519::Signature),
	/// An Sr25519 signature.
	Sr25519(sr25519::Signature),
	/// An ECDSA/SECP256k1 signature.
	Ecdsa(ecdsa::Signature),
}

impl From<ed25519::Signature> for MultiSignature {
	fn from(x: ed25519::Signature) -> Self {
		MultiSignature::Ed25519(x)
	}
}

impl TryFrom<MultiSignature> for ed25519::Signature {
	type Error = ();
	fn try_from(m: MultiSignature) -> Result<Self, Self::Error> {
		if let MultiSignature::Ed25519(x) = m { Ok(x) } else { Err(()) }
	}
}

impl From<sr25519::Signature> for MultiSignature {
	fn from(x: sr25519::Signature) -> Self {
		MultiSignature::Sr25519(x)
	}
}

impl TryFrom<MultiSignature> for sr25519::Signature {
	type Error = ();
	fn try_from(m: MultiSignature) -> Result<Self, Self::Error> {
		if let MultiSignature::Sr25519(x) = m { Ok(x) } else { Err(()) }
	}
}

impl From<ecdsa::Signature> for MultiSignature {
	fn from(x: ecdsa::Signature) -> Self {
		MultiSignature::Ecdsa(x)
	}
}

impl TryFrom<MultiSignature> for ecdsa::Signature {
	type Error = ();
	fn try_from(m: MultiSignature) -> Result<Self, Self::Error> {
		if let MultiSignature::Ecdsa(x) = m { Ok(x) } else { Err(()) }
	}
}

impl Default for MultiSignature {
	fn default() -> Self {
		MultiSignature::Ed25519(Default::default())
	}
}

/// Public key for any known crypto algorithm.
#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Encode, Decode, sp_core::RuntimeDebug)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub enum MultiSigner {
	/// An Ed25519 identity.
	Ed25519(ed25519::Public),
	/// An Sr25519 identity.
	Sr25519(sr25519::Public),
	/// An SECP256k1/ECDSA identity (actually, the Blake2 hash of the compressed pub key).
	Ecdsa(ecdsa::Public),
}

impl Default for MultiSigner {
	fn default() -> Self {
		MultiSigner::Ed25519(Default::default())
	}
}

/// NOTE: This implementations is required by `SimpleAddressDeterminer`,
/// we convert the hash into some AccountId, it's fine to use any scheme.
impl<T: Into<H256>> sp_core::crypto::UncheckedFrom<T> for MultiSigner {
	fn unchecked_from(x: T) -> Self {
		ed25519::Public::unchecked_from(x.into()).into()
	}
}

impl AsRef<[u8]> for MultiSigner {
	fn as_ref(&self) -> &[u8] {
		match *self {
			MultiSigner::Ed25519(ref who) => who.as_ref(),
			MultiSigner::Sr25519(ref who) => who.as_ref(),
			MultiSigner::Ecdsa(ref who) => who.as_ref(),
		}
	}
}

impl sp_runtime::traits::IdentifyAccount for MultiSigner {
	type AccountId = super::account::AccountId20;
	fn into_account(self) -> super::account::AccountId20 {
		match self {
			MultiSigner::Ed25519(who) => {
				let mut value = [0u8; 20];
				value.copy_from_slice(&who.0[(32 - 20)..]);
				value.into()
			},
			MultiSigner::Sr25519(who) => {
				let mut value = [0u8; 20];
				value.copy_from_slice(&who.0[(32 - 20)..]);
				value.into()
			},
			MultiSigner::Ecdsa(who) => {
				let mut value = [0u8; 20];
				value.copy_from_slice(&sp_io::hashing::blake2_256(&who.as_ref()[..])[(32 - 20)..]);
				value.into()
			}
		}
	}
}

impl From<ed25519::Public> for MultiSigner {
	fn from(x: ed25519::Public) -> Self {
		MultiSigner::Ed25519(x)
	}
}

impl TryFrom<MultiSigner> for ed25519::Public {
	type Error = ();
	fn try_from(m: MultiSigner) -> Result<Self, Self::Error> {
		if let MultiSigner::Ed25519(x) = m { Ok(x) } else { Err(()) }
	}
}

impl From<sr25519::Public> for MultiSigner {
	fn from(x: sr25519::Public) -> Self {
		MultiSigner::Sr25519(x)
	}
}

impl TryFrom<MultiSigner> for sr25519::Public {
	type Error = ();
	fn try_from(m: MultiSigner) -> Result<Self, Self::Error> {
		if let MultiSigner::Sr25519(x) = m { Ok(x) } else { Err(()) }
	}
}

impl From<ecdsa::Public> for MultiSigner {
	fn from(x: ecdsa::Public) -> Self {
		MultiSigner::Ecdsa(x)
	}
}

impl TryFrom<MultiSigner> for ecdsa::Public {
	type Error = ();
	fn try_from(m: MultiSigner) -> Result<Self, Self::Error> {
		if let MultiSigner::Ecdsa(x) = m { Ok(x) } else { Err(()) }
	}
}

#[cfg(feature = "std")]
impl std::fmt::Display for MultiSigner {
	fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
		match *self {
			MultiSigner::Ed25519(ref who) => write!(fmt, "ed25519: {}", who),
			MultiSigner::Sr25519(ref who) => write!(fmt, "sr25519: {}", who),
			MultiSigner::Ecdsa(ref who) => write!(fmt, "ecdsa: {}", who),
		}
	}
}

impl sp_runtime::traits::Verify for MultiSignature {
	type Signer = MultiSigner;
	fn verify<L: sp_runtime::traits::Lazy<[u8]>>(&self, mut msg: L, signer: &super::account::AccountId20) -> bool {
		match (self, signer) {
			(MultiSignature::Ed25519(ref sig), who) => {
				true // TODO: Remove this unsafe code
			},
			(MultiSignature::Sr25519(ref sig), who) => {
				true // TODO: Remove this unsafe code
			},
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
