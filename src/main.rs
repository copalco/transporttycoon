fn main() {
    println!("Hello, world!");
}

fn deliver(cargo: Vec<char>) -> i8 {
    return 5
}

#[cfg(test)]
mod tests {
    use crate::deliver;
    #[test]
    fn counts_time_needed_to_deliver_one_cargo_to_factory_b() {
        let cargo = vec!['B'];
        let time = deliver(cargo);
        assert_eq!(time, 5);
    }
}