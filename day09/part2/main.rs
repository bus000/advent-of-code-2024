// --- Part Two ---
//
// Upon completion, two things immediately become clear. First, the disk
// definitely has a lot more contiguous free space, just like the amphipod
// hoped. Second, the computer is running much more slowly! Maybe introducing
// all of that file system fragmentation was a bad idea?
//
// The eager amphipod already has a new plan: rather than move individual
// blocks, he'd like to try compacting the files on his disk by moving whole
// files instead.
//
// This time, attempt to move whole files to the leftmost span of free space
// blocks that could fit the file. Attempt to move each file exactly once in
// order of decreasing file ID number starting with the file with the highest
// file ID number. If there is no span of free space to the left of a file that
// is large enough to fit the file, the file does not move.
//
// The first example from above now proceeds differently:
//
//    00...111...2...333.44.5555.6666.777.888899
//    0099.111...2...333.44.5555.6666.777.8888..
//    0099.1117772...333.44.5555.6666.....8888..
//    0099.111777244.333....5555.6666.....8888..
//    00992111777.44.333....5555.6666.....8888..
//
// The process of updating the filesystem checksum is the same; now, this
// example's checksum would be 2858.
//
// Start over, now compacting the amphipod's hard drive using this new method
// instead. What is the resulting filesystem checksum?
use std::io;
use std::io::Read;
use std::fmt::Debug;
use aoc2024::aoc::AocError;

fn main() -> Result<(), AocError> {
    let mut disk = read_disk()?;
    disk.compact();
    println!("{:?}", disk.checksum());
    return Ok(());
}

/// Read disk from stdin.
fn read_disk() -> Result<Disk, AocError> {
    let mut input = String::new();
    let stdin = io::stdin();
    let mut lock = stdin.lock();
    lock.read_to_string(&mut input)?;
    let ns = input.as_bytes().iter()
        .filter(|b| **b != b'\n')
        .map(|b| b - b'0')
        .collect();
    return Ok(Disk::init_disk(&ns));
}

/// A list of blocks that are part of a file.
#[derive(PartialEq, Eq, Debug, Clone)]
struct FileBlocks {

    /// If this is a file then this is the ID of the file.
    file_id: u64,

    /// The index of the first block in the file system.
    start_position: u64,

    /// The number of file system blocks this block contains.
    size: u64,

}

/// A list of blocks that are free.
#[derive(PartialEq, Eq, Debug, Clone)]
struct FreeBlocks {

    /// The index of the first block in the file system.
    start_position: u64,

    /// The number of file system blocks this block contains.
    size: u64,

}

/// A disk consisting of a list of files and a list of unoccupied spaces.
#[derive(PartialEq, Eq, Debug)]
struct Disk {

    /// List of file blocks stored in increasing position.
    file_blocks: Vec<FileBlocks>,

    /// Free space on the disk stored in increasing position.
    free_blocks: Vec<FreeBlocks>,

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
                file_blocks.push(FileBlocks {
                    file_id: (i / 2).try_into().unwrap(),
                    start_position: position,
                    size: size
                });
            } else {
                free_blocks.push(FreeBlocks {
                    start_position: position,
                    size: size
                });
            }

            position = position + size;
        }

        return Disk { file_blocks: file_blocks, free_blocks: free_blocks };
    }

    /// Compute the checksum of the disk.
    fn checksum(&self) -> u64 {
        let mut sum = 0;
        for file in self.file_blocks.iter() {
            let start_position = file.start_position;
            let size = file.size;
            for pos in start_position..start_position + size {
                sum = sum + pos * file.file_id;
            }
        }

        return sum;
    }

    /// Compact the disk.
    ///
    /// Will move all file blocks to the beginning of the disk such that all
    /// free spaces are at the end of the disk. Will begin by moving the last
    /// file to the first free position. Will only move complete files and will
    /// not split up files in multiple different locations.
    fn compact(&mut self) {
        let mut extra_free = Vec::new();
        for file in self.file_blocks.iter_mut().rev() {
            for free in self.free_blocks.iter_mut() {
                if free.start_position > file.start_position {
                    break;
                } else if free.size >= file.size {
                    extra_free.push(FreeBlocks {
                        start_position: file.start_position,
                        size: file.size
                    });
                    file.start_position = free.start_position;
                    free.size -= file.size;
                    free.start_position += file.size;
                    break;
                }
            }
        }

        self.file_blocks.sort_by(|a, b| a.start_position.cmp(&b.start_position));
        self.free_blocks.retain(|f| f.size > 0);
        for free in extra_free.into_iter() {
            self.free_blocks.push(free);
        }
        self.free_blocks.sort_by(|a, b| a.start_position.cmp(&b.start_position));
    }

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
                FileBlocks {
                    file_id: 0,
                    start_position: 0,
                    size: 1
                },
                FileBlocks {
                    file_id: 1,
                    start_position: 3,
                    size: 3
                },
                FileBlocks {
                    file_id: 2,
                    start_position: 10,
                    size: 5
                },
            ],
            free_blocks: vec![
                FreeBlocks {
                    start_position: 1,
                    size: 2
                },
                FreeBlocks {
                    start_position: 6,
                    size: 4
                },
            ],
        };
        assert_eq!(disk, expected);
    }

    /// Test that computing checksum works as expected.
    #[test]
    fn test_checksum_empty() {
        let disk = Disk::init_disk(&vec![]);
        assert_eq!(disk.checksum(), 0);
    }

    /// Test that computing checksum works as expected.
    #[test]
    fn test_checksum() {
        let disk = Disk::init_disk(&vec![1, 2, 3, 4, 5]);
        let expected =
            0 * 0 + // First file.
            1 * 3 + 1 * 4 + 1 * 5 + // Second file.
            2 * 10 + 2 * 11 + 2 * 12 + 2 * 13 + 2 * 14; // Third file.
        assert_eq!(disk.checksum(), expected);
    }

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
            FileBlocks {
                file_id: 0,
                start_position: 0,
                size: 1
            },
            FileBlocks {
                file_id: 1,
                start_position: 3,
                size: 3
            },
            FileBlocks {
                file_id: 2,
                start_position: 10,
                size: 5
            },
        ]);
        assert_eq!(disk.free_blocks, vec![
            FreeBlocks {
                start_position: 1,
                size: 2
            },
            FreeBlocks {
                start_position: 6,
                size: 4
            }
        ]);
    }

    /// Test that compacting a non empty disk will move files accordingly.
    #[test]
    fn test_compact_disk_2() {
        let mut disk = Disk::init_disk(&vec![5, 4, 3, 2, 1]);
        disk.compact();
        assert_eq!(disk.file_blocks, vec![
            FileBlocks {
                file_id: 0,
                start_position: 0,
                size: 5
            },
            FileBlocks {
                file_id: 2,
                start_position: 5,
                size: 1
            },
            FileBlocks {
                file_id: 1,
                start_position: 6,
                size: 3
            },
        ]);
        assert_eq!(disk.free_blocks, vec![
            FreeBlocks {
                start_position: 9,
                size: 3
            },
            FreeBlocks {
                start_position: 12,
                size: 2
            },
            FreeBlocks {
                start_position: 14,
                size: 1
            },
        ]);
    }

}
