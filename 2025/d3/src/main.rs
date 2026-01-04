fn main() {
    let input = include_str!("input.txt");

    fn find_max_joltage(l: &str) -> i64 {
        let numbers: Vec<i64>= l
            .chars()
            .map(|c| c.to_string().parse::<i64>().unwrap())
            .collect();

        let max = numbers.iter().max().unwrap();
        let max_s = max.to_string();

        let max_numbers: Vec<(usize, &i64)> = numbers
            .iter()
            .enumerate()
            .filter(|&(_, item)| item == max)
            .collect();

        if max_numbers.len() > 1 {
            format!("{max_s}{max_s}").parse::<i64>().unwrap()
        } else {
            let (only_index, _) =  max_numbers[0];
            if only_index == numbers.len() - 1{

                let no_max_vec: Vec<&i64> = numbers
                    .iter()
                    .filter(|n| *n < max)
                    .collect();

                let second_max = no_max_vec.iter().max().unwrap();
                let smax_s = second_max.to_string();

                format!("{smax_s}{max_s}").parse::<i64>().unwrap()
            } else {
                let behind_max = &numbers[(only_index+1)..numbers.len()];

                let max_behind_max = behind_max.iter().max().unwrap();
                let mbm_s = max_behind_max.to_string();

                format!("{max_s}{mbm_s}").parse::<i64>().unwrap()
            }
        }
    }

    let sum: i64 = input        .lines()
        .map(find_max_joltage)
        .sum();

    println!("{:?}", sum);
}
