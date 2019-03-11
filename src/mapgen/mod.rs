use rand::{
    Rng,
    distributions::{Standard},
};

pub struct GeneratorConfig {
    num_points: usize,
    num_lloyd: usize,
    box_size: f64,
}
impl Default for GeneratorConfig {
    fn default() -> Self {
        Self {
            num_points: 100,
            num_lloyd: 2,
            box_size: 500.0,
        }
    }
}

struct VoronoiDiagram {

}

pub struct Generator<R> {
    phantom: std::marker::PhantomData<R>,
    rng: R,
}
impl<R> Generator<R>
    where R: Rng + Send + Sync + Clone + ?Sized
{
    pub fn new(rng: R, ) -> Self {
        Self {
            rng,
            phantom: std::marker::PhantomData{},
        }
    }

    pub fn gen_voronoi(&mut self, config: &GeneratorConfig) -> VoronoiDiagram {
        VoronoiDiagram{}
    }

    fn sample_point(&mut self, config: &GeneratorConfig) -> Point {
        let x: f64 = self.rng.sample(Standard);
        let y: f64 = self.rng.sample(Standard);
        Point::new(x * config.box_size, y * config.box_size)
    }
}

use num_traits::Float;
use ordered_float::OrderedFloat;

trait Bound<T> {
    fn bound(self, min: T, max: T) -> Self;
}
impl<T: Ord + Copy> Bound<T> for T {
    fn bound(self, min: Self, max: Self) -> Self {
        self.min(max).max(min)
    }
}

impl<T: Ord + Copy + Bound<T>> Bound<T> for (T, T)
{
    fn bound(self, min: T, max: T) -> Self {
        (self.0.bound(min, max), self.1.bound(min, max))
    }
}

impl<T: Copy + Float> Bound<T> for OrderedFloat<T> {
    fn bound(self, min: T, max: T) -> Self {
        self.min(Self(max)).max(Self(min))
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use amethyst::core::nalgebra::Point2;
    use rand::SeedableRng;

    #[test]
    pub fn voronoi_1() {

        use std::path::Path;

        let seed = [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16
        ];
        let mut master_rand = rand::rngs::StdRng::from_seed(seed);

        let mut imgbuf = image::ImageBuffer::new(600, 600);

        const BOX_SIZE: f64 = 450.0;

        let d = Generator::new(master_rand.clone()).gen_voronoi(
            &GeneratorConfig {
                box_size: BOX_SIZE,
                num_points: 300,
                ..Default::default()
            }
        );



        imgbuf.save(&Path::new("/tmp/test.png")).unwrap();   }
}