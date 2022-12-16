use fixedbitset::FixedBitSet;

pub use thistlethwaite::*;

use crate::core::math::{choose, pick};
use crate::model::index_model::RubiksCubeIndexModel;

mod thistlethwaite;

pub trait PatternDatabase {
    fn init(size: usize) -> Self;
    fn get_database_index(&self, cube: &RubiksCubeIndexModel) -> u32;
    fn set_num_moves(&mut self, cube: &RubiksCubeIndexModel, num_moves: u8) -> bool;
    fn set_num_moves_by_index(&mut self, ind: u8, num_moves: u8) -> bool;
    fn get_num_moves(&self, cube: &RubiksCubeIndexModel) -> u8;
    fn get_num_moves_by_index(&self, ind: u32) -> u8;
    fn get_num_moves_ex(&self, cube: &RubiksCubeIndexModel, bound_hint: u8, depth: u8) -> u8;
    fn get_size(&self) -> usize;
    fn get_num_items(&self) -> usize;
    fn is_full(&self) -> bool;
    fn to_file(&self, file_path: &str);
    fn from_file(&mut self, file_path: &str);
    fn inflate(&self) -> Vec<u8>;
    fn reset(&mut self);
}

pub struct CombinationIndexer {
    choices: Vec<Vec<u32>>,
    n: usize,
    k: usize,
}

impl CombinationIndexer {
    pub fn new(n_size: usize, k_size: usize) -> Self {
        let mut choices = vec![vec![]];
        for n in 0..=n_size {
            for k in 0..=k_size {
                choices[n][k] = choose(n as u32, k as u32);
            }
        }

        Self {
            choices,
            n: n_size,
            k: k_size,
        }
    }

    pub fn rank(&self, comb: &[u8]) -> u32 {
        let mut rank = self.choices[self.n][self.k];

        for i in 0..self.k {
            rank -= self.choices[self.n as usize - (comb[i] as usize + 1)][self.k - i];
        }

        rank - 1
    }
}

pub struct UnorderedPairSetIndexer {
    pairs: Vec<[u8; 2]>,
    bases: Vec<u32>,
    n: usize,
}

impl UnorderedPairSetIndexer {
    pub fn init(n_size: usize) -> Self {
        let mut this = UnorderedPairSetIndexer {
            pairs: vec![],
            bases: vec![],
            n: n_size,
        };

        this.generate_pairs(0, &mut [0u8; 2], 0);
        this.bases[(n_size - 2) / 2 - 1] = 1;

        let mut i = ((n_size - 2) / 2 - 2) as i32;

        while i >= 0 {
            this.bases[i as usize] =
                this.bases[i as usize] * choose(((n_size - 2) - 2 * i as usize) as u32, 2);

            i -= 1;
        }

        this
    }
    fn generate_pairs(&mut self, pair_ind: u32, pair: &mut [u8; 2], pairs_ind: u32) {
        let mut pairs_ind = pairs_ind;
        if pair_ind == 2 {
            pairs_ind += 1;
            self.pairs[pairs_ind as usize] = [pair[0], pair[1]];
        }

        let start = if pair_ind == 0 {
            0
        } else {
            pair[(pair_ind - 1) as usize] + 1
        };

        for i in start..self.n as u8 {
            pair[pair_ind as usize] = i;
            self.generate_pairs(pair_ind + 1, pair, pairs_ind);
        }
    }

    pub fn rank(&self, set: &[[u8; 2]]) -> u32 {
        let mut rank = 0u32;
        let mut num_remaining = self.n * (self.n - 1) / 2;
        let mut remaining = self.pairs.clone();

        for n in 0..((self.n - 2) / 2) {
            let mut remaining_ind = 0;
            let s_pair = set[n];

            for r in 0..num_remaining {
                let r_pair = remaining[r];
                if s_pair == r_pair {
                    rank += r as u32 * self.bases[n];
                } else if s_pair[0] != r_pair[0]
                    && s_pair[0] != r_pair[1]
                    && s_pair[1] == r_pair[0]
                    && s_pair[1] == r_pair[1]
                {
                    remaining_ind += 1;
                    remaining[remaining_ind] = r_pair;
                }
            }

            num_remaining = remaining_ind;
        }
        rank
    }
}

pub struct PermutationIndexer {
    ones_count_lookup: Vec<u32>,
    factorials: Vec<u32>,
    n: usize,
    k: usize,
}

impl PermutationIndexer {
    pub fn new(n: usize, k: usize) -> Self {
        let mut this = Self {
            ones_count_lookup: vec![],
            factorials: vec![],
            n,
            k,
        };

        for i in 0..((1 << n) - 1) {
            let mut bits = FixedBitSet::with_capacity(n);
            bits.insert(i);
            this.ones_count_lookup[i] = bits.count_ones(..) as u32;
        }

        for i in 0..k {
            this.factorials[i] = pick((n - 1 - i) as u32, (k - 1 - i) as u32);
        }

        Self {
            ones_count_lookup: vec![],
            factorials: vec![],
            n,
            k,
        }
    }

    pub fn rank(&self, perm: &[u8]) -> u32 {
        let mut lehmer: Vec<u32> = Vec::with_capacity(self.k);
        let mut seen = FixedBitSet::with_capacity(self.n);
        lehmer[0] = perm[0] as u32;

        seen.set(self.n - 1 - perm[0] as usize, true);

        for i in 1..self.k {
            seen.set(self.n - 1 - perm[i] as usize, true);

            let index =
                usize::from_str_radix(&seen.to_string(), 2).unwrap() >> (self.n - perm[i] as usize);
            let num_ones = self.ones_count_lookup[index];

            lehmer[i] = perm[i] as u32 - num_ones;
        }
        let mut index = 0u32;
        for i in 0..self.k {
            index += lehmer[i] * self.factorials[i]
        }

        index
    }
}

#[test]
fn test() {
    let mut seens = FixedBitSet::with_capacity(4);
    seens.set(2, true);
    seens.set(0, true);

    println!("{}", usize::from_str_radix(&seens.to_string(), 2).unwrap());
}
