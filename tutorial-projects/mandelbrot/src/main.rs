use num::Complex;

///
/// The Mandelbrot set is defined as the set of complex numbers c for which z does not fly out to infinity.
/// 

fn main() {
    println!("Hello, world!");
}

fn complex_square_add_loop(c: Complex<f64>) {
    let mut z = Complex{ re: 0.0, im: 0.0 };
    loop {
        z = z * z + c;
    }
}

/// Try to determine if 'c' is in the Mandelbrot set, using at most 'limit' iterations to decide.
/// 
/// If 'c' is not a member, return 'Some(i)' where 'i' is the number of iterations it took for 
/// 'c' to leave the circle of radius 2 centered on the origin. If 'c' seems to be a member
/// (more precisely, if we reached the iteration limit without being able to prove that 'c' is not a member),
/// return 'None'.
fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        // Find the square of z's disatance from the origin. To decide wheter z has left the circle of 
        // radius 2, instead of computing a square root, we just compare the squared distance with 4.0 which is faster.
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c
    }
    None
}