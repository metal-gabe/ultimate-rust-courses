pub fn blur(infile: String, outfile: String) {
  // Here's how you open an existing image file
  let img = image::open(infile).expect("Failed to open INFILE.");
  // **OPTION**
  // Parse the blur amount (an f32) from the command-line and pass it through
  // to this function, instead of hard-coding it to 2.0.
  let img2 = img.blur(2.0);
  // Here's how you save an image to a file.
  img2.save(outfile).expect("Failed writing OUTFILE.");
}

pub fn brighten(infile: String, outfile: String) {
  // See blur() for an example of how to open / save an image.

  // .brighten() takes one argument, an i32.  Positive numbers brighten the
  // image. Negative numbers darken it.  It returns a new image.

  // Challenge: parse the brightness amount from the command-line and pass it
  // through to this function.
}

pub fn crop(infile: String, outfile: String) {
  // See blur() for an example of how to open an image.

  // .crop() takes four arguments: x: u32, y: u32, width: u32, height: u32
  // You may hard-code them, if you like.  It returns a new image.

  // Challenge: parse the four values from the command-line and pass them
  // through to this function.

  // See blur() for an example of how to save the image.
}

// This code was adapted from https://github.com/PistonDevelopers/image
pub fn fractal(outfile: String) {
  let width = 800;
  let height = 800;

  let mut imgbuf = image::ImageBuffer::new(width, height);

  let scale_x = 3.0 / width as f32;
  let scale_y = 3.0 / height as f32;

  // Iterate over the coordinates and pixels of the image
  for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
    // Use red and blue to be a pretty gradient background
    let red = (0.3 * x as f32) as u8;
    let blue = (0.3 * y as f32) as u8;

    // Use green as the fractal foreground (here is the fractal math part)
    let cx = y as f32 * scale_x - 1.5;
    let cy = x as f32 * scale_y - 1.5;

    let c = num_complex::Complex::new(-0.4, 0.6);
    let mut z = num_complex::Complex::new(cx, cy);

    let mut green = 0;
    while green < 255 && z.norm() <= 2.0 {
      z = z * z + c;
      green += 1;
    }

    // Actually set the pixel. red, green, and blue are u8 values!
    *pixel = image::Rgb([red, green, blue]);
  }

  imgbuf.save(outfile).unwrap();
}

pub fn generate(outfile: String) {
  // Create an ImageBuffer -- see fractal() for an example

  // Iterate over the coordinates and pixels of the image -- see fractal() for an example

  // Set the image to some solid color. -- see fractal() for an example

  // Challenge: parse some color data from the command-line, pass it through
  // to this function to use for the solid color.

  // Challenge 2: Generate something more interesting!

  // See blur() for an example of how to save the image
}

pub fn grayscale(infile: String, outfile: String) {
  // See blur() for an example of how to open an image.

  // .grayscale() takes no arguments. It returns a new image.

  // See blur() for an example of how to save the image.
}

pub fn invert(infile: String, outfile: String) {
  // See blur() for an example of how to open an image.

  // .invert() takes no arguments and converts the image in-place, so you
  // will use the same image to save out to a different file.

  // See blur() for an example of how to save the image.
}

pub fn print_usage_and_exit() {
  println!("USAGE (when in doubt, use a .png extension on your filenames)");
  println!("blur INFILE OUTFILE");
  println!("fractal OUTFILE");
  // **OPTION**
  // Print useful information about what subcommands and arguments you can use
  // println!("...");
  std::process::exit(-1);
}

pub fn rotate(infile: String, outfile: String) {
  // See blur() for an example of how to open an image.

  // There are 3 rotate functions to choose from (all clockwise):
  //   .rotate90()
  //   .rotate180()
  //   .rotate270()
  // All three methods return a new image.  Pick one and use it!

  // Challenge: parse the rotation amount from the command-line, pass it
  // through to this function to select which method to call.

  // See blur() for an example of how to save the image.
}

// **SUPER CHALLENGE FOR LATER** - Let's face it, you don't have time for this during class.
//
// Make all the subcommands stackable!
//
// For example, if you run:
//
//   cargo run infile.png outfile.png blur 2.5 invert rotate 180 brighten 10
//
// ...then your program would:
// - read infile.png
// - apply a blur of 2.5
// - invert the colors
// - rotate the image 180 degrees clockwise
// - brighten the image by 10
// - and write the result to outfile.png
//
// Good luck!
