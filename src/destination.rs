use crate::Truck;

#[derive(Debug)]
#[derive(PartialEq)]
enum Destination {
    Factory,
    Port,
}

impl Destination {
    fn of(truck: Truck) -> Destination {
        match (truck.cargo) {
           None => Destination::Factory,
           Some('A') => Destination::Port,
           _ => panic!("Unexpected!")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Truck};

    use super::Destination;
    #[test]
    fn destination_for_an_empty_truck_is_a_factory() {
        let truck = Truck::new();
        assert_eq!(Destination::of(truck), Destination::Factory)
    }

    #[test]
    fn destination_for_a_truck_with_a_cargo_is_a_port() {
        let mut truck = Truck::new();
        truck.load('A');
        assert_eq!(Destination::of(truck), Destination::Port)
    }
} 