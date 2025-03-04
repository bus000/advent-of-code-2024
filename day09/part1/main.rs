// --- Day 9: Disk Fragmenter ---
//
// Another push of the button leaves you in the familiar hallways of some
// friendly amphipods! Good thing you each somehow got your own personal mini
// submarine. The Historians jet away in search of the Chief, mostly by driving
// directly into walls.
//
// While The Historians quickly figure out how to pilot these things, you notice
// an amphipod in the corner struggling with his computer. He's trying to make
// more contiguous free space by compacting all of the files, but his program
// isn't working; you offer to help.
//
// He shows you the disk map (your puzzle input) he's already generated. For
// example:
//
//    2333133121414131402
//
// The disk map uses a dense format to represent the layout of files and free
// space on the disk. The digits alternate between indicating the length of a
// file and the length of free space.
//
// So, a disk map like 12345 would represent a one-block file, two blocks of
// free space, a three-block file, four blocks of free space, and then a
// five-block file. A disk map like 90909 would represent three nine-block files
// in a row (with no free space between them).
//
// Each file on disk also has an ID number based on the order of the files as
// they appear before they are rearranged, starting with ID 0. So, the disk map
// 12345 has three files: a one-block file with ID 0, a three-block file with ID
// 1, and a five-block file with ID 2. Using one character for each block where
// digits are the file ID and . is free space, the disk map 12345 represents
// these individual blocks:
//
//    0..111....22222
//
// The first example above, 2333133121414131402, represents these individual
// blocks:
//
//    00...111...2...333.44.5555.6666.777.888899
//
// The amphipod would like to move file blocks one at a time from the end of the
// disk to the leftmost free space block (until there are no gaps remaining
// between file blocks). For the disk map 12345, the process looks like this:
//
//    0..111....22222
//    02.111....2222.
//    022111....222..
//    0221112...22...
//    02211122..2....
//    022111222......
//
// The first example requires a few more steps:
//
//    00...111...2...333.44.5555.6666.777.888899
//    009..111...2...333.44.5555.6666.777.88889.
//    0099.111...2...333.44.5555.6666.777.8888..
//    00998111...2...333.44.5555.6666.777.888...
//    009981118..2...333.44.5555.6666.777.88....
//    0099811188.2...333.44.5555.6666.777.8.....
//    009981118882...333.44.5555.6666.777.......
//    0099811188827..333.44.5555.6666.77........
//    00998111888277.333.44.5555.6666.7.........
//    009981118882777333.44.5555.6666...........
//    009981118882777333644.5555.666............
//    00998111888277733364465555.66.............
//    0099811188827773336446555566..............
//
// The final step of this file-compacting process is to update the filesystem
// checksum. To calculate the checksum, add up the result of multiplying each of
// these blocks' position with the file ID number it contains. The leftmost
// block is in position 0. If a block contains free space, skip it instead.
//
// Continuing the first example, the first few blocks' position multiplied by
// its file ID number are 0 * 0 = 0, 1 * 0 = 0, 2 * 9 = 18, 3 * 9 = 27,
// 4 * 8 = 32, and so on. In this example, the checksum is the sum of these,
// 1928.
//
// Compact the amphipod's hard drive using the process he requested. What is the
// resulting filesystem checksum?
use std::io;
use std::io::Read;
use std::fmt::Debug;
use std::cmp::max;
use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use aoc2024::aoc::AocError;

fn main() -> Result<(), AocError> {
    return Ok(());
}

/// A disk consisting of a list of files and a list of unoccupied spaces.
#[derive(PartialEq, Eq, Debug)]
struct Disk {

    /// List of file blocks stored in increasing position.
    file_blocks: Vec<Blocks>,

    /// List of free blocks stored in decreasing position.
    free_blocks: Vec<Blocks>,

}

impl Disk {

    /// Initialize a new disk from a list of files and free spaces.
    fn init_disk(disk: &Vec<u8>) -> Self {
        let mut file_blocks = Vec::with_capacity(disk.len() / 2);
        let mut free_blocks = Vec::with_capacity(disk.len() / 2);
        let mut position = 0;
        for i in 0..disk.len() {
            let size: u64 = disk[i].into();
            if i % 2 == 0 {
                file_blocks.push(Blocks {
                    file_id: Some((i / 2).try_into().unwrap()),
                    start_position: position,
                    size: size
                });
            } else {
                free_blocks.push(Blocks {
                    file_id: None,
                    start_position: position,
                    size: size
                });
            }

            position = position + size;
        }

        free_blocks.reverse();
        return Disk { file_blocks: file_blocks, free_blocks: free_blocks };
    }

    /// Compute the checksum of the disk.
    fn checksum(&self) -> u64 {
        let mut sum = 0;
        for blocks in self.file_blocks.iter() {
            match blocks.file_id {
                Some(file_id) => {
                    let start_position = blocks.start_position;
                    let size = blocks.size;
                    for pos in start_position..start_position + size {
                        sum = sum + pos * file_id;
                    }
                },
                _ => {}
            }
        }

        return sum;
    }

