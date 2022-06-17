
struct Waypoint {
    name: String,
    latitude: f64,
    longitude: f64
}

struct Segment{
    start: Waypoint,
    end: Waypoint
}

impl Segment{
    fn new(start: Waypoint, end: Waypoint) -> Self{
        Self{
            start,
            end
        }
    }

    fn distance(&self) -> String{
        String::from(format!("calculating distance between {} and {}", self.start.name, self.end.name))
    }
}

// traits is like a interface.


pub fn run(){

    let mut kcle = Waypoint{
        name: "KCLE".to_string(),
        latitude: 41.4075,
        longitude: -81.851111,
    };

    let mut kslc = Waypoint{
        name: "KSLC".to_string(),
        ..kcle
    };

    let kcle_kslc = Segment::new(kcle, kslc);
    println!("{}", kcle_kslc.distance());

}