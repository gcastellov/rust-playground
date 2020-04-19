pub struct Point {
    pub x: i32,
    pub y: i32
}

pub fn some_mem_allocation() {
    let point: Point = create(15, 20);
    println!("This is point x:{}  y:{}", point.x, point.y);

    let point_at_heap = Box::new(point);
    let point_back = *point_at_heap;
    println!("This is point x:{}  y:{}", point_back.x, point_back.y);

    let some_string: String = String::from("wooooola");
    println!("This is: {}", some_string);

    let boxed_int = Box::new(10);
    println!("This is the address: {}", boxed_int);
}

fn create(x: i32, y: i32) -> Point {
    Point { x: x, y: y}
}