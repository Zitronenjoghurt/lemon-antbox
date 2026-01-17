use crate::simulation::pheromones::PheromoneType;
use crate::simulation::settings::AntSettings;

#[derive(Debug, Default, Clone, Copy)]
pub struct AntSenses {
    pub left: f32,
    pub forward: f32,
    pub right: f32,
    pub food: u8,
    pub at_home: bool,
}

impl AntSenses {
    pub fn desired_turn(&self, turn_angle: f32) -> f32 {
        if self.forward == 0.0 && self.left == 0.0 && self.right == 0.0 {
            return 0.0;
        }

        if self.forward > self.left && self.forward > self.right {
            0.0
        } else if self.left > self.right {
            -turn_angle
        } else {
            turn_angle
        }
    }
}

pub struct AntAction {
    pub turn: f32,
    pub deposit_pheromone_strength: f32,
    pub deposit_pheromone: Option<PheromoneType>,
    pub pickup_food: bool,
    pub deposit_food: bool,
}

pub struct AntFeedback {
    pub turn: f32,
    pub picked_up_food: bool,
    pub deposited_food: bool,
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum AntMode {
    #[default]
    Exploring,
    // Following pheromones
    FoodToHome,
    // Spiral search
    SearchingHome,
}

#[derive(Debug, Clone, Default)]
pub struct Ant {
    pub x: f32,
    pub y: f32,
    pub tribe: u8,
    pub angle: f32,
    pub has_food: bool,
    pub mode: AntMode,
    // ToDo: implement reservoir, dead reckoning and spiral search
    pub pheromone_reservoir: f32,
    pub home: Option<(u16, u16)>,
    pub spiral_radius: f32,
}

impl Ant {
    pub fn color_rgba(&self, inspected: bool) -> [u8; 4] {
        if inspected {
            return [255, 255, 255, 255];
        }

        if self.has_food {
            [201, 160, 56, 255]
        } else {
            [165, 102, 47, 255]
        }
    }
}

// Sense and act
impl Ant {
    pub fn sense(&self, senses: AntSenses, settings: &AntSettings) -> AntAction {
        let turn = if self.mode == AntMode::SearchingHome {
            self.spiral_turn(settings)
        } else if (self.mode == AntMode::Exploring && senses.food > 0)
            || (self.mode == AntMode::FoodToHome && senses.at_home)
        {
            self.angle + std::f32::consts::PI
        } else {
            senses.desired_turn(settings.turn_angle)
        } + (fastrand::f32() - 0.5) * settings.wobble_strength;

        let pheromone_strength = if self.mode == AntMode::FoodToHome {
            settings.pheromone_strength.max(senses.food as f32)
        } else {
            settings.pheromone_strength
        };

        AntAction {
            turn,
            deposit_pheromone_strength: pheromone_strength,
            deposit_pheromone: self.excreted_pheromone(),
            pickup_food: senses.food > 0,
            deposit_food: senses.at_home,
        }
    }

    pub fn desired_pheromone(&self) -> Option<PheromoneType> {
        match self.mode {
            AntMode::Exploring => Some(PheromoneType::Food),
            AntMode::FoodToHome => Some(PheromoneType::Home),
            AntMode::SearchingHome => None,
        }
    }

    pub fn excreted_pheromone(&self) -> Option<PheromoneType> {
        match self.mode {
            AntMode::Exploring => None,
            AntMode::FoodToHome => Some(PheromoneType::Food),
            AntMode::SearchingHome => None,
        }
    }

    fn spiral_turn(&self, settings: &AntSettings) -> f32 {
        if self.spiral_radius <= 0.0 {
            0.0
        } else {
            settings.speed / self.spiral_radius
        }
    }
}

// Apply action
impl Ant {
    pub fn update(&mut self, feedback: &AntFeedback, settings: &AntSettings) {
        self.angle += feedback.turn;
        self.x += self.angle.cos() * settings.speed;
        self.y += self.angle.sin() * settings.speed;

        if feedback.picked_up_food {
            self.has_food = true;
            self.mode = AntMode::FoodToHome;
        } else if feedback.deposited_food {
            self.has_food = false;
            self.mode = AntMode::Exploring;
        }
    }
}
