mod puzzle_1;
mod puzzle_2;

fn main() {
    println!("Puzzle 1.1: There are {} instances of increased depth.", 
             puzzle_1::count_increasing_depth());
    println!("Puzzle 1.2: There are {} instances of increased depth using a sliding window of size 3.", 
             puzzle_1::count_sliding_window_increasing_depth());
    println!("Puzzle 2.1: The final posision metric of the sub (horizontal x depth) is {}.",
             puzzle_2::compute_position());
}
