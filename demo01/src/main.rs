use raylib::prelude::*;

// ── Constants ────────────────────────────────────────────────────────────────

const SCREEN_WIDTH: i32 = 1024;
const SCREEN_HEIGHT: i32 = 768;
const PLAYER_SPEED: f32 = 200.0;
const PLAYER_SIZE: f32 = 20.0;
const PLAYER_MAX_HP: i32 = 5;
const BULLET_SPEED: f32 = 500.0;
const BULLET_RADIUS: f32 = 4.0;
const BULLET_LIFETIME: f32 = 1.5;
const SHOOT_COOLDOWN: f32 = 0.15;
const ENEMY_SPEED: f32 = 80.0;
const ENEMY_SIZE: f32 = 18.0;
const ENEMY_HP: i32 = 2;
const ENEMY_SPAWN_INTERVAL: f32 = 1.5;
const ENEMY_DAMAGE_COOLDOWN: f32 = 0.5;

// ── Game Objects ──────────────────────────────────────────────────────────────

#[derive(Debug)]
struct Player {
    pos: Vector2,
    hp: i32,
    shoot_timer: f32,
}

#[derive(Debug)]
struct Bullet {
    pos: Vector2,
    vel: Vector2,
    lifetime: f32,
}

#[derive(Debug)]
struct Enemy {
    pos: Vector2,
    hp: i32,
    hit_timer: f32, // flash white after being hit
}

#[derive(Debug)]
struct Particle {
    pos: Vector2,
    vel: Vector2,
    lifetime: f32,
    color: Color,
}

// ── Entry Point ──────────────────────────────────────────────────────────────

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("demo01 — top-down survival shooter")
        .vsync()
        .build();

    rl.set_target_fps(60);
    rl.hide_cursor(); // hide system cursor, we'll draw a crosshair

    // ── Game State ────────────────────────────────────────────────────────
    let mut player = Player {
        pos: Vector2::new(SCREEN_WIDTH as f32 / 2.0, SCREEN_HEIGHT as f32 / 2.0),
        hp: PLAYER_MAX_HP,
        shoot_timer: 0.0,
    };

    let mut bullets: Vec<Bullet> = Vec::new();
    let mut enemies: Vec<Enemy> = Vec::new();
    let mut particles: Vec<Particle> = Vec::new();
    let mut score: i32 = 0;
    let mut wave: i32 = 1;
    let mut spawn_timer: f32 = 0.0;
    let mut enemies_killed: i32 = 0;
    let mut enemy_dmg_timer: f32 = 0.0;
    let mut game_over: bool = false;
    let mut paused: bool = false;

    // ── Main Game Loop ────────────────────────────────────────────────────
    while !rl.window_should_close() {
        let dt = rl.get_frame_time();

        // ── Toggle pause ──────────────────────────────────────────────
        if rl.is_key_pressed(KeyboardKey::KEY_P) {
            paused = !paused;
        }
        if paused {
            let mut d = rl.begin_drawing(&thread);
            draw_frame(&mut d, &player, &bullets, &enemies, &particles, score, wave, game_over);
            draw_centered_text(&mut d, "PAUSED — Press P to resume", 30, Color::YELLOW);
            continue;
        }

        // ══════════════════════════════════════════════════════════════════
        //  UPDATE PHASE
        // ══════════════════════════════════════════════════════════════════
        if !game_over {
            update_player(&mut rl, &mut player, dt);
            update_shooting(&mut rl, &mut player, &mut bullets);
            update_bullets(&mut bullets, &mut particles, dt);
            update_enemy_spawning(&mut enemies, dt, &mut spawn_timer, wave);
            update_enemies(&mut enemies, &player, dt);

            // Collisions: bullets → enemies
            update_collisions(&mut bullets, &mut enemies, &mut particles, &mut score, &mut enemies_killed);

            // Collisions: enemies → player (returns true if player took damage this frame)
            if check_player_damage(&enemies, &player, dt, &mut enemy_dmg_timer, &mut particles) {
                player.hp -= 1;
            }

            update_particles(&mut particles, dt);

            // Wave progression
            let wave_kill_target = 5 + wave * 3;
            if enemies_killed >= wave_kill_target {
                wave += 1;
                enemies_killed = 0;
                player.hp = (player.hp + 1).min(PLAYER_MAX_HP);
            }
            if player.hp <= 0 {
                game_over = true;
            }
        } else {
            if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
                player = Player {
                    pos: Vector2::new(SCREEN_WIDTH as f32 / 2.0, SCREEN_HEIGHT as f32 / 2.0),
                    hp: PLAYER_MAX_HP,
                    shoot_timer: 0.0,
                };
                bullets.clear();
                enemies.clear();
                particles.clear();
                score = 0;
                wave = 1;
                spawn_timer = 0.0;
                enemies_killed = 0;
                enemy_dmg_timer = 0.0;
                game_over = false;
            }
        }

        // ══════════════════════════════════════════════════════════════════
        //  RENDER PHASE
        // ══════════════════════════════════════════════════════════════════
        let mut d = rl.begin_drawing(&thread);
        draw_frame(&mut d, &player, &bullets, &enemies, &particles, score, wave, game_over);
    }
}

