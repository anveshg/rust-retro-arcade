use crate::pong::ball::Ball;

pub struct Paddle {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub speed: f32,
}

impl Paddle {
    pub fn new(x: f32, y: f32) -> Self {
        Paddle {
            x,
            y,
            w: 12.0,
            h: 80.0,
            speed: 320.0,
        }
    }

    pub fn center_y(&self) -> f32 {
        self.y + self.h / 2.0
    }

    pub fn move_by(&mut self, dy: f32, top: f32, bottom: f32) {
        self.y = (self.y + dy).clamp(top, bottom - self.h);
    }

    /// Move toward `target_y`, capped by `speed * dt`, clamped to the field.
    pub fn track(&mut self, target_y: f32, dt: f32, top: f32, bottom: f32) {
        let delta = target_y - self.center_y();
        let max = self.speed * dt;
        let step = delta.clamp(-max, max);
        self.y = (self.y + step).clamp(top, bottom - self.h);
    }

    pub fn hits(&self, ball: &Ball) -> bool {
        ball.pos.x - ball.radius <= self.x + self.w
            && ball.pos.x + ball.radius >= self.x
            && ball.pos.y >= self.y
            && ball.pos.y <= self.y + self.h
    }

    /// Hit position relative to paddle center, normalized to [-1, 1].
    pub fn bounce_offset(&self, ball: &Ball) -> f32 {
        ((ball.pos.y - self.center_y()) / (self.h / 2.0)).clamp(-1.0, 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_by_clamps_to_field() {
        let mut p = Paddle::new(10.0, 10.0);
        p.move_by(-100.0, 0.0, 480.0);
        assert_eq!(p.y, 0.0);
    }

    #[test]
    fn track_is_capped_by_speed() {
        let mut p = Paddle::new(10.0, 200.0);
        // target far above; dt small so movement is capped to speed*dt = 32.
        p.track(0.0, 0.1, 0.0, 480.0);
        assert_eq!(p.y, 200.0 - 32.0);
    }

    #[test]
    fn bounce_offset_is_zero_at_center() {
        let p = Paddle::new(10.0, 100.0);
        let ball = Ball::new(20.0, p.center_y(), 0.0, 0.0);
        assert!(p.bounce_offset(&ball).abs() < 0.001);
    }

    #[test]
    fn bounce_offset_is_negative_above_center() {
        let p = Paddle::new(10.0, 100.0);
        let ball = Ball::new(20.0, p.y, 0.0, 0.0); // top edge
        assert!(p.bounce_offset(&ball) < 0.0);
    }

    #[test]
    fn hits_detects_overlap() {
        let p = Paddle::new(10.0, 100.0);
        let ball = Ball::new(18.0, 140.0, 0.0, 0.0);
        assert!(p.hits(&ball));
    }

    #[test]
    fn misses_when_ball_is_past_paddle_vertically() {
        let p = Paddle::new(10.0, 100.0);
        let ball = Ball::new(18.0, 300.0, 0.0, 0.0);
        assert!(!p.hits(&ball));
    }
}
