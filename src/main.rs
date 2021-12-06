mod puzzle_1;
mod puzzle_2;
mod puzzle_3;
mod puzzle_4;

fn main() {
    println!("Puzzle 1.1: There are {} instances of increased depth.", 
             puzzle_1::count_increasing_depth());
    println!("Puzzle 1.2: There are {} instances of increased depth using a sliding window of size 3.", 
             puzzle_1::count_sliding_window_increasing_depth());
    println!("Puzzle 2.1: The final posision metric of the sub (horizontal x depth) is {}.",
             puzzle_2::compute_position());
    println!("Puzzle 2.2: The final posision metric of the sub (horizontal x depth) using the aimed algorthim is {}.",
             puzzle_2::compute_position_aimed());
    println!("Puzzle 3.1: The power consumption (gamma x epsilon) is {}",
             puzzle_3::calculate_power_consumption());
    println!("Puzzle 3.2: The life rating (oxygen x co2 rating) is {}",
             puzzle_3::calculate_life_rating());
    println!("Puzzle 4.1: The winning bingo board has the score {}",
             puzzle_4::winning_bingo_score());
}
