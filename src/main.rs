use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets};

#[derive(Clone, Copy)]
struct Particle {
    pos: Vec2,
    old_pos: Vec2,
    acceleration: Vec2,
    mass: f32,
    is_pinned: bool
}

impl Particle {
    fn new(x: f32, y: f32) -> Self {
        Particle {
            pos: vec2(x, y),
            old_pos: vec2(x, y),
            acceleration: Vec2::ZERO,
            mass: 1.0,
            is_pinned: false
        }
    }

    fn update(&mut self, dt: f32) {
        if self.is_pinned {
            return;
        }

        let velocity = self.pos - self.old_pos;
        self.old_pos = self.pos;
        self.pos += velocity + self.acceleration * dt * dt;
        self.acceleration = Vec2::ZERO;
    }

    fn apply_force(&mut self, force: Vec2) {
        self.acceleration += force / self.mass;
    }
}

struct Spring {
    p1_idx: usize,
    p2_idx: usize,
    rest_length: f32
}

struct Cloth {
    particles: Vec<Particle>,
    springs: Vec<Spring>,
    width: usize,
    height: usize
}

impl Cloth {
    fn new(width: usize, height: usize, spacing: f32, start_x: f32, start_y: f32) -> Self {
        let mut particles = Vec::with_capacity(width * height);
        for y in 0..height {
            for x in 0..width {
                let mut p = Particle::new(start_x + x as f32 * spacing, start_y + y as f32 * spacing);
                if y == 0 {
                    p.is_pinned = true;
                }
                particles.push(p);
            }
        }

        let mut springs = Vec::new();
        for y in 0..height {
            for x in 0..width {
                let current_idx = y * width + x;
                if x < width - 1 {
                    springs.push(Spring { p1_idx: current_idx, p2_idx: current_idx + 1, rest_length: spacing });
                    if x < width - 2 {
                         springs.push(Spring { p1_idx: current_idx, p2_idx: current_idx + 2, rest_length: spacing * 2.0 });
                    }
                }
                if y < height - 1 {
                    springs.push(Spring { p1_idx: current_idx, p2_idx: current_idx + width, rest_length: spacing });
                    if y < height - 2 {
                        springs.push(Spring { p1_idx: current_idx, p2_idx: current_idx + (2 * width), rest_length: spacing * 2.0 });
                    }
                }
                if x < width - 1 && y < height - 1 {
                    let diagonal_len = (spacing.powi(2) + spacing.powi(2)).sqrt();
                    springs.push(Spring { p1_idx: current_idx, p2_idx: current_idx + width + 1, rest_length: diagonal_len });
                    springs.push(Spring { p1_idx: (y * width) + (x + 1), p2_idx: (y + 1) * width + x, rest_length: diagonal_len });
                }
            }
        }

        Cloth { particles, springs, width, height }
    }

    fn update(
        &mut self,
        dt: f32,
        iterations: usize,
        gravity: Vec2,
        stiffness: f32,
        tear_threshold: f32,
    ) {
        for p in self.particles.iter_mut() {
            p.apply_force(gravity);
        }

        for p in self.particles.iter_mut() {
            p.update(dt);
        }

        for _ in 0..iterations {
            self.springs.retain(|s| {
                let p1 = self.particles[s.p1_idx];
                let p2 = self.particles[s.p2_idx];
                let dist = p1.pos.distance(p2.pos);
                dist < s.rest_length * tear_threshold
            });

            for spring in &self.springs {
                let p1 = &mut self.particles[spring.p1_idx] as *mut Particle;
                let p2 = &mut self.particles[spring.p2_idx] as *mut Particle;

                unsafe {
                    let delta = (*p2).pos - (*p1).pos;
                    let dist = delta.length();
                    if dist == 0.0 { continue; }

                    let diff = (dist - spring.rest_length) / dist;
                    let correction = delta * 0.5 * diff * stiffness;
                    
                    if !(*p1).is_pinned {
                        (*p1).pos += correction;
                    }
                    if !(*p2).is_pinned {
                        (*p2).pos -= correction;
                    }
                }
            }
        }
    }

    fn draw(&self) {
        for spring in &self.springs {
            let p1 = self.particles[spring.p1_idx];
            let p2 = self.particles[spring.p2_idx];
            draw_line(p1.pos.x, p1.pos.y, p2.pos.x, p2.pos.y, 1.0, WHITE);
        }
        for p in &self.particles {
            if p.is_pinned {
                draw_circle(p.pos.x, p.pos.y, 3.0, RED);
            } else {
                draw_circle(p.pos.x, p.pos.y, 2.0, BLUE);
            }
        }
    }
}

