fn adjacent(i: usize, j: usize) -> Vec<(usize,usize)> {
    let (x, y): (i32,i32) = (i.try_into().unwrap(),j.try_into().unwrap());
    [
        (x+1,y),
        (x-1,y),
        (x,y+1),
        (x,y-1),
        (x+1,y+1),
        (x+1,y-1),
        (x-1,y+1),
        (x-1,y-1),
    ].iter()
    .filter_map(
        |(x,y)| 
            (*x).try_into().ok().zip((*y).try_into().ok())
        )
    .collect()
}

fn number_positions(input: &Vec<Vec<char>>) -> Vec<(usize,Vec<(usize,usize)>)> {
    let digits: Vec<(usize,usize,usize)> = (0..).zip(input).map(|(y, line)| (0..).zip(line).filter_map(|(x, c)|
                    match c.to_digit(10) {
                        Some(num) => Some((x as usize,y as usize,num as usize)),
                        None => None,
                    }).collect::<Vec<(usize,usize,usize)>>()).flatten().collect();
    let mut lastx = 0;
    let mut lasty = 0;
    let mut lastnum = 0;
    let mut firstloop = true;
    let mut buffer = vec![];
    let mut coordinate_buffer = vec![];
    for (x,y,digit) in digits {
        if x == lastx + 1 && y == lasty {
            lastx = x;
            lasty = y;
            lastnum = lastnum*10 + digit
        } else {
            if !firstloop {buffer.push((lastnum,coordinate_buffer))};
            coordinate_buffer = vec![];
            lastx = x;
            lasty = y;
            lastnum = digit;
        }
        coordinate_buffer.push((x,y));
        firstloop = false;
    }
    buffer.push((lastnum,coordinate_buffer));
    buffer
}

fn main() {
    let input = std::fs::read_to_string("./input.txt").expect("Could not read input.txt in current directory");
    let input_array= input.lines().map(|s| s.chars().collect()).collect();
    let numbers = number_positions(&input_array);
    let mut marked_positions: Vec<Vec<(usize, usize)>> = vec![];
    for (i, line) in input_array.into_iter().enumerate() {
        for (j, character) in line.into_iter().enumerate() {
            if character == '*' {
                   marked_positions.push(adjacent(j,i))
            }
        }
    }
    let mut total: usize = 0;
    for mark in marked_positions {
        let included_numbers: Vec<usize> = numbers.iter()
                                      .filter(|(_,position_list)| 
                                                !position_list.iter()
                                                              .filter(|position| mark.contains(position))
                                                              .collect::<Vec<&(usize,usize)>>()
                                                              .is_empty()
                                             )
                                      .map(|(number,_)| *number)
                                      .collect();
        if included_numbers.len() == 2 { total += included_numbers.into_iter().product::<usize>() }
    }
    println!("{:?}",total);
}
