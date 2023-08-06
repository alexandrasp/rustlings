// This powerful wrapper provides the ability to store a positive integer value.
// Rewrite it using generics so that it supports wrapping ANY type.

// Execute `rustlings hint generics2` or use the `hint` watch subcommand for a hint.

struct Wrapper<T> {
    value: T,
}

impl<T> Wrapper<T>{
    pub fn new(self) -> T {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        let testInt = Wrapper{value: 42};
        assert_eq!(testInt.new(), 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        let testStr = Wrapper{value: String::from("Foo")};
        assert_eq!(testStr.new(), "Foo");
    }
}
