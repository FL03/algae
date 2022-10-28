/*
    Appellation: hash <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
*/
pub use self::{hasher::*,  primitives::*, utils::*};

pub(crate) mod hasher;

pub(crate) mod primitives {
    use generic_array::GenericArray;
    use typenum::{UInt, UTerm, B0, B1};

    pub type HashOutputSize = UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>;
    pub type HashGeneric<T = u8> = GenericArray<T, HashOutputSize>;
}

pub(crate) mod utils {
    use super::primitives::HashGeneric;
    use serde::Serialize;
    use sha2::{Digest, Sha256};

    pub fn hasher<T: Serialize>(data: &T) -> HashGeneric {
        let mut hasher = Sha256::new();
        hasher.update(serde_json::to_string(data).unwrap().as_bytes());
        hasher.finalize()
    }
}
