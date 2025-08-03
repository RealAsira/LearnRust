// create a "Rectangle" structure
#[derive(Debug)]
struct Rectangle {
    width:u32,
    height:u32,
}


fn main() {
    // new rectangle
    let rectangle = Rectangle{width:5, height:6};
    println!("rect: {rectangle:?}");

    // pass rectangle to calc_area function
    // note: BORROWED
    println!("Rectangle area: {}^units",{calc_area(&rectangle)});
}


// accepts Rectangle type, returns u32 representing area
fn calc_area(rectangle:&Rectangle)->u32 {
    return rectangle.width * rectangle.height
}
