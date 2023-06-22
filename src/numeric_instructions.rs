use ff::Field;
use halo2_proofs::{
  circuit::{AssignedCell, Chip, Layouter, Region, Value},
  plonk::Error,
};

use crate::chip::MyChip;

// The instructions our circuit relies on.
// Instructions are the API between high-level gadgets and low-level chips.
//
// We need three instructions:
// - load a private number
// - multiply two numbers
// - expose a number as public to the circuit
pub trait NumericInstructions<F: Field>: Chip<F> {
  /// Variable representing a number.
  type U;

  /// Loads a number into the circuit as a private input.
  fn load_private(&self, layouter: impl Layouter<F>, a: Value<F>) -> Result<Self::U, Error>;

  /// Loads a number into the circuit as a fixed constant.
  fn load_constant(&self, layouter: impl Layouter<F>, constant: F) -> Result<Self::U, Error>;

  /// Returns `c = a * b`.
  fn mul(&self, layouter: impl Layouter<F>, a: Self::U, b: Self::U) -> Result<Self::U, Error>;

  /// Exposes a number as a public input to the circuit.
  fn expose_public(
    &self,
    layouter: impl Layouter<F>,
    num: Self::U,
    row: usize,
  ) -> Result<(), Error>;
}

// Next up, we implement a gadget for MyChip
#[derive(Clone, Debug)]

/// Represent a value at a cell
pub struct Number<F: Field>(AssignedCell<F, F>);

impl<F: Field> NumericInstructions<F> for MyChip<F> {
  type U = Number<F>;

  // load the private input
  fn load_private(
    &self,
    mut layouter: impl halo2_proofs::circuit::Layouter<F>,
    value: Value<F>,
  ) -> Result<Self::U, halo2_proofs::plonk::Error> {
    let config = self.config();

    layouter.assign_region(
      || "load private",
      |mut region| {
        region.assign_advice(|| "private input", config.advice[0], 0, || value).map(Number)
      },
    )
  }

  // load the constant
  fn load_constant(&self, mut layouter: impl Layouter<F>, constant: F) -> Result<Self::U, Error> {
    let config = self.config();

    layouter.assign_region(
      || "load constant",
      |mut region| {
        region
          .assign_advice_from_constant(|| "constant value", config.advice[0], 0, constant)
          .map(Number)
      },
    )
  }

  fn mul(&self, mut layouter: impl Layouter<F>, a: Self::U, b: Self::U) -> Result<Self::U, Error> {
    let config = self.config();

    layouter.assign_region(
      || "mul",
      |mut region: Region<'_, F>| {
        // We only want to use a single multiplication gate in this region,
        // so we enable it at region offset 0; this means it will constrain
        // cells at offsets 0 and 1.
        config.s_mul.enable(&mut region, 0)?;

        // The inputs we've been given could be located anywhere in the circuit,
        // but we can only rely on relative offsets inside this region. So we
        // assign new cells inside the region and constrain them to have the
        // same values as the inputs.
        dbg!(&a, &b);
        a.0.copy_advice(|| "lhs", &mut region, config.advice[0], 0)?;
        b.0.copy_advice(|| "rhs", &mut region, config.advice[1], 0)?;
        dbg!(&a, &b);

        // Now we can assign the multiplication result, which is to be assigned
        // into the output position.
        let value = a.0.value().copied() * b.0.value();

        // Finally, we do the assignment to the output, returning a
        // variable to be used in another part of the circuit.
        region.assign_advice(|| "lhs * rhs", config.advice[0], 1, || value).map(Number)
      },
    )
  }

  fn expose_public(
    &self,
    mut layouter: impl Layouter<F>,
    num: Self::U,
    row: usize,
  ) -> Result<(), Error> {
    let config = self.config();

    layouter.constrain_instance(num.0.cell(), config.instance, row)
  }
}
