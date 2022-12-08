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

    let mut visible_trees: u32 = 0;
    for i in 0..height {
        for j in 0..width {
            if i == 0 || j == 0 || i == height - 1 || j == width - 1 {
                visible_trees += 1;
                continue;
            }

            let cur_tree_height = tree_map[[i, j]];

            let cur_column = tree_map.column(j);
            let up = cur_column.iter().take(i).collect::<Vec<_>>();
            if up.iter().find(|h| ***h >= cur_tree_height) == None {
                visible_trees += 1;
                continue;
            }

            let down = cur_column
                .iter()
                .rev()
                .take(height - i - 1)
                .collect::<Vec<_>>();
            if down.iter().find(|h| ***h >= cur_tree_height) == None {
                visible_trees += 1;
                continue;
            }

            let cur_row = tree_map.row(i);
            let left = cur_row.iter().take(j).collect::<Vec<_>>();
            if left.iter().find(|h| ***h >= cur_tree_height) == None {
                visible_trees += 1;
                continue;
            }

            let right = cur_row.iter().rev().take(width - j - 1).collect::<Vec<_>>();
            if right.iter().find(|h| ***h >= cur_tree_height) == None {
                visible_trees += 1;
                continue;
            }
        }
    }

    print!("Part 1 - visible trees: {}\n", visible_trees);

    let mut highest_scenic_score = 0;

    for i in 0..height {
        for j in 0..width {
            let mut up_score = 0;
            let mut down_score = 0;
            let mut left_score = 0;
            let mut right_score = 0;

            let cur_tree_height = tree_map[[i, j]];

            let cur_column = tree_map.column(j);
            let cur_row = tree_map.row(i);

            if i != 0 {
                let up = cur_column.iter().take(i).collect::<Vec<_>>();
                let score = up
                    .iter()
                    .rev()
                    .enumerate()
                    .find(|(_idx, h)| ***h >= cur_tree_height);
                match score {
                    None => up_score = i,
                    Some(s) => up_score = s.0 + 1,
                }
            }

            if i != height - 1 {
                let down = cur_column
                    .iter()
                    .rev()
                    .take(height - i - 1)
                    .collect::<Vec<_>>();
                let score = down
                    .iter()
                    .rev()
                    .enumerate()
                    .find(|(_idx, h)| ***h >= cur_tree_height);
                match score {
                    None => down_score = height - i - 1,
                    Some(s) => down_score = s.0 + 1,
                }
            }

            if j != 0 {
                let left = cur_row.iter().take(j).collect::<Vec<_>>();
                let score = left
                    .iter()
                    .rev()
                    .enumerate()
                    .find(|(_idx, h)| ***h >= cur_tree_height);
                match score {
                    None => left_score = j,
                    Some(s) => left_score = s.0 + 1,
                }
            }

            if j != width - 1 {
                let right = cur_row.iter().rev().take(width - j - 1).collect::<Vec<_>>();
                let score = right
                    .iter()
                    .rev()
                    .enumerate()
                    .find(|(_idx, h)| ***h >= cur_tree_height);
                match score {
                    None => right_score = height - j - 1,
                    Some(s) => right_score = s.0 + 1,
                }
            }

            highest_scenic_score =
                highest_scenic_score.max(up_score * down_score * left_score * right_score);
        }
    }

    print!("Part 2 - highest scenic score: {}\n", highest_scenic_score);
}
