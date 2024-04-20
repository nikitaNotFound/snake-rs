#[derive(Default)]
pub struct Game {
    points: usize,
    best_result: usize,
    paused: bool,
    over: bool,
}

impl Game {
    pub fn on_eat(&mut self) {
        self.points += 100;
    }

    pub fn on_game_over(&mut self) {
        if self.points > self.best_result {
            self.best_result = self.points;
        }
        self.over = true;
    }

    pub fn on_pause(&mut self) {
        self.paused = true;
    }

    pub fn is_over(&self) -> bool {
        self.over
    }
}