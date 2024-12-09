mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;

fn main() {
    println!("\n==== DAY 01 ====");
    day01::part1::calc_total_difference();
    day01::part2::calc_similarity_score();
    println!("\n==== DAY 02 ====");
    day02::part1::sum_safe_reports();
    day02::part2::run_problem_dampener();
    println!("\n==== DAY 03 ====");
    day03::part1::print_reg_instruction_prod_sum();
    day03::part2::print_cond_instruction_prod_sum();
    println!("\n==== DAY 04 ====");
    day04::part1::solve_xmas_word_search();
    day04::part2::solve_x_mas_word_search();
    println!("\n==== DAY 05 ====");
    day05::part1::print_median_safety_sum();
    println!("\n==== DAY 06 ====");
    //
    println!("\n==== DAY 07 ====");
    day07::part1::print_calibration_total();
    println!("\n==== DAY 08 ====");
    day08::part1::print_antinode_total();
    day08::part2::print_inline_antinode_total();
    println!("\n==== DAY 09 ====");
    day09::part1::print_filesystem_checksum_value();
    println!("\n==== DAY 10 ====");
    day10::part1::print_total_trailhead_score();
    day10::part2::print_trail_area_rating();
    println!("\n==== DAY 11 ====");
    day11::part1::print_little_stone_magnitude();
    day11::part2::print_big_stone_magnitude();
}
