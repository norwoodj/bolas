use crate::metrics::metrics;
use bio::data_structures::interval_tree::IntervalTree;
use foundations::telemetry::log;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::ops::Range;
use std::time::Duration;
use uuid::Uuid;

const BOLA_COLLISION_RADIUS: i32 = 20;

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Vector {
    vel_x: f64,
    vel_y: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Bola {
    #[serde(rename = "c")]
    center: Point,

    #[serde(skip_serializing, rename = "v")]
    velocity: Vector,
}

impl Bola {
    fn update_position(&mut self, canvas_height: f64, canvas_width: f64) {
        let mut new_center_x = self.center.x + self.velocity.vel_x;
        let mut new_center_y = self.center.y + self.velocity.vel_y;

        if new_center_x < 0. {
            new_center_x = -new_center_x;
            self.velocity.vel_x = -self.velocity.vel_x;
        }
        if new_center_y < 0. {
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
            (self.center.x.round() as i32) - BOLA_COLLISION_RADIUS
                ..(self.center.x.round() as i32) + BOLA_COLLISION_RADIUS,
            (self.center.y.round() as i32) - BOLA_COLLISION_RADIUS
                ..(self.center.y.round() as i32) + BOLA_COLLISION_RADIUS,
        )
    }
}

#[derive(Serialize)]
pub(crate) struct BolasArena {
    bolas: Vec<Bola>,

    #[serde(skip_serializing)]
    velocity_scaling_factor: i32,

    #[serde(skip_serializing)]
    refresh_rate: Duration,

    #[serde(skip_serializing)]
    canvas_height: i32,

    #[serde(skip_serializing)]
    canvas_width: i32,

    #[serde(skip_serializing)]
    last_collisions: HashSet<(usize, usize)>,

    #[serde(skip_serializing)]
    id: Uuid,
}

impl Drop for BolasArena {
    fn drop(&mut self) {
        metrics::arenas_active().dec();
        metrics::bolas_active().dec_by(self.bolas.len() as u64);
    }
}

impl BolasArena {
    pub(crate) fn new(refresh_rate_ms: u64, velocity_scaling_factor: i32) -> Self {
        metrics::arenas_active().inc();
        metrics::arenas_total().inc();

        Self {
            bolas: Default::default(),
            refresh_rate: Duration::from_millis(refresh_rate_ms),
            canvas_height: 0,
            canvas_width: 0,
            last_collisions: Default::default(),
            velocity_scaling_factor,
            id: Uuid::new_v4(),
        }
    }

    pub(crate) fn get_id(&self) -> Uuid {
        self.id
    }

    pub(crate) fn add_bola(&mut self, mut bola: Bola) {
        metrics::bolas_active().inc();
        metrics::bolas_total().inc();

        bola.velocity.vel_x /= self.velocity_scaling_factor as f64;
        bola.velocity.vel_y /= self.velocity_scaling_factor as f64;
        self.bolas.push(bola);
    }

    pub(crate) fn set_canvas_dimensions(&mut self, height: i32, width: i32) {
        self.canvas_height = height;
        self.canvas_width = width;
    }

    pub(crate) fn update_state(&mut self) {
        for b in &mut self.bolas {
            b.update_position(self.canvas_height as f64, self.canvas_width as f64);
        }

        self.update_for_collisions();
    }

    fn update_for_collisions(&mut self) {
        let mut colliding_bolas = HashSet::new();
        let mut overlaps_x: IntervalTree<i32, usize> = IntervalTree::default();
        let mut overlaps_y: IntervalTree<i32, usize> = IntervalTree::default();

        for (i, b) in self.bolas.iter().enumerate() {
            let (x_range, y_range) = b.get_location_ranges();
            let collision_x: HashSet<usize> =
                overlaps_x.find(&x_range).map(|e| *e.data()).collect();
            let collision_y: HashSet<usize> =
                overlaps_y.find(&y_range).map(|e| *e.data()).collect();

            let collisions = collision_x.intersection(&collision_y);
            for c in collisions {
                colliding_bolas.insert((*c, i));
            }

            overlaps_x.insert(x_range, i);
            overlaps_y.insert(y_range, i);
        }

        for collision in &colliding_bolas {
            if self.last_collisions.contains(collision) {
                continue;
            }

            let (one, two) = *collision;

            let bola_one = &self.bolas[one];
            let bola_two = &self.bolas[two];

            let distance = ((bola_one.center.x - bola_two.center.x).powf(2.)
                + (bola_one.center.y - bola_two.center.y).powf(2.))
            .sqrt();

            if distance == 0. {
                continue;
            }

            let collision_vector = (
                (bola_one.center.x - bola_two.center.x),
                (bola_one.center.y - bola_two.center.y),
            );
            let collision_vector_normalized =
                (collision_vector.0 / distance, collision_vector.1 / distance);
            let relative_velocity_vector = (
                (bola_one.velocity.vel_x - bola_two.velocity.vel_x),
                (bola_one.velocity.vel_y - bola_two.velocity.vel_y),
            );
            let speed = relative_velocity_vector.0 * collision_vector_normalized.0
                + relative_velocity_vector.1 * collision_vector_normalized.1;

            let bola_one = &mut self.bolas[one];
            bola_one.velocity.vel_x -= collision_vector_normalized.0 * speed;
            bola_one.velocity.vel_y -= collision_vector_normalized.1 * speed;

            let bola_two = &mut self.bolas[two];
            bola_two.velocity.vel_x += collision_vector_normalized.0 * speed;
            bola_two.velocity.vel_y += collision_vector_normalized.1 * speed;

            let bola_one = &self.bolas[one];
            let bola_two = &self.bolas[two];
            log::debug!(
                "Updated for collision between bolas";
                "arena" => %self.id,
                "bola_one" => ?bola_one,
                "bola_two" => ?bola_two,
            )
        }

        self.last_collisions = colliding_bolas;
    }

    pub(crate) fn get_refresh_rate(&self) -> Duration {
        self.refresh_rate
    }
}
