use anyhow::Result;
use thiserror::Error;

#[derive(Error, Debug)]
enum Errors {
    #[error("Invalid index: {index} > {capacity}")]
    OutOfBoundsError { index: usize, capacity: usize },
}

#[derive(Debug)]
pub struct KmerBitSet {
    pub data: Vec<u64>,
    pub capacity: usize,
}

pub struct GetOnesIterator<'a> {
    pub data: &'a [u64],
    pub capacity: usize,
    pub bucket_index: usize,
    pub current_word: u64,
}

impl<'a> Iterator for GetOnesIterator<'a> {
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // If the remaining u64 is zero, we don't have to look anymore there.
            if self.current_word != 0 {
                // Get the index of the smallest indexed one in current word.
                let index = self.current_word.trailing_zeros() as usize;

                // Global index across data.
                let global_index = self.bucket_index * 64 + index;

                if global_index >= self.capacity {
                    return None;
                }

                // Remove the smallest indedex one.
                self.current_word &= self.current_word - 1;

                return Some(global_index);
            }

            self.bucket_index += 1;

            if self.bucket_index >= self.data.len() {
                return None;
            }

            self.current_word = self.data[self.bucket_index]
        }
    }
}

impl KmerBitSet {
    pub fn new(capacity: usize) -> Self {
        // Each entry can store information of up to 64 sequences.
        // Use ceiling division to always have margin.
        let num_buckets = capacity.div_ceil(64);

        return Self {
            data: vec![0u64; num_buckets as usize],
            capacity: capacity,
        };
    }

    #[inline]
    /// Slow, but index safe version.
    pub fn set(&mut self, i: usize) -> Result<(), Errors> {
        if i > self.capacity {
            return Err(Errors::OutOfBoundsError {
                index: i,
                capacity: self.capacity,
            });
        }

        // 0-based bucket index we need to look at.
        let bucket_index = i / 64;
        let word_index = i % 64;

        self.data[bucket_index] |= 1u64 << word_index;

        Ok(())
    }

    #[inline]
    /// Fast, unsafe version.
    pub unsafe fn set_unchecked(&mut self, i: usize) {
        let bucket_index = i / 64;
        let word_index = i % 64;

        unsafe { *self.data.get_unchecked_mut(bucket_index) |= 1u64 << word_index };
    }

    //
    #[inline]
    pub fn is_empty(&self) -> bool {
        return self.data.iter().all(|v| v == &0u64);
    }

    #[inline]
    pub fn ones(&mut self) -> Vec<usize> {
        let mut indices: Vec<usize> = vec![];

        for (i, v) in self.data.iter().enumerate() {
            let mut d = v.clone();

            while d != 0_u64 {
                let first_one_at_index = d.trailing_zeros() as usize;
                indices.push(first_one_at_index + (i * 64));
                d &= d - 1;
            }
        }

        return indices;
    }

    #[inline]
    pub fn ones_by_iterator<'a>(&'a self) -> GetOnesIterator<'a> {
        GetOnesIterator {
            data: &self.data,
            capacity: self.capacity,
            bucket_index: 0,
            current_word: self.data.get(0).copied().unwrap_or(0),
        }
    }
}
