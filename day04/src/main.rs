#[derive(Debug)]
struct Elf {
    start: u32,
    end: u32,
}

#[derive(Debug)]
struct ElvesPair {
    elf1: Elf,
    elf2: Elf,
}

fn main() {
    println!("Day 4");

    let input = include_str!("../input/part1.txt");

    let mut elves_pairs: Vec<ElvesPair> = Vec::new();
    for line in input.lines() {
        let splitted_areas: Vec<&str> = line.split(",").collect();
        assert_eq!(splitted_areas.len(), 2);

        let area1: Vec<&str> = splitted_areas[0].split("-").collect();
        let area2: Vec<&str> = splitted_areas[1].split("-").collect();
        assert_eq!(area1.len(), 2);
        assert_eq!(area2.len(), 2);

        let elf1 = Elf {
            start: area1[0].parse::<u32>().unwrap(),
            end: area1[1].parse::<u32>().unwrap(),
        };
        let elf2 = Elf {
            start: area2[0].parse::<u32>().unwrap(),
            end: area2[1].parse::<u32>().unwrap(),
        };

        elves_pairs.push(ElvesPair { elf1, elf2 });
    }

    let count_complete_overlaps = elves_pairs
        .iter()
        .filter(|pair| is_completely_overlapping(pair))
        .count();
    print!("Part 1 - complete overlaps: {}\n", count_complete_overlaps);

    let count_overlaps = elves_pairs
        .iter()
        .filter(|pair| is_overlapping(pair))
        .count();
    print!("Part 2 - overlaps: {}\n", count_overlaps);
}

fn is_completely_overlapping(pair: &ElvesPair) -> bool {
    if pair.elf1.start <= pair.elf2.start && pair.elf1.end >= pair.elf2.end {
        return true;
    }

    if pair.elf2.start <= pair.elf1.start && pair.elf2.end >= pair.elf1.end {
        return true;
    }

    return false;
}

fn is_overlapping(pair: &ElvesPair) -> bool {
    if (pair.elf1.start >= pair.elf2.start && pair.elf1.start <= pair.elf2.end)
        || (pair.elf1.end >= pair.elf2.start && pair.elf1.end <= pair.elf2.end)
    {
        return true;
    }

    if (pair.elf2.start >= pair.elf1.start && pair.elf2.start <= pair.elf1.end)
        || (pair.elf2.end >= pair.elf1.start && pair.elf2.end <= pair.elf1.end)
    {
        return true;
    }

    return false;
}
