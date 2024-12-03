mod day1;
mod day2;
mod day3;

fn main() {
    println!("\n==== DAY 01 ====");
    day1::part1::calc_total_difference();
    day1::part2::calc_similarity_score();
    println!("\n==== DAY 02 ====");
    day2::part1::sum_safe_reports();
    day2::part2::run_problem_dampener();
    println!("\n==== DAY 03 ====");
    day3::part1::print_instruction_results();
    day3::part2::do_part_2();
}
