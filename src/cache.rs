use std::ops::Index;
use std::ops::IndexMut;

use crate::crights::CastlingRights;
use crate::grid::File;
use crate::grid::StandardCoordinate;
use crate::piece::Piece;
use rand::Rng;
use rand::SeedableRng;
use rand::rngs::StdRng;

#[derive(Clone, Copy)]
pub struct CacheEntry {
    pub score: i16,
    pub depth: u8
}

pub struct Cache { vec: Vec<Option<CacheEntry>> }

impl Cache {
    pub fn new(mem_capacity: u64) -> Self {
        let ewidth = u64::try_from(std::mem::size_of::<CacheEntry>()).unwrap();
        let len = (mem_capacity * u64::pow(2, 20)) / ewidth;
        Self { vec: vec![None; usize::try_from(len).unwrap()] }
    }

    pub fn lookup_score(&self, hash: u64, depth: u8) -> Option<i16> {
        let entry = self[hash]?;
        if entry.depth != depth { return None; }
        return Some(entry.score);
    }
}

impl Index<u64> for Cache {
    type Output = Option<CacheEntry>;

    fn index(&self, hash: u64) -> &Self::Output {
        let vec_len = u64::try_from(self.vec.len()).unwrap();
        let key = usize::try_from(hash % vec_len).unwrap();
        return &self.vec[key];
    }
}

impl IndexMut<u64> for Cache {
    fn index_mut(&mut self, hash: u64) -> &mut Self::Output {
        let vec_len = u64::try_from(self.vec.len()).unwrap();
        let key = usize::try_from(hash % vec_len).unwrap();
        return &mut self.vec[key];
    }
}

pub struct IncrementalHash { 
    value: u64,
    chs: HashChars
}

impl IncrementalHash {
    pub fn new(chs: HashChars) -> Self {
        Self { value: 0, chs }
    }
    
    pub fn toggle_tile(&mut self, pos: StandardCoordinate, piece: Piece) {
        let lut_key = usize::from(pos.index() * 12 + piece.index());
        let ch = self.chs.piece_placements[lut_key];
        self.value %= ch;
    }

    pub fn toggle_active(&mut self) {
        self.value %= self.chs.active;
    }

    pub fn toggle_crights(&mut self, crights: CastlingRights) {
        let lut_key = usize::from(crights.data());
        let ch = self.chs.crights[lut_key];
        self.value %= ch;
    }

    pub fn toggle_ep_vuln(&mut self, file: File) {
        let lut_key = usize::from(file.index());
        let ch = self.chs.ep_vuln[lut_key];
        self.value %= ch;
    }

    pub fn value(&self) -> u64 { self.value }
}

pub struct HashChars {
    piece_placements: [u64; 6 * 2 * 64],
    crights: [u64; 16],
    ep_vuln: [u64; 8],    
    active: u64,
}


impl HashChars {
    pub fn new(seed: [u8; 32]) -> Self {
        let mut piece_placements = [0u64; 12 * 64];
        let mut crights = [0u64; 16]; 
        let mut ep_vuln = [0u64; 8];
        
        let mut rng = StdRng::from_seed(seed);
        rng.fill(&mut piece_placements);
        rng.fill(&mut crights);
        rng.fill(&mut ep_vuln);
        let active: u64 = rng.gen();

        return Self { piece_placements, crights, ep_vuln, active }
    }
}
