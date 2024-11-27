pub trait Algorithm {
    fn validate(digits: Vec<u8>) -> bool;
}

pub struct LuhnAlgorithm;

impl LuhnAlgorithm {
    fn split_payload_and_check(mut digits: Vec<u8>) -> (Vec<u8>, u8) {
        let check: u8 = digits.pop().unwrap();
        (digits, check)
    }
    fn compute_check_digit(digits: Vec<u8>) -> u8 {
        let result: u8 = digits
            .iter()
            .rev()
            .enumerate()
            .map(|(index, elem)| {
                if index % 2 == 0 {
                    let double = elem * 2;
                    if double > 9 {
                        return double - 9;
                    } else {
                        return double;
                    }
                } else {
                    return *elem;
                }
            })
            .sum();
        (10 - (result % 10)) % 10
    }
}

impl Algorithm for LuhnAlgorithm {
    fn validate(digits: Vec<u8>) -> bool {
        let (payload, check): (Vec<u8>, u8) = Self::split_payload_and_check(digits);
        let computed_check: u8 = Self::compute_check_digit(payload);
        check == computed_check
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_payload_and_check() {
        let digits: Vec<u8> = vec![1, 7, 8, 9, 3, 7, 2, 9, 9, 7, 4];
        let expected_check = 4;
        let expected_payload: Vec<u8> = vec![1, 7, 8, 9, 3, 7, 2, 9, 9, 7];

        let (payload, check) = LuhnAlgorithm::split_payload_and_check(digits);
        assert_eq!(check, expected_check);
        assert_eq!(payload, expected_payload);
    }
    #[test]
    fn test_compute_check_digit() {
        let digits: Vec<u8> = vec![1, 7, 8, 9, 3, 7, 2, 9, 9, 7];
        let expected_check = 4;
        let check = LuhnAlgorithm::compute_check_digit(digits);
        assert_eq!(check, expected_check);
    }

    #[test]
    fn test_luhn_algorithm_validation() {
        let digits: Vec<u8> = vec![1, 7, 8, 9, 3, 7, 2, 9, 9, 7, 4];
        let expected_result: bool = true;
        let result: bool = LuhnAlgorithm::validate(digits);
        assert_eq!(result, expected_result);
    }
}
