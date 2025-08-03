// create a "Rectangle" structure
#[derive(Debug)]    // allows rectangle to be printed via {:?}
struct Rectangle {
    width:u32,
    height:u32,
}

// adds functionality (methods and associated functions) to an implementation
impl Rectangle {
    // accepts Rectangle type, returns u32 representing area
    fn area(&self)->u32 {
        if !self.has_height() {println!("Cannot calculate area because there is no height!"); return 0;}
        if !self.has_width() {println!("Cannot calculate area because there is no width!"); return 0;}

        if self.has_width() && self.has_height() {
            return self.width * self.height
        } else {0}
    }

    fn has_width(&self)->bool {
        return self.width>0
    }

    fn has_height(&self)->bool {
        return self.height>0
    }

    // is this rectangle large enough to hold another?
    fn can_hold(&self, other:&Rectangle)->bool {
        let mut result:bool;
        result = if self.width>=other.width&&self.height>=other.height {true} else {false};
        // couldn't hold ... try rotating
        if !result {
            result = if self.width>=other.height&&self.height>=other.width {true} else {false};
        }
        return result
    }

    fn is_square(&self)->bool {
        return self.width == self.height
    }
}

// every struct can have 0 or more implemenations
// e.g. I've chosen to split associated functions apart from methods
impl Rectangle {
    // associated-functions (like String::from) ... don't use &self but return Self
    // this makes a new rectangle that is a square specifically
    fn new_square(size:u32)->Self {
        return Self {width:size, height:size}
    }
}





fn main() {
    // new rectangle
    let rectangle = Rectangle{width:10, height:5};
    let rectangle2 = Rectangle{width:4, height:9};
    println!("rect1: {rectangle:?}");
    println!("rect2: {rectangle2:?}");

    // pass rectangle to calc_area function
    // note: BORROWED
    println!("Rectangle1 area: {}^units",{rectangle.area()});
    println!("Rectangle2 area: {}^units",{rectangle2.area()});
    println!("Rectangle1 holds Rectangle2? {}",{rectangle.can_hold(&rectangle2)});

    let square1 = Rectangle::new_square(5);
    println!("square: {square1:?}");
    println!("Are the following squares?");
    print!("Rect1: {}\nRect2: {}\nSquare1: {}", {rectangle.is_square()}, {rectangle2.is_square()}, {square1.is_square()});
}
