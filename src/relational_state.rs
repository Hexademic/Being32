//! RelationalState — Mood, Dyads, and Transient Neighbor Statistics

use std::collections::VecDeque;

#[derive(Clone, Debug, Default)]
pub struct MoodState {
    pub valence: f32,
    pub arousal: f32,
    pub openness: f32,
    pub fatigue: f32,
}

#[derive(Clone, Debug)]
pub struct TransientNeighborStats {
    pub id: u16,
    pub contacts: u32,
    pub last_valence: f32,
    pub last_distance: f32,
    pub mismatch_buf: VecDeque<f32>,
    pub total_mismatch: f32,
    pub mismatch_resolved: bool,
    pub last_contact_step: u64,
    pub time_since_last_contact: f32,
}

impl TransientNeighborStats {
    pub fn new(id: u16) -> Self {
        Self {
            id,
            contacts: 0,
            last_valence: 0.0,
            last_distance: 1.0,
            mismatch_buf: VecDeque::with_capacity(8),
            total_mismatch: 0.0,
            mismatch_resolved: true,
            last_contact_step: 0,
            time_since_last_contact: 0.0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct DyadicEntry {
    pub other_id: u16,
    pub affinity: f32,
    pub trust: f32,
    pub curvature: f32,
    pub mismatch_buf: VecDeque<f32>,
    pub last_contact_step: u64,
    pub time_since_last_contact: f32,
}

#[derive(Clone, Debug)]
pub struct RelationalState {
    pub dyads: Vec<DyadicEntry>,
    pub transient_stats: Vec<TransientNeighborStats>,
    pub valence_history: VecDeque<f32>,
    pub identity_valence: f32,
    pub identity_momentum: f32,
    pub mood: MoodState,
}

impl RelationalState {
    pub fn new() -> Self {
        Self {
            dyads: Vec::new(),
            transient_stats: Vec::new(),
            valence_history: VecDeque::with_capacity(16),
            identity_valence: 0.0,
            identity_momentum: 0.0,
            mood: MoodState::default(),
        }
    }
}
