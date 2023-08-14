use ark_bls12_377::{
    G1Affine as G1,
    G2Affine as G2,
    G1Projective as PG1,
    G2Projective as PG2,
    Fr,
};
use ark_ec::{
    VariableBaseMSM, Group
};
use ark_std::{
    UniformRand,
    log2
};

pub struct Kzg {
    pub srs: (Vec<G1>, Vec<G2>)
}

impl Kzg {
    pub fn new(size1: u32, size2: u32) -> Self{
        // Size must be an exeponential number
        assert!(size1 > 2);
        assert!(2_u32.pow(log2(size1 as usize)) == size1);
        assert!(size2 > 2);
        
        let mut g1s = Vec::<G1>::new();
        let mut g2s = Vec::<G2>::new();
        let g_1 = PG1::generator();
        let g_2 = PG2::generator();
        g1s.push(g_1.into());
        g2s.push(g_2.into());
        let mut rng_gen = ark_std::test_rng();
        let tau = Fr::rand(&mut rng_gen);
        for _i in 1..size1 {
            g1s.push((*(g1s.last().unwrap()) * &tau).into());
        }
        for _i in 1..size2 {
            g2s.push((*(g2s.last().unwrap()) * &tau).into());
        }
        Self {
            srs: (g1s, g2s)
        }
    }

    pub fn commit(self, coeffs: &Vec<Fr>) -> G1 {
        assert!(coeffs.len() <= self.srs.0.len());
        PG1::msm(&self.srs.0[..coeffs.len()], &coeffs)
            .unwrap()
            .into()
    }

    pub fn log_size(self) -> u32 {
        log2(self.srs.0.len())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_kzg() {
        let kzg = Kzg::new(512, 4);
        let coeffs = vec![Fr::from(1928); 6];
        let com = kzg.commit(&coeffs);
        println!("{}", com);
    }
}