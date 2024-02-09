fn main() {
    let mut args = std::env::args();
    args.next();
    if let Some(option) = args.next() {
        if let Some(pathname) = args.next() {
            match option.as_str() {
                "comment" => comment_lines(&pathname),
                "uncomment" => uncomment_lines(&pathname),
                _ => panic!("Invalid option: comment, uncomment"),
            }
        } else {
            panic!("Missing pathname after option");
        }
    } else {
        panic!("Missing option: comment, uncomment");
    }
}

fn comment_lines(pathname: &str) {
    let contents = std::fs::read_to_string(pathname)
        .unwrap_or_else(|_| panic!("Failed to read file \"{}\".", pathname));
    let mut result = String::new();
    let mut n = 0;
    let mut line1 = String::new();
    let mut line2 = String::new();
    for line in contents.lines() {
        n += 1;
        match n {
            1 => {
                line1 = format!("//{}", line);
            }
            2 => {
                line2 = format!("//{}", line);
            }
            3 => {
                result += &line1;
                result.push('\n');
                line1 = line2;
                line2 = format!("//{}", line);
            }
            _ => {
                result += &line1;
                result.push('\n');
                line1 = line2;
                line2 = line.to_string();
            }
        }
    }
    result += &format!("//{}\n//{}\n", line1, line2);
    std::fs::write(pathname, result)
        .unwrap_or_else(|_| panic!("Failed to write file \"{}\".", pathname));
}

fn uncomment_lines(pathname: &str) {
    let contents = std::fs::read_to_string(pathname)
        .unwrap_or_else(|_| panic!("Failed to read file \"{}\".", pathname));
    let mut result = String::new();
    let mut n = 0;
    let mut line1 = String::new();
    let mut line2 = String::new();
    for line in contents.lines() {
        n += 1;
        match n {
            1 => {
                line1 = uncomment_line(n, line);
            }
            2 => {
                line2 = uncomment_line(n, line);
            }
            3 => {
                result += &line1;
                result.push('\n');
                line1 = line2;
                line2 = uncomment_line(n, line);
            }
            _ => {
                result += &line1;
                result.push('\n');
                line1 = line2;
                line2 = if line.is_empty() {
                    line.to_string()
                } else {
                    " ".repeat(8) + line
                };
            }
        }
    }
    result += &format!(
        "{}\n{}\n",
        uncomment_line(n - 1, &line1.chars().skip(8).collect::<String>()),
        uncomment_line(n, &line2.chars().skip(8).collect::<String>())
    );
    std::fs::write(pathname, result)
        .unwrap_or_else(|_| panic!("Failed to write file \"{}\".", pathname));
}

fn uncomment_line(n: usize, line: &str) -> String {
    if line.starts_with("//") {
        line.chars().skip(2).collect::<String>()
    } else {
        panic!("Unexpected uncommented line {}: {}", n, line)
    }
}
