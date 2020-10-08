use std::{
    convert::TryFrom, 
    str::FromStr
};

use tss_esapi::{
    constants::algorithm::HashingAlgorithm,
    handles::PcrHandle,
    structures::{Digest, DigestValues},
    Context,
    Tcti
};

fn main() {
    let tcti = Tcti::from_str("device:/try/this/path").unwrap();
    let mut c = unsafe { Context::new(tcti).unwrap() };
    let mut vals = DigestValues::new();
    vals.set(HashingAlgorithm::Sha256, Digest::try_from(vec![1, 1, 1]).unwrap());
    let _ = c.pcr_extend(PcrHandle::Pcr0, vals);
}
