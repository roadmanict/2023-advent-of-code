use std::fs;

const SYMBOLS: [char; 11] = ['/', '*', '#', '&', '+', '-', '@', '&', '$', '=', '%'];

#[test]
fn test_day_3_part_1() {
    let file = fs::read_to_string("resources/day_3.txt").expect("File should be available");

    let lines = file.lines().collect::<Vec<_>>(); //[0..2].to_owned();
    let total = sum_lines(&lines);

    assert_eq!(total, 100);
}

//#[test]
fn test_day_3_part_1_line_parsing() {
    let test_lines = vec![
        "...788.............................54.........501...........555.........270.................................521......893....................",
        "..../..*963........................*..860......................*....53...../.....................52.................&....347........428*522.",
        "............*......41..481+.......462....$..187......678.......420....-....................&115.+...........................+..............."
    ];

    let total = sum_lines(&test_lines);

    assert_eq!(total, 1137);
}

fn sum_lines(lines: &Vec<&str>) -> usize {
    let mut total: usize = 0;
    for (i, line) in lines.iter().enumerate() {
        if line.is_empty() {
            continue;
        }

        let mut index: usize = 0;

        for split in line.split(|c| SYMBOLS.contains(&c) || c == '.') {
            if split.is_empty() {
                index += 1;
                continue;
            }
            index += split.len();
            println!(
                "line {}, index {}, {}, len {}, {}",
                i + 1,
                index + 1,
                split,
                split.len(),
                find_adjacent_symbol(&lines, &i, index, split.len())
            );

            if find_adjacent_symbol(&lines, &i, index, split.len()) {
                println!("{}", split);
                total += split.parse::<usize>().expect("Number should be parsed");
                //println!("split: {}, column: {}", split, index);
            }

            println!();
            index += 1;
        }
    }

    total
}

fn find_adjacent_symbol(lines: &Vec<&str>, line: &usize, column: usize, len: usize) -> bool {
    let start = if line == &0 { 0 } else { line - 1 };
    let end = if line == &(lines.len() - 1) {
        lines.len()
    } else {
        line + 2
    };

    let subset = &lines[start..end];
    println!("start {}, end {}, lines {}", start, end, subset.len());

    for line in subset.iter() {
        let start = column - len;
        let start = if start == 0 { 0 } else { start - 1 };
        let end = column;
        let end = if end == line.len() {
            column
        } else {
            column + 1
        };
        println!("columns: {}, {}, {}", column, start, end);

        let col_subset = &line.chars().collect::<Vec<_>>()[start..end];
        println!("{:?}", col_subset);

        for char in col_subset {
            if SYMBOLS.contains(char) {
                // println!("Contains!!");
                return true;
            }
        }
    }

    return false;
}