// ── Update: Player ───────────────────────────────────────────────────────────

fn update_player(rl: &mut RaylibHandle, player: &mut Player, dt: f32) {
    let mut dx = 0.0;
    let mut dy = 0.0;
    if rl.is_key_down(KeyboardKey::KEY_W) || rl.is_key_down(KeyboardKey::KEY_UP) { dy -= 1.0; }
    if rl.is_key_down(KeyboardKey::KEY_S) || rl.is_key_down(KeyboardKey::KEY_DOWN) { dy += 1.0; }
    if rl.is_key_down(KeyboardKey::KEY_A) || rl.is_key_down(KeyboardKey::KEY_LEFT) { dx -= 1.0; }
    if rl.is_key_down(KeyboardKey::KEY_D) || rl.is_key_down(KeyboardKey::KEY_RIGHT) { dx += 1.0; }

    if dx != 0.0 && dy != 0.0 {
        let inv = std::f32::consts::FRAC_1_SQRT_2;
        dx *= inv;
        dy *= inv;
    }

    player.pos.x = (player.pos.x + dx * PLAYER_SPEED * dt)
        .clamp(PLAYER_SIZE, SCREEN_WIDTH as f32 - PLAYER_SIZE);
    player.pos.y = (player.pos.y + dy * PLAYER_SPEED * dt)
        .clamp(PLAYER_SIZE, SCREEN_HEIGHT as f32 - PLAYER_SIZE);
}

// ── Update: Shooting ─────────────────────────────────────────────────────────

fn update_shooting(rl: &mut RaylibHandle, player: &mut Player, bullets: &mut Vec<Bullet>) {
    player.shoot_timer -= rl.get_frame_time();
    if rl.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON) && player.shoot_timer <= 0.0 {
        let mouse = rl.get_mouse_position();
        let dir = (mouse - player.pos).normalized();
        bullets.push(Bullet {
            pos: player.pos + dir * (PLAYER_SIZE + 4.0),
            vel: dir * BULLET_SPEED,
            lifetime: BULLET_LIFETIME,
        });
        player.shoot_timer = SHOOT_COOLDOWN;
    }
}

// ── Update: Bullets ──────────────────────────────────────────────────────────

fn update_bullets(bullets: &mut Vec<Bullet>, particles: &mut Vec<Particle>, dt: f32) {
    let screen = Rectangle::new(0.0, 0.0, SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32);
    bullets.retain_mut(|b| {
        b.pos += b.vel * dt;
        b.lifetime -= dt;
        if b.lifetime <= 0.0 || !screen.check_collision_point_rec(b.pos) {
            for _ in 0..3 {
                particles.push(Particle {
                    pos: b.pos,
                    vel: Vector2::new(rand_range(-30.0, 30.0), rand_range(-30.0, 30.0)),
                    lifetime: 0.2,
                    color: Color::RAYWHITE,
                });
            }
            false
        } else {
            true
        }
    });
}

// ── Update: Enemy Spawning ───────────────────────────────────────────────────

fn update_enemy_spawning(enemies: &mut Vec<Enemy>, dt: f32, spawn_timer: &mut f32, wave: i32) {
    *spawn_timer -= dt;
    if *spawn_timer <= 0.0 {
        let count = (1 + wave / 2).min(8);
        for _ in 0..count {
            let (x, y) = random_edge_pos();
            enemies.push(Enemy {
                pos: Vector2::new(x, y),
                hp: ENEMY_HP + (wave / 3),
                hit_timer: 0.0,
            });
        }
        *spawn_timer = ENEMY_SPAWN_INTERVAL;
    }
}

fn random_edge_pos() -> (f32, f32) {
    let margin = 30.0;
    match fastrand::i32(0..4) {
        0 => (fastrand::f32() * SCREEN_WIDTH as f32, -margin),
        1 => (fastrand::f32() * SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32 + margin),
        2 => (-margin, fastrand::f32() * SCREEN_HEIGHT as f32),
        _ => (SCREEN_WIDTH as f32 + margin, fastrand::f32() * SCREEN_HEIGHT as f32),
    }
}

