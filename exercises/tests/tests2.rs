// tests2.rs
// This test has a problem with it -- make the test compile! Make the test
// pass! Make the test fail! Execute `rustlings hint tests2` for hints :)


fn call_abc(ori: String) -> String{
    let mut b = ori;
    b += "bar";
    b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn you_can_assert_eq() {
        assert_eq!(call_abc(String::from("foo")), String::from("foobar"));
    }
}
