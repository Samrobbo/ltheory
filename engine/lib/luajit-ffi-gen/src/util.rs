/// Convert snake case string into a camel case one.
///
/// Rules:
/// - make first letter an upper case
/// - remove underscore and make after that the first letter upper case
/// - make first letter after digit an upper case
pub fn as_camel_case(s: &str, first_upper: bool) -> String {
    let mut res = String::new();
    let mut to_upper = first_upper;

    for c in s.trim_start_matches('_').chars() {
        if c == '_' {
            // Skip underscores
            to_upper = true;
        } else if c.is_digit(10) {
            res.push(c);
            // First letter after numbers should be uppercase
            to_upper = true;
        } else if to_upper {
            res += &c.to_uppercase().to_string();
            to_upper = false;
        } else {
            res.push(c);
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::as_camel_case;

    #[test]
    fn test_as_camel_case1() {
        let res = as_camel_case("set_var", true);
        assert_eq!(res, "SetVar")
    }

    #[test]
    fn test_as_camel_case2() {
        let res = as_camel_case("set2d", true);
        assert_eq!(res, "Set2D")
    }

    #[test]
    fn test_as_camel_case3() {
        let res = as_camel_case("set_2d", true);
        assert_eq!(res, "Set2D")
    }

    #[test]
    fn test_as_camel_case4() {
        let res = as_camel_case("set_var", false);
        assert_eq!(res, "setVar")
    }
}
