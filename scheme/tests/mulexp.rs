#![allow(non_snake_case)]

use math::msm::{FixedBaseMSM, VariableBaseMSM};
use math::{
    AffineCurve, Group, One, PairingEngine, PrimeField, ProjectiveCurve, UniformRand, Zero,test_rng,
};
use curve::bn_256::{Bn_256, Fr};

use std::time::{Duration, Instant};
use core::marker::PhantomData;

#[cfg(feature = "parallel")]
use rayon::prelude::*;

#[derive(Debug)]
struct MulExpTest<E: PairingEngine> {
    n: usize,
    _g1_generator: PhantomData<E>,
}

impl<E: PairingEngine> MulExpTest<E> {

    fn test_mulexp()
    {
        let n = 5;
        let shift = 14;
        let mut rng = &mut test_rng();
    
        let scalar_bits = E::Fr::size_in_bits();
        let mut g1_generator = E::G1Projective::rand(&mut rng);
        let mut g1_window = FixedBaseMSM::get_mul_window_size(n);
        let mut g1_table = FixedBaseMSM::get_window_table::<E::G1Projective>(scalar_bits, g1_window, g1_generator);
    
        let mut v = vec![];
        let mut t = Duration::new(0, 0);
        let mut start = Instant::now();
    
        for _ in 0..2<<shift {
            v = (0..n).map(|_| E::Fr::rand(&mut rng)).collect::<_>();
            FixedBaseMSM::multi_scalar_mul::<E::G1Projective>(scalar_bits, g1_window, &g1_table, &v);
        }
        t += start.elapsed();
        println!("{:?}", t);
    
        t = Duration::new(0, 0);
    
        for _ in 0..2<<shift {
            g1_generator = E::G1Projective::rand(&mut rng);
            g1_window = FixedBaseMSM::get_mul_window_size(n);
            g1_table = FixedBaseMSM::get_window_table::<E::G1Projective>(scalar_bits, g1_window, g1_generator);
            v = (0..n).map(|_| E::Fr::rand(&mut rng)).collect::<_>();
            
            start = Instant::now();
            FixedBaseMSM::multi_scalar_mul::<E::G1Projective>(scalar_bits, g1_window, &g1_table, &v);
            t += start.elapsed();
        }
        println!("{:?}", t);
    
    }
}

#[test]
fn mulexp()
{
    MulExpTest::<Bn_256>::test_mulexp();
}

