use csv;

const LOC_ID_FILE_PATH: &str = "./src/day1/location_ids.csv";

#[derive(Debug)]
pub struct SortedRecord {
    list_a_item: u32,
    list_b_item: u32,
}

impl SortedRecord {
    pub fn new(list_a_item: u32, list_b_item: u32) -> Self {
        Self {
            list_a_item,
            list_b_item,
        }
    }
}

pub fn calc_total_difference() {
    let mut sorted_records: Vec<SortedRecord> = Vec::new();
    let mut total_difference: u32 = 0;

    let (mut list_a, mut list_b) = load_location_ids();

    list_a.sort();
    list_b.sort();

    assert_eq!(list_a.len(), list_b.len());
    let length = list_a.len();

    for i in 0..length {
        let new_record = SortedRecord::new(list_a[i], list_b[i]);
        sorted_records.push(new_record);
    }

    for record in sorted_records {
        let difference: u32 = record.list_a_item.abs_diff(record.list_b_item);
        total_difference += difference;
    }
    println!("D1P1 | Total List Difference => {}", total_difference);
}

pub fn load_location_ids() -> (Vec<u32>, Vec<u32>) {
    let reader = csv::Reader::from_path(LOC_ID_FILE_PATH).expect("file not found");
    let records = reader.into_records();

    let mut list_a: Vec<u32> = Vec::new();
    let mut list_b: Vec<u32> = Vec::new();

    for result in records {
        match result {
            Ok(rec) => {
                let list_a_item = rec.get(0).unwrap();
                let list_b_item = rec.get(1).unwrap();

                list_a.push(list_a_item.parse().unwrap());
                list_b.push(list_b_item.parse().unwrap());
            }
            Err(e) => println!("{}", e),
        }
    }
    return (list_a, list_b);
}
