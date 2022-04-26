use dusk_plonk::prelude::*;

pub const LOG_TREE_SIZE: usize = 10;

lazy_static! {
    pub static ref MIMC_PARAMS: Vec<BlsScalar> = vec![BlsScalar::from(1u64), BlsScalar::from(2u64)];
}
