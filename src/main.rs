mod lib;

use num::Complex;

fn main() {
    for num in construct_numbers(10) {
        println!("{num}");
    }
}

fn construct_numbers(n: usize) -> Vec<Complex<f32>> {

    // Initate the current coordinate and the fib numbers
    let fib_nums = fib(n);
    let mut x = 0;
    let mut y = 0;

    let mut nums = Vec::with_capacity(n);

    // The sign of the direction for the next coordinate
    let mut directon = Direction::TOPRIGHT;

    for i in 0..n {

        let (sign_x, sign_y) = directon.get_multiplier();

        x += fib_nums[i] as i32 * sign_x;
        y += fib_nums[i]  as i32 * sign_y;

        nums.push(Complex::new(x as f32, y as f32));

        directon = match directon {
            Direction::TOPRIGHT => Direction::TOPLEFT,
            Direction::TOPLEFT => Direction::DOWNLEFT,
            Direction::DOWNRIGHT => Direction::TOPRIGHT,
            Direction::DOWNLEFT => Direction::DOWNRIGHT,
        }

    }

    nums
}

enum Direction {
    TOPRIGHT,
    TOPLEFT,
    DOWNRIGHT,
    DOWNLEFT
}

impl Direction {
    fn get_multiplier(&self) -> (i32, i32) {


        match *self {
            Direction::TOPRIGHT => (1, 1),
            Direction::TOPLEFT => (-1, 1),
            Direction::DOWNRIGHT => (1, -1),
            Direction::DOWNLEFT => (-1, -1),
        }
    }

}



fn fib(n: usize) -> Vec<usize> {
    let mut vec = vec![0; n];

    vec[0] = 1;
    vec[1] = 1;

    for i in 2..n {
        vec[i] = vec[i-1] + vec[i - 2]
    }

    vec
}