fn distance_point_to_segment(p: Vec2, a: Vec2, b: Vec2) -> f32 {
    let ab = b - a;
    let ap = p - a;
    let len_sq = ab.length_squared();

    if len_sq == 0.0 {
        return p.distance(a);
    }

    let t = (ap.dot(ab) / len_sq).clamp(0.0, 1.0);

    let closest_point = a + t * ab;
    p.distance(closest_point)
}

fn window_conf() -> Conf {
    Conf {
        window_title: "2D Cloth Simulator".to_owned(),
        window_width: 1200,
        window_height: 800,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut cloth_width: f32 = 40.;
    let mut cloth_height: f32 = 25.;
    let cloth_spacing: f32 = 15.0;
    let cloth_start_pos: Vec2 = vec2(300.0, 50.0);

    let mut last_width = cloth_width;
    let mut last_height = cloth_height;

    let mut cloth = Cloth::new(cloth_width as usize, cloth_height as usize, cloth_spacing, cloth_start_pos.x, cloth_start_pos.y);
    let mut selected_particle_idx: Option<usize> = None;

    let mut stiffness = 0.9;
    let mut tear_threshold = 4.5;
    let mut gravity_y = 980.0;
    let mut iterations = 5.0;
    let mut cut_radius = 10.0;

    loop {
        clear_background(BLACK);

        let dt = get_frame_time().min(1.0 / 30.0);

        widgets::Window::new(hash!(), vec2(10., 40.), vec2(280., 260.))
            .label("Simulation Configurations")
            .ui(&mut root_ui(), |ui| {
                ui.label(None, "Cloth Size:");
                ui.slider(hash!(), &format!("Width ({})", cloth_width as usize), 4. ..64., &mut cloth_width);
                ui.slider(hash!(), &format!("Height ({})", cloth_height as usize), 4. ..64., &mut cloth_height);
                ui.slider(hash!(), "Cut radius", 10. ..50.0, &mut cut_radius);
                
                ui.separator();
                ui.slider(hash!(), "Gravity", 0. ..2000.0, &mut gravity_y);
                ui.slider(hash!(), "Stiffness", 0.1..1.0, &mut stiffness);
                ui.slider(hash!(), "Tear threshold", 1.1..10.0, &mut tear_threshold);
                ui.slider(hash!(), "Iterations", 1. ..20., &mut iterations);
                ui.label(None, &format!("(Current: {})", iterations as usize));
                ui.separator();

                if ui.button(None, "Reset Cloth") {
                    cloth = Cloth::new(cloth_width as usize, cloth_height as usize, cloth_spacing, cloth_start_pos.x, cloth_start_pos.y);
                    selected_particle_idx = None;
                }
            });

        if (last_width - cloth_width).abs() > 0.1 || (last_height - cloth_height).abs() > 0.1 {
            cloth = Cloth::new(cloth_width as usize, cloth_height as usize, cloth_spacing, cloth_start_pos.x, cloth_start_pos.y);
            selected_particle_idx = None;
            last_width = cloth_width;
            last_height = cloth_height;
        }

        let (mouse_x, mouse_y) = mouse_position();
        let mouse_pos = vec2(mouse_x, mouse_y);

        if is_mouse_button_pressed(MouseButton::Left) {
            if !root_ui().is_mouse_over(mouse_pos) {
                let mut closest_dist = f32::MAX;
                let mut closest_idx = 0;
                for (i, p) in cloth.particles.iter().enumerate() {
                    let dist_sq = (p.pos - mouse_pos).length_squared();
                    if dist_sq < closest_dist {
                        closest_dist = dist_sq;
                        closest_idx = i;
                    }
                }
                if closest_dist < 400.0 {
                    selected_particle_idx = Some(closest_idx);
                }
            }
        }
        
        if is_mouse_button_down(MouseButton::Left) {
            if let Some(idx) = selected_particle_idx {
                if idx < cloth.particles.len() {
                    let original_pos = cloth.particles[idx].pos;
                    cloth.particles[idx].pos = mouse_pos;
                    cloth.particles[idx].old_pos = original_pos;
                } else {
                    selected_particle_idx = None;
                }
            }
        }

        if is_mouse_button_released(MouseButton::Left) {
            selected_particle_idx = None;
        }
        
        if is_mouse_button_down(MouseButton::Right) {
            let mouse_pos = vec2(mouse_position().0, mouse_position().1);

            cloth.springs.retain(|spring| {
                let p1 = cloth.particles[spring.p1_idx];
                let p2 = cloth.particles[spring.p2_idx];
                let dist_to_spring = distance_point_to_segment(mouse_pos, p1.pos, p2.pos);
                dist_to_spring > cut_radius
            });
        }

        cloth.update(
            dt,
            iterations as usize, 
            vec2(0.0, gravity_y),
            stiffness,
            tear_threshold,
        );
        cloth.draw();

        draw_text("Left Mouse: Drag and Tear | Right Mouse: Cut", 10.0, 20.0, 20.0, WHITE);

        next_frame().await;
    }
}
