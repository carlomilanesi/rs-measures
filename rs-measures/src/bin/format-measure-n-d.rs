fn main() {
    let mut args = std::env::args();
    args.next();
    if let Some(option) = args.next() {
        match option.as_str() {
            "comment" => comment_lines(),
            "uncomment" => uncomment_lines(),
            _ => panic!("Invalid option: comment, uncomment"),
        }
    } else {
        panic!("Missing option: comment, uncomment");
    }
}

fn comment_lines() {
    let mut n = 0;
    let mut line1 = String::new();
    let mut line2 = String::new();
    for line in std::io::stdin().lines() {
        if let Ok(line) = line {
            n += 1;
            match n {
                1 => {
                    line1 = format!("//{}", line);
                }
                2 => {
                    line2 = format!("//{}", line);
                }
                3 => {
                    println!("{}", line1);
                    line1 = line2;
                    line2 = format!("//{}", line);
                }
                _ => {
                    println!("{}", line1);
                    line1 = line2;
                    line2 = line;
                }
            }
        } else {
            break;
        }
    }
    println!("//{}", line1);
    println!("//{}", line2);
}

fn uncomment_lines() {
    let mut n = 0;
    let mut line1 = String::new();
    let mut line2 = String::new();
    for line in std::io::stdin().lines() {
        if let Ok(line) = line {
            n += 1;
            match n {
                1 => {
                    line1 = uncomment_line(n, &line);
                }
                2 => {
                    line2 = uncomment_line(n, &line);
                }
                3 => {
                    println!("{}", line1);
                    line1 = line2;
                    line2 = uncomment_line(n, &line);
                }
                _ => {
                    println!("{}", line1);
                    line1 = line2;
                    line2 = if line.is_empty() {
                        line
                    } else {
                        " ".repeat(8) + &line
                    };
                }
            }
        } else {
            break;
        }
    }
    println!(
        "{}",
        uncomment_line(n - 1, &line1.chars().skip(8).collect::<String>())
    );
    println!(
        "{}",
        uncomment_line(n, &line2.chars().skip(8).collect::<String>())
    );
}

fn uncomment_line(n: usize, line: &str) -> String {
    if line.starts_with("//") {
        line.chars().skip(2).collect::<String>()
    } else {
        panic!("Unexpected uncommented line {}: {}", n, line)
    }
}
