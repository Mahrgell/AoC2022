use std::cell::RefCell;
use std::fs;
use std::rc::{Rc, Weak};
use std::str::Lines;

struct ADir {
    name: String,
    subdirs: Vec<Rc<RefCell<ADir>>>,
    files: Vec<AFile>,
    parent: Option<Weak<RefCell<ADir>>>,
}

struct AFile {
    _name: String,
    size: u64,
}

fn create_dir(name: &str, parent: Option<Weak<RefCell<ADir>>>) -> Rc<RefCell<ADir>> {
    Rc::new(RefCell::new(ADir {
        name: String::from(name),
        subdirs: vec![],
        files: vec![],
        parent,
    }))
}

fn next_line_has_output(l: &Lines) -> bool {
    match l.clone().next() {
        Some(s) => s.split_whitespace().next().unwrap() != "$",
        None => false,
    }
}

fn read_ls(l: &mut Lines, rcd: Rc<RefCell<ADir>>) {
    let mut dir = rcd.borrow_mut();
    while next_line_has_output(l) {
        let s = l.next().unwrap();
        let words: Vec<_> = s.split_whitespace().collect();
        match words[0] {
            "dir" => dir
                .subdirs
                .push(create_dir(words[1], Some(Rc::downgrade(&rcd)))),
            _ => dir.files.push(AFile {
                _name: String::from(words[1]),
                size: words[0].parse::<u64>().unwrap(),
            }),
        }
    }
}

fn process_cd(current_dir: &Weak<RefCell<ADir>>, dir_name: &str) -> Weak<RefCell<ADir>> {
    let cd = current_dir.upgrade().unwrap();
    match dir_name {
        "/" => match &cd.borrow().parent {
            Some(p) => process_cd(p, dir_name),
            None => current_dir.clone(),
        },
        ".." => cd.borrow().parent.clone().unwrap(),
        _ => {
            let children = &cd.borrow().subdirs;
            let child = children
                .iter()
                .find(|&rc| rc.borrow().name == dir_name)
                .unwrap();
            Rc::downgrade(child)
        }
    }
}

fn compute_size(dir: &ADir) -> u64 {
    let sz = dir
        .subdirs
        .iter()
        .fold(0, |a, d| a + compute_size(&d.borrow()));
    dir.files.iter().fold(sz, |a, f| a + f.size)
}

fn collect_dirs(dir: &Rc<RefCell<ADir>>) -> Vec<Rc<RefCell<ADir>>> {
    let mut dirs = vec![dir.clone()];
    for subdir in &dir.borrow().subdirs {
        dirs.append(&mut collect_dirs(subdir));
    }
    dirs
}

fn build_fs(input: &String) -> Rc<RefCell<ADir>> {
    let root = create_dir("root", None);
    let mut current_dir = Rc::downgrade(&root);
    let mut l = input.lines();
    while let Some(s) = l.next() {
        let words: Vec<_> = s.split_whitespace().collect();
        assert_eq!(words[0], "$");
        match words[1] {
            "cd" => current_dir = process_cd(&current_dir, words[2]),
            "ls" => read_ls(&mut l, current_dir.upgrade().unwrap()),
            _ => panic!(),
        }
    }
    root
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read file.");
    let root = build_fs(&input);

    let total_sz = compute_size(&root.borrow());
    let mut best_del = 70000000;
    let mut result1 = 0;
    for d in collect_dirs(&root) {
        let sz = compute_size(&d.borrow());
        if sz <= 100000 {
            result1 += sz;
        }
        if sz > total_sz - 40000000 && sz < best_del {
            best_del = sz;
        }
    }
    println!("{}", result1);
    println!("{}", best_del);
}
