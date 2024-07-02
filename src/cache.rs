use std::ops::Index;
use std::ops::IndexMut;

use crate::crights::CastlingRights;
use crate::enpassant::is_enpassant_vuln;
use crate::gamestate::FastPosition;
use crate::grid::File;
use crate::grid::StandardCoordinate;
use crate::piece::Color;
use crate::piece::Piece;
use crate::piece::PieceGrid;
use rand::Rng;
use rand::SeedableRng;
use rand::rngs::StdRng;
use rand::thread_rng;

#[derive(Clone, Copy)]
pub struct CacheEntry {
    pub score: i16,
    pub depth: u8,
    // pub key: PackedPosition,
    pub hash: u64,
}

pub struct Cache { 
    vec: Vec<Option<CacheEntry>>,
    occupancy: usize
}

impl Cache {
    pub fn new(mem_capacity: u64) -> Self {
        let ewidth = u64::try_from(std::mem::size_of::<CacheEntry>()).unwrap();
        let len = (mem_capacity * u64::pow(2, 20)) / ewidth;
        Self { vec: vec![None; usize::try_from(len).unwrap()], occupancy: 0 }
    }

    pub fn lookup_score(&self, state: &FastPosition, depth: u8) -> Option<i16> {
        let lut_key = usize::try_from(state.hash.value() % 
            u64::try_from(self.vec.len()).unwrap()).unwrap();
        if let Some(entry) = self.vec[lut_key] {
            if entry.hash == state.hash.value() {
            // if entry.key == pack(state) {
                if entry.depth >= depth {
                    return Some(entry.score)
                }
            }
        }
        return None;
    }

    pub fn update_score(&mut self, state: &FastPosition, score: i16, depth: u8) {
        if self.lookup_score(state, depth).is_some() { return; }
        let lut_key = usize::try_from(state.hash.value() % 
            u64::try_from(self.vec.len()).unwrap()).unwrap();
        // let key = pack(state); 
        if self.vec[lut_key].is_none() { self.occupancy += 1; }
        self.vec[lut_key] = Some(CacheEntry { score, depth, 
            hash: state.hash.value(), /* key */ });
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct PackedPosition {
    p_lut: PieceGrid,
    active_player: Color,
    crights: CastlingRights,
    enpassant_vuln: Option<File>
}

fn pack(state: &FastPosition) -> PackedPosition {
    PackedPosition { 
        p_lut: state.p_lut, 
        active_player: state.active_player(),
        crights: state.crights, 
        enpassant_vuln: is_enpassant_vuln(state)
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
        self.value ^= ch;
    }

    pub fn toggle_active(&mut self) {
        self.value ^= self.chs.active;
    }

    pub fn toggle_crights(&mut self, crights: CastlingRights) {
        let lut_key = usize::from(crights.data());
        let ch = self.chs.crights[lut_key];
        self.value ^= ch;
    }

    pub fn toggle_ep_vuln(&mut self, file: File) {
        let lut_key = usize::from(file.index());
        let ch = self.chs.ep_vuln[lut_key];
        self.value ^= ch;
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

    pub fn new_random() -> Self {
        let mut seed = [0u8; 32];
        thread_rng().fill(&mut seed);
        return Self::new(seed);
    }
}
