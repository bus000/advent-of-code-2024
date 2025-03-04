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

    /// Ordered list of blocks.
    blocks: Vec<Blocks>,

}

impl Disk {

    /// Initialize a new disk from a list of files and free spaces.
    fn init_disk(disk: &Vec<u8>) -> Self {
        let mut blocks = Vec::with_capacity(disk.len());
        let mut position = 0;
        for i in 0..disk.len() {
            let size: u64 = disk[i].into();
            if i % 2 == 0 {
                blocks.push(Blocks {
                    file_id: Some((i / 2).try_into().unwrap()),
                    start_position: position,
                    size: size
                });
            } else {
                blocks.push(Blocks {
                    file_id: None,
                    start_position: position,
                    size: size
                });
            }

            position = position + size;
        }

        return Disk { blocks: blocks };
    }

    /// Compute the checksum of the disk.
    fn checksum(&self) -> u64 {
        let mut sum = 0;
        for blocks in self.blocks.iter() {
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
        //let frees = self.blocks.iter().filter(|x| x.is_free());
        let mut files = self.blocks.iter().rev().filter(|x| x.is_file());
        let mut blocks = Vec::with_capacity(self.blocks.len());
        let mut next_file = files.next().clone();

        for block in self.blocks.iter() {
            match block.file_id {
                None => {
                    let mut free = block.clone();
                    while free.size > 0 && next_file.is_some() {
                        let file = next_file.unwrap();
                        if free.start_position > file.start_position {
                            blocks.push(free);
                            next_file = None;
                        } else if free.size >= file.size {
                            blocks.push(Blocks {
                                file_id: file.file_id,
                                start_position: free.start_position,
                                size: file.size
                            })
                            free.start_position += file.size;
                            free.size -= file.size;
                            file = files.next().clone();
                        } else {
                            blocks.push
                        }
                    }
                },
                Some(_) => {
                    blocks.push(block);
                }
            }





            match (block.file_id, next_file) {
                //(Some(_), _) => {
                    //blocks.push(block.clone()); // TODO
                //},
                (None, Some(file)) => {
                    let mut free = block.clone();
                    while free.size > 0 {
                        if free.start_position > file.start_position {
                            file = None;
                        } else if free.size >= file.size {
                            blocks.push(Blocks {
                                file_id: file.file_id,
                                start_position: free.start_position,
                                size: file.size
                            })
                            free.start_position += file.size;
                            free.size -= file.size;
                            file = files.next().clone();
                        } else {
                            blocks.push
                        }
                    }
                }
                _ => {
                    blocks.push(block.clone());
                }
            }
        }

        self.blocks = blocks;
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

impl Blocks {

    fn is_file(&self) -> bool {
        return self.file_id.is_some();
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    ///// Test that initializing a new disk works as expected.
    //#[test]
    //fn test_init_disk_empty() {
        //let disk = Disk::init_disk(&vec![]);
        //assert_eq!(disk, Disk {
            //blocks: vec![]
        //});
    //}

    ///// Test that initializing a new disk works as expected.
    //#[test]
    //fn test_init_disk() {
        //let disk = Disk::init_disk(&vec![1, 2, 3, 4, 5]);
        //let expected = Disk {
            //blocks: vec![
                //Blocks::File {
                    //file_id: 0,
                    //start_position: 0,
                    //size: 1
                //},
                //Blocks::Free {
                    //start_position: 1,
                    //size: 2
                //},
                //Blocks::File {
                    //file_id: 1,
                    //start_position: 3,
                    //size: 3
                //},
                //Blocks::Free {
                    //start_position: 6,
                    //size: 4
                //},
                //Blocks::File {
                    //file_id: 2,
                    //start_position: 10,
                    //size: 5
                //},
            //],
        //};
        //assert_eq!(disk, expected);
    //}

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

    ///// Test that empty disks have no move targets.
    //#[test]
    //fn test_find_move_target_empty() {
        //let disk = Disk::init_disk(&vec![]);
        //assert_eq!(disk.find_move_target(), None);
    //}

    ///// Test that disks that contain only files have no move targets.
    //#[test]
    //fn test_find_move_target_only_files() {
        //let disk = Disk {
            //files: vec![
                //FileBlock {
                    //file_id: 0,
                    //start_position: 0,
                    //size: 2
                //},
                //FileBlock {
                    //file_id: 1,
                    //start_position: 2,
                    //size: 3
                //},
            //],
            //free: vec![]
        //};
        //assert_eq!(disk.find_move_target(), None);
    //}

    ///// Test that disks that contain only free space have no move targets.
    //#[test]
    //fn test_find_move_target_only_free() {
        //let disk = Disk {
            //files: vec![],
            //free: vec![
                //FreeSpace {
                    //start_position: 10000,
                    //size: 1
                //},
                //FreeSpace {
                    //start_position: 0,
                    //size: 10000
                //},
            //]
        //};
        //assert_eq!(disk.find_move_target(), None);
    //}

    ///// Test that disks that contain only free space at the end and files at
    ///// the start has no move targets.
    //#[test]
    //fn test_find_move_target_only_free_at_end() {
        //let disk = Disk {
            //files: vec![
                //FileBlock {
                    //file_id: 0,
                    //start_position: 0,
                    //size: 2
                //},
                //FileBlock {
                    //file_id: 1,
                    //start_position: 2,
                    //size: 3
                //},
            //],
            //free: vec![
                //FreeSpace {
                    //start_position: 6,
                    //size: 2
                //},
                //FreeSpace {
                    //start_position: 5,
                    //size: 1
                //},
            //]
        //};
        //assert_eq!(disk.find_move_target(), None);
    //}

    ///// Test that disks will find the correct move target (first free and last
    ///// file).
    //#[test]
    //fn test_find_move_target() {
        //let disk = Disk {
            //files: vec![
                //FileBlock {
                    //file_id: 0,
                    //start_position: 0,
                    //size: 2
                //},
                //FileBlock {
                    //file_id: 1,
                    //start_position: 5,
                    //size: 3
                //},
            //],
            //free: vec![
                //FreeSpace {
                    //start_position: 3,
                    //size: 2
                //},
                //FreeSpace {
                    //start_position: 2,
                    //size: 1
                //},
            //]
        //};
        //assert_eq!(disk.find_move_target(), Some((
                //&FreeSpace {
                    //start_position: 2,
                    //size: 1
                //},
                //&FileBlock {
                    //file_id: 1,
                    //start_position: 5,
                    //size: 3
                //})));
    //}

    ///// Test that the empty disk has a 0 checksum.
    //#[test]
    //fn test_compute_checksum() {
        //let disk = vec![];
        //assert_eq!(Ok(0), compute_checksum(disk));
    //}

    ///// Test that the singleton disk has a 0 checksum.
    //#[test]
    //fn test_compute_checksum() {
        //let disk = vec![9];
        //assert_eq!(Ok(0), compute_checksum(disk));
    //}

}
