//! Social — SocialField, Affective Distance, and Neighbor Context

use crate::being32::Being32;

#[derive(Clone, Debug, Default)]
pub struct SocialField {
    pub avg_valence: f32,
    pub avg_arousal: f32,
    pub avg_permeability: f32,
    pub density: f32,
    pub valence_spread: f32,
}

#[derive(Clone, Debug)]
pub struct NeighborSnapshot {
    pub id: u16,
    pub distance: f32,
    pub aff_valence: f32,
    pub aff_arousal: f32,
    pub bnd_permeability: f32,
    pub rel_curvature: f32,
}

#[derive(Clone, Debug)]
pub struct LocalContext {
    pub neighbors: Vec<NeighborSnapshot>,
    pub field: SocialField,
}

pub fn affective_distance(a: &Being32, b: &Being32) -> f32 {
    let valence_gap = (a.aff_valence() - b.aff_valence()).abs() / 2.0;
    let arousal_gap = (a.aff_arousal() - b.aff_arousal()).abs() / 2.0;
    let boundary_sum = (a.bnd_permeability() + b.bnd_permeability()).clamp(0.0, 2.0);
    (valence_gap + arousal_gap) * (2.0 - boundary_sum)
}

pub fn compute_social_field(beings: &[Being32]) -> SocialField {
    if beings.is_empty() {
        return SocialField::default();
    }

    let mut sum_val = 0.0;
    let mut sum_ar = 0.0;
    let mut sum_perm = 0.0;
    let mut vals = Vec::with_capacity(beings.len());

    for b in beings {
        let v = b.aff_valence();
        let a = b.aff_arousal();
        let p = b.bnd_permeability();
        sum_val += v;
        sum_ar += a;
        sum_perm += p;
        vals.push(v);
    }

    let n = beings.len() as f32;
    let avg_valence = sum_val / n;
    let avg_arousal = sum_ar / n;
    let avg_permeability = sum_perm / n;

    let spread = if n > 1.0 {
        let mean = avg_valence;
        let var = vals.iter().map(|v| (v - mean).powi(2)).sum::<f32>() / n;
        var.sqrt()
    } else {
        0.0
    };

    SocialField {
        avg_valence,
        avg_arousal,
        avg_permeability,
        density: n,
        valence_spread: spread,
    }
}
