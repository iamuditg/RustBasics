impl Shuttle {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn new(name: &str) -> Shuttle {
        Shuttle{
            name: String::from(name),
            crew_size: 7,
            propellant:0.0
        }
    }
}

struct Color(u8,u8,u8);

#[derive(Debug)]
struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64
}

fn structs() {
    let mut vehicle = Shuttle{
        name: String::from("Endeavour"),
        crew_size: 7,
        propellant: 856564.0
    };

    let vehicle2 = Shuttle{
        name: String::from("discover"),
        ..vehicle
    };

    let vehicle_name = vehicle.get_name();
    println!("name is {}", vehicle_name);
    println!("name is {}", vehicle.name);
    vehicle.name = String::from("Atlantis");
    println!("vehicle is {:?}",vehicle);
    println!("vehicle is {:?}",vehicle2);

    //let mut veh = Shuttle::new("hdhd");
    //print!("veh is {}",veh);

    let red = Color(22,0,0);
    print!("first value is {}",red.0);
}