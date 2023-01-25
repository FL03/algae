#[cfg(test)]
#[test]
fn lib_compiles() {
    let f = |i| i + 1;
    assert_eq!(f(10), 11)
}