// ── Update: Enemies AI ───────────────────────────────────────────────────────

fn update_enemies(enemies: &mut Vec<Enemy>, player: &Player, dt: f32) {
    for e in enemies.iter_mut() {
        let dir = (player.pos - e.pos).normalized();
        e.pos += dir * ENEMY_SPEED * dt;
        e.hit_timer = (e.hit_timer - dt).max(0.0);
    }
}

// ── Update: Bullet-Enemy Collisions ──────────────────────────────────────────

fn update_collisions(
    bullets: &mut Vec<Bullet>,
    enemies: &mut Vec<Enemy>,
    particles: &mut Vec<Particle>,
    score: &mut i32,
    enemies_killed: &mut i32,
) {
    bullets.retain(|b| {
        let mut hit = false;
        for e in enemies.iter_mut() {
            if b.pos.distance_to(e.pos) < BULLET_RADIUS + ENEMY_SIZE {
                e.hp -= 1;
                e.hit_timer = 0.1;
                hit = true;
                for _ in 0..5 {
                    particles.push(Particle {
                        pos: b.pos,
                        vel: Vector2::new(rand_range(-100.0, 100.0), rand_range(-100.0, 100.0)),
                        lifetime: 0.3,
                        color: Color::RED,
                    });
                }
                break;
            }
        }
        !hit
    });

    enemies.retain(|e| {
        let alive = e.hp > 0;
        if !alive {
            *score += 10;
            *enemies_killed += 1;
            for _ in 0..15 {
                particles.push(Particle {
                    pos: e.pos,
                    vel: Vector2::new(rand_range(-150.0, 150.0), rand_range(-150.0, 150.0)),
                    lifetime: 0.5,
                    color: Color::ORANGE,
                });
            }
        }
        alive
    });
}

// ── Update: Enemy → Player Damage (returns true if player took damage) ───────

fn check_player_damage(
    enemies: &[Enemy],
    player: &Player,
    dt: f32,
    dmg_timer: &mut f32,
    particles: &mut Vec<Particle>,
) -> bool {
    *dmg_timer -= dt;
    if *dmg_timer > 0.0 {
        return false;
    }
    for e in enemies {
        if player.pos.distance_to(e.pos) < PLAYER_SIZE + ENEMY_SIZE - 2.0 {
            *dmg_timer = ENEMY_DAMAGE_COOLDOWN;
            for _ in 0..8 {
                particles.push(Particle {
                    pos: player.pos + Vector2::new(rand_range(-10.0, 10.0), rand_range(-10.0, 10.0)),
                    vel: Vector2::new(rand_range(-80.0, 80.0), rand_range(-80.0, 80.0)),
                    lifetime: 0.4,
                    color: Color::RED,
                });
            }
            return true;
        }
    }
    false
}

// ── Update: Particles ────────────────────────────────────────────────────────

fn update_particles(particles: &mut Vec<Particle>, dt: f32) {
    particles.retain_mut(|p| {
        p.pos += p.vel * dt;
        p.vel *= 0.9;
        p.lifetime -= dt;
        p.lifetime > 0.0
    });
}

// ── Render ───────────────────────────────────────────────────────────────────

