use super::part1::{load_reports, Report, ReportSafety, SafetyOrder};

#[derive(Debug)]
struct ProblemDampener {
    reports: Vec<Report>,
}

impl ProblemDampener {
    fn new(reports: Vec<Report>) -> Self {
        Self { reports }
    }
    fn run(mut self) -> Vec<Report> {
        for report in &mut self.reports {
            let sequence = &report.sequence;
            let mut vecs: Vec<Vec<u32>> = Vec::new();

            for i in 0..sequence.len() {
                let mut new_vec = sequence.clone();
                new_vec.remove(i);
                vecs.push(new_vec);
            }

            for part_seq in vecs.iter() {
                let order = SafetyOrder::get(part_seq.to_vec());
                match order {
                    Some(ord) => {
                        report.safety = ReportSafety::Safe(ord);
                        break;
                    }
                    None => report.safety = ReportSafety::Unsafe,
                };
            }
        }
        self.reports
    }
}

pub fn run_problem_dampener() {
    let reports = load_reports();
    let dampener = ProblemDampener::new(reports);
    let mut safe_reports = Vec::new();

    let damp_reports = dampener.run();
    damp_reports.iter().for_each(|report| {
        if report.safe() {
            safe_reports.push(report);
        }
    });
    println!("D2P2 | Dampened Safe Count => {}", safe_reports.len())
}
