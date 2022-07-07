/// Compare 2 slices of &str and returns 2 vectors.
/// The first element contains the list of elements `lhs` not contained in `rhs`.
/// The second element contains the list of elements `rhs` not contained in `lhs`.
pub fn compare_vec<'value>(
    lhs: &[&'value str],
    rhs: &[&'value str],
) -> (Vec<&'value str>, Vec<&'value str>) {
    let mut ret_lhs = Vec::new();
    let mut ret_rhs = Vec::new();
    // search the abscense of the elements of lhs in rhs
    for lhs_item in lhs.iter() {
        if !rhs.contains(lhs_item) {
            ret_lhs.push(*lhs_item);
        }
    }

    // search the abscense of the elements of rhs in lhs
    for rhs_item in rhs.iter() {
        if !lhs.contains(rhs_item) {
            ret_rhs.push(*rhs_item);
        }
    }

    (ret_lhs, ret_rhs)
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_compare_vec_equals() {
        let lhs = ["alfa", "beta", "gamma"];
        let rhs = ["alfa", "beta", "gamma"];
        let (ret_lhs, ret_rhs) = compare_vec(&lhs, &rhs);
        assert!(ret_lhs.is_empty());
        assert!(ret_rhs.is_empty());
    }

    #[test]
    fn test_compare_vec_lhs_greater() {
        let lhs = ["alfa", "beta", "gamma", "ommega", "epsilon"];
        let rhs = ["alfa", "beta", "gamma"];
        let (ret_lhs, ret_rhs) = compare_vec(&lhs, &rhs);
        assert_eq!(ret_lhs[0], "ommega");
        assert_eq!(ret_lhs[1], "epsilon");
        assert!(ret_rhs.is_empty());
    }

    #[test]
    fn test_compare_vec_rhs_greater() {
        let lhs = ["alfa", "beta", "gamma"];
        let rhs = ["alfa", "beta", "gamma", "ommega", "epsilon"];
        let (ret_lhs, ret_rhs) = compare_vec(&lhs, &rhs);
        assert_eq!(ret_rhs[0], "ommega");
        assert_eq!(ret_rhs[1], "epsilon");
        assert!(ret_lhs.is_empty());
    }

    #[test]
    fn test_compare_vec_with_differences() {
        let lhs = ["alfa", "beta", "gamma", "epsilon"];
        let rhs = ["alfa", "beta", "gamma", "ommega"];
        let (ret_lhs, ret_rhs) = compare_vec(&lhs, &rhs);
        assert_eq!(ret_lhs[0], "epsilon");
        assert_eq!(ret_rhs[0], "ommega");
    }
}
