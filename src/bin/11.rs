advent_of_code::solution!(11);

use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u64> {
    let graph = parse_input(input);
    let mut memo = HashMap::new();
    Some(count_paths("you", "out", &graph, &mut memo))
}

pub fn part_two(input: &str) -> Option<u64> {
    let graph = parse_input(input);

    // Path must be svr -> ... -> dac -> ... -> fft -> ... -> out
    // OR svr -> ... -> fft -> ... -> dac -> ... -> out

    // Check path: svr -> dac -> fft -> out
    let svr_dac = count_paths_fresh("svr", "dac", &graph);
    let dac_fft = count_paths_fresh("dac", "fft", &graph);
    let fft_out = count_paths_fresh("fft", "out", &graph);

    let path1 = svr_dac * dac_fft * fft_out;

    // Check path: svr -> fft -> dac -> out
    let svr_fft = count_paths_fresh("svr", "fft", &graph);
    let fft_dac = count_paths_fresh("fft", "dac", &graph);
    let dac_out = count_paths_fresh("dac", "out", &graph);

    let path2 = svr_fft * fft_dac * dac_out;

    Some(path1 + path2)
}

fn count_paths_fresh(start: &str, end: &str, graph: &HashMap<&str, Vec<&str>>) -> u64 {
    let mut memo = HashMap::new();
    count_paths(start, end, graph, &mut memo)
}

fn parse_input(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut graph = HashMap::new();
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some((src, dests_str)) = line.split_once(':') {
            let dests: Vec<&str> = dests_str.split_whitespace().collect();
            graph.insert(src, dests);
        }
    }
    graph
}

fn count_paths<'a>(
    node: &'a str,
    target: &'a str,
    graph: &HashMap<&'a str, Vec<&'a str>>,
    memo: &mut HashMap<&'a str, u64>,
) -> u64 {
    if node == target {
        return 1;
    }
    if let Some(&count) = memo.get(node) {
        return count;
    }

    let mut total_paths = 0;
    if let Some(neighbors) = graph.get(node) {
        for neighbor in neighbors {
            total_paths += count_paths(neighbor, target, graph, memo);
        }
    }

    memo.insert(node, total_paths);
    total_paths
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let input = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";
        let result = part_two(input);
        assert_eq!(result, Some(2));
    }
}
