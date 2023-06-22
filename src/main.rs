//! following instructions at https://zcash.github.io/halo2/user/simple-example.html#define-instructions
//! A simple example of programming a^2+b^2=c in Halo 2
//! see: https://github.com/zcash/halo2/blob/main/halo2_proofs/examples/simple-example.rs
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
// use error::MyError;
// // use tracing::info;
// use log::{info, error};

// mod error;
// #[cfg(test)] mod tests;
// mod utils;
mod circuit;
mod chip;

use ff::Field;
use halo2_proofs::circuit::Value;

use crate::circuit::MyCircuit;

fn main() {
  use halo2_proofs::{dev::MockProver, pasta::Fp};

  // ANCHOR: test-circuit
  // 2^k is the number of rows in our circuit
  let k = 4;

  // Instantiate the circuit with the private inputs.
  let a = Fp::from(2);
  let b = Fp::from(3);
  let c = a.square() + b.square();
  // just for the sake of demonstration, show we can used fixed columns to load constants
  let constant = Fp::from(1);
  let (a, b) = (Value::known(a), Value::known(b));
  let my_circuit = MyCircuit { a, b, constant };

  // Arrange the public input. We expose the multiplication result in row 0
  // of the instance column, so we position it there in our public inputs.
  let mut public_inputs = vec![c];

  // // Given the correct public input, our circuit will verify.
  let prover = MockProver::run(k, &my_circuit, vec![public_inputs.clone()]).unwrap();
  assert_eq!(prover.verify(), Ok(()));

  // // If we try some other public input, the proof will fail!
  public_inputs[0] += Fp::one();
  let prover = MockProver::run(k, &my_circuit, vec![public_inputs]).unwrap();
  assert!(prover.verify().is_err());
}
