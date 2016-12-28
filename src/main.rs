use std::f64;

mod dotmatrix;

fn main() {
    let mut matrix = dotmatrix::DotMatrix::new(200, 101);

    for x in 0..200 {
        let y = ((-1.0*(x as f64)/10.0).sin() * 40.0 + 50.0) as usize;
        matrix.set(x, y, true);
        matrix.set(x, 50, true);
    }
    for y in 0..100 {
        matrix.set(0, y, true);
    }
    matrix.print();
}
