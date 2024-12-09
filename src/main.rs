mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day7;
mod day8;

fn main() {
    println!("\n==== DAY 01 ====");
    day1::part1::calc_total_difference();
    day1::part2::calc_similarity_score();
    println!("\n==== DAY 02 ====");
    day2::part1::sum_safe_reports();
    day2::part2::run_problem_dampener();
    println!("\n==== DAY 03 ====");
    day3::part1::print_reg_instruction_prod_sum();
    day3::part2::print_cond_instruction_prod_sum();
    println!("\n==== DAY 04 ====");
    day4::part1::solve_xmas_word_search();
    day4::part2::solve_x_mas_word_search();
    println!("\n==== DAY 05 ====");
    day5::part1::print_median_safety_sum();
    println!("\n==== DAY 07 ====");
    day7::part1::print_calibration_total();
    println!("\n==== DAY 08 ====");
    day8::part1::print_antinode_total();
    day8::part2::print_inline_antinode_total();
}
