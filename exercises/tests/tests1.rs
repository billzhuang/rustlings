// tests1.rs
// Tests are important to ensure that your code does what you think it should do.
// Tests can be run on this file with the following command:
// rustlings run tests1

// This test has a problem with it -- make the test compile! Make the test
// pass! Make the test fail! Execute `rustlings hint tests1` for hints :)


fn main(){
    let result = call_abc("Foo".to_string());
    println!("Result is {}", result);
}

fn call_abc(s: String) -> String {
    let mut s2 = s;
    s2 += "Bar";
    s2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn you_can_assert() {
        let result = call_abc("Foo".to_string());
        assert!(result == "FooBar");
    }
}
