use std::env;
use std::fs;

fn main() {
    let input = env::args_os().nth(1).unwrap();
    let contents = fs::read_to_string(input).unwrap();
    let sizes: Vec<usize> = contents
        .chars()
        .filter_map(|c| c.to_digit(10).map(|d| d as usize))
        .collect();
    let mut id = 0;
    let mut offset = 0;
    let mut memory: Vec<i32> = vec![-1; sizes.iter().cloned().sum()];
    let mut empties: Vec<(usize, usize)> = Vec::with_capacity(sizes.len() / 2);
    let mut files: Vec<(usize, usize)> = Vec::with_capacity((sizes.len() + 1) / 2);
    for (&size, is_file) in sizes.iter().zip([true, false].into_iter().cycle()) {
        if size == 0 {
            continue;
        }
        if is_file {
            files.push((offset, size));
            memory[offset..offset + size].fill(id);
            id += 1;
        } else {
            empties.push((offset, size));
        }
        offset += size;
    }
    empties.reverse();

    let mut memory1 = memory.clone();
    let mut empties1 = empties.clone();
    let mut files1 = files.clone();

    loop {
        let Some((file_offset, file_size)) = files1.last_mut() else {
            break;
        };
        let Some((empty_offset, empty_size)) = empties1.last_mut() else {
            break;
        };
        if empty_offset > file_offset {
            break;
        }
        let fragment = *file_size.min(empty_size);
        for i in 0..fragment {
            memory1.swap(*empty_offset + i, *file_offset + *file_size - fragment + i);
        }
        *file_size -= fragment;
        *empty_offset += fragment;
        *empty_size -= fragment;
        if *file_size == 0 {
            files1.pop();
        }
        if *empty_size == 0 {
            empties1.pop();
        }
    }

    let ans1: usize = memory1
        .iter()
        .take_while(|&&id| id != -1)
        .enumerate()
        .map(|(pos, &id)| (id as usize) * pos)
        .sum();
    println!("ans1 = {ans1}");

    let mut memory2 = memory.clone();
    let mut empties2 = empties.clone();
    let mut files2 = files.clone();

    loop {
        let Some((file_offset, file_size)) = files2.last_mut() else {
            break;
        };
        let Some((empty_offset, empty_size)) = empties2
            .iter_mut()
            .rev()
            .take_while(|block| block.0 < *file_offset)
            .find(|block| block.1 >= *file_size)
        else {
            files2.pop();
            continue;
        };
        let fragment = *file_size.min(empty_size);
        for i in 0..fragment {
            memory2.swap(*empty_offset + i, *file_offset + *file_size - fragment + i);
        }
        *file_size -= fragment;
        *empty_offset += fragment;
        *empty_size -= fragment;
        if *file_size == 0 {
            files2.pop();
        }
        while let Some((_, empty_size)) = empties2.last() {
            if *empty_size == 0 {
                empties2.pop();
            } else {
                break;
            }
        }
    }

    let ans2: usize = memory2
        .iter()
        .enumerate()
        .filter(|(_, &id)| id != -1)
        .map(|(pos, &id)| (id as usize) * pos)
        .sum();
    println!("ans2 = {ans2}");
}
