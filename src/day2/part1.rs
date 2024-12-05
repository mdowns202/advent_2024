use std::{fs, path::Path};

const REPORTS_FILE_PATH: &str = "src/day2/data/reports.txt";

#[derive(Debug)]
pub struct Report {
    pub sequence: Vec<u32>,
    pub length: u16,
    pub safety: ReportSafety,
}

impl Report {
    pub fn new(sequence: Vec<u32>) -> Self {
        let length = sequence.len().try_into().unwrap();
        let safety: ReportSafety;

        if let Some(ord) = SafetyOrder::get(sequence.clone()) {
            safety = ReportSafety::Safe(ord)
        } else {
            safety = ReportSafety::Unsafe
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
        let mut order: Option<SafetyOrder> = None;
        let mut prev_ord: Option<SafetyOrder> = None;
        let mut prev_num = &0;
        let seq_iter = part_seq.iter();

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
    let safe_reports: Vec<Report> = reports.into_iter().filter(|report| report.safe()).collect();
    println!("D2P1 | Safe Report Count => {}", safe_reports.len());
}

pub fn load_reports() -> Vec<Report> {
    let file_path = Path::new(REPORTS_FILE_PATH);
    let file = fs::read_to_string(file_path).expect("could not read file to string");
    let mut reports: Vec<Report> = Vec::new();

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
