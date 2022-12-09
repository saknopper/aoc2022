use ndarray::Array2;

fn main() {
    println!("Day 8");

    let input = include_str!("../input/part1.txt");
    let height = input.lines().count();
    let width = input.lines().next().unwrap().chars().count();

    let mut tree_map = Array2::<u8>::zeros((height, width));
    for (i, line) in input.lines().enumerate() {
        for (j, tree_height) in line.chars().enumerate() {
            tree_map[[i, j]] = tree_height.to_digit(10).unwrap() as u8;
        }
    }

    let mut visible_trees = (2 * height) + ((width - 2) * 2);
    for i in 1..height - 1 {
        for j in 1..width - 1 {
            let (up, down, left, right) =
                get_search_directions(i, j, height, width, tree_map[[i, j]], &tree_map);

            if up == None {
                visible_trees += 1;
                continue;
            }

            if down == None {
                visible_trees += 1;
                continue;
            }

            if left == None {
                visible_trees += 1;
                continue;
            }

            if right == None {
                visible_trees += 1;
                continue;
            }
        }
    }

    print!("Part 1 - visible trees: {}\n", visible_trees);

    let mut max_scenic_score = 0;

    for i in 1..height - 1 {
        for j in 1..width - 1 {
            let up_score;
            let down_score;
            let left_score;
            let right_score;

            let (up, down, left, right) =
                get_search_directions(i, j, height, width, tree_map[[i, j]], &tree_map);

            match up {
                None => up_score = i,
                Some(s) => up_score = s.0 + 1,
            }

            match down {
                None => down_score = height - i - 1,
                Some(s) => down_score = s.0 + 1,
            }

            match left {
                None => left_score = j,
                Some(s) => left_score = s.0 + 1,
            }

            match right {
                None => right_score = height - j - 1,
                Some(s) => right_score = s.0 + 1,
            }

            max_scenic_score =
                max_scenic_score.max(up_score * down_score * left_score * right_score);
        }
    }

    print!("Part 2 - highest scenic score: {}\n", max_scenic_score);
}

fn get_search_directions(
    i: usize,
    j: usize,
    height: usize,
    width: usize,
    cur_tree_height: u8,
    tree_map: &Array2<u8>,
) -> (
    Option<(usize, u8)>,
    Option<(usize, u8)>,
    Option<(usize, u8)>,
    Option<(usize, u8)>,
) {
    let cur_column = tree_map.column(j);
    let cur_row = tree_map.row(i);

    let up = cur_column
        .iter()
        .take(i)
        .copied()
        .rev()
        .enumerate()
        .find(|(_idx, h)| *h >= cur_tree_height);

    let down = cur_column
        .iter()
        .rev()
        .take(height - i - 1)
        .copied()
        .rev()
        .enumerate()
        .find(|(_idx, h)| *h >= cur_tree_height);

    let left = cur_row
        .iter()
        .take(j)
        .copied()
        .rev()
        .enumerate()
        .find(|(_idx, h)| *h >= cur_tree_height);

    let right = cur_row
        .iter()
        .rev()
        .copied()
        .take(width - j - 1)
        .rev()
        .enumerate()
        .find(|(_idx, h)| *h >= cur_tree_height);

    (up, down, left, right)
}
