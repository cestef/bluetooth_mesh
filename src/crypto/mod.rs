pub mod aes;
mod aes_cmac;
pub mod k_funcs;
pub mod key;
#[derive(Clone, Copy, Debug, Hash, Eq, PartialOrd, PartialEq, Ord)]
pub struct AID(u8);
#[derive(Clone, Copy, Debug, Hash, Eq, PartialOrd, PartialEq, Ord)]
pub struct AKF(bool);
impl From<bool> for AKF {
    fn from(b: bool) -> Self {
        AKF(b)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct TryFromBlockError(());
const SALT_LEN: usize = 16;
#[derive(Clone, Copy, Debug, Hash, Eq, PartialOrd, PartialEq, Ord)]
pub struct Salt([u8; SALT_LEN]);
impl TryFrom<&[u8]> for Salt {
    type Error = TryFromBlockError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != SALT_LEN {
            Err(TryFromBlockError(()))
        } else {
            let mut buf = Salt([0_u8; SALT_LEN]);
            buf.0.copy_from_slice(value);
            Ok(buf)
        }
    }
}
#[derive(Clone, Copy, Debug, Hash, Eq, PartialOrd, PartialEq, Ord)]
pub struct ECDHSecret();
#[derive(Clone, Copy, Debug, Hash, Eq, PartialOrd, PartialEq, Ord)]
pub struct NetworkID();
impl NetworkID {
    /// Derives `NetworkID` from `key::NetKey` by calling `k3` on `key`.
    pub fn from_net_key(key: key::NetKey) -> NetworkID {
        k3(key)
    }
}

use core::array::TryFromSliceError;
use core::convert::TryFrom;
pub use k_funcs::{k1, k2, k3, k4};
