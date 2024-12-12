use std::env;
use std::fs;

fn solve<P>(
    mut checksum: usize,
    mut empties: Vec<(usize, usize)>,
    mut files: Vec<(usize, usize, usize)>,
    pred: P,
) -> usize
where
    P: Fn(usize, usize) -> bool,
{
    loop {
        let Some((id, file_offset, file_size)) = files.last_mut() else {
            break;
        };
        let Some((empty_offset, empty_size)) = empties
            .iter_mut()
            .rev()
            .take_while(|block| block.0 < *file_offset)
            .find(|block| pred(block.1, *file_size))
        else {
            files.pop();
            continue;
        };
        let fragment = *file_size.min(empty_size);
        checksum -= *id * (*file_offset + *file_size - fragment - *empty_offset) * fragment;
        *file_size -= fragment;
        if *file_size == 0 {
            files.pop();
        }
        *empty_offset += fragment;
        *empty_size -= fragment;
        let redundant = empties.iter().rev().take_while(|(_, es)| *es == 0).count();
        empties.truncate(empties.len() - redundant);
    }
    checksum
}

fn main() {
    let input = env::args_os().nth(1).unwrap();
    let mut contents = fs::read_to_string(input).unwrap();
    contents.truncate(contents.trim_end().len());
    let sizes: Vec<usize> = contents
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();
    let mut offset = 0;
    let mut empties: Vec<(usize, usize)> = Vec::with_capacity(sizes.len() / 2);
    let mut files: Vec<(usize, usize, usize)> = Vec::with_capacity((sizes.len() + 1) / 2);
    let mut checksum: usize = 0;
    for (i, size) in sizes.into_iter().enumerate() {
        if size == 0 {
            continue;
        }
        if i % 2 == 0 {
            let id = i / 2;
            files.push((id, offset, size));
            checksum += id * (2 * offset + size - 1) * size / 2;
        } else {
            empties.push((offset, size));
        }
        offset += size;
    }
    empties.reverse();

    let ans1 = solve(checksum, empties.clone(), files.clone(), |_, _| true);
    println!("ans1 = {ans1}");

    let ans2 = solve(checksum, empties, files, |size, file_size| {
        size >= file_size
    });
    println!("ans2 = {ans2}");
}
