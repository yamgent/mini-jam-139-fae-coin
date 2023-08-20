use bevy::prelude::*;

pub struct ScoresPlugin;

#[derive(Resource)]
pub struct Scores {
    pub end_score: i32,
    pub best_score: i32,
}

impl Default for Scores {
    fn default() -> Self {
        Self {
            end_score: 0,
            best_score: 0,
        }
    }
}

impl Scores {
    pub fn register_score(&mut self, new_score: i32) {
        self.end_score = new_score;
        self.best_score = self.best_score.max(self.end_score);
    }
}

impl Plugin for ScoresPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Scores::default());
    }
}
