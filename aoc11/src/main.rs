use std::fs;

fn main() {
    const SERIAL_NUMBER: i32 = 1788;
    const GRID_SIZE: i32 = 300;

    fn calc_power(x: i32, y: i32) -> i32 {
        let rack_id = x + 10;
        let mut power = rack_id * y;
        power += SERIAL_NUMBER;
        power = power * rack_id % 1000 / 100;
        power -= 5;
        power
    }

    let mut best_sum = i32::MIN;
    let mut best_x = 0;
    let mut best_y = 0;
    let mut best_size = 0;
    for x in 1..=GRID_SIZE {
        for y in 1..=GRID_SIZE {

            let mut sum_power = 0;

            for i in 0..GRID_SIZE {

                if x + i > GRID_SIZE || y + i > GRID_SIZE {
                    break;
                }

                sum_power += calc_power(x + i, y + i);

                for k in x..(x+i) {
                    sum_power += calc_power(k, y + i);
                }

                for k in y..(y+i) {
                    sum_power += calc_power(x + i, k);
                }


                if sum_power > best_sum {
                    best_sum = sum_power;
                    best_x = x;
                    best_y = y;
                    best_size = i + 1;
                }
            }
        }
    }

    println!("{best_x},{best_y},{best_size}");

}
