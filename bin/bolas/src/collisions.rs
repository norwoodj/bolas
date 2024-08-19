use crate::bolas::Bola;
use bio::data_structures::interval_tree::IntervalTree;
use foundations::settings::settings;
use std::collections::HashSet;
use std::ops::Range;

pub(crate) const BOLA_COLLISION_RADIUS: i32 = 20;

#[derive(Eq, Hash, PartialEq)]
pub(crate) struct Collision {
    pub(crate) one: usize,
    pub(crate) two: usize,
}

#[settings]
#[derive(Copy)]
pub(crate) enum CollisionDetectionAlgorithm {
    #[default]
    IntervalTrees,
    Distance,
}

pub(crate) enum CollisionDetector {
    Distance,
    IntervalTrees {
        overlaps_x: IntervalTree<i32, usize>,
        overlaps_y: IntervalTree<i32, usize>,
    },
}

impl From<CollisionDetectionAlgorithm> for CollisionDetector {
    fn from(collision_detection_algorithm: CollisionDetectionAlgorithm) -> Self {
        match collision_detection_algorithm {
            CollisionDetectionAlgorithm::Distance => CollisionDetector::Distance,
            CollisionDetectionAlgorithm::IntervalTrees => CollisionDetector::IntervalTrees {
                overlaps_x: Default::default(),
                overlaps_y: Default::default(),
            },
        }
    }
}

impl CollisionDetector {
    pub(crate) fn detect_collisions_for_bola(
        &mut self,
        bolas: &[Bola],
        bola_one_idx: usize,
    ) -> Vec<Collision> {
        match self {
            Self::Distance => Self::detect_collisions_for_bola_distance(bolas, bola_one_idx),
            Self::IntervalTrees {
                overlaps_x,
                overlaps_y,
            } => Self::detect_collisions_for_bola_interval_trees(
                bolas,
                bola_one_idx,
                overlaps_x,
                overlaps_y,
            ),
        }
    }

    fn detect_collisions_for_bola_distance(bolas: &[Bola], bola_one_idx: usize) -> Vec<Collision> {
        let mut collisions = Vec::new();
        let bola_one = &bolas[bola_one_idx];
        for (bola_two_idx, bola_two) in bolas.iter().enumerate().skip(bola_one_idx + 1) {
            let distance = ((bola_one.center.x - bola_two.center.x).powf(2.)
                + (bola_one.center.y - bola_two.center.y).powf(2.))
            .sqrt();

            if (distance as i32) < (BOLA_COLLISION_RADIUS * 2) {
                collisions.push(Collision {
                    one: bola_one_idx,
                    two: bola_two_idx,
                });
            }
        }

        collisions
    }

    fn detect_collisions_for_bola_interval_trees(
        bolas: &[Bola],
        bola_one_idx: usize,
        overlaps_x: &mut IntervalTree<i32, usize>,
        overlaps_y: &mut IntervalTree<i32, usize>,
    ) -> Vec<Collision> {
        let bola_one = &bolas[bola_one_idx];
        let (x_range, y_range) = Self::get_location_ranges_for_bola(bola_one);
        let collision_x: HashSet<usize> = overlaps_x.find(&x_range).map(|e| *e.data()).collect();
        let collision_y: HashSet<usize> = overlaps_y.find(&y_range).map(|e| *e.data()).collect();

        overlaps_x.insert(x_range, bola_one_idx);
        overlaps_y.insert(y_range, bola_one_idx);

        collision_x
            .intersection(&collision_y)
            .map(|bola_two_idx| Collision {
                one: bola_one_idx,
                two: *bola_two_idx,
            })
            .collect()
    }

    fn get_location_ranges_for_bola(bola: &Bola) -> (Range<i32>, Range<i32>) {
        (
            (bola.center.x.round() as i32) - BOLA_COLLISION_RADIUS
                ..(bola.center.x.round() as i32) + BOLA_COLLISION_RADIUS,
            (bola.center.y.round() as i32) - BOLA_COLLISION_RADIUS
                ..(bola.center.y.round() as i32) + BOLA_COLLISION_RADIUS,
        )
    }
}
