// 1. Thank you for volunteering to document our pumpkin library! Let's start by grabbing the first
// paragraph from https://en.wikipedia.org/wiki/Pumpkin and pasting it as our module-level
// documentation. Hint: Use inner-documentation comments.
//
// Once you've got the documentation here, run `cargo doc --no-deps --open` and take a look!

// 2. What about an image!? Add an image of a pumpkin to the end of the module-level documentation.
// The markdown format is ![some alt text](https://url-to-the-image.png)
// Here's the image to link to: https://upload.wikimedia.org/wikipedia/commons/thumb/5/5c/FrenchMarketPumpkinsB.jpg/700px-FrenchMarketPumpkinsB.jpg

// 3. Document the Pumpkin struct.
// - The description on the index page should be "Big orange thing"
// - Make a section header called "Recipes"
// - Explain that recipes will be coming soon.
// - Document the "roundness" field, explaining that it is a percentage
// - Document the "orangeness" field, explaining that it is a number from 8 to 27

/*!
A pumpkin is a cultivated winter squash in the genus Cucurbita.
The term is most commonly applied to round, orange-colored squash varieties,
but does not possess a scientific definition.
It may be used in reference to many different squashes of varied appearance
and belonging to multiple species in the Cucurbita genus.

![A Pumpkin, oui, oui](https://upload.wikimedia.org/wikipedia/commons/thumb/5/5c/FrenchMarketPumpkinsB.jpg/700px-FrenchMarketPumpkinsB.jpg)

!*/

/// A pumpkin is a winter squash with large orange seeds.
///
/// # Recipes
/// Coming soon...
pub struct Pumpkin {
   /// The range, from 8 to 27, of how orange a pumpkin is.
   pub orangeness: i32,
   /// The percentage of how round a pumpkin is.
   pub roundness: f32,
}

// 4. Document the "smash" method. Explain that if you smash the pumpkin, it will be gone. Then it
// can't be used for pie. :'-(

impl Pumpkin {
   /// If you smash a pumpkin, it will be gone.
   /// Then it can't be used for pie. 😭
   pub fn smash(self) {}
}

// 5. Document that BURNT_ORANGE is for the "orangeness" field in the Pumpkin struct.
// - Link to the Pumpkin struct in your description
/// This is used for the "orangeness" field in the Pumpkin struct
/// Clickable link: [`Pumpkin`]
pub const BURNT_ORANGE: i32 = 13;

// Challenge: Find the option to pass to `cargo doc` so that documentation for this private item
// gets generated as well.  Hint: `cargo doc -h` will show you all the relevant options.

/// For internal use only. In fact, this documentation is so private that it won't be generated.
/// At least not by default. But if you pass the correct option in, it will magically appear!
#[allow(dead_code)] // to silence the warning
enum PrivateEnum {
   /// For Halloween. To be lit by candlelight.
   JackOLantern,
   /// For dessert during North American winter holidays.
   PumpkinPie,
}
