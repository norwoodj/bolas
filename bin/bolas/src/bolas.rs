use serde::{Deserialize, Serialize};

const VELOCITY_SCALING_FACTOR: i32 = 16;

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
    #[serde(skip_serializing)]
    #[serde(rename = "v")]
    velocity: Vector,
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
    }
}
