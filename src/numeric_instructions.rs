use ff::Field;
use halo2_proofs::{
  circuit::{AssignedCell, Chip, Layouter, Value},
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
#[derive(Clone)]
pub struct Number<F: Field>(AssignedCell<F, F>);

impl<F: Field> NumericInstructions<F> for MyChip<F> {
  type U = Number<F>;

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

  fn load_constant(
    &self,
    layouter: impl halo2_proofs::circuit::Layouter<F>,
    constant: F,
  ) -> Result<Self::U, halo2_proofs::plonk::Error> {
    todo!()
  }

  fn mul(
    &self,
    layouter: impl halo2_proofs::circuit::Layouter<F>,
    a: Self::U,
    b: Self::U,
  ) -> Result<Self::U, halo2_proofs::plonk::Error> {
    todo!()
  }

  fn expose_public(
    &self,
    layouter: impl halo2_proofs::circuit::Layouter<F>,
    num: Self::U,
    row: usize,
  ) -> Result<(), halo2_proofs::plonk::Error> {
    todo!()
  }
}
