use macroquad::math::Vec2;

pub struct Ball {
    pub pos: Vec2,
    pub vel: Vec2,
    pub radius: f32,
}

impl Ball {
    pub fn new(cx: f32, cy: f32, vx: f32, vy: f32) -> Self {
        Ball {
            pos: Vec2::new(cx, cy),
            vel: Vec2::new(vx, vy),
            radius: 8.0,
        }
    }

    pub fn step(&mut self, dt: f32) {
        self.pos += self.vel * dt;
    }

    /// Reflect off the top/bottom walls. Returns true if a wall was hit.
    pub fn bounce_walls(&mut self, top: f32, bottom: f32) -> bool {
        if self.pos.y - self.radius <= top && self.vel.y < 0.0 {
            self.pos.y = top + self.radius;
            self.vel.y = -self.vel.y;
            return true;
        }
        if self.pos.y + self.radius >= bottom && self.vel.y > 0.0 {
            self.pos.y = bottom - self.radius;
            self.vel.y = -self.vel.y;
            return true;
        }
        false
    }
}

/// New velocity after a paddle hit.
/// `offset` in [-1,1] (paddle-relative hit position), `dir` is +1 (rightward) or -1 (leftward).
pub fn reflect(speed: f32, offset: f32, dir: f32) -> Vec2 {
    let max_angle = std::f32::consts::FRAC_PI_3; // 60 degrees
    let angle = offset.clamp(-1.0, 1.0) * max_angle;
    Vec2::new(dir * speed * angle.cos(), speed * angle.sin())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step_advances_by_velocity_times_dt() {
        let mut b = Ball::new(100.0, 100.0, 50.0, 0.0);
        b.step(1.0);
        assert_eq!(b.pos.x, 150.0);
        assert_eq!(b.pos.y, 100.0);
    }

    #[test]
    fn bounces_off_top_wall() {
        let mut b = Ball::new(100.0, 2.0, 0.0, -30.0);
        let hit = b.bounce_walls(0.0, 480.0);
        assert!(hit);
        assert!(b.vel.y > 0.0);
        assert_eq!(b.pos.y, b.radius);
    }

    #[test]
    fn no_bounce_when_moving_away_from_wall() {
        let mut b = Ball::new(100.0, 2.0, 0.0, 30.0);
        assert!(!b.bounce_walls(0.0, 480.0));
    }

    #[test]
    fn reflect_center_goes_straight() {
        let v = reflect(300.0, 0.0, 1.0);
        assert!((v.x - 300.0).abs() < 0.001);
        assert!(v.y.abs() < 0.001);
    }

    #[test]
    fn reflect_direction_sign_is_respected() {
        let v = reflect(300.0, 0.0, -1.0);
        assert!(v.x < 0.0);
    }

    #[test]
    fn reflect_edge_hit_adds_vertical_speed() {
        let v = reflect(300.0, 1.0, 1.0);
        assert!(v.y > 0.0);
        assert!(v.x > 0.0);
    }
}
