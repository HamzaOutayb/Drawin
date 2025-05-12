fn main() {
    // Create a blank image of size 1000x1000
    let mut image = Image::blank(1000, 1000);

    // Modify individual pixels by setting their colors
    for x in 0..1000 {
        for y in 0..1000 {
            if x%2 == 0 && y%2 == 0 {
                image.set_pixel(x, y, Color::rgb(0, 0, 255)); // Blue color
            } else {
                image.set_pixel(x, y, Color::rgb(255, 81, 0)); // Blue color
            }
            }
        }

    // Save the image to a file
    raster::save(&image, "modified_image.png").unwrap();
}