pub mod destination;
fn main() {
    println!("Hello, world!");
}

#[derive(Debug)]
#[derive(PartialEq)]
enum Destination {
    Port,
    IslandWarehouse,
    Warehouse,
    Factory,
}

enum Vehicle {
    Truck,
    Ship,
}


impl Destination {
    fn from(vehicle: Vehicle, cargo: Option<char>) -> Destination {
        match (vehicle, cargo) {
            (Vehicle::Truck, Some('A')) => Self::Port,
            (Vehicle::Truck, Some('B')) => Self::Warehouse,
            (Vehicle::Truck, None) => Self::Factory,
            (Vehicle::Ship, Some('A')) => Self::IslandWarehouse,
            (Vehicle::Ship, None) => Self::Port,
            _ => panic!("Wrong cargo!")
        }
    }
}

#[derive(Debug)]
struct Truck {
    cargo: Option<char>,
    destination: Destination,
    position: i8,
}

impl Truck {
    fn new() -> Truck {
        Truck {
            cargo: None,
            destination: Destination::Factory,
            position: 0,
        }
    }

    fn load(&mut self, cargo: char) {
        self.cargo = Some(cargo);
        self.destination = Destination::from(Vehicle::Truck, Some(cargo));
    }

    fn loaded(&self) -> bool {
        return self.cargo.is_some();
    }

    fn unload(&mut self) -> char {
        let cargo = self.cargo.unwrap();
        self.cargo = None;
        self.destination = Destination::Factory;
        cargo
    }

    fn empty(&self) -> bool {
        self.cargo.is_none()
    }


    fn advance(&mut self) {
        if [Destination::Port, Destination::Warehouse].contains(&self.destination) {
            self.position += 1
        }
        else {
            self.position -= 1
        }
    }

    fn arrived(&self) -> bool {
        match self.destination {
          Destination::Port => self.position == 1,
          Destination::Warehouse => self.position == 5,
          Destination::Factory => self.position == 0,
          _ => panic!("Unable to get there!")
        }
    }
    fn in_transit(&self) -> bool {
        !self.arrived()
    }
}

#[derive(Debug)]
struct Ship {
    cargo: Option<char>,
    destination: Destination,
    position: i8,
}

impl Ship {
    fn new() -> Ship {
        return Ship { cargo: None, destination: Destination::Port, position: 0 };
    }

    fn available(&self) -> bool {
        self.empty() && self.position == 0
    }

    fn load(&mut self, cargo: char) -> () {
        self.cargo = Some(cargo);
        self.destination = Destination::from(Vehicle::Ship, Some(cargo));
    }

    fn advance(&mut self) -> () {
        match self.destination {
            Destination::IslandWarehouse => self.position += 1,
            Destination::Port => self.position -= 1,
            _ => panic!("Wrong destination!"),
        };
    }

    fn in_transit(&self) -> bool {
        !self.arrived()
    }

    fn arrived(&self) -> bool {
        match self.destination {
            Destination::Port => self.position == 0 && self.empty(),
            Destination::IslandWarehouse => self.position == 4 && self.loaded(),
            _ => panic!("It's a shore!")
        }
    }

    fn loaded(&self) -> bool {
        self.cargo.is_some()
    }

    fn unload(&mut self) -> char {
        let cargo = self.cargo.unwrap();
        self.cargo = None;
        self.destination = Destination::Port;
        cargo
    }

    fn empty(&self) -> bool {
        !self.loaded()
    }
}


