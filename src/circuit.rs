// next: implement Circuit for MyCircuit

use ff::Field;
use halo2_proofs::{
  circuit::{SimpleFloorPlanner, Value},
  plonk::{Advice, Circuit, Column, ConstraintSystem, Fixed, Instance},
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
  type Config = FieldConfig;
  // algorithm to plan table layout, using the default here
  type FloorPlanner = SimpleFloorPlanner;

  // typically just default
  fn without_witnesses(&self) -> Self { Self::default() }

  // describe exact gate/column arrangement
  fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
    // used for IO; have a fan-in 2 circuit gate, so need 2 advice cols
    let advice = [meta.advice_column(); 2];
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
    layouter: impl halo2_proofs::circuit::Layouter<F>,
  ) -> Result<(), halo2_proofs::plonk::Error> {
    // construct any arithmetic chips; see below for the construction of our chip
    let field_chip = FieldChip::<F>::construct(config);

    // Load {private, constant} values into the circuit
    let a = field_chip.load_private(layouter.namespace(|| "load a"), self.a)?;
    let b = field_chip.load_private(layouter.namespace(|| "load b"), self.b)?;
    let constant =
      field_chip.load_constant(layouter.namespace(|| "load constant"), self.constant)?;

    // Finally, tell the circuit how to use our Chip
    let ab = field_chip.mul(layouter.namespace(|| "a * b"), a, b)?;
    let ab_sq = field_chip.mul(layouter.namespace(|| "a * b"), ab.clone(), b)?;
    let c = field_chip.mul(layouter.namespace(|| "a * b"), ab_sq, constant)?;

    // and "return" the result as a public input to the circuit
    field_chip.expose_public(layouter.namespace(|| "expose result"), c, 0)
  }
}

/// The state of each chip is stored in a field config struct.
#[derive(Clone)]
pub struct FieldConfig;

impl FieldConfig {
  fn configure<F: Field>(
    meta: &mut ConstraintSystem<F>,
    advice: [Column<Advice>; 2],
    instance: Column<Instance>,
    constant: Column<Fixed>,
  ) -> Self {
    todo!()
  }
}

#[derive(Clone)]
pub struct FieldChip<F: Field> {
  // the advice columns
  advice:   [Column<Advice>; 2],
  // the instance column
  instance: Column<Instance>,
  // the constant column
  constant: Column<Fixed>,
}

impl<F: Field> FieldChip<F> {
  fn construct(config: FieldConfig) -> Self { todo!() }
}
