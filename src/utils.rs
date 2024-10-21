use rand::Rng;

//  =========
//  UTILITIES
//  =========

/// Generates a random number between `min` and `max`.
///
/// #### Arguments
///
/// * `min` - The lower bound of the range (inclusive).
/// * `max` - The upper bound of the range (exclusive).
///
/// #### Panics
///
/// Panics if `min` is equal to or greater than `max`.
pub fn random_number_between<T: PartialOrd + rand::distributions::uniform::SampleUniform>(
    min: T,
    max: T,
) -> T {
    rand::thread_rng().gen_range(min..max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_random_number_between_0_and_1() {
        assert_eq!(random_number_between(0, 1), 0);
    }

    #[test]
    fn random_number_within_range() {
        let min = -15;
        let max = 45;
        for _ in 0..100 {
            let x = random_number_between(min, max);
            if x < min || x >= max {
                panic!("Random number not between specified range")
            }
        }
    }

    #[test]
    fn random_number_between_floats() {
        let min = 0.5;
        let max = 5.5;
        for _ in 0..100 {
            let x = random_number_between(min, max);
            assert!(
                x >= min && x < max,
                "Random float not within specified range"
            );
        }
    }

    #[test]
    fn random_number_with_negative_min() {
        let min = -50;
        let max = 50;
        for _ in 0..100 {
            let x = random_number_between(min, max);
            assert!(
                x >= min && x < max,
                "Random number not within specified range when min is negative"
            );
        }
    }

    /// Returns the type of the variable
    fn type_of<T>(_: &T) -> &str {
        std::any::type_name::<T>()
    }

    #[test]
    fn random_i32s_return_i32() {
        let number = random_number_between(-15, 32);
        let type_of_number = type_of(&number);
        assert_eq!(type_of_number, "i32");
    }

    #[test]
    fn random_u16s_return_u16() {
        let number = random_number_between::<u16>(0, 120);
        let type_of_number = type_of(&number);
        assert_eq!(type_of_number, "u16");
    }

    #[test]
    fn random_u32s_return_u32() {
        let number = random_number_between::<u32>(100, 1000);
        let type_of_number = type_of(&number);
        assert_eq!(type_of_number, "u32");
    }

    #[test]
    fn random_i64s_return_i64() {
        let number = random_number_between::<i64>(-1000, 1000);
        let type_of_number = type_of(&number);
        assert_eq!(type_of_number, "i64");
    }
}
