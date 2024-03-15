use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

pub fn get_from_path<P>(filepath:P, data_path:&String, indent_size:usize, path_separator: &String) -> Option<String>
where P:AsRef<Path>, {
    if let Ok(lines) = read_lines(filepath) {
        let mut data_tree = data_path.split(path_separator).peekable();
        let mut current_indent = 0;
        for line in lines.flatten() {
            let line_spacing = get_indentation(&line, indent_size);
            let begin_index = { line_spacing * indent_size };
            let end_index = line
                .find(": ")
                .or(line.rfind(":"))
                .unwrap_or(line.len());
            let target_node = data_tree.peek(); // TODO: find a way to keep this between lines, probably via a mutable variable
            if line_spacing > current_indent {
                continue
            } else if line_spacing < current_indent {
                // This happens when we reach the end of a branch without finding
                // the wanted node.
                return None
            }
            let node = &line[begin_index..end_index];
            if target_node.is_some_and(|f| f == &node) {
                // found the good node
                current_indent += 1;
                data_tree.next();
                if data_tree.peek().is_none() {
                    // we're on the last node, attempting to fetch data
                    let subline = &line[end_index+1..line.len()];
                    for c in subline.split("").enumerate() {
                        if c.1.len() > 0 && c.1 != " " && c.1 != "\"" && c.1 != "'" {
                            let data_index = c.0 + end_index;
                            let last_char = line.chars().last().unwrap();
                            let data_end = if last_char == '"' || last_char == '\'' { line.len()-1 } else { line.len() };
                            let data = &line[data_index..data_end];
                            return Some(String::from(data));
                        }
                    }
                }
            }
        }
    }
    return None
}

fn read_lines<P>(filename:P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_indentation(line:&String, indent_size:usize) -> usize {
    let mut indent = 0;
    for c in line.split(&" ".repeat(indent_size)) {
        if c.len() < 1 {
            indent += 1;
        } else {
            break
        }
    }
    return indent
}

fn get_spacing(line:&str) -> usize {
    let iter_result = line.split(" ").enumerate().find(|&a| a.1.len() > 0);
    return iter_result.unwrap_or((line.len(),&line)).0;
}

pub fn get_indent_size<P>(filepath:P) -> Option<usize>
    where P: AsRef<Path> {
    if let Ok(lines) = read_lines(filepath) {
        let mut previous_amount = 0;
        for line in lines.flatten() {
            let line_indent_char = get_spacing(&line);
            if line_indent_char > previous_amount {
                return Some(line_indent_char)
            } else {
                previous_amount = line_indent_char
            }
        }
    }

    return None
}