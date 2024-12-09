use std::collections::VecDeque;

use span_1d::Span1D;
use utils::AocBufReader;

fn main() {
    part_1(AocBufReader::from_string("aoc/src/day_9/data/part_1.txt"));
    part_2(AocBufReader::from_string("aoc/src/day_9/data/part_1.txt"));
}

fn part_1(mut input: AocBufReader) {
    println!("part 1: {}", part_1_inner(input.next().unwrap()))
}

fn part_2(mut input: AocBufReader) {
    println!("part 2: {}", part_2_inner(input.next().unwrap()))
}

fn part_1_inner(s: String) -> usize {
    let mut disk = Disk::from_string(s);
    disk.compact_part_1();
    disk.check_sum()
}

fn part_2_inner(s: String) -> usize {
    let mut disk = Disk::from_string(s);
    disk.compact_part_2();
    disk.check_sum()
}

struct Disk {
    data: Vec<Option<usize>>,
    full_idx_stack: Vec<usize>,
    file_span_stack: Vec<Span1D<usize>>,
    empty_idx_stack: VecDeque<usize>,
}

impl Disk {
    fn compact_part_1(&mut self) {
        while let Some(next_empty_idx) = self.empty_idx_stack.pop_back() {
            let next_full_idx = self.full_idx_stack.pop().unwrap();
            if next_empty_idx > next_full_idx {
                break;
            }

            self.data.swap(next_empty_idx, next_full_idx);
            self.empty_idx_stack.push_front(next_full_idx);
        }
    }

    fn move_file(&mut self, file: &Span1D<usize>, dest: usize) {
        for block_idx in 0..file.len {
            self.data.swap(file.start + block_idx, dest + block_idx);
        }
    }

    fn compact_part_2(&mut self) {
        let files = self
            .file_span_stack
            .iter()
            .rev()
            .cloned()
            .collect::<Vec<Span1D<usize>>>();

        for file in files {
            for empty_span in self.empty_spans() {
                if empty_span.start > file.start {
                    break;
                }

                if empty_span.len >= file.len {
                    // we found a home! we cna move the file
                    self.move_file(&file, empty_span.start);
                    break;
                }
            }
        }
    }

    fn check_sum(&self) -> usize {
        self.data
            .iter()
            .enumerate()
            .map(|(idx, x)| match x {
                Some(val) => *val * idx,
                None => 0,
            })
            .sum()
    }

    fn empty_spans(&self) -> Vec<Span1D<usize>> {
        let mut result: Vec<Span1D<usize>> = Vec::new();
        let mut current_span: Option<Span1D<usize>> = None;
        for (idx, x) in self.data.iter().enumerate() {
            match x {
                Some(_) => {
                    if let Some(span) = current_span {
                        // we've reached the end of our current span!
                        // We add the span to our result and set current span to None
                        result.push(span);
                        current_span = None;
                    }
                }
                None => {
                    // we're either in a span or are starting a new span
                    if let Some(ref mut span) = current_span {
                        span.increment();
                    } else {
                        current_span = Some(Span1D::new(idx, 1));
                    }
                }
            }
        }
        result
    }

    fn from_string(s: String) -> Self {
        let mut data: Vec<Option<usize>> = Vec::new();
        let mut full_idx_stack: Vec<usize> = Vec::new();
        let mut file_span_stack: Vec<Span1D<usize>> = Vec::new();
        let mut empty_idx_stack_rev: Vec<usize> = Vec::new();
        let mut file_id: usize = 0;
        let mut write_head: usize = 0;

        let mut chars = s.chars();
        while let Some(full_block_count) = chars.next() {
            let digit = full_block_count.to_digit(10u32).unwrap();
            file_span_stack.push(Span1D::new(write_head, digit as usize));
            for _ in 0..digit {
                data.push(Some(file_id));
                full_idx_stack.push(write_head);
                write_head += 1;
            }
            file_id += 1;

            if let Some(empty_block_count) = chars.next() {
                let digit = empty_block_count.to_digit(10u32).unwrap();
                for _ in 0..digit {
                    data.push(None);
                    empty_idx_stack_rev.push(write_head);
                    write_head += 1;
                }
            }
        }

        let empty_idx_stack: VecDeque<usize> = empty_idx_stack_rev.into_iter().rev().collect();
        Self {
            data,
            full_idx_stack,
            file_span_stack,
            empty_idx_stack,
        }
    }

    #[allow(dead_code)]
    fn print(&self) {
        println!(
            "{}",
            self.data
                .iter()
                .map(|x| {
                    match x {
                        Some(val) => format!("{}", val),
                        None => ".".to_string(),
                    }
                })
                .collect::<String>()
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1_inner("2333133121414131402".to_string()), 1928);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2_inner("2333133121414131402".to_string()), 2858);
    }
}
