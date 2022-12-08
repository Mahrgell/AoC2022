use ndarray::{s, Array2};
use std::fs;

fn read_forest() -> Array2<u32> {
    let mut f = vec![];
    let input = fs::read_to_string("input.txt").expect("Failed to read file.");
    for l in input.lines() {
        let row: Vec<_> = l.chars().map(|c| c.to_digit(10).unwrap()).collect();
        f.push(row);
    }
    Array2::from_shape_fn((f.len(), f[0].len()), |(x, y)| f[x][y])
}

fn get_view_result(forest: &Array2<u32>, x: usize, y: usize, dir: usize) -> (u32, bool) {
    let view = match dir {
        0 => forest.slice(s![x, ..=y;-1]),
        1 => forest.slice(s![..=x;-1, y]),
        2 => forest.slice(s![x, y..]),
        3 => forest.slice(s![x.., y]),
        _ => panic!(),
    };
    let mut itr = view.iter();
    let &height = itr.next().unwrap();
    itr.fold((0, true), |(r, v), &h| {
        (if v { r + 1 } else { r }, v && h < height)
    })
}

fn main() {
    let forest = read_forest();

    let mut nb_edge_viewer = 0;
    let mut best_view = 0;
    for x in 0..forest.dim().0 {
        for y in 0..forest.dim().1 {
            let mut view_range = 1;
            let mut can_see_edge = false;
            for dir in 0..4 {
                let (r, v) = get_view_result(&forest, x, y, dir);
                view_range *= r;
                can_see_edge |= v;
            }
            best_view = best_view.max(view_range);
            if can_see_edge {
                nb_edge_viewer += 1;
            }
        }
    }
    println!("Can see edge: {}", nb_edge_viewer);
    println!("Best view score: {}", best_view);
}
