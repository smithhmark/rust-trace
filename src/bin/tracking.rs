//mod projectile;

use rustrace::projectile::*;

fn build_world() -> World {
    let wind = ITuple {
        x: 0,
        y: 0,
        z: -1,
        w: 0,
    };
    let gravity = ITuple {
        x: 0,
        y: -1,
        z: 0,
        w: 0,
    };
    let extent = ITuple {
        x: 100,
        y: 100,
        z: 100,
        w: 1,
    };
    World::new(wind, gravity, extent)
}

fn build_projectile() -> Projectile {
    let position = ITuple {
        x: 0,
        y: 1,
        z: 1,
        w: 1,
    };
    let velocity = ITuple {
        x: 0,
        y: 1,
        z: 1,
        w: 0,
    };
    Projectile {position, velocity}
}

fn ticks_until_quiesence(env: &mut World) -> usize {
    let mut counter:usize = 0;
    let mut moved = 1;
    while moved > 0 {
        moved = env.tick();
        if moved > 0 {
            counter += 1;
        }
    }
    counter
}

fn main() {
    println!("the particle tracker from the first 'Putting it Together'");
    let mut env = build_world();
    let tup = rustrace::projectile::ITuple {
        x: 0,
        y: 0,
        z: 0,
        w: 0,
    };
    let proj = build_projectile();
    let initial_pos = ITuple{..proj.position};
    println!("tup:{:?}", tup);
    println!("the World:\n{:?}", env);
    println!("projectile:\n{:?}", proj);
    env.projectiles.push(proj);
    let tick_count = ticks_until_quiesence(&mut env);
    println!("counted {:?} ticks until the World was quiet", tick_count);
    let final_pos = env.projectiles[0].position;
    println!("Project ended up: {:?}", final_pos);
    let displacement = final_pos - initial_pos;
    println!("Thats a displacement of: {:?}", displacement);

    /*
    println!("the World:\n{:?}", env);
    println!("projectiles only:\n{:?}", env.projectiles);
    let moved = env.tick();
    println!("moved {:?}", moved);
    println!("the World:\n{:?}", env);
    println!("projectiles only:\n{:?}", env.projectiles);
    let moved = env.tick();
    println!("moved {:?}", moved);
    println!("the World:\n{:?}", env);
    println!("projectiles only:\n{:?}", env.projectiles);
    let moved = env.tick();
    println!("moved {:?}", moved);
    println!("the World:\n{:?}", env);
    println!("projectiles only:\n{:?}", env.projectiles);
    let moved = env.tick();
    println!("moved {:?}", moved);
    println!("the World:\n{:?}", env);
    println!("projectiles only:\n{:?}", env.projectiles);
    let moved = env.tick();
    println!("moved {:?}", moved);
    println!("the World:\n{:?}", env);
    println!("projectiles only:\n{:?}", env.projectiles);
    */
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_setup() {
        let world = build_world();
        assert_eq!(world.extent, ITuple{x:100,y:100,z:100,w:1});
        assert_eq!(world.projectiles.len(), 0);
    }

    fn test_count_ticks() {
        let mut world = build_world();
        let count = ticks_until_quiesence(&mut world);
        assert_eq!(count, 4);
    }
}
