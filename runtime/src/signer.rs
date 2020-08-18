

use sp_core::{H160, H256, ecdsa};
use codec::{Decode, Encode};
use sha3::{Digest, Keccak256};

#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[derive(Eq, PartialEq, Clone, Encode, Decode, sp_core::RuntimeDebug)]
pub struct EthereumSignature(ecdsa::Signature);

impl From<ecdsa::Signature> for EthereumSignature {
	fn from(x: ecdsa::Signature) -> Self {
		EthereumSignature(x)
	}
}

/// Public key for any known crypto algorithm.
#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Encode, Decode, sp_core::RuntimeDebug)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct EthereumSigner (ecdsa::Public);

impl<T: Into<H256> + sp_std::fmt::Debug> sp_core::crypto::UncheckedFrom<T> for EthereumSigner {
	fn unchecked_from(x: T) -> Self {
		let mut value = [0u8; 33];
		value[0] = 4u8;
		value[1..33].copy_from_slice(&x.into()[0..32]);
		ecdsa::Public::unchecked_from(value.into()).into()
	}
}

impl sp_runtime::traits::IdentifyAccount for EthereumSigner {
	type AccountId = super::account::AccountId20;
	fn into_account(self) -> super::account::AccountId20 {
		let mut value = [0u8; 20];
		value.copy_from_slice(
			&H160::from(
				H256::from_slice(
					Keccak256::digest(&self.0.as_ref()[..]).as_slice()
				)
			)[..]
		);
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
		write!(fmt, "ethereum signature: {}", self.0)
	}
}

impl sp_runtime::traits::Verify for EthereumSignature {
	type Signer = EthereumSigner;
	fn verify<L: sp_runtime::traits::Lazy<[u8]>>(&self, mut msg: L, signer: &super::account::AccountId20) -> bool {
		let mut m = [0u8; 32];
		m.copy_from_slice(Keccak256::digest(msg.get()).as_slice());
		match sp_io::crypto::secp256k1_ecdsa_recover(self.0.as_ref(), &m) {
			Ok(pubkey) => {
				let mut value = [0u8; 20];
				value.copy_from_slice(&H160::from(H256::from_slice(Keccak256::digest(&pubkey).as_slice()))[..]);
				&value == <dyn AsRef<[u8; 20]>>::as_ref(signer)
			},
			_ => false,
		}
	}
}
