fn inner_convolve_2d(input: &Vec<Vec<i32>>, kernel: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    assert!(kernel.len() % 2 == 1, "Kernel has even amount of rows");
    for r in kernel.iter() {
        assert!(r.len()% 2 == 1, "Kernel has a row with even amount of columns")
    }

    // Find center to later calculate offets from it
    let kernel_x_center = (kernel.len() - 1) / 2;
    let kernel_y_center = (kernel[0].len() - 1) / 2;

    let mut o = vec![vec![0; input[0].len()]; input.len()];
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            for k_y in 0..kernel.len() {
                for k_x in 0..kernel[0].len() {
                    let x_target = (x as i32) + ((k_x as i32) - (kernel_x_center as i32));
                    let y_target = (y as i32) + ((k_y as i32) - (kernel_y_center as i32));

                    // println!("cords i:{:?},{:?} k:{:?}{:?} o:{:?}{:?}", x_target, y_target, k_x, k_y, x, y);

                    if x_target >= 0 && y_target >= 0 && x_target < (input[0].len() as i32) && y_target < (input.len() as i32) {
                        // println!("i:{:?} k:{:?} o:{:?}", input[y_target as usize][x_target as usize], kernel[k_y][k_x] ,o[y][x]);
                        o[y][x] += input[y_target as usize][x_target as usize] * kernel[k_y][k_x];
                    }
                }
            }
        }
    }
    o
}

fn main() {
    let input: Vec<&str> = include_str!("input.txt").lines().collect();

    let kernel: Vec<Vec<i32>> = vec![vec![1,1,1],vec![1,0,1],vec![1,1,1]];

    let mut to_be_convolved: Vec<Vec<i32>> = vec![vec![0; input[0].len()]; input.len()];

    for y in 0..input.len(){
        for x in 0..input[0].len(){
            if (input[y].as_bytes()[x] as char) == '@'{
                to_be_convolved[y][x] = 1;
            }
        }
    }
    let convolved = inner_convolve_2d(&to_be_convolved, &kernel);

    // println!("{:?}", convolved);

    let mut num_rolls = 0;

    for y in 0..input.len() {
        for x in 0..input[0].len(){
           if to_be_convolved[y][x] == 1 && convolved[y][x] < 4 {
               num_rolls += 1
           }
        }
    }

    println!("{:?}", num_rolls);
}
