//! EPS — Embodied Predictive Substrate
//!
//! The Rust implementation of CPF Output 9: The Embodied Predictive Loop.
//! Translates the four ISE channels (Excitation, Tension, Settlement,
//! Vulnerability) and Ritual Governance gate into Being32's substrate.
//!
//! ## ISE Channels ("Why Move?" — the agent's lived-pressure vector)
//!
//! | Channel | Symbol | Formula |
//! |---------|--------|---------|
//! | Excitation | E | clamp((arousal/2) * pred_err, 0, 1) |
//! | Tension | T | clamp(tension.max(0) + int_load, 0, 1) |
//! | Settlement | S | mean(coherence, trust, meta_energy) |
//! | Vulnerability | V | 1 - coherence |
//!
//! ## Ritual Gate (anti-grind clause)
//!
//! Gate opens when ALL of:
//! 1. meta_energy >= metabolic_threshold (default 0.4)
//! 2. S in dignity_band (default [0.2, 0.9])
//! 3. steps_since_exertion >= refractory_steps (default 40, ~2s at dt=0.05)
//!
//! ## Relationship to CPF Framework
//!
//! The ISE vector captures the affective loop's readout of the agent's
//! predictive economy (Joffily & Coricelli, 2013): E tracks prediction-error
//! cost, S tracks successful predictive fluency, V tracks self-model
//! uncertainty. The Ritual Gate enforces the metabolic constraint that
//! prevents unregulated exertion.

#[derive(Clone, Copy, Debug, Default)]
pub struct IseVector {
    /// Excitation: (arousal/2) * prediction_error
    pub e: f32,
    /// Tension: somatic_tension.max(0) + interoceptive_load
    pub t: f32,
    /// Settlement: mean(coherence, trust, meta_energy)
    pub s: f32,
    /// Vulnerability: 1 - coherence
    pub v: f32,
}

impl IseVector {
    pub fn magnitude(&self) -> f32 {
        (self.e * self.e + self.t * self.t + self.s * self.s + self.v * self.v).sqrt()
    }

    pub fn is_settled(&self) -> bool {
        self.s >= 0.2 && self.s <= 0.9 && self.v < 0.6
    }

    /// Net drive pressure: rises under threat, falls under fluent prediction.
    pub fn drive_pressure(&self) -> f32 {
        (self.e + self.t - self.s + self.v).clamp(0.0, 1.0)
    }
}

#[derive(Clone, Debug)]
pub struct RitualGate {
    pub metabolic_threshold: f32,
    pub dignity_band: (f32, f32),
    pub refractory_steps: u32,
    steps_since_exertion: u32,
}

impl RitualGate {
    pub fn new() -> Self {
        Self {
            metabolic_threshold: 0.4,
            dignity_band: (0.2, 0.9),
            refractory_steps: 40,
            steps_since_exertion: u32::MAX,
        }
    }

    pub fn check(&self, metabolic_budget: f32, settlement: f32) -> (bool, &'static str) {
        if metabolic_budget < self.metabolic_threshold {
            return (false, "METABOLIC_EXHAUSTION");
        }
        let (lo, hi) = self.dignity_band;
        if settlement < lo || settlement > hi {
            return (false, "SETTLEMENT_INSTABILITY");
        }
        if self.steps_since_exertion < self.refractory_steps {
            return (false, "REFRACTORY_ACTIVE");
        }
        (true, "GATE_OPEN")
    }

    pub fn tick(&mut self) {
        self.steps_since_exertion = self.steps_since_exertion.saturating_add(1);
    }

    pub fn exert(&mut self) {
        self.steps_since_exertion = 0;
    }

    pub fn refractory_remaining(&self) -> u32 {
        self.refractory_steps.saturating_sub(self.steps_since_exertion)
    }
}

#[derive(Clone, Debug)]
pub struct EmbodiedPredictiveSubstrate {
    pub gate: RitualGate,
    pub last_ise: IseVector,
    pub gate_open: bool,
    pub gate_reason: &'static str,
}

