use std::io;

#[derive(Debug)]
enum FSNode {
    File {
        name: String,
        size: usize,
    },
    Directory {
        name: String,
        children: Vec<FSNode>,
        total_size: usize,
    },
}

fn read_line() -> String {
    let mut line = String::new();
    match io::stdin().read_line(&mut line) {
        Ok(_) => String::from(line.trim()),
        _ => String::from(""),
    }
}

fn read_fs(dir_name: String) -> FSNode {
    let mut input = read_line();
    println!("In read_fs({}), input: {}", dir_name, input);

    if input == "$ cd /" {
        input = read_line();
    }

    if input != "$ ls" {
        panic!(
            "Unexpected state, this should be a directory listing, instead: {}",
            input
        );
    }

    let mut children = Vec::<FSNode>::new();
    let mut total_size = 0;
    loop {
        input = read_line();
        if input.is_empty() || input.starts_with("$ cd ") {
            break;
        }

        if input.starts_with("dir ") {
            continue;
        }

        let mut parts = input.split(" ");
        let size = parts.next().unwrap().parse::<usize>().unwrap();
        let name = parts.next().unwrap();

        children.push(FSNode::File {
            name: String::from(name),
            size: size,
        });
        total_size += size;
    }

    loop {
        let parts = input.split(" ");
        let name = parts.last().unwrap();
        if name == ".." || name.is_empty() {
            break;
        }

        let subdir = read_fs(String::from(name));
        match subdir {
            FSNode::Directory { total_size: t, .. } => total_size += t,
            _ => (),
        };
        children.push(subdir);

        input = read_line();
        if input.is_empty() {
            break;
        }
    }

    FSNode::Directory {
        name: dir_name,
        children: children,
        total_size: total_size,
    }
}

fn sum_large_dirs(fs: &FSNode) -> usize {
    let mut sum = 0;

    match fs {
        FSNode::Directory {
            total_size,
            children,
            ..
        } => {
            if total_size < &100000 {
                sum += total_size
            }
            for child in children {
                sum += sum_large_dirs(&child);
            }
        }
        _ => (),
    }

    sum
}

fn find_smallest_to_delete(fs: &FSNode, limit: usize) -> usize {
    let mut smallest = usize::MAX;

    match fs {
        FSNode::Directory {
            total_size,
            children,
            ..
        } => {
            for child in children {
                let local_smallest = find_smallest_to_delete(&child, limit);
                if local_smallest < smallest {
                    smallest = local_smallest;
                }
            }
            if total_size >= &limit && total_size < &smallest {
                smallest = *total_size;
            }
        }
        _ => (),
    }

    smallest
}

fn main() {
    let fs = read_fs(String::from("/"));

    let used_space = match fs {
        FSNode::Directory { total_size, .. } => total_size,
        _ => 0,
    };

    println!("{}", sum_large_dirs(&fs));
    println!(
        "{}",
        find_smallest_to_delete(&fs, 30000000 - (70000000 - used_space))
    );
}
