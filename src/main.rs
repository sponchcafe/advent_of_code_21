mod puzzle_1;
mod puzzle_2;
mod puzzle_3; 
mod puzzle_4;
mod puzzle_5;
mod puzzle_6;
mod puzzle_7;
mod puzzle_8;
mod puzzle_9;
mod puzzle_10;
mod puzzle_11;
mod puzzle_12;
mod puzzle_13;
mod puzzle_14;

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
    println!("Puzzle 4.2: The loosing bingo board has the score {}",
             puzzle_4::loosing_bingo_score());
    println!("Puzzle 5.1: The number of points at which straight lines overlap is {}",
             puzzle_5::overlapping_straight_line_count());
    println!("Puzzle 5.2: The number of points at which straight and diagonal lines overlap is {}",
             puzzle_5::overlapping_diagonal_line_count());
    println!("Puzzle 6.1: After 80 days the number of lanternfish is {}",
             puzzle_6::lanternfish_population(80));
    println!("Puzzle 6.2: After 256 days the number of lanternfish is {}",
             puzzle_6::lanternfish_population(256));
    println!("Puzzle 7.1: The fuel cost to align the crabs is {}",
             puzzle_7::align_crabs_cost());
    println!("Puzzle 7.2: The fuel cost to align the crabs with linearly increasing fuel cost is {}",
             puzzle_7::align_crabs_cost_linear());
    println!("Puzzle 8.1: The number of digitis that use unique segments (1,4,7,8) is {}",
             puzzle_8::count_digits_1478());
    println!("Puzzle 8.2: The sum of all decoded seven segment digits is {}",
             puzzle_8::sum_all_decoded_outputs());
    println!("Puzzle 9.1: The sum of all risk levels is {}",
             puzzle_9::sum_risk_levels());
    println!("Puzzle 9.2: The basin risk level is {}",
             puzzle_9::basin_risk_level());
    println!("Puzzle 10.1: The syntax error score is {}",
             puzzle_10::syntax_error_score());
    println!("Puzzle 10.2: The middle complettion score is {}",
             puzzle_10::completion_score());
    println!("Puzzle 11.1: After 100 steps, the number of octupus flashes is {}",
             puzzle_11::octopus_flashes());
    println!("Puzzle 11.2: All octupus flashes are in sync after {} steps.",
             puzzle_11::octopus_sync_flashes());
    println!("Puzzle 12.1: The number of paths from start to end is {}", 
             puzzle_12::number_of_paths());
    println!("Puzzle 12.2: The number of paths from start to end with one double visit is {}", 
             puzzle_12::number_of_paths_double_visit());
    println!("Puzzle 13.1: After one fold the number of visible dots is {}",
             puzzle_13::dots_after_one_fold());
    println!("Puzzle 13.2: The infrared system activation code is:\n{}",
             puzzle_13::fold_and_format_dots());
    println!("Puzzle 14.1: The difference between the number of the most and least common polymer element after 10 steps is {}",
             puzzle_14::polymer_index(10));
    println!("Puzzle 14.2: The difference between the number of the most and least common polymer element after 40 steps is {}",
             puzzle_14::polymer_index(40));

}