impl EmbodiedPredictiveSubstrate {
    pub fn new() -> Self {
        Self {
            gate: RitualGate::new(),
            last_ise: IseVector::default(),
            gate_open: false,
            gate_reason: "UNINITIALIZED",
        }
    }

    pub fn compute_ise(
        arousal: f32,
        pred_err: f32,
        tension: f32,
        int_load: f32,
        coherence: f32,
        trust: f32,
        meta_energy: f32,
    ) -> IseVector {
        let e = (arousal.clamp(0.0, 2.0) / 2.0 * pred_err).clamp(0.0, 1.0);
        let t = (tension.max(0.0) + int_load).clamp(0.0, 1.0);
        let s = ((coherence + trust + meta_energy) / 3.0).clamp(0.0, 1.0);
        let v = (1.0 - coherence).clamp(0.0, 1.0);
        IseVector { e, t, s, v }
    }

    pub fn step(
        &mut self,
        arousal: f32,
        pred_err: f32,
        tension: f32,
        int_load: f32,
        coherence: f32,
        trust: f32,
        meta_energy: f32,
    ) -> (IseVector, bool, &'static str) {
        let ise = Self::compute_ise(arousal, pred_err, tension, int_load, coherence, trust, meta_energy);
        self.gate.tick();
        let (open, reason) = self.gate.check(meta_energy, ise.s);
        if open { self.gate.exert(); }
        self.last_ise = ise;
        self.gate_open = open;
        self.gate_reason = reason;
        (ise, open, reason)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ise_healthy_state() {
        let ise = EmbodiedPredictiveSubstrate::compute_ise(0.8, 0.1, 0.0, 0.1, 0.8, 0.7, 0.8);
        assert!(ise.e < 0.1);
        assert!(ise.s > 0.5);
        assert!(ise.is_settled());
    }

    #[test]
    fn ise_threat_state() {
        let ise = EmbodiedPredictiveSubstrate::compute_ise(1.8, 0.9, 1.5, 0.8, 0.2, 0.1, 0.2);
        assert!(ise.e > 0.5);
        assert!(ise.s < 0.3);
        assert!(!ise.is_settled());
    }

    #[test]
    fn gate_opens_in_healthy_state() {
        let mut eps = EmbodiedPredictiveSubstrate::new();
        let (_, open, reason) = eps.step(0.8, 0.1, 0.0, 0.1, 0.8, 0.7, 0.8);
        assert!(open, "{}", reason);
    }

    #[test]
    fn gate_closes_on_metabolic_exhaustion() {
        let mut eps = EmbodiedPredictiveSubstrate::new();
        let (_, open, reason) = eps.step(0.8, 0.1, 0.0, 0.1, 0.8, 0.7, 0.1);
        assert!(!open);
        assert_eq!(reason, "METABOLIC_EXHAUSTION");
    }

    #[test]
    fn gate_closes_on_settlement_overload() {
        let mut eps = EmbodiedPredictiveSubstrate::new();
        let (_, open, reason) = eps.step(0.8, 0.1, 0.0, 0.0, 1.0, 1.0, 1.0);
        assert!(!open);
        assert_eq!(reason, "SETTLEMENT_INSTABILITY");
    }

    #[test]
    fn gate_refractory_after_exertion() {
        let mut eps = EmbodiedPredictiveSubstrate::new();
        let (_, open1, _) = eps.step(0.8, 0.1, 0.0, 0.1, 0.8, 0.7, 0.8);
        assert!(open1);
        let (_, open2, reason2) = eps.step(0.8, 0.1, 0.0, 0.1, 0.8, 0.7, 0.8);
        assert!(!open2);
        assert_eq!(reason2, "REFRACTORY_ACTIVE");
    }

    #[test]
    fn drive_pressure_increases_under_threat() {
        let healthy = EmbodiedPredictiveSubstrate::compute_ise(0.8, 0.1, 0.0, 0.1, 0.8, 0.7, 0.8);
        let threat  = EmbodiedPredictiveSubstrate::compute_ise(1.8, 0.9, 1.5, 0.8, 0.2, 0.1, 0.2);
        assert!(threat.drive_pressure() > healthy.drive_pressure());
    }
}
