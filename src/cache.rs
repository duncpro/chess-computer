use crate::crights::CastlingRights;
use crate::gamestate::FastPosition;
use crate::grid::File;
use crate::grid::StandardCoordinate;
use crate::piece::Piece;
use crate::piece::PieceGrid;
use crate::piece::Color;
use rand::Rng;
use rand::SeedableRng;
use rand::rngs::StdRng;

#[derive(Clone, Copy)]
pub struct CacheEntry {
    pub bestmove: u8
}

struct Cache { vec: Vec<Option<CacheEntry>> }

impl Cache {
    fn new(size: usize) -> Self {
        Self { vec: vec![None; size] }
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
