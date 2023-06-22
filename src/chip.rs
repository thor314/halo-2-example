
use std::marker::PhantomData;

use ff::Field;
use halo2_proofs::{
  circuit::{Chip, SimpleFloorPlanner, Value},
  plonk::{Advice, Circuit, Column, ConstraintSystem, Fixed, Instance},
};

/// The state of each chip is stored in a field config struct.
#[derive(Clone, Debug)]
pub struct MyChipConfig;

impl MyChipConfig {
  pub fn configure<F: Field>(
    meta: &mut ConstraintSystem<F>,
    advice: [Column<Advice>; 2],
    instance: Column<Instance>,
    constant: Column<Fixed>,
  ) -> Self {
    todo!()
  }
}

// the chip implements functionality, and stores config. Most of the actual chip logic happens in
// FieldConfig (why?)
#[derive(Clone)]
pub struct MyChip<F: Field> {
  config:  MyChipConfig,
  _marker: PhantomData<F>, 
}

impl<F: Field> Chip<F> for MyChip<F> {
  // holds the state of the Chip
  type Config = MyChipConfig;
  // holds any optional state needing to be loaded prior to Circuit::synthesize
  type Loaded = ();

  fn config(&self) -> &Self::Config { &self.config }

  fn loaded(&self) -> &Self::Loaded { &() }
}

// Chip state and instructions are stored in the config struct. This will mirror the declaration of
// columns in MyCircuit::configure.