    /// Compact the disk.
    ///
    /// Will move all file blocks to the beginning of the disk such that all
    /// free spaces are at the end of the disk. Will begin by moving the last
    /// file to the first free position.
    fn compact(&mut self) {
        let mut file_blocks = Vec::with_capacity(self.file_blocks.len());
        let mut space = 0;

        loop {
            match (self.file_blocks.pop(), self.free_blocks.pop()) {
                (None, None) => {
                    break;
                },
                (None, Some(free)) => {
                    space += free.size;
                },
                (Some(file), None) => {
                    file_blocks.push(file);
                },
                (Some(mut file), Some(mut free)) => {
                    if file.start_position < free.start_position {
                        space += free.size;
                        self.file_blocks.push(file);
                    } else if free.size > file.size {
                        space += file.size;
                        file.start_position = free.start_position;
                        free.start_position += file.size;
                        free.size -= file.size;
                        file_blocks.push(file);
                        self.free_blocks.push(free);
                    } else if free.size == file.size {
                        file.start_position = free.start_position;
                        file_blocks.push(file);
                        space += free.size;
                    } else {
                        file_blocks.push(Blocks {
                            file_id: file.file_id,
                            start_position: free.start_position,
                            size: free.size
                        });
                        file.size -= free.size;
                        space += free.size;
                        self.file_blocks.push(file);
                    }
                },
            }
        }

        file_blocks.sort_by(|a, b| a.start_position.cmp(&b.start_position));
        if space > 0 {
            let start_position = file_blocks
                .last()
                .map(|f| f.start_position + f.size)
                .unwrap_or(0);
            self.free_blocks = vec![Blocks {
                file_id: None,
                start_position: start_position,
                size: space
            }];
        } else {
            self.free_blocks = vec![];
        }
        self.file_blocks = file_blocks;
    }

}

/// A list of blocks that are part of a file.
#[derive(PartialEq, Eq, Debug, Clone)]
struct Blocks {

    /// If this is a file then this is the ID of the file.
    file_id: Option<u64>,

    /// The index of the first block in the file system.
    start_position: u64,

    /// The number of file system blocks this block contains.
    size: u64,

}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that initializing a new disk works as expected.
    #[test]
    fn test_init_disk_empty() {
        let disk = Disk::init_disk(&vec![]);
        assert_eq!(disk, Disk {
            file_blocks: vec![],
            free_blocks: vec![]
        });
    }

    /// Test that initializing a new disk works as expected.
    #[test]
    fn test_init_disk() {
        let disk = Disk::init_disk(&vec![1, 2, 3, 4, 5]);
        let expected = Disk {
            file_blocks: vec![
                Blocks {
                    file_id: Some(0),
                    start_position: 0,
                    size: 1
                },
                Blocks {
                    file_id: Some(1),
                    start_position: 3,
                    size: 3
                },
                Blocks {
                    file_id: Some(2),
                    start_position: 10,
                    size: 5
                },
            ],
            free_blocks: vec![
                Blocks {
                    file_id: None,
                    start_position: 6,
                    size: 4
                },
                Blocks {
                    file_id: None,
                    start_position: 1,
                    size: 2
                },
            ],
        };
        assert_eq!(disk, expected);
    }

    ///// Test that computing checksum works as expected.
    //#[test]
    //fn test_checksum_empty() {
        //let disk = Disk::init_disk(&vec![]);
        //assert_eq!(disk.checksum(), 0);
    //}

    ///// Test that computing checksum works as expected.
    //#[test]
    //fn test_checksum() {
        //let disk = Disk::init_disk(&vec![1, 2, 3, 4, 5]);
        //let expected =
            //0 * 0 + // First file.
            //1 * 3 + 1 * 4 + 1 * 5 + // Second file.
            //2 * 10 + 2 * 11 + 2 * 12 + 2 * 13 + 2 * 14; // Third file.
        //assert_eq!(disk.checksum(), expected);
    //}

    /// Test that compacting an empty disk will do nothing.
    #[test]
    fn test_compact_empty_disk() {
        let mut disk = Disk::init_disk(&vec![]);
        disk.compact();
        assert_eq!(disk, Disk {
            file_blocks: vec![],
            free_blocks: vec![]
        });
    }

    /// Test that compacting a non empty disk will move files accordingly.
    #[test]
    fn test_compact_disk_1() {
        let mut disk = Disk::init_disk(&vec![1, 2, 3, 4, 5]);
        disk.compact();
        assert_eq!(disk.file_blocks, vec![
            Blocks {
                file_id: Some(0),
                start_position: 0,
                size: 1
            },
            Blocks {
                file_id: Some(2),
                start_position: 1,
                size: 2
            },
            Blocks {
                file_id: Some(1),
                start_position: 3,
                size: 3
            },
            Blocks {
                file_id: Some(2),
                start_position: 6,
                size: 3
            }
        ]);
        assert_eq!(disk.free_blocks, vec![
            Blocks {
                file_id: None,
                start_position: 9,
                size: 6
            }
        ]);
    }

    /// Test that compacting a non empty disk will move files accordingly.
    #[test]
    fn test_compact_disk_2() {
        let mut disk = Disk::init_disk(&vec![5, 4, 3, 2, 1]);
        disk.compact();
        assert_eq!(disk.file_blocks, vec![
            Blocks {
                file_id: Some(0),
                start_position: 0,
                size: 5
            },
            Blocks {
                file_id: Some(2),
                start_position: 5,
                size: 1
            },
            Blocks {
                file_id: Some(1),
                start_position: 6,
                size: 3
            },
        ]);
        assert_eq!(disk.free_blocks, vec![
            Blocks {
                file_id: None,
                start_position: 9,
                size: 6
            }
        ]);
    }

}
