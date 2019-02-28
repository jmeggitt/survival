use voronoi::{Point, lloyd_relaxation, VoronoiDiagram, VoronoiCell};
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
        // Generate points
        let mut vor_pts = Vec::new();
        for _ in 0..config.num_points {
            vor_pts.push(self.sample_point(&config));
        }

        // Generate lloyd
        let mut lloyd = vor_pts;
        for _ in 0..config.num_lloyd {
            lloyd = lloyd_relaxation(&lloyd, config.box_size);
        }
        let voronoi_data = voronoi::voronoi(&lloyd.clone(), config.box_size);

        //let lines = make_line_segments(&voronoi_data);
        //let faces = make_polygons(&voronoi_data);

        //(lines, faces)
        VoronoiDiagram::new(voronoi_data)
    }

    fn sample_point(&mut self, config: &GeneratorConfig) -> Point {
        let x: f64 = self.rng.sample(Standard);
        let y: f64 = self.rng.sample(Standard);
        Point::new(x * config.box_size, y * config.box_size)
    }
}

pub struct CellWrapper<'a> {
    inner: VoronoiCell<'a>,
    pub height: f64,
}
impl<'a> CellWrapper<'a> {
    pub fn new(cell: VoronoiCell<'a>,) -> Self {
        Self {
            inner: cell,
            height: 0.,
        }
    }
}
use num_traits::Float;
use ordered_float::OrderedFloat;

trait Bound<T> {
    fn bound(self, min: T, max: T) -> Self;
}
impl<T: Ord + Copy> Bound<T> for T {
    fn bound(self, min: T, max: T) -> Self {
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
        self.min(OrderedFloat(max)).max(OrderedFloat(min))
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::Point2;
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
                num_points: 20,
                ..Default::default()
            }
        );
        let cells = d.cells().map(|c| { CellWrapper::new(c) }).collect::<Vec<_>>();
        let len = cells.len();
        let _random_cell = &cells[master_rand.gen_range(0, len)];

        let mut dt = delaunay2d::Delaunay2D::new((450./2., 450./2.), 450./2.);
        for cell in d.cells() {
            // if cell == random_cell.inner {
            let center = cell.centroid();
            dt.add_point((center.x(), center.y()));
        }
        let (coords, regions) = dt.export_voronoi_regions();

        for region in &regions {
            let points = region.iter().map(|i| {
                let p = coords[*i];
                imageproc::drawing::Point::new(OrderedFloat(p.0).bound(0., BOX_SIZE).into_inner() as i32,
                            OrderedFloat(p.1).bound(0., BOX_SIZE).into_inner() as i32
                )
            }).collect::<Vec<_>>();

            imageproc::drawing::draw_convex_polygon_mut(&mut imgbuf,
                                                        &points, image::Rgb([0,0,255])
            )

            /*
            let mut i = 0;

            for i in 0..region.len() {
                let mut cur = coords[region[i]];
                let mut next;
                if i == region.len() - 1 {
                    next = coords[region[0]];
                } else {
                    next = coords[region[i+1]];
                }

                cur = (OrderedFloat(cur.0).bound(0., BOX_SIZE).into_inner(),
                       OrderedFloat(cur.1).bound(0., BOX_SIZE).into_inner());
                next = (OrderedFloat(next.0).bound(0., BOX_SIZE).into_inner(),
                       OrderedFloat(next.1).bound(0., BOX_SIZE).into_inner());

                /*let mut c = Point2::new(cur.0, cur.1);
                let d = Point2::new(next.0 as u32, next.1 as u32);
                let v = Point2::new(next.0, next.1) - c;

                while c.x as u32 != d.x && c.y as u32 != d.y {
                    let pixel = imgbuf.get_pixel_mut(c.x as u32, c.y as u32);
                    *pixel = image::Rgb([0,255,0]);
                    c += (v * 0.1);
                }*/
            }*/
        }

        for cell in d.cells() {
            for line in cell.segments() {
                let mut c = Point2::new(line.0.x(), line.0.y());
                let d = Point2::new(line.1.x() as u32, line.1.y() as u32);
                let v = Point2::new(line.1.x(), line.1.y()) - c;

                while c.x as u32 != d.x && c.y as u32 != d.y {
                    let pixel = imgbuf.get_pixel_mut(c.x as u32, c.y as u32);
                    *pixel = image::Rgb([255,255,255]);
                    c += v * 0.1;
                }
            }
            // if cell == random_cell.inner {
            let center = cell.centroid();
            let pixel = imgbuf.get_pixel_mut(center.x() as u32, center.y() as u32);
            *pixel = image::Rgb([255,0,0]);
        }



        // Build a d2d out of the thing and see what happens


        /*
        for line in voronoi::make_line_segments(&d.dcel) {
            let mut c = Point2::new(line.0.x(), line.0.y());
            let d = Point2::new(line.1.x() as u32, line.1.y() as u32);
            let v = Point2::new(line.1.x(), line.1.y()) - c;

            while c.x as u32 != d.x && c.y as u32 != d.y {
                let pixel = imgbuf.get_pixel_mut(c.x as u32, c.y as u32);
                *pixel = image::Rgb([255,255,255]);
                c += (v * 0.1);
            }
        }*/

        imgbuf.save(&Path::new("/tmp/test.png")).unwrap();
    }
}