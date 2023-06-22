use std::marker::PhantomData;

use ff::Field;
use halo2_proofs::{
  circuit::{Chip, SimpleFloorPlanner, Value},
  plonk::{Advice, Circuit, Column, ConstraintSystem, Fixed, Instance},
};

use crate::{
  chip::{MyChip, MyChipConfig},
  numeric_instructions::NumericInstructions,
};

// 1. Create MyCircuit
// taking input Field elements: a, b
// returning a*b*constant
#[derive(Default)]
pub struct MyCircuit<F: Field> {
  pub a:        Value<F>,
  pub b:        Value<F>,
  // just for the sake of demonstration, show we can used fixed columns to load constants
  pub constant: F,
}

impl<F: Field> Circuit<F> for MyCircuit<F> {
  // the chip needs to be configured
  // field choice for the Circuit, see below
  // can have Circuit config overlap with Chip config since only one Chip
  type Config = MyChipConfig;
  // algorithm to plan table layout, using the default here
  type FloorPlanner = SimpleFloorPlanner;

  // typically just default
  fn without_witnesses(&self) -> Self { Self::default() }

  // describe exact gate/column arrangement
  fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
    // used for IO; have a fan-in 2 circuit gate, so need 2 advice cols
    let advice = [meta.advice_column(), meta.advice_column()];
    // store public inputs in Instance columns
    let instance = meta.instance_column();
    // for loading a constant
    let constant = meta.fixed_column();
    // return the column configuration
    Self::Config::configure(meta, advice, instance, constant)
  }

  // Create the circuit WRT the constraint system
  fn synthesize(
    &self,
    config: Self::Config,
    mut layouter: impl halo2_proofs::circuit::Layouter<F>,
  ) -> Result<(), halo2_proofs::plonk::Error> {
    // load any used arithmetic chips; see below for the construction of our chip
    let field_chip = MyChip::<F>::construct(config);

    // Load {private, constant} values into the circuit
    let a = field_chip.load_private(layouter.namespace(|| "load a"), self.a)?;
    let b = field_chip.load_private(layouter.namespace(|| "load b"), self.b)?;
    let constant =
      field_chip.load_constant(layouter.namespace(|| "load constant"), self.constant)?;

    // Finally, tell the circuit how to use our Chip
    let aa = field_chip.mul(layouter.namespace(|| "a * b"), a.clone(), a)?;
    let bb = field_chip.mul(layouter.namespace(|| "b * b"), b.clone(), b)?;
    let c = field_chip.mul(layouter.namespace(|| "aa * bb"), aa, bb)?;
    // let c = field_chip.mul(layouter.namespace(|| "_c * constant"), _c, constant)?;

    // and "return" the result as a public input to the circuit
    field_chip.expose_public(layouter.namespace(|| "expose result"), c, 0)
  }
}
