use std::fs;

const REPORTS_FILE_PATH: &str = "./src/day2/reports.txt";

#[derive(Debug)]
pub struct Report {
    pub sequence: Vec<u32>,
    #[allow(dead_code)]
    length: u16,
    pub safety: ReportSafety,
}

impl Report {
    pub fn new(sequence: Vec<u32>) -> Self {
        let length = sequence.len().try_into().unwrap();
        let safety: ReportSafety;

        match SafetyOrder::get(sequence.clone()) {
            Some(ord) => safety = ReportSafety::Safe(ord),
            None => safety = ReportSafety::Unsafe,
        }
        Self {
            sequence,
            length,
            safety,
        }
    }

    pub fn safe(&self) -> bool {
        match self.safety {
            ReportSafety::Safe(SafetyOrder::Ascending) => true,
            ReportSafety::Safe(SafetyOrder::Descending) => true,
            ReportSafety::Unsafe => false,
        }
    }
}

#[derive(Debug)]
pub enum ReportSafety {
    Safe(SafetyOrder),
    Unsafe,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SafetyOrder {
    Ascending,
    Descending,
}

impl SafetyOrder {
    pub fn get(part_seq: Vec<u32>) -> Option<Self> {
        let seq_iter = part_seq.iter();
        let mut order: Option<SafetyOrder> = None;
        let mut prev_ord: Option<SafetyOrder> = None;
        let mut prev_num = &0;

        for (i, num) in seq_iter.enumerate() {
            // Can only determine asc/desc on 2nd loop
            if i > 0 {
                let adj_diff = num.abs_diff(*prev_num);
                if (1 > adj_diff) || (adj_diff > 3) {
                    order = None;
                    break;
                }
                if num > &prev_num {
                    order = Some(SafetyOrder::Ascending);
                } else if num < &prev_num {
                    order = Some(SafetyOrder::Descending);
                } else {
                    order = None;
                    break;
                }
            }

            // Can only determine if order is maintained on 3rd loop
            if i > 1 && order != prev_ord {
                order = None;
                break;
            }
            prev_num = num;
            prev_ord = order.clone();
        }
        order
    }
}

pub fn sum_safe_reports() {
    let reports = load_reports();
    let mut safe_reports: Vec<Report> = Vec::new();

    for report in reports {
        if report.safe() {
            safe_reports.push(report);
        }
    }
    // let test_report = Report::new(vec![11, 9, 6, 7, 5, 4]);
    // println!("{:?}", test_report);

    let safe_count = safe_reports.len();
    println!("D2P1 | Safe Report Count => {}", safe_count);
}

pub fn load_reports() -> Vec<Report> {
    let mut reports: Vec<Report> = Vec::new();
    let file = fs::read_to_string(REPORTS_FILE_PATH).expect("could not read file to string");
    for line in file.lines() {
        let mut sequence: Vec<u32> = Vec::new();
        line.split(' ')
            .collect::<Vec<&str>>()
            .iter()
            .for_each(|num_str| {
                let seq_item: u32 = num_str.parse().unwrap();
                sequence.push(seq_item);
            });

        let new_report = Report::new(sequence);
        reports.push(new_report);
    }

    // reports.iter().for_each(|report| println!("{:?}", report));
    reports
}
