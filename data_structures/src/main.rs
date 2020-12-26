struct Point {
    x: f64,
    y: f64
}

struct Line {
    start: Point,
    end: Point
}

fn structures() {
    let p1 = Point {x: 1.0, y: 2.0};
    let p2 = Point {x: 4.0, y: 5.0};

    let my_line = Line {start: p1, end: p2};

    println!("my line with start: x={} y={}, end: x={}, y={}",
        my_line.start.x, my_line.start.y, my_line.end.x, my_line.end.y);
}

fn main() {
    structures();
}
