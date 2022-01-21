use super::tuple::Tuple;

pub type ITuple = Tuple<i32>;

pub struct Projectile {
    pub position: ITuple,
    pub velocity: ITuple,
}

pub struct World {
    projectiles: Vec<Projectile>,
    gravity: ITuple,
    wind: ITuple,
    extent: ITuple,
}

fn within(pos: &ITuple, region: &ITuple) -> bool {
    let mut so_far: bool;
    if pos.x < region.x && pos.x >= 0 {
        so_far = true;
    } else {
        so_far = false;
    }
    if so_far && pos.y >= 0 && pos.y < region.y {
        so_far = true;
    } else {
        so_far = false;
    }
    if so_far && pos.z >= 0 && pos.z < region.z {
        so_far = true;
    } else {
        so_far = false;
    }
    so_far
}

impl World {
    pub fn tick(&mut self) -> usize {
        let mut updated: usize = 0;
        for proj in self.projectiles.iter_mut() {
            if within(&proj.position, &self.extent) {
                let new_pos = proj.position + proj.velocity;
                println!("{:?}", new_pos);
                let new_vel = proj.velocity + self.wind + self.gravity;
                println!("{:?}", new_vel);
                proj.position = new_pos;
                proj.velocity = new_vel;
                updated += 1;
            }
        }
        updated
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_within() {
        let extent: ITuple = Tuple::point(10, 10, 10);
        let pos0: ITuple = Tuple::point(0, 0, 0);
        let pos1: ITuple = Tuple::point(5, 5, 5);
        let pos2: ITuple = Tuple::point(5, 5, 10);
        assert!(within(&pos0, &extent));
        assert!(within(&pos1, &extent));
        assert!(!within(&pos2, &extent));
    }

    #[test]
    fn test_tick() {
        let proj = Projectile {
            position: Tuple::point(0, 1, 0),
            velocity: Tuple::vector(0, 0, 0),
        };
        let mut env = World {
            projectiles: vec![proj],
            gravity: Tuple::vector(0, -1, 0),
            wind: Tuple::vector(0, 0, 0),
            extent: Tuple::point(10, 10, 10),
        };
        assert!(within(&env.projectiles[0].position, &env.extent));
        assert_eq!(env.projectiles[0].position.y, 1);
        assert_eq!(env.projectiles[0].velocity.y, 0);
        let updated = env.tick();
        assert_eq!(env.projectiles[0].position.y, 1);
        assert_eq!(env.projectiles[0].velocity.y, -1);
        assert_eq!(updated, 1);
        assert!(within(&env.projectiles[0].position, &env.extent));
        let updated = env.tick();
        assert_eq!(updated, 1);
        assert_eq!(env.projectiles[0].position.y, 0);
        assert_eq!(env.projectiles[0].velocity.y, -2);
        assert!(within(&env.projectiles[0].position, &env.extent));
        let updated = env.tick();
        assert_eq!(updated, 1);
        assert!(!within(&env.projectiles[0].position, &env.extent));
    }
}
