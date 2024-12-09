// A coordinate of where an Arrow hit
#[derive(Debug)]
pub struct Coord {
    x: f64,
    y: f64,
}

impl Coord {
    pub fn distance_from_center(&self) -> f64 {
        (self.x.powf(2.0) + self.y.powf(2.0)).sqrt()
    }

    pub fn print_description(&self) {
        println!(
            "coord is {:.1} away, at ({:.1}, {:.1})",
            self.distance_from_center(),
            self.x,
            self.y
        );
    }
}

// Generate some random coordinates
pub fn get_arrow_coords(num: u32) -> Vec<Coord> {
    let mut coords: Vec<Coord> = Vec::new();

    for _ in 0..num {
        let coord = Coord {
            x: (rand::random::<f64>() - 0.5) * 12.0,
            y: (rand::random::<f64>() - 0.5) * 12.0,
        };
        coords.push(coord);
    }

    coords
}
