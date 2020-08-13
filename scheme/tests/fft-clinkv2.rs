// #![allow(unused_imports)]

// use math::fft::EvaluationDomain;
// use math::{test_rng, Field};

// // For randomness (during paramgen and proof generation)
// use rand::Rng;

// // For benchmarking
// use std::time::{Duration, Instant};

// // Bring in some tools for using pairing-friendly curves
// use curve::bn_256::{Bn_256, Fr};

// // We're going to use the BN-256 pairing-friendly elliptic curve.

// // We'll use these interfaces to construct our circuit.
// use scheme::r1cs::{ConstraintSynthesizer, ConstraintSystem, SynthesisError};



// #[test]
// #[allow(variant_size_differences)]
// fn test_fft() //-> Result<usize, SynthesisError>
// {
//     let domain_size = 5;
//     let domain = EvaluationDomain::<Fr>::new(domain_size).unwrap();
//     let rng = &mut test_rng();
//     let evals:Vec<Fr> = (0..domain_size).map(|_| rng.gen()).collect::<Vec<_>>();
//     // //let evals = vec![Fr::rand(&mut rng); domain_size];
//     let coeffs = domain.ifft(&evals);
//     println!("{:?}", coeffs);
//     // Ok(domain_size)
// }


// // // For randomness (during paramgen and proof generation)
// // use rand::Rng;

// // // Bring in some tools for using pairing-friendly curves
// // use curve::bn_256::{Bn_256, Fr};
// // use math::{test_rng, Field};

// // // We're going to use the BN-256 pairing-friendly elliptic curve.

// // // We'll use these interfaces to construct our circuit.
// // use scheme::r1cs::{SynthesisError};


// // #[test]
// // fn test_mimc_groth_16() {
// //     // We're going to use the Groth16 proving system.
// //     use scheme::groth16::{
// //         create_random_proof, generate_random_parameters, prepare_verifying_key, verify_proof,
// //     };

// //     // This may not be cryptographically safe, use
// //     // `OsRng` (for example) in production software.
// //     let rng = &mut test_rng();

// //     // Generate the MiMC round constants
// //     let constants = (0..MIMC_ROUNDS).map(|_| rng.gen()).collect::<Vec<_>>();

// //     println!("Creating parameters...");

// //     // Create parameters for our circuit
// //     let params = {
// //         let c = MiMCDemo::<Fr> {
// //             xl: None,
// //             xr: None,
// //             constants: &constants,
// //         };

// //         generate_random_parameters::<Bn_256, _, _>(c, rng).unwrap()
// //     };

// //     // Prepare the verification key (for proof verification)
// //     let pvk = prepare_verifying_key(&params.vk);

// //     println!("Creating proofs...");

// //     // Let's benchmark stuff!
// //     const SAMPLES: u32 = 3;
// //     let mut total_proving = Duration::new(0, 0);
// //     let mut total_verifying = Duration::new(0, 0);

// //     // Just a place to put the proof data, so we can
// //     // benchmark deserialization.
// //     // let mut proof_vec = vec![];

// //     for _ in 0..SAMPLES {
// //         // Generate a random preimage and compute the image
// //         let xl = rng.gen();
// //         let xr = rng.gen();
// //         let image = mimc(xl, xr, &constants);

// //         // proof_vec.truncate(0);

// //         let start = Instant::now();
// //         {
// //             // Create an instance of our circuit (with the
// //             // witness)
// //             let c = MiMCDemo {
// //                 xl: Some(xl),
// //                 xr: Some(xr),
// //                 constants: &constants,
// //             };

// //             // Create a groth16 proof with our parameters.
// //             let proof = create_random_proof(c, &params, rng).unwrap();
// //             assert!(verify_proof(&pvk, &proof, &[image]).unwrap());

// //             // proof.write(&mut proof_vec).unwrap();
// //         }

// //         total_proving += start.elapsed();

// //         let start = Instant::now();
// //         // let proof = Proof::read(&proof_vec[..]).unwrap();
// //         // Check the proof

// //         total_verifying += start.elapsed();
// //     }
// //     let proving_avg = total_proving / SAMPLES;
// //     let proving_avg =
// //         proving_avg.subsec_nanos() as f64 / 1_000_000_000f64 + (proving_avg.as_secs() as f64);

// //     let verifying_avg = total_verifying / SAMPLES;
// //     let verifying_avg =
// //         verifying_avg.subsec_nanos() as f64 / 1_000_000_000f64 + (verifying_avg.as_secs() as f64);

// //     println!("Average proving time: {:?} seconds", proving_avg);
// //     println!("Average verifying time: {:?} seconds", verifying_avg);
// // }
