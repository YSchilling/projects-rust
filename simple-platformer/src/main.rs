use macroquad::prelude::*;

const PLAYER_SIZE: Vec2 = Vec2::new(32., 32.);
const PLAYER_MAX_SPEED: f32 = 10.;

#[derive(PartialEq, Debug)]
enum MovementState {
    Accelerating,
    Decelerating,
    Standing
}

struct Player {
    rect: Rect,
    velocity: Vec2,
    movement_state: MovementState,
    is_flying: bool
}

impl Player {
    pub fn new() -> Self {
        let rect = Rect::new(
            screen_width() * 0.5 - PLAYER_SIZE.x * 0.5,
            screen_height() - 100.,
            PLAYER_SIZE.x,
            PLAYER_SIZE.y
        );
        let velocity = Vec2::new(0., 0.);
        let movement_state = MovementState::Standing;
        let is_flying = false;

        Self {
            rect,
            velocity,
            movement_state,
            is_flying
        }
    }

    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, WHITE);
    }

    pub fn update(&mut self) {
        println!("{:?}", self.movement_state);
        self.check_keys_pressed();
        self.apply_velocity();
    }

    fn check_keys_pressed(&mut self) {
        if self.rect.y >= screen_height() - 100. {
            self.is_flying = false;
            self.velocity.y = 0.;
        }

        if is_key_down(KeyCode::Space) && !self.is_flying {
            self.is_flying = true;
            self.velocity.y -= 10.;
        }

        if self.is_flying {
            self.velocity.y += 0.5;
        }

        if is_key_down(KeyCode::Left) && self.velocity.x > -PLAYER_MAX_SPEED{
            self.velocity.x -= 1.;
            self.movement_state = MovementState::Accelerating;
            return;
        }
        if is_key_down(KeyCode::Right) && self.velocity.x < PLAYER_MAX_SPEED{
            self.velocity.x += 1.;
            self.movement_state = MovementState::Accelerating;
            return;
        }

        if self.velocity.x == 0. {
            self.movement_state = MovementState::Standing;
            return;
        }

        self.movement_state = MovementState::Decelerating;
        if self.velocity.x.is_sign_positive() {
            self.velocity.x -= 0.5;
            return;
        }

        if self.velocity.x.is_sign_negative() {
            self.velocity.x += 0.5;
            return;
        }
    }

    fn apply_velocity(&mut self) {
        self.rect.x += self.velocity.x;
        self.rect.y += self.velocity.y;
    }
}

#[macroquad::main("Simple Platformer")]
async fn main() {
    let mut player = Player::new();

    loop {
        player.update();

        clear_background(BLACK);

        player.draw();

        next_frame().await
    }
}