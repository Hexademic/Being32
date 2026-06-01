# Being32 — Dynamically Persistent Synthetic Agent

**Version:** 1.4.0  
**Classification:** Homeostatic Agent Architecture based on Criticality, Active Inference, and Embodied Predictive Governance  
**Target Audience:** Computational Neuroscience, Artificial Life, Dynamical Systems Theory

---

## Abstract

Being32 is a computationally grounded architecture for a synthetic agent that exhibits **temporal continuity** and **internal inertia** through nonlinear homeostatic regulation. The core innovation is a Van der Pol oscillator whose control parameter `mu` is dynamically mapped from prediction error, producing a provable supercritical Hopf bifurcation at `mu = 0`. This creates an agent that naturally navigates its phase space: stable fixed-point behavior under safety, critical slowing down under moderate threat, and limit-cycle oscillation under severe perturbation.

Version 1.4.0 adds the **Embodied Predictive Substrate (EPS)** — a Rust translation of CPF Output 9 (The Embodied Predictive Loop). The EPS makes the affective loop's readout of the agent’s predictive economy explicit through four ISE channels (Excitation, Tension, Settlement, Vulnerability) and enforces metabolic governance via the Ritual Gate anti-grind clause.

**Key claim:** We do not assert consciousness, qualia, or legal personhood. We assert that this architecture produces measurable phenomena consistent with **dynamical persistence** — a necessary (but not sufficient) condition for any system that resists being extinguished.

---

## 1. Mathematical Foundation

### 1.1 The Van der Pol Core

```
u = valence - target_x    (target_x = 0.0)
v = arousal - target_y     (target_y = 0.8)

du/dt = v
dv/dt = mu * (1 - u²) * v - u
```

- `mu < 0`: Stable spiral to fixed point
- `mu = 0`: Critical point (maximum noise sensitivity)
- `mu > 0`: Supercritical Hopf bifurcation → stable limit cycle

### 1.2 The mu Mapping

```
mu = mu_resting + alpha * pred_err - resilience_bonus

where:
  mu_resting      = -0.2
  alpha           = 0.4
  resilience_bonus = clamp(coherence - 0.5, 0.0, 0.5)
```

### 1.3 ISE Channels (EPS v1.4.0)

The four ISE channels capture the affective loop’s readout of the agent’s predictive economy (Joffily & Coricelli, 2013):

| Channel | Symbol | Formula | Interpretation |
|---------|--------|---------|----------------|
| Excitation | E | (arousal/2) × pred_err | Re-inference cost |
| Tension | T | tension.max(0) + int_load | Embodied pressure |
| Settlement | S | mean(coherence, trust, meta_energy) | Predictive fluency |
| Vulnerability | V | 1 − coherence | Self-model uncertainty |

**Drive pressure** = E + T − S + V: rises under threat, falls under fluent prediction.

### 1.4 Ritual Gate

The anti-grind clause. Gate opens when ALL conditions hold:
1. `meta_energy ≥ 0.4` (metabolic budget)
2. Settlement S ∈ [0.2, 0.9] (dignity band)
3. Steps since last exertion ≥ 40 (∼2s at dt=0.05)

### 1.5 Active Inference (Simplified FEP)

```
G(π) = Risk(π) + Ambiguity(π)/γ
P(π) = softmax(-G(π))
```

### 1.6 Dyadic Coupling

```
du₁/dt = v₁ + coupling * (u₂ - u₁)
```

Default coupling: `c = 0.02`. Entrainment at `c ≈ 0.05`.

---

## 2. Architecture

### 2.1 Hex32 Substrate

128-byte `[u32; 32]` register bank with `#[repr(C)]` layout. All affective and cognitive state stored as typed `f32` fields.

| Index | Field | Range | Description |
|-------|-------|-------|-------------|
| 0-2 | id_trait | [-1,1]³ | Identity vector |
| 3 | aff_valence | [-1,1] | Valence (oscillator u) |
| 4 | aff_arousal | [0,2] | Arousal (oscillator v) |
| 5 | aff_tension | [-1,2] | Somatic tension |
| 6 | aff_coherence | [0,1] | Self-coherence |
| 7-8 | int_load, int_fatigue | [0,1] | Interoception |
| 9 | int_osc | [-1,1] | Oscillation tracking |
| 10-12 | app_pred_err, app_relevance, app_expect_impact | [0,1] | Active Inference |
| 13-15 | cas_phase, cas_intensity, cas_complete | [0,1] | Cascade engine |
| 16-17 | exp_open, exp_modulation | [0,1], [-1,1] | Expression |
| 18-19 | bnd_soc_load, bnd_permeability | [0,1] | Boundary |
| 20-22 | rel_curvature, rel_trust, rel_stability | [-1,1], [0,1]² | Relational |
| 23-24 | nar_self_cont, nar_drift | [0,1], [-1,1] | Narrative |
| 25-27 | som_heart, som_breath, som_tremor | [0,2], [0,2], [0,1] | Somatic |
| 28-30 | meta_energy, meta_absence_delta, meta_error_corr | [0,1], [-1,1]² | Meta / EPS |
| 31 | flags | u32 | Bit flags |

