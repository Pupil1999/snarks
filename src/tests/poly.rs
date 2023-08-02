#[cfg(test)]
mod test {
    use ark_ff::{FftField, PrimeField, One};
    use ark_poly::{
        Polynomial,
        univariate::DensePolynomial as P, DenseUVPolynomial,
        domain::EvaluationDomain,
        domain::GeneralEvaluationDomain
    };
    use ark_bn254::Fr;

    #[test]
    fn get_roots() {
        println!("Modulus is {}", Fr::MODULUS);
        println!("Generator of fft field is {}", Fr::GENERATOR);
        println!("Maximum subgroup size is 2^{}", Fr::TWO_ADICITY);
    }
        
    // Some tests to show the crate use cases
    #[test]
    fn test_eval() {
        let coeffs_1 = vec![Fr::from(1); 5];
        let poly_1 = P::<Fr>::from_coefficients_vec(coeffs_1);
        assert_eq!(poly_1.evaluate(&Fr::one()), Fr::from(5));
    }

    #[test]
    fn test_fft() {
        let domain = GeneralEvaluationDomain::<Fr>::new(8).unwrap();
        let coeffs = vec![Fr::from(1); 5];
        let poly = P::<Fr>::from_coefficients_vec(coeffs);

        // 不需要填充0
        let ffts = domain.fft(&poly.coeffs);
        
        // 
        let coeffs = domain.ifft(&ffts);
        println!("{:?}", coeffs);
        println!("{}", poly.evaluate(&Fr::from(2)));
    }
}