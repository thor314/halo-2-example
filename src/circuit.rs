
// next: implement Circuit for MyCircuit

use ff::Field;
use halo2_proofs::{circuit::Value, plonk::Circuit};

// 1. Create MyCircuit
// taking input Field elements: a, b
// returning a*b*constant
#[derive(Default)]
pub struct MyCircuit<F: Field> {
  a: Value<F>,
  b: Value<F>,
}

impl<F: Field> Circuit<F> for MyCircuit<F> {
    type Config;

    type FloorPlanner;

    fn without_witnesses(&self) -> Self {
        todo!()
    }

    fn configure(meta: &mut halo2_proofs::plonk::ConstraintSystem<F>) -> Self::Config {
        todo!()
    }

    fn synthesize(&self, config: Self::Config, layouter: impl halo2_proofs::circuit::Layouter<F>) -> Result<(), halo2_proofs::plonk::Error> {
        todo!()
    }
}