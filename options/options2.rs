// options2.rs
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a hint.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        if let Some(t) = optional_target {
            assert_eq!(t, target);
        }
    }

    #[test]
    fn layered_option() {
        let mut range = 10;
        let mut optional_integers: Vec<Option<i8>> = Vec::new();
        for i in 0..(range + 1) {
            optional_integers.push(Some(i));
        }

        while range < 10 {
            let integer = optional_integers.pop();
            assert_eq!(integer.unwrap().unwrap(), range);
            range -= 1;
        }
    }
}
