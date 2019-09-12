extern crate csv;

use gtk::*;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;
use std::io::Write;

pub mod file_dialog;
pub mod deposits;

use crate::file_dialog::OpenDialog;
use crate::deposits::Deposits;

fn send_failure_popup() {
    MessageDialog::new(None::<&Window>,
                       DialogFlags::empty(),
                       MessageType::Info,
                       ButtonsType::Ok,
                       "Failed to open file!").run();
}

fn main() {
    match gtk::init() {
        Ok(_) => (),
        Err(_) => {
            send_failure_popup();
            return;
        }
    }
    let file_path = match get_file_path() {
        Some(path) => path,
        None => return
    };

    let path = file_path.as_path();

    let file = match get_file(&file_path) {
        Some(file) => file,
        None => return
    };

    let accounted = match read_file(file) {
        Some(d) => d,
        None => return
    };

    write_file (accounted, path);
}

fn get_file_path() -> Option<PathBuf> {
    match OpenDialog::new().run() {
        Some(file_path) => Some(file_path),
        None => {
            send_failure_popup();
            return None
        },
    }
}

fn get_file(file_path: &PathBuf) -> Option<File> {
    match File::open(file_path) {
        Ok(input_file) => Some(input_file),
        Err(_) => {
            send_failure_popup();
            return None
        },
    }
}

fn read_file(file: File) -> Option<Deposits> {
    let mut deposits = Deposits {
        checks: Vec::new(),
        output_strings: Vec::new(),
        prices: Vec::new(),
        count: 0
    };

    let mut reader = csv::Reader::from_reader(file);
    for result in reader.records() {
        let row = match result {
            Ok(t) => t,
            Err(_) => {
                send_failure_popup();
                return None;
            }
        };

        let date = match &row[0].split_whitespace().next() {
            Some(s) => String::from(*s),
            None => String::from("No Date"),
        };

        let mut unit = String::from(&row[5]);
        if unit == "" {
            let name = &row[1];

            // Remove all delimiters from the name
            let mut out_name = String::new();
            for c in name.chars() {
                if c != ',' && c != ';' && c != '.' {
                    out_name.push(c);
                } else {
                    out_name.push('/');
                }
            }

            unit = "No Unit Found for (".to_owned() + &out_name + ")";
        } else {
            unit = String::from("Unit: ") + &unit;
        }

        let check = row[6].to_string();
        let amount: String = row[7].to_string();

        deposits.push_tuple(check, date, unit, amount);
    }

    Some(deposits)
}

fn create_lines(deposits: &Deposits) -> (Vec<Vec<String>>, usize) {
    let mut lines: Vec<Vec<String>> = Vec::new();
    let mut max_count = 0;
    for i in 0..deposits.count {
        lines.push(Vec::new());
        let input = &mut String::from(",Check #,");
        input.push_str(&deposits.checks[i]);
        input.push_str(",,");
        lines[i].push(input.to_string());
        lines[i].push(String::from("Date, Unit, Amount,,"));

        let count = deposits.output_strings[i].len();

        if count > max_count {
            max_count = count;
        }

        for j in 0..count {
            let (date, unit, amount) = &deposits.output_strings[i][j];
            let input = String::from(date) + "," + unit + "," + amount + ",,";
            lines[i].push(input);
        }

        let mut input = String::from(",Total,");
        let mut amount = format!("${:.2}", deposits.get_sum(i));
        amount.push_str(",,");
        input.push_str(&amount);

        lines[i].push(input);
    };

    (lines, max_count)
}

fn write_file(deposits: Deposits, path: &Path) -> () {
    let directory = match path.parent() {
        Some(t) => t,
        None => {
            send_failure_popup();
            return;
        }
    };

    let file_name = match path.file_stem() {
        Some(t) => t,
        None => {
            send_failure_popup();
            return;
        }
    };

    let pwd = String::from(directory.as_os_str().to_str().unwrap());
    let file = &mut File::create(String::from(pwd + "/" + file_name.to_str().unwrap() +  "_results.csv")).unwrap();

    let deposits = create_lines(&deposits);
    let lines = deposits.0;

    // Add 3 here to account for the header and the footers
    let max = deposits.1 + 3;



    for j in 0..max {
        for i in 0..lines.len() {
            if j >= lines[i].len() {
                if let Err(_) = file.write(",,,,".as_bytes()) {
                    send_failure_popup();
                    return;
                }
            } else {
                if let Err(_) = file.write(lines[i][j].as_bytes()) {
                    send_failure_popup();
                    return;
                }
            }
        }
        if let Err(_) = file.write("\n".as_bytes()) {
            send_failure_popup();
            return;
        }
    }
}