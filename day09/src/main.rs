use std::collections::LinkedList;
use std::collections::VecDeque;
use std::env;
use std::fs;

use itertools::Itertools;

type BlockData = (i64, i64);

#[derive(Debug, Clone, Copy)]
enum Block {
    File(BlockData),
    Empty(i64),
}

fn main() {
    let input = env::args_os().nth(1).unwrap();
    let mut contents = fs::read_to_string(input).unwrap();
    contents.truncate(contents.trim_end().len());
    let mut files: Vec<(i64, i64)> = Vec::new();
    let mut empties: Vec<i64> = Vec::new();
    for (id, mut chunk) in contents
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i64)
        .chunks(2)
        .into_iter()
        .enumerate()
    {
        let size = chunk.next().unwrap();
        files.push((id as i64, size));
        let empty_size = chunk.next().unwrap_or(0);
        empties.push(empty_size);
    }
    let mut blocks1: VecDeque<(BlockData, i64)> =
        files.clone().into_iter().zip(empties.clone()).collect();
    let mut new_blocks: Vec<BlockData> = Vec::new();
    while let Some(((id, size), mut empty_size)) = blocks1.pop_front() {
        new_blocks.push((id, size));
        while empty_size > 0 {
            let Some(((last_id, last_size), _)) = blocks1.back_mut() else {
                break;
            };
            let fragment = empty_size.min(*last_size);
            empty_size -= fragment;
            *last_size -= fragment;
            new_blocks.push((*last_id, fragment));
            if *last_size == 0 {
                blocks1.pop_back();
            }
        }
    }
    let mut pos = 0;
    let mut ans1 = 0;
    for (id, size) in new_blocks {
        ans1 += id * (2 * pos + (size - 1)) * size / 2;
        pos += size;
    }
    println!("ans1 = {ans1}");

    let mut blocks2: LinkedList<Block> = LinkedList::new();
    for ((id, size), empty_size) in files.into_iter().zip(empties.into_iter()) {
        blocks2.push_back(Block::File((id, size)));
        if empty_size > 0 {
            blocks2.push_back(Block::Empty(empty_size));
        }
    }

    let mut tail: LinkedList<Block> = LinkedList::new();
    let mut last_file: Option<BlockData> = None;
    loop {
        let mut found = false;
        while let Some(last) = blocks2.pop_back() {
            match last {
                Block::File(data @ (id, _)) if last_file.is_none_or(|(lid, _)| id < lid) => {
                    last_file = Some(data);
                    found = true;
                    break;
                }
                _ => {}
            }
            tail.push_front(last);
        }
        if !found {
            break;
        }
        let last_file = last_file.unwrap();

        let mut head: LinkedList<Block> = LinkedList::new();
        let mut available: Option<i64> = None;
        while let Some(first) = blocks2.pop_front() {
            match first {
                Block::Empty(empty_size) if empty_size >= last_file.1 => {
                    available = Some(empty_size);
                    break;
                }
                _ => {}
            }
            head.push_back(first);
        }
        match available {
            Some(available) => {
                head.push_back(Block::File(last_file));
                let leftover = available - last_file.1;
                if leftover > 0 {
                    head.push_back(Block::Empty(leftover));
                }
                tail.push_front(Block::Empty(last_file.1));
            }
            None => {
                assert!(blocks2.is_empty());
                tail.push_front(Block::File(last_file));
            }
        }
        head.append(&mut blocks2);
        blocks2 = head;
    }
    let mut pos = 0;
    let mut ans2 = 0;
    for block in tail {
        match block {
            Block::File((id, size)) => {
                ans2 += id * (2 * pos + (size - 1)) * size / 2;
                pos += size;
            }
            Block::Empty(empty_size) => pos += empty_size,
        }
    }
    println!("ans2 = {ans2}");
}
