use serde::{Deserialize, Serialize};
#[derive(Clone, Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub enum InvasionRandomPool {
    Level1Easy,
}

#[derive(Clone, Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub enum InvasionFormationPool {
    Level1Easy,
    Level1Medium,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum PhaseType {
    InvasionRandom(InvasionRandomPool),
    InvasionFormation(InvasionFormationPool),
    Rest,
    Boss,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum BossType {
    Repeater,
    None,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Phase {
    pub phase_type: PhaseType,
    pub boss_type: BossType,
    pub length: usize,
    pub boss_spawned: bool,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct PhaseManagerResource {
    pub phase_map: Vec<Phase>,
    pub phase_idx: usize,
    pub current_tick: usize,
    pub tick_timer: f32,
    pub tick_length: f32,
}

impl PhaseManagerResource {
    pub fn update(&mut self, dt: f32) {
        if self.tick_timer > 0.0 {
            self.tick_timer -= dt;
        } else {
            println!(
                "phase index: {}\tcurrent_tick: {}",
                self.phase_idx, self.current_tick
            );
            self.tick_timer = self.tick_length;
            self.current_tick += 1;

            // check if the phase is over
            if self.current_tick >= self.phase_map[self.phase_idx].length {
                if self.phase_idx == self.phase_map.len() - 1 {
                    // level is over
                    // TODO: end level
                    self.phase_idx -= 1; // repeat last phase for now
                }
                self.phase_idx += 1;
                self.current_tick = 0;
            }
        }
    }
}
