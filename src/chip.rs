use std::marker::PhantomData;

use ff::Field;
use halo2_proofs::{
  circuit::{AssignedCell, Chip, SimpleFloorPlanner, Value},
  plonk::{Advice, Circuit, Column, ConstraintSystem, Fixed, Instance, Selector, Expression},
  poly::Rotation,
};

use crate::numeric_instructions::NumericInstructions;

// the chip implements functionality, and stores config. Most of the actual chip logic happens in
// FieldConfig (why?)
#[derive(Clone)]
pub struct MyChip<F: Field> {
  pub config:  MyChipConfig,
  _marker: PhantomData<F>,
}

/// The state of each chip is stored in a config struct.
/// This includes advice and instance columns, and a selector to enable the multiplication gate.
#[derive(Clone, Debug)]
pub struct MyChipConfig {
  // Use two advice columns
  pub advice:   [Column<Advice>; 2],
  // and one instance column to store the public input
  pub instance: Column<Instance>,
  // finally use a selector to enable the multiplication gate
  pub s_mul:    Selector,
}

impl<F: Field> Chip<F> for MyChip<F> {
  // holds the state of the Chip
  type Config = MyChipConfig;
  // holds any optional state needing to be loaded prior to Circuit::synthesize
  type Loaded = ();

  fn config(&self) -> &Self::Config { &self.config }

  fn loaded(&self) -> &Self::Loaded { &() }
}

impl<F: Field> MyChip<F> {
  // construct a chip from a config, weird naming conventions in this place
  pub fn construct(config: <Self as Chip<F>>::Config) -> Self {
    Self { config, _marker: PhantomData }
  }
}

impl MyChipConfig {
  pub fn configure<F: Field>(
    meta: &mut ConstraintSystem<F>,
    advice: [Column<Advice>; 2],
    instance: Column<Instance>,
    constant: Column<Fixed>,
  ) -> Self {
    // specify the columns that can be compared used by the constraint system
    meta.enable_equality(instance);
    meta.enable_constant(constant);
    for column in &advice {
      meta.enable_equality(*column);
    }

    // meta selector is used to enable gates
    let s_mul = meta.selector();

    // Define our multiplication gate
    meta.create_gate("mul", |meta| {
      // To implement multiplication, we need three advice cells and a selector
      // cell. We arrange them like so:
      //
      // | a0  | a1  | s_mul |
      // |-----|-----|-------|
      // | lhs | rhs | s_mul |
      // | out |     |       |
      //
      // Gates may refer to any relative offsets we want, but each distinct
      // offset adds a cost to the proof. The most common offsets are 0 (the
      // current row), 1 (the next row), and -1 (the previous row), for which
      // `Rotation` has specific constructors.
      let lhs = meta.query_advice(advice[0], Rotation::cur());
      let rhs = meta.query_advice(advice[1], Rotation::cur());
      let out = meta.query_advice(advice[0], Rotation::next());
      let s_mul = meta.query_selector(s_mul);

      // Finally, we return the polynomial expressions that constrain this gate.
      // For our multiplication gate, we only need a single polynomial constraint.
      //
      // The polynomial expressions returned from `create_gate` will be
      // constrained by the proving system to equal zero. Our expression
      // has the following properties:
      // - When s_mul = 0, any value is allowed in lhs, rhs, and out.
      // - When s_mul != 0, this constrains lhs * rhs = out.
      vec![s_mul * (lhs * rhs - out)]
      // vec![Expression::Constant(F::ZERO)]
    });

    MyChipConfig { advice, instance, s_mul }
  }
}
