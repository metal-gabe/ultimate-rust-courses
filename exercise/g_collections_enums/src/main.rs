// Silence some warnings that could distract from the exercise
#![allow(unused_variables, unused_mut, dead_code)]

use g_collections_enums::{get_arrow_coords, Coord};

// Someone is shooting arrows at a target.  We need to classify the shots.
//
// 1a. Create an enum called `Shot` with variants:
// - `Bullseye`
// - `Hit`, containing the distance from the center (a f64)
// - `Miss`
//
// You will need to complete 1b as well before you will be able to run this program successfully.
enum Shot {
    Bullseye,
    Hit(f64),
    Miss,
}

impl Shot {
    // Here is a method for the `Shot` enum you just defined.
    fn points(self) -> i32 {
        // 1b. Implement this method to convert a Shot into points
        // - return 5 points if `self` is a `Shot::Bullseye`
        // - return 2 points if `self` is a `Shot::Hit(x)` where x < 3.0
        // - return 1 point if `self` is a `Shot::Hit(x)` where x >= 3.0
        // - return 0 points if `self` is a Miss
        match self {
            Shot::Bullseye => 5,
            Shot::Hit(x) if x < 3.0 => 2,
            Shot::Hit(x) => 1,
            Shot::Miss => 0,
        }
    }
}

fn main() {
    // Simulate shooting a bunch of arrows and gathering their coordinates on the target.
    let arrow_coords: Vec<Coord> = get_arrow_coords(5);
    let mut shots: Vec<Shot> = Vec::new();

    // 2. For each coord in arrow_coords:
    //
    //   A. Call `coord.print_description()`
    //   B. Append the correct variant of `Shot` to the `shots` vector depending on the value of
    //   `coord.distance_from_center()`
    //      - Less than 1.0 -- `Shot::Bullseye`
    //      - Between 1.0 and 5.0 -- `Shot::Hit(value)`
    //      - Greater than 5.0 -- `Shot::Miss`
    for coord in arrow_coords {
        coord.print_description();

        let shot = match coord.distance_from_center() {
            x if x < 1.0 => Shot::Bullseye,
            x if x < 5.0 => Shot::Hit(x),
            _ => Shot::Miss,
        };

        shots.push(shot);
    }

    let mut total = 0;
    // 3. Finally, loop through each shot in shots and add its points to total
    for shot in shots {
        let points: i32 = shot.points();
        total += points;
        println!("Shot points: {}", points);
    }

    println!("Final point total is: {total}");
}
