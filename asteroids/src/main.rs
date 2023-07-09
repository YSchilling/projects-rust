use macroquad::prelude::*;

const PLAYER_SIZE: Vec2 = Vec2::from_array([24., 24.]);
const PLAYER_SPEED: f32 = 20.;
const PLAYER_SPEED_CAP: f32 = 7.;
const PLAYER_SPEED_ROTATION: f32 = 6.;
const PLAYER_VELOCITY_REDUCTION: f32 = 10.;

const BULLET_SPEED: f32 = 800.;
const BULLET_COOLDOWN: f32 = 5.;

struct Bullet {
    rect: Rect,
    direction: Vec2,
}

impl Bullet {
    pub fn new(x: f32, y: f32, direction: Vec2) -> Self {
        Self {
            rect: Rect::new(x, y, 2., 2.),
            direction,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.update_pos(dt);
    }

    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, WHITE)
    }

    fn update_pos(&mut self, dt: f32) {
        self.rect.x += self.direction.x * dt * BULLET_SPEED;
        self.rect.y += self.direction.y * dt * BULLET_SPEED;
    }
}

struct Player {
    rect: Rect,
    view_direction: Vec2,
    velocity: Vec2,
    color: Color,
    bullets: Vec<Bullet>,
    bullet_timer: f32,
}

impl Player {
    pub fn new() -> Self {
        let rect = Rect::new(
            screen_width() * 0.5 - PLAYER_SIZE.x * 0.5,
            screen_height() - 100.,
            PLAYER_SIZE.x,
            PLAYER_SIZE.y,
        );
        let view_direction = vec2(0., -1.).normalize();
        let velocity = vec2(0., 0.);
        Self {
            rect,
            view_direction,
            velocity,
            color: WHITE,
            bullets: Vec::new(),
            bullet_timer: 0.,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.update_view_direction(dt);
        self.update_velocity(dt);
        self.apply_velocity();
        self.update_bullet_timer(dt);

        if is_key_down(KeyCode::Space) && self.bullet_timer <= 0. {
            self.shoot();
            self.bullet_timer = BULLET_COOLDOWN;
        }

        for laser in self.bullets.iter_mut() {
            laser.update(get_frame_time());
        }
    }

    pub fn draw(&self) {
        let triangle = self.get_triangle();
        draw_triangle_lines(triangle.0, triangle.1, triangle.2, 1., self.color);

        for laser in self.bullets.iter() {
            laser.draw();
        }
    }

    fn update_bullet_timer(&mut self, dt: f32) {
        if self.bullet_timer > 0. {
            self.bullet_timer -= 10. * dt;
        }
    }

    fn shoot(&mut self) {
        let x = self.rect.x + self.view_direction.x * PLAYER_SIZE.x;
        let y = self.rect.y + self.view_direction.y * PLAYER_SIZE.y;
        self.bullets.push(Bullet::new(x, y, self.view_direction));
    }

    fn apply_velocity(&mut self) {
        // check for screen borders
        let new_x = self.rect.x + self.velocity.x;
        let new_y = self.rect.y + self.velocity.y;
        if new_x > screen_width() || new_x < 0. {
            self.velocity.x = 0.;
        } else if new_y > screen_height() || new_y < 0. {
            self.velocity.y = 0.;
        }

        // move
        self.rect.x += self.velocity.x;
        self.rect.y += self.velocity.y;
    }

    fn update_velocity(&mut self, dt: f32) {
        // increase if pressed
        if is_key_down(KeyCode::Up) && self.velocity.length() < PLAYER_SPEED_CAP {
            self.velocity.x += self.view_direction.x * dt * PLAYER_SPEED;
            self.velocity.y += self.view_direction.y * dt * PLAYER_SPEED;
        }
        // decrease if not
        else {
            println!("{}", self.velocity.length());
            if self.velocity.length() < 0.1 {
                self.velocity.x = 0.;
                self.velocity.y = 0.;
            } else {
                self.velocity.x = self.reduce_value(self.velocity.x, dt);
                self.velocity.y = self.reduce_value(self.velocity.y, dt);
            }
        }
    }

    fn reduce_value(&self, value: f32, dt: f32) -> f32 {
        let mut new_value = 0.;
        if value > 0. {
            new_value = value - dt * PLAYER_VELOCITY_REDUCTION;
        }
        if value < 0. {
            new_value = value + dt * PLAYER_VELOCITY_REDUCTION;
        }
        return new_value;
    }

    fn update_view_direction(&mut self, dt: f32) {
        if is_key_down(KeyCode::Left) {
            let angle = -(dt * PLAYER_SPEED_ROTATION);
            self.view_direction = Vec2::from_angle(angle)
                .normalize()
                .rotate(self.view_direction);
        }
        if is_key_down(KeyCode::Right) {
            let angle = dt * PLAYER_SPEED_ROTATION;
            self.view_direction = Vec2::from_angle(angle)
                .normalize()
                .rotate(self.view_direction);
        }
    }

    fn get_triangle(&self) -> (Vec2, Vec2, Vec2) {
        let vec_left_rotate = Vec2::from_angle(90.)
            .normalize()
            .rotate(self.view_direction);
        let vec_right_rotate = Vec2::from_angle(-90.)
            .normalize()
            .rotate(self.view_direction);
        (
            vec2(
                self.rect.x + self.view_direction.x * PLAYER_SIZE.y,
                self.rect.y + self.view_direction.y * PLAYER_SIZE.y,
            ),
            vec2(
                self.rect.x + vec_left_rotate.x * PLAYER_SIZE.x / 2.,
                self.rect.y + vec_left_rotate.y * PLAYER_SIZE.x / 2.,
            ),
            vec2(
                self.rect.x + vec_right_rotate.x * PLAYER_SIZE.x / 2.,
                self.rect.y + vec_right_rotate.y * PLAYER_SIZE.x / 2.,
            ),
        )
    }
}

#[macroquad::main("Asteroids")]
async fn main() {
    let mut player = Player::new();
    let mut lasers: Vec<Bullet> = Vec::new();

    loop {
        //logic
        player.update(get_frame_time());

        // drawing
        clear_background(BLACK);

        player.draw();

        next_frame().await
    }
}
