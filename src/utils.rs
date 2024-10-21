use rand::Rng;

//  =========
//  UTILITIES
//  =========

/// Generate a random number between min and max
pub fn random_number_between<T: PartialOrd + rand::distributions::uniform::SampleUniform>(
    min: T,
    max: T,
) -> T {
    rand::thread_rng().gen_range(min..max)
}

#[cfg(test)]
mod tests {

    #[test]
    fn no_random_number_between_0_and_1() {
        assert_eq!(super::random_number_between(0, 1), 0);
    }

    #[test]
    fn random_number_within_range() {
        let min = -15;
        let max = 45;
        for _ in 0..100 {
            let x = super::random_number_between(min, max);
            if x < min || x >= max {
                panic!("Random number not between specified range")
            }
        }
    }

    /// Returns the type of the variable
    fn type_of<T>(_: &T) -> &str {
        std::any::type_name::<T>()
    }

    #[test]
    fn random_i32s_return_i32() {
        let number = super::random_number_between(-15, 32);
        let type_of_number = type_of(&number);
        assert_eq!(type_of_number, "i32");
    }

    #[test]
    fn random_u16s_return_u16() {
        let number = super::random_number_between::<u16>(0, 120);
        let type_of_number = type_of(&number);
        assert_eq!(type_of_number, "u16");
    }
}
