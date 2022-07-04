fn small_factorial(n: usize) -> usize {
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
fn small_chudnovsky_algorithm(n: usize) -> f64 {
    let mut rat = 0_f64;
    let mut q: usize = 0;
    while q < n + 1 {
        let top: i64 = (small_factorial(6 * q) * (545140134 * q + 13591409))
            .try_into()
            .unwrap();
        let bot_1: i64 = (small_factorial(3 * q) * small_factorial(q).pow(3))
            .try_into()
            .unwrap();
        let bot_2: i64 = (-262537412640768000_i64).pow(q as u32);
        let bottom = bot_1 * bot_2;

        rat += top as f64 / bottom as f64;

        q += 1;
    }

    1_f64 / (rat / (426880_f64 * 10005_f64.sqrt()))
}
/*
fn chudnovsky_algorithm(n: usize) -> BigRational {
    let rat = BigRational::new(Zero::one(), One::one());

    1 / BigRational(426880 * sqrt(10005), rat)
}
*/

fn main() {
    // This is a very large number.
    println!("alg(1000) = {}", small_chudnovsky_algorithm(1));
}

#[cfg(test)]
mod tests {
    use super::*;

    mod small {
        use super::*;
        #[test]
        fn chudnovsky_algorithm() {
            let pi = small_chudnovsky_algorithm(1);
            assert_eq!(pi, 3.141592653589793);
        }

        mod factorial {
            use super::*;
            #[test]
            fn zero() {
                let fac = small_factorial(0);
                assert_eq!(fac, 1);
            }

            #[test]
            fn one() {
                let fac = small_factorial(1);
                assert_eq!(fac, 1);
            }

            #[test]
            fn ten() {
                let fac = small_factorial(10);
                assert_eq!(fac, 3_628_800);
            }
        }
    }

    mod big {
        use super::*;
        #[test]
        fn chudnovsky_algorithm() {
            let pi = small_chudnovsky_algorithm(1);
            assert_eq!(pi, 3.141592653589793);
        }

        mod factorial {
            use super::*;
            #[test]
            fn zero() {
                let fac = small_factorial(0);
                assert_eq!(fac, 1);
            }

            #[test]
            fn one() {
                let fac = small_factorial(1);
                assert_eq!(fac, 1);
            }

            #[test]
            fn ten() {
                let fac = small_factorial(10);
                assert_eq!(fac, 3_628_800);
            }
        }
    }
}
