use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Rule {
    before: u32,
    after: u32,
}

fn main() {
    // let file_path = "res/demo_input.txt";
    let file_path = "res/input.txt";

    let data: String =
        fs::read_to_string(file_path).expect(format!("failed to read file: {file_path}").as_str());
    println!("{}", data);

    let (rules, pages): (Vec<Rule>, Vec<Vec<u32>>) = parse_lines(data);
    println!("{:?}", rules);
    println!("{:?}", pages);

    let sum: u32 = process_data(&rules, &pages);
    println!("sum={}", sum)
}

fn process_data(rules: &Vec<Rule>, pages: &Vec<Vec<u32>>) -> u32 {
    let mut before_rules: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut after_rules: HashMap<u32, Vec<u32>> = HashMap::new();
    for rule in rules {
        before_rules
            .entry(rule.before)
            .or_insert(Vec::new())
            .push(rule.after);
        after_rules
            .entry(rule.after)
            .or_insert(Vec::new())
            .push(rule.before);
    }

    let mut sum = 0;
    for page in pages {
        let mut page_ok = true;
        let mut editable_page = page.clone();
        loop {
            let (new_page, page_changed) = fix_page(&before_rules, &after_rules, &editable_page);
            if page_changed {
                editable_page = new_page;
                page_ok = false;
                continue;
            }
            break;
        }

        if !page_ok {
            println!("old page = {:?}, new page = {:?}", page, editable_page);
            let middle = editable_page.len() / 2;
            sum += editable_page[middle];
        }
    }

    sum
}

fn fix_page(
    before_rules: &HashMap<u32, Vec<u32>>,
    after_rules: &HashMap<u32, Vec<u32>>,
    page: &Vec<u32>,
) -> (Vec<u32>, bool) {
    let mut editable_page = page.clone();
    let mut page_changed = false;
    for checked_i in 0..page.len() {
        let checked_number = page[checked_i];
        for (i, number) in page.iter().enumerate() {
            if i == checked_i {
                continue;
            }
            if i < checked_i {
                if let Some(after) = after_rules.get(number) {
                    if after.contains(&checked_number) {
                        page_changed = true;
                        editable_page[i] = checked_number;
                        editable_page[checked_i] = *number;
                        break;
                    }
                }
            }
            if checked_i < i {
                if let Some(before) = before_rules.get(number) {
                    if before.contains(&checked_number) {
                        page_changed = true;
                        editable_page[i] = checked_number;
                        editable_page[checked_i] = *number;
                        break;
                    }
                }
            }
        }
        if page_changed {
            break;
        }
    }
    (editable_page, page_changed)
}

fn parse_lines(data: String) -> (Vec<Rule>, Vec<Vec<u32>>) {
    let (rules, pages) = data.split_once("\n\n").unwrap();
    let rules: Vec<Rule> = rules
        .split('\n')
        .map(|rule| {
            let (left, right) = rule.split_once('|').unwrap();
            let before = left.parse::<u32>().unwrap();
            let after = right.parse::<u32>().unwrap();
            return Rule { before, after };
        })
        .collect();

    let pages: Vec<Vec<u32>> = pages
        .split('\n')
        .map(|line| {
            let digits = line
                .split(',')
                .map(|digit| digit.parse::<u32>().unwrap())
                .collect();
            return digits;
        })
        .collect();

    (rules, pages)
}
