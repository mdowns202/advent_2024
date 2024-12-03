use super::part1::load_location_ids;

pub fn calc_similarity_score() {
    let mut sim_score: u32 = 0;
    let location_lists = load_location_ids();
    let (list_a, list_b) = (location_lists.0, location_lists.1);

    for item_a in list_a {
        let mut item_count: u32 = 0;

        for j in 0..list_b.len() {
            if list_b[j] == item_a {
                item_count += item_a;
            }
        }
        if item_count > 0 {
            sim_score += item_count;
        }
    }
    println!("D1P2 | Similarity Score => {}", sim_score);
}
