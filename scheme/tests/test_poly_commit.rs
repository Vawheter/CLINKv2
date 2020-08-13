// #![allow(non_camel_case_types)]
// #![allow(unused_imports)]

// use crate::{Error, LabeledPolynomial, PCRandomness, Polynomial};

// use poly_commit::kzg10::*;
// use poly_commit::*;

// use algebra::bls12_381::Fr;
// use algebra::test_rng;
// use algebra::Bls12_377;
// use algebra::Bls12_381;

// use algebra_core::msm::{FixedBaseMSM, VariableBaseMSM};
// use algebra_core::{
//     AffineCurve, Group, One, PairingEngine, PrimeField, ProjectiveCurve, UniformRand, Zero,
// };

// use std::borrow::Cow;

// type KZG_Bls12_381 = KZG10<Bls12_381>;

// fn end_to_end_test_template<E: PairingEngine>() -> Result<(), Error> {
//     let rng = &mut test_rng();
//     for _ in 0..100 {
//         let mut degree = 0;
//         while degree <= 1 {
//             degree = usize::rand(rng) % 20;
//         }
//         let pp = KZG10::<E>::setup(degree, false, rng)?;
//         let (ck, vk) = KZG10::trim(&pp, degree)?;
//         let p = Polynomial::rand(degree, rng);
//         let hiding_bound = Some(1);
//         let (comm, rand) = KZG10::<E>::commit(&ck, &p, hiding_bound, Some(rng))?;
//         let point = E::Fr::rand(rng);
//         let value = p.evaluate(point);
//         let proof = KZG10::<E>::open(&ck, &p, point, &rand)?;
//         assert!(
//             KZG10::<E>::check(&vk, &comm, point, value, &proof)?,
//             "proof was incorrect for max_degree = {}, polynomial_degree = {}, hiding_bound = {:?}",
//             degree,
//             p.degree(),
//             hiding_bound,
//         );
//     }
//     Ok(())
// }

// fn batch_check_test_template<E: PairingEngine>() -> Result<(), Error> {
//     let rng = &mut test_rng();
//     for _ in 0..10 {
//         let mut degree = 0;
//         while degree <= 1 {
//             degree = usize::rand(rng) % 20;
//         }
//         let pp = KZG10::<E>::setup(degree, false, rng)?;
//         let (ck, vk) = KZG10::trim(&pp, degree)?;
//         let mut comms = Vec::new();
//         let mut values = Vec::new();
//         let mut points = Vec::new();
//         let mut proofs = Vec::new();
//         for _ in 0..10 {
//             let p = Polynomial::rand(degree, rng);
//             let hiding_bound = Some(1);
//             let (comm, rand) = KZG10::<E>::commit(&ck, &p, hiding_bound, Some(rng))?;
//             let point = E::Fr::rand(rng);
//             let value = p.evaluate(point);
//             let proof = KZG10::<E>::open(&ck, &p, point, &rand)?;

//             assert!(KZG10::<E>::check(&vk, &comm, point, value, &proof)?);
//             comms.push(comm);
//             values.push(value);
//             points.push(point);
//             proofs.push(proof);
//         }
//         assert!(KZG10::<E>::batch_check(
//             &vk, &comms, &points, &values, &proofs, rng
//         )?);
//     }
//     Ok(())
// }

// #[test]
// fn end_to_end_test() {
//     end_to_end_test_template::<Bls12_377>().expect("test failed for bls12-377");
//     end_to_end_test_template::<Bls12_381>().expect("test failed for bls12-381");
// }

// #[test]
// fn batch_check_test() {
//     batch_check_test_template::<Bls12_377>().expect("test failed for bls12-377");
//     batch_check_test_template::<Bls12_381>().expect("test failed for bls12-381");
// }