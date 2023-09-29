use bio::data_structures::interval_tree::IntervalTree;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::ops::Range;

const VELOCITY_SCALING_FACTOR: i32 = 8;
const BOLA_COLLISION_RADIUS: i32 = 20;
const COLLISION_FRAMES: usize = 5;

#[derive(Debug, Default, Deserialize, Serialize)]
pub(crate) struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Default, Deserialize)]
pub(crate) struct Vector {
    vel_x: i32,
    vel_y: i32,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub(crate) struct Bola {
    #[serde(rename = "c")]
    center: Point,

    #[serde(skip_serializing, rename = "v")]
    velocity: Vector,

    #[serde(skip_deserializing, rename = "t")]
    collision_frames_remaining: usize,
}

impl Bola {
    fn update_position(&mut self, canvas_height: i32, canvas_width: i32) {
        let mut new_center_x = self.center.x + self.velocity.vel_x;
        let mut new_center_y = self.center.y + self.velocity.vel_y;

        if new_center_x < 0 {
            new_center_x = -new_center_x;
            self.velocity.vel_x = -self.velocity.vel_x;
        }
        if new_center_y < 0 {
            new_center_y = -new_center_y;
            self.velocity.vel_y = -self.velocity.vel_y;
        }

        if new_center_x > canvas_width {
            new_center_x = canvas_width - (new_center_x - canvas_width);
            self.velocity.vel_x = -self.velocity.vel_x;
        }

        if new_center_y > canvas_height {
            new_center_y = canvas_height - (new_center_y - canvas_height);
            self.velocity.vel_y = -self.velocity.vel_y;
        }

        self.center.x = new_center_x;
        self.center.y = new_center_y;
    }

    fn get_location_ranges(&self) -> (Range<i32>, Range<i32>) {
        (
            self.center.x - BOLA_COLLISION_RADIUS..self.center.x + BOLA_COLLISION_RADIUS,
            self.center.y - BOLA_COLLISION_RADIUS..self.center.y + BOLA_COLLISION_RADIUS,
        )
    }
}

#[derive(Default, Serialize)]
pub(crate) struct BolaState {
    bolas: Vec<Bola>,

    #[serde(skip_serializing)]
    canvas_height: i32,

    #[serde(skip_serializing)]
    canvas_width: i32,
}

impl BolaState {
    pub(crate) fn add_bola(&mut self, mut bola: Bola) {
        bola.velocity.vel_x /= VELOCITY_SCALING_FACTOR;
        bola.velocity.vel_y /= VELOCITY_SCALING_FACTOR;
        self.bolas.push(bola);
    }

    pub(crate) fn set_canvas_dimensions(&mut self, height: i32, width: i32) {
        self.canvas_height = height;
        self.canvas_width = width;
    }

    pub(crate) fn tick(&mut self) {
        for b in &mut self.bolas {
            b.update_position(self.canvas_height, self.canvas_width);
        }

        self.update_for_collisions();
    }

    fn update_for_collisions(&mut self) {
        let mut colliding_bolas = vec![];
        let mut overlaps_x: IntervalTree<i32, usize> = IntervalTree::default();
        let mut overlaps_y: IntervalTree<i32, usize> = IntervalTree::default();
        for b in &mut self.bolas {
            if b.collision_frames_remaining > 0 {
                b.collision_frames_remaining -= 1;
            }
        }

        for (i, b) in self.bolas.iter().enumerate() {
            let (x_range, y_range) = b.get_location_ranges();
            let collision_x: HashSet<usize> =
                overlaps_x.find(&x_range).map(|e| *e.data()).collect();
            let collision_y: HashSet<usize> =
                overlaps_y.find(&y_range).map(|e| *e.data()).collect();

            let collisions = collision_x.intersection(&collision_y);
            for c in collisions {
                colliding_bolas.push((*c, i));
            }

            overlaps_x.insert(x_range, i);
            overlaps_y.insert(y_range, i);
        }

        for (one, two) in colliding_bolas {
            let bola_one = &self.bolas[one];
            let bola_two = &self.bolas[two];
            let distance = (((bola_one.center.x - bola_two.center.x) as f64).powf(2.)
                + ((bola_one.center.y - bola_two.center.y) as f64).powf(2.))
            .sqrt();

            let collision_vector = (
                (bola_one.center.x - bola_two.center.x) as f64,
                (bola_one.center.y - bola_two.center.y) as f64,
            );
            let collision_vector_normalized =
                (collision_vector.0 / distance, collision_vector.1 / distance);
            let relative_velocity_vector = (
                (bola_one.velocity.vel_x - bola_two.velocity.vel_x) as f64,
                (bola_one.velocity.vel_y - bola_two.velocity.vel_y) as f64,
            );
            let speed = relative_velocity_vector.0 * collision_vector_normalized.0
                + relative_velocity_vector.1 * collision_vector_normalized.1;

            let bola_one = &mut self.bolas[one];
            bola_one.velocity.vel_x -= (collision_vector_normalized.0 * speed) as i32;
            bola_one.velocity.vel_y -= (collision_vector_normalized.1 * speed) as i32;
            bola_one.collision_frames_remaining = COLLISION_FRAMES;

            let bola_two = &mut self.bolas[two];
            bola_two.velocity.vel_x += (collision_vector_normalized.0 * speed) as i32;
            bola_two.velocity.vel_y += (collision_vector_normalized.1 * speed) as i32;
            bola_two.collision_frames_remaining = COLLISION_FRAMES;

            let bola_one = &self.bolas[one];
            let bola_two = &self.bolas[two];
            log::debug!(
                "Updated for collision between bolas at {:?} and {:?}",
                bola_one.center,
                bola_two.center
            )
        }
    }
}
