mod puzzle_1;

fn main() {
    println!("Puzzle 1.1: There are {} instances of increased depth.", 
             puzzle_1::count_increasing_depth());
    println!("Puzzle 1.2: There are {} instances of increased depth using a sliding window of size 3.", 
             puzzle_1::count_sliding_window_increasing_depth());
}
