#[derive(Debug, Default, Clone)]
struct TTEntry<D>
where
    D: Default + Clone,
{
    key: u64,
    data: D,
}

pub struct TT<D>
where
    D: Default + Clone,
{
    data: Vec<TTEntry<D>>,
}

impl<D> TT<D>
where
    D: Default + Clone,
{
    pub fn new(mb: usize) -> Self {
        let bytes = mb * 1024 * 1024;
        let entries = bytes / std::mem::size_of::<D>();
        Self {
            data: vec![TTEntry::default(); entries],
        }
    }

    pub fn clear(&mut self) {
        self.data.fill(TTEntry::default());
    }

    pub fn probe(&mut self, key: u64) -> Option<D> {
        let idx = key as usize % self.data.len();
        let entry = &self.data[idx];
        if entry.key == key {
            Some(entry.data.clone())
        } else {
            None
        }
    }

    pub fn store(&mut self, key: u64, data: D) {
        let idx = key as usize % self.data.len();
        self.data[idx] = TTEntry::<D> {
            key: key,
            data: data,
        };
    }
}

#[repr(u8)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum TTBound {
    #[default]
    NONE,
    EXACT,
    LOWER,
    UPPER,
}

pub fn decisive_score_from_tt(score: i32, ply: i32) -> i32 {
    if score < 0 { score + ply } else { score - ply }
}

pub fn decisive_score_to_tt(score: i32, ply: i32) -> i32 {
    if score > 0 { score + ply } else { score - ply }
}