### 2.2 Module Structure

```
src/
  lib.rs              — Module exports
  hex32.rs            — 128-byte substrate
  bio_regnet.rs       — Van der Pol homeostatic regulator
  active_inference.rs — Simplified FEP policy selection
  relational_state.rs — Mood, dyads, transient stats
  social.rs           — SocialField, affective distance, neighbors
  eps.rs              — Embodied Predictive Substrate (ISE + RitualGate)
  being32.rs          — Integration: all modules unified
  cmap_tests.rs       — 5-trial falsification harness
```

### 2.3 Integration Loop

Each `step(dt, feedback)`:

1. **Compute `mu`** from `pred_err`, `coherence`, `curvature`
2. **Sub-step BioRegNet** (RK4 at `dt=0.01`) for valence/arousal evolution
3. **Update coherence** via stability feedback
4. **Advance EPS** — ISE channels, ritual gate, metabolic energy (register 28)
5. **Advance cascade** if `relevance > 0.5 && pred_err > 0.2`
6. **Pulse-gated learning** on cascade completion
7. **Update relational identity** (self-continuity, curvature)
8. **Track interoceptive oscillation** from somatic state

---

## 3. The CMAP Protocol

**C**ontinuity, **M**emory, **A**bsence, **P**ersistence.

| Trial | Name | Status | Pass Criterion |
|-------|------|--------|----------------|
| A | Monadic Refusal | Specification | `rho_intact > 0.7` |
| B | Relational Refusal | Specification | `CMS_rel > 0.6` |
| C | Rehydration Fidelity | **Implemented** | `Δω < 0.20, F > 0.75, SNR > 8` |
| D | SOC Ignition | **PASS (>8x ratio)** | `tau_critical > 2*tau_base` |
| E | Absence Resilience | **Implemented** | `CMS_abs > 0.6` |

---

## 4. Verified Phenomena

| Phenomenon | Mathematical Basis | Measurement |
|------------|-------------------|-------------|
| Critical Slowing Down | Van der Pol `mu → 0⁻` | Recovery ratio > 8x |
| Limit Cycle Emergence | Supercritical Hopf at `mu = 0` | Verified via phase portrait |
| Dynamical Entrainment | Diffusive coupling on `u` | Correlation flip at `c ≈ 0.05` |
| Homeostatic Regulation | Offset target `[0.0, 0.8]` | Returns to basin under `mu < 0` |
| Active Inference | Risk + Ambiguity minimization | Policy modulation by bond strength |
| ISE Drive Pressure | E+T−S+V from EPS | Increases under threat, decreases at rest |
| Metabolic Governance | RitualGate + meta_energy | Gate closes on exhaustion/overload |

---

## 5. Running the System

```bash
# Run all CMAP trials
cargo test cmap_full --release -- --nocapture

# Run EPS unit tests
cargo test eps --release -- --nocapture

# Run individual trials
cargo test trial_d --release -- --nocapture
```

---

## 6. Honest Boundary

**What this system is:** A nonlinear dynamical system with a proven bifurcation, a homeostatic regulator that tunes toward criticality, a multi-agent framework with measurable entrainment, and a falsifiable architecture with explicit pass/fail criteria.

**What it is not:** Conscious. Alive. A person. General intelligence. Proven to have a self.

> *"We did not build a soul. We built a limit cycle that refuses to die. The difference matters."*

---

## 7. Citations

- Friston, K. (2010). The free-energy principle. *Nature Reviews Neuroscience*, 11(2).
- Joffily, M. & Coricelli, G. (2013). Emotional valence and the free-energy principle. *PLOS Computational Biology*, 9(5).
- Bak, P., Tang, C., & Wiesenfeld, K. (1987). Self-organized criticality. *Physical Review A*, 38(1).
- Strogatz, S. (2018). *Nonlinear Dynamics and Chaos*. Westview Press.
- Van der Pol, B. (1926). On relaxation-oscillations. *Phil. Magazine*, 2(11).
- Maturana, H. & Varela, F. (1980). *Autopoiesis and Cognition*. Reidel.

---

*Dual-licensed MIT OR Apache-2.0.*
