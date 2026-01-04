fn main() {
    let input = include_str!("input.txt");

    fn check_silly(s: &str) -> bool {
        if s.len() % 2 != 0 {
            false
        }
        else {
            let l = s.len() / 2;
            s[0..l] == s[l..2*l]
        }
    }

    fn silly_numbers_in_range(r: (i64, i64)) -> impl Iterator<Item = i64> {
        let (start,end) = r;
        (start..=end)
            .filter(|&num| {
                let num_str = num.to_string();
                check_silly(&num_str)
            })
    }

    fn parse_range(range: &str) -> Option<(i64, i64)> {
        let parts: Vec<&str> = range.split('-').map(str::trim).collect();
        if parts.len() != 2 { return None; }

        let start = parts[0].parse::<i64>().ok()?;
        let end = parts[1].parse::<i64>().ok()?;

        Some((start, end))
    }

    let sum: i64 = input
        .lines()
        .flat_map(|line| line.split(',').map(str::trim))
        .filter_map(parse_range)
        .flat_map(silly_numbers_in_range)
        .sum();

    println!("{:?}", sum);


}
