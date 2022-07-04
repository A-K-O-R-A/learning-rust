use num::{BigInt, ToPrimitive};

#[allow(dead_code)]
mod small {

    pub fn factorial(n: usize) -> usize {
        let mut k = 1;
        let mut prod = 1;
        while k < n + 1 {
            prod *= k;
            k += 1;
        }

        prod
    }

    // Calculate large pi digits.
    // Calculate large pi digits.
    pub fn chudnovsky_algorithm(n: usize) -> f64 {
        let mut rat = 0_f64;
        let mut q: usize = 0;
        while q < n + 1 {
            let top: i64 = (factorial(6 * q) * (545140134 * q + 13591409))
                .try_into()
                .unwrap();
            let bot_1: i64 = (factorial(3 * q) * factorial(q).pow(3)).try_into().unwrap();
            let bot_2: i64 = (-262537412640768000_i64).pow(q as u32);
            let bottom = bot_1 * bot_2;

            rat += top as f64 / bottom as f64;

            q += 1;
        }

        1_f64 / (rat / (426880_f64 * 10005_f64.sqrt()))
    }
}

mod big {
    use num::{bigint::ToBigInt, traits::Pow, BigInt, BigRational, One, ToPrimitive, Zero};

    pub fn factorial(n: BigInt) -> BigInt {
        let mut k: BigInt = One::one();
        let mut prod: BigInt = One::one();
        while &k <= &n {
            prod *= &k;
            k += 1;
        }

        prod
    }
    pub fn factorial_ref(n: &BigInt) -> BigInt {
        let mut k: BigInt = One::one();
        let mut prod: BigInt = One::one();
        while &k <= &n {
            prod *= &k;
            k += 1;
        }

        prod
    }

    pub fn factorial_rat(n: BigRational) -> BigRational {
        let mut k: BigRational = One::one();
        let mut prod: BigRational = One::one();
        while &k <= &n {
            prod *= &k;
            k += BigRational::one();
        }

        prod
    }
    pub fn factorial_rat_ref(n: &BigRational) -> BigRational {
        let mut k: BigRational = One::one();
        let mut prod: BigRational = One::one();
        while &k <= &n {
            prod *= &k;
            k += BigRational::one();
        }

        prod
    }

    // Calculate large pi digits.
    pub fn chudnovsky_algorithm(n: BigInt) -> BigRational {
        let mut rat: BigRational = One::one();
        let mut q: BigInt = Zero::zero();
        while &q <= &n {
            let top: BigRational = factorial(6_u8.to_bigint().unwrap() * &q)
                * (545140134_u64.to_bigint().unwrap() * &q + 13591409_u64.to_bigint().unwrap());
            let bot_1: BigInt =
                factorial(3_u8.to_bigint().unwrap() * &q) * factorial_ref(&q).pow(3_u32);

            let qwq = (&q + BigInt::zero()).to_u32().unwrap();
            let bot_2: BigInt = (&(-262537412640768000_i64.to_bigint().unwrap())).pow(qwq);

            println!(
                "{} / {} * {} = {}",
                top,
                bot_1,
                bot_2,
                &top / (&bot_1 * &bot_2)
            );
            let bottom = bot_1 * bot_2;

            rat += top / bottom;

            println!("{}", rat);
            q += BigInt::one();
        }

        BigRational::one()
            / (rat / 426880_u32.to_bigint().unwrap() * (10005_u32.to_bigint().unwrap()).sqrt())
    }
}

/*
fn chudnovsky_algorithm(n: usize) -> BigRational {
    let rat = BigRational::new(Zero::one(), One::one());

    1 / BigRational(426880 * sqrt(10005), rat)
}
*/

fn main() {
    // This is a very large number.
    println!(
        "alg(1000) = {}",
        big::chudnovsky_algorithm(BigInt::from(1)).to_f64().unwrap()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    mod small_tests {
        use super::*;
        #[test]
        fn chudnovsky_algorithm() {
            let pi = small::chudnovsky_algorithm(1);
            assert_eq!(pi, 3.141592653589793);
        }

        mod factorial {
            use super::*;
            #[test]
            fn zero() {
                let fac = small::factorial(0);
                assert_eq!(fac, 1);
            }

            #[test]
            fn one() {
                let fac = small::factorial(1);
                assert_eq!(fac, 1);
            }

            #[test]
            fn ten() {
                let fac = small::factorial(10);
                assert_eq!(fac, 3_628_800);
            }
        }
    }

    mod big_tests {
        use super::*;
        // #[test]
        // fn chudnovsky_algorithm() {
        //     let pi = big::chudnovsky_algorithm(1);
        //     assert_eq!(pi, 3.141592653589793);
        // }

        mod factorial {
            use num::{One, Zero};

            use super::*;
            #[test]
            fn zero() {
                let fac = big::factorial(BigInt::zero());
                assert_eq!(fac, BigInt::one());
            }

            #[test]
            fn one() {
                let fac = big::factorial(BigInt::one());
                assert_eq!(fac, BigInt::one());
            }

            #[test]
            fn ten() {
                let fac = big::factorial(BigInt::from(10_u8));
                assert_eq!(fac, BigInt::from(3_628_800_u64));
            }
        }

        mod factorial_ref {
            use num::{One, Zero};

            use super::*;
            #[test]
            fn zero() {
                let fac = big::factorial_ref(&BigInt::zero());
                assert_eq!(fac, BigInt::one());
            }

            #[test]
            fn one() {
                let fac = big::factorial_ref(&BigInt::one());
                assert_eq!(fac, BigInt::one());
            }

            #[test]
            fn ten() {
                let fac = big::factorial_ref(&BigInt::from(10_u8));
                assert_eq!(fac, BigInt::from(3_628_800_u64));
            }
        }
    }
}
