fn main() {
    part1();
    part2();
}

fn part1(){
    const RADIX: u32 = 10;
    let mut count = 0;
    for x in 128392..643281{
        let num_string = x.to_string();
        
        let mut double = false;
        let mut never_decreases = true;

        for (i, c) in num_string.chars().enumerate() {
            if i == 0 {
                continue;
            }
            
            let last: u32 = num_string.chars().nth(i - 1).unwrap().to_digit(RADIX).unwrap();
            let curr: u32 = c.to_digit(RADIX).unwrap();

            if curr < last {
                never_decreases = false;
                break;
            }

            if curr == last {
                double = true;
            }
        }

        if double && never_decreases {
            count+=1;
        }
    }

    println!("Part 1: {}", count);
}

fn part2(){
    const RADIX: u32 = 10;
    let mut count = 0;
    for x in 128392..643281 {
        
        let num_string = x.to_string();
        let mut atleast_one_double = false;
        let mut never_decreases = true;
        let mut same_in_a_row = 1;

        for (i, c) in num_string.chars().enumerate() {
            if i == 0 {
                continue;
            }
            
            let last: u32 = num_string.chars().nth(i - 1).unwrap().to_digit(RADIX).unwrap();
            let curr: u32 = c.to_digit(RADIX).unwrap();

            if curr < last {
                never_decreases = false;
                break;
            }

            if curr == last {
                same_in_a_row += 1;
            }
            
            if same_in_a_row == 2 && curr != last { 
                atleast_one_double = true;
            }
            
            if curr != last {
                same_in_a_row = 1;
            }
        }

        if atleast_one_double && never_decreases {
            count += 1;
        }
    }

    println!("Part 2: {}", count);
}