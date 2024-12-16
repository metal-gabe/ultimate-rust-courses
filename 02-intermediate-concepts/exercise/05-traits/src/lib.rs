#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Cake {
   Chocolate,
   MapleBacon,
   Spice,
}

impl From<&Party> for Cake {
   fn from(party: &Party) -> Self {
      party.cake.clone()
   }
}

// deriving `PartialEq` here for `Party` means that all properties must be the same
// using the manually implemented `PartialEq` below means we can customize & specify the comparison
#[derive(Debug)] // #[derive(Debug, PartialEq)]
pub struct Party {
   pub at_restaurant: bool,
   pub num_people: u8,
   pub cake: Cake,
}

impl Default for Party {
   fn default() -> Self {
      Party {
         at_restaurant: true,
         cake: Cake::Chocolate,
         num_people: 8,
      }
   }
}

impl PartialEq for Party {
   fn eq(
      &self,
      other: &Self,
   ) -> bool {
      // because `cake` is a single field/enum, we can derive `PartialEq` for it above
      self.cake == other.cake
   }
}

pub fn admire_cake(cake: Cake) {
   println!("What a nice {:#?} cake! 🎂", cake);
}

pub fn smell_cake<T: Into<Cake>>(something: T) {
   println!("Hmm...something smells like a {:?} cake!", something.into());
}