fn draw_frame(
    d: &mut RaylibDrawHandle,
    player: &Player,
    bullets: &[Bullet],
    enemies: &[Enemy],
    particles: &[Particle],
    score: i32,
    wave: i32,
    game_over: bool,
) {
    d.clear_background(Color::new(20, 20, 25, 255));
    draw_grid(d);

    // Particles
    for p in particles {
        let alpha = (p.lifetime / 0.5).clamp(0.0, 1.0);
        let c = Color::new(p.color.r, p.color.g, p.color.b, (alpha * 255.0) as u8);
        d.draw_circle_v(p.pos, 3.0, c);
    }

    // Bullets
    for b in bullets {
        d.draw_circle_v(b.pos, BULLET_RADIUS, Color::YELLOW);
    }

    // Enemies
    for e in enemies {
        let color = if e.hit_timer > 0.0 { Color::WHITE } else { Color::RED };
        d.draw_rectangle_rounded(
            Rectangle::new(
                e.pos.x - ENEMY_SIZE / 2.0,
                e.pos.y - ENEMY_SIZE / 2.0,
                ENEMY_SIZE,
                ENEMY_SIZE,
            ),
            0.3,
            6,
            color,
        );
        // HP bar
        let hp_ratio = e.hp as f32 / (ENEMY_HP + wave.saturating_sub(1) / 3).max(1) as f32;
        let bar_w = ENEMY_SIZE + 4.0;
        d.draw_rectangle_v(
            Vector2::new(e.pos.x - bar_w / 2.0, e.pos.y - ENEMY_SIZE / 2.0 - 8.0),
            Vector2::new(bar_w, 3.0),
            Color::DARKGRAY,
        );
        d.draw_rectangle_v(
            Vector2::new(e.pos.x - bar_w / 2.0, e.pos.y - ENEMY_SIZE / 2.0 - 8.0),
            Vector2::new(bar_w * hp_ratio, 3.0),
            Color::RED,
        );
    }

    // Player
    d.draw_rectangle_rounded(
        Rectangle::new(
            player.pos.x - PLAYER_SIZE / 2.0,
            player.pos.y - PLAYER_SIZE / 2.0,
            PLAYER_SIZE,
            PLAYER_SIZE,
        ),
        0.3,
        8,
        Color::GREEN,
    );
    // "eyes" to show facing direction
    let mouse = d.get_mouse_position();
    let dir = (mouse - player.pos).normalized();
    let eye_offset = 5.0;
    let eye_pos = player.pos + dir * eye_offset;
    d.draw_circle_v(eye_pos, 3.0, Color::WHITE);
    // perpendicular = (-y, x)
    d.draw_circle_v(
        Vector2::new(eye_pos.x + dir.y * (-4.0), eye_pos.y + dir.x * 4.0),
        2.5,
        Color::WHITE,
    );

    // Crosshair
    d.draw_circle_v(mouse, 3.0, Color::WHITE);
    d.draw_circle_lines(mouse.x as i32, mouse.y as i32, 8.0, Color::new(255, 255, 255, 120));
    d.draw_line_v(
        Vector2::new(mouse.x - 12.0, mouse.y),
        Vector2::new(mouse.x - 5.0, mouse.y),
        Color::new(255, 255, 255, 100),
    );
    d.draw_line_v(
        Vector2::new(mouse.x + 5.0, mouse.y),
        Vector2::new(mouse.x + 12.0, mouse.y),
        Color::new(255, 255, 255, 100),
    );
    d.draw_line_v(
        Vector2::new(mouse.x, mouse.y - 12.0),
        Vector2::new(mouse.x, mouse.y - 5.0),
        Color::new(255, 255, 255, 100),
    );
    d.draw_line_v(
        Vector2::new(mouse.x, mouse.y + 5.0),
        Vector2::new(mouse.x, mouse.y + 12.0),
        Color::new(255, 255, 255, 100),
    );

    // HUD
    d.draw_text(
        &format!("HP: {} / {}", player.hp, PLAYER_MAX_HP),
        16, 16, 20, Color::GREEN,
    );
    d.draw_text(&format!("Score: {}", score), 16, 42, 20, Color::RAYWHITE);
    d.draw_text(&format!("Wave: {}", wave), 16, 68, 20, Color::ORANGE);
    d.draw_text(&format!("Enemies: {}", enemies.len()), 16, 94, 16, Color::LIGHTGRAY);

    // Controls hint
    d.draw_text("WASD move | Mouse aim+shoot | P pause", 16, SCREEN_HEIGHT - 30, 14, Color::DARKGRAY);

    // Game Over overlay
    if game_over {
        draw_centered_text(d, "GAME OVER", 50, Color::RED);
        draw_centered_text(d, &format!("Final Score: {}", score), 30, Color::RAYWHITE);
        draw_centered_text(d, "Press ENTER to restart", 20, Color::GRAY);
    }
}

// ── Helpers ──────────────────────────────────────────────────────────────────

fn draw_grid(d: &mut RaylibDrawHandle) {
    let step = 64.0;
    let grid_color = Color::new(40, 40, 50, 80);
    let mut x = 0.0;
    while x < SCREEN_WIDTH as f32 {
        d.draw_line_v(Vector2::new(x, 0.0), Vector2::new(x, SCREEN_HEIGHT as f32), grid_color);
        x += step;
    }
    let mut y = 0.0;
    while y < SCREEN_HEIGHT as f32 {
        d.draw_line_v(Vector2::new(0.0, y), Vector2::new(SCREEN_WIDTH as f32, y), grid_color);
        y += step;
    }
}

fn draw_centered_text(d: &mut RaylibDrawHandle, text: &str, font_size: i32, color: Color) {
    let w = raylib::text::measure_text(text, font_size);
    let x = (SCREEN_WIDTH - w) / 2;
    let y = (SCREEN_HEIGHT - font_size) / 2;
    d.draw_text(text, x, y, font_size, color);
}

fn rand_range(min: f32, max: f32) -> f32 {
    min + (max - min) * fastrand::f32()
}