fn deliver(mut to_deliver: Vec<char>) -> i8 {
    let to_deliver_count = to_deliver.len();
    let mut port_depot: Vec<char> = Vec::new();
    let mut port_terminal: Vec<char> = Vec::new();
    let mut delivered: Vec<char> = Vec::new();
    let mut available: Vec<&mut Truck> = Vec::new();
    let mut truck1 = Truck::new();
    let mut truck2 = Truck::new();
    let mut ship = Ship::new();
    &available.insert(0, &mut truck1);
    &available.insert(0, &mut truck2);
    let mut in_transit: Vec<&mut Truck> = Vec::new();
    let mut time = 0;
    let mut arrived: Vec<usize> = Vec::new();
    let mut counter = 1;
    println!("to be delivered {:#?}", to_deliver_count);
    while delivered.len() < to_deliver_count {
        println!("{}", delivered.len() < to_deliver_count);
        println!("########## Main loop: {counter} #############");
        print_state(&delivered, &in_transit, &available, &ship, &port_terminal);
        let mut transport = available.pop();
        while transport.is_some() {
            let cargo = to_deliver.pop();
            if cargo.is_some() {
                let truck = transport.expect("No truck!!!!");
                truck.load(cargo.unwrap());
                in_transit.push(truck);
                transport = available.pop();
            } else {
                available.insert(0, transport.unwrap());
                break;
            }
        }
        print_state(&delivered, &in_transit, &available, &ship, &port_terminal);
        println!("trucks loaded");
        for (index, vehicle) in in_transit.iter_mut().enumerate() {
            if vehicle.in_transit() {
                vehicle.advance();
            }
        }
        for (index, vehicle) in in_transit.iter_mut().enumerate() {
            if vehicle.arrived() && vehicle.loaded() {
                match vehicle.destination {
                    Destination::Port => port_depot.push(vehicle.unload()),
                    _ => delivered.push(vehicle.unload()),
                }
            }
            else if vehicle.arrived() && vehicle.empty() {
                arrived.push(index);
            }
        }
        println!("trucks unloaded");
        println!("trucks moved");
        print_state(&delivered, &in_transit, &available, &ship, &port_terminal);
        let mut arriving = arrived.pop();
        while arriving.is_some() {
            let vehicle_number = arriving.unwrap();
            let vehicle = in_transit.remove(vehicle_number);
            &available.insert(0, vehicle);
            arriving = arrived.pop();
        }
        println!("trucks arrived");
        print_state(&delivered, &in_transit, &available, &ship, &port_terminal);
        if ship.available() {
            let cargo = port_terminal.pop();
            if cargo.is_some() {
                ship.load(cargo.unwrap());
            }
        }
        println!("ships loaded");
        print_state(&delivered, &in_transit, &available, &ship, &port_terminal);
        if ship.in_transit() {
            ship.advance();
        }
        println!("ship moved");
        print_state(&delivered, &in_transit, &available, &ship, &port_terminal);
        if ship.arrived() && ship.loaded() {
            let cargo = ship.unload();
            delivered.push(cargo);
        }
        println!("ship unloaded");
        print_state(&delivered, &in_transit, &available, &ship, &port_terminal);
        let mut cargo_for_terminal = port_depot.pop();
        while cargo_for_terminal.is_some() {
            port_terminal.push(cargo_for_terminal.unwrap());
            cargo_for_terminal = port_depot.pop()
        }
        time += 1;
        counter += 1;
    }
    return time;
}

fn print_state(delivered: &Vec<char>, in_transit: &Vec<&mut Truck>, available: &Vec<&mut Truck>, ship: &Ship, port: &Vec<char>) {
    println!("delivered: {delivered:#?}");
    println!("delivered length : {:#?}", delivered.len());
    println!("in transit: {in_transit:#?}");
    println!("available: {available:#?}");
    println!("ship: {ship:#?}");
    println!("port: {port:#?}");
}

#[cfg(test)]
mod tests {
    use crate::deliver;
    #[test]
    fn counts_time_needed_to_deliver_one_cargo_to_warehouse_b() {
        let to_deliver = vec!['B'];
        let time = deliver(to_deliver);
        assert_eq!(time, 5);
    }

    #[test]
    fn counts_time_needed_to_deliver_one_cargo_to_island_warehause() {
        let to_deliver = vec!['A'];
        let time = deliver(to_deliver);
        assert_eq!(time, 5);
    }

    #[test]
    fn delivers_cargo_to_island_warehouse() {
        let to_deliver = vec!['A', 'A'];
        let time = deliver(to_deliver);
        assert_eq!(time, 13);
    }
}