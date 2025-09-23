// traits1.rs
//
// Time to implement some traits! Your task is to implement the trait
// `AppendBar` for the type `String`. The trait AppendBar has only one function,
// which appends "Bar" to any object implementing this trait.
//
// Execute `rustlings hint traits1` or use the `hint` watch subcommand for a
// hint.



trait AppendBar {
    fn append_bar(self) -> Self;
}

impl AppendBar for String {
     fn append_bar(self) -> Self
      {
        // String 类型的 `+` 操作符会消耗掉左边的 String (self)，
        // 然后连接右边的 &str，最后返回一个新的 String。
        // 这完美地匹配了我们的方法签名。
        self + "Bar"
    // TODO: Implement `AppendBar` for type `String`.
    }
}

fn main() {
    let s = String::from("Foo");
    let s = s.append_bar();
    println!("s: {}", s);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_foo_bar() {
        assert_eq!(String::from("Foo").append_bar(), String::from("FooBar"));
    }

    #[test]
    fn is_bar_bar() {
        assert_eq!(
            String::from("").append_bar().append_bar(),
            String::from("BarBar")
        );
    }
}
