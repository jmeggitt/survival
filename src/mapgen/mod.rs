use rand::{
    Rng,
    distributions::{Standard},
};
use delaunay2d::Delaunay2D;
use voronoi::VoronoiDiagram;
use ordered_float::OrderedFloat;
use std::collections::{HashSet, HashMap};

use rayon::prelude::*;

pub struct GeneratorConfig {
    pub num_points: usize,
    pub num_lloyd: usize,
    pub box_size: f64,
}
impl Default for GeneratorConfig {
    fn default() -> Self {
        Self {
            num_points: 10,
            num_lloyd: 2,
            box_size: 500.0,
        }
    }
}
pub type Point = amethyst::core::math::Point2<f64>;
pub type IndexPoint = amethyst::core::math::Point2<OrderedFloat<f64>>;


fn convert_point(other: Point) -> IndexPoint {
    IndexPoint::new(OrderedFloat(other.x), OrderedFloat(other.y))
}


fn inside_poly(target: Point, points: &[Point]) -> bool {
    let mut c: i32 = 0;
    for i in 0..points.len() {
        for j in (0..points.len()).rev() {
            if (points[i].y > target.y) != (points[j].y > target.y) &&
                (target.x < (points[j].x - points[i].x) * (target.y - points[i].y) / (points[j].y - points[i].y) + points[i].x ) {
                c = !c;
            }
        }
    }
    c == 0
}


#[derive(Default, Debug)]
pub struct CellData {
    height: f64,
    used: bool,
}

#[derive(Debug)]
pub struct Cell<T> {
    position: IndexPoint,
    polygon: Vec<Point>,
    neighbors: Vec<IndexPoint>,
    data: T,
}

pub struct Generator<R> {
    phantom: std::marker::PhantomData<R>,
    rng: R,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct IslandGeneratorSettings {
    pub height: f64,
    pub radius: f64,
    pub sharpness: f64,
}
impl Default for IslandGeneratorSettings {
    fn default() -> Self {
        Self {
            height: 1.0,
            radius: 0.95,
            sharpness: 0.2
        }
    }
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

    pub fn run(&mut self, config: &GeneratorConfig) {

    }

    pub fn create_island(&mut self, config: &GeneratorConfig, settings: &IslandGeneratorSettings, cells: &mut HashMap<IndexPoint, Cell<CellData>>,) {
        //let start_cell = self.rng.gen_range(0, cells.len());
        // Find the center polygon

        use amethyst::core::math as na;
        let mut center = Point::new(0., 0.);
        let target = Point::new(config.box_size / 2., config.box_size / 2.);
        for (key, _) in cells.iter() {
            let point = Point::new(key.x.into_inner(), key.y.into_inner());
            if na::distance(&target, &center) > na::distance(&target, &point) {
                center = point;
            }
        }

        let mut height = settings.height;

        let mut queue = Vec::new();
        queue.push(IndexPoint::new(OrderedFloat(center.x), OrderedFloat(center.y)));
        cells.get_mut(&queue[0]).unwrap().data.height = height;

        let mut i = 0;
        while i < queue.len() && height > 0.01 {
            height = cells[&queue[i]].data.height * settings.radius;

            let neighbors = cells[&queue[i]].neighbors.clone();
            neighbors.iter().for_each(|n| {
                let cell = cells.get_mut(n).unwrap();
                if ! cell.data.used {
                    let modifier: f64 = if settings.sharpness == 0. {
                        1.0
                    } else {
                        let r: f64 = self.rng.gen();
                        r * settings.sharpness + 1.1 - settings.sharpness
                    };

                    cell.data.height += height * modifier;
                    cell.data.used = true;

                    if cell.data.height > 1. { cell.data.height = 1.; }

                    queue.push(*n);
                }
            });

            i += 1;
        }

    }

    pub fn gen_voronoi<T: Default>(&mut self, config: &GeneratorConfig) -> HashMap<IndexPoint, Cell<T>> {
        let mut ret = HashMap::new();

        let mut vor_pts = Vec::new();
        for i in 0..config.num_points as usize {
            let p = self.sample_point(config);
            vor_pts.push(voronoi::Point::new(p.0, p.1));
        }

        for i in 0..config.num_lloyd {
            vor_pts = voronoi::lloyd_relaxation(&vor_pts, config.box_size);
        }

        let diagram = voronoi::VoronoiDiagram::new(&vor_pts, config.box_size, 2);

        // Build the DT out of the centroids so they are associated

        let mut dt = delaunay2d::Delaunay2D::new((config.box_size/2., config.box_size/2.), config.box_size / 2.);
        for cell in diagram.cells().iter() {
            dt.add_point((cell.centroid.x(), cell.centroid.y()));
        }

        // Now extract the actual cells from this
        let dt_points = dt.export_points().par_iter().map(|p| IndexPoint::new(OrderedFloat(p.0), OrderedFloat(p.1))).collect::<Vec<_>>();
        let mut triangles = dt.export_triangles().par_iter().map(|t| (dt_points[t.0], dt_points[t.1], dt_points[t.2]) ).collect::<Vec<_>>();

        for cell in diagram.cells().iter() {
            let mut neighbors = HashSet::new();

            let point = IndexPoint::new(cell.centroid.x, cell.centroid.y);
            for triangle in &triangles {
                if triangle.0 == point || triangle.1 == point || triangle.2 == point {
                    neighbors.insert(triangle.0);
                    neighbors.insert(triangle.1);
                    neighbors.insert(triangle.2);
                }
            }
            neighbors.remove(&point);
            let mut n_vec = neighbors.drain().collect::<Vec<IndexPoint>>();
            n_vec.sort_by(|a, b| {
                use std::cmp::Ordering;
                let x = a.x.cmp(&b.x);
                let y = a.x.cmp(&b.y);
                if x == Ordering::Equal && y == Ordering::Equal {
                    return Ordering::Equal
                } else {
                    return x
                }
            });

            ret.insert(point, Cell {
                position: point,
                polygon: cell.points.par_iter().map(|p| Point::new(p.x(), p.y())).collect::<Vec<_>>(),
                neighbors: n_vec,
                data: T::default(),
            });
        }

        ret
    }

    pub fn save_heightmap_image(&self, config: &GeneratorConfig, path: &std::path::Path,
                       cells: &HashMap<IndexPoint, Cell<CellData>>,) -> std::io::Result<()> {

        let mut imgbuf = image::ImageBuffer::new(config.box_size as u32, config.box_size as u32);

        for (n, (point, cell)) in cells.iter().enumerate() {
            let mut points = cell.polygon.iter().map(|p| {
                imageproc::drawing::Point::new(p.x as i32,
                                               p.y as i32)
            }).collect::<Vec<_>>();

            while points[0] == points[points.len()-1] {
                points.remove(points.len()-1);
            }

            let height_color = (cell.data.height * 255.) as u8;
            let color = if cell.data.height < 0.5 {
                image::Rgb([0, 191, 255])
            } else {
                image::Rgb([height_color, 0, 0])
            };

            imageproc::drawing::draw_convex_polygon_mut(&mut imgbuf, &points, color);
        }

        imgbuf.save(path)
    }

    fn sample_point(&mut self, config: &GeneratorConfig) -> (f64, f64) {
        let x: f64 = self.rng.gen();
        let y: f64 = self.rng.gen();
        (x * config.box_size, y * config.box_size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use amethyst::core::math::Point2;
    use rand::SeedableRng;

    #[test]
    pub fn rng_sample_test() {
        use rand::SeedableRng;
        let seed = [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16
        ];
        let mut rand1 = rand::rngs::StdRng::from_seed(seed);
        let mut rand2 = rand::rngs::StdRng::from_seed(seed);

        let samples1 = vec![rand1.gen::<f64>(), rand1.gen::<f64>(), rand1.gen::<f64>(),];
        println!("samples1={:?}", samples1);
        let samples2 = vec![rand2.gen::<f64>(), rand2.gen::<f64>(), rand2.gen::<f64>(),];
        println!("samples2={:?}", samples2);

        assert_eq!(samples1, samples2);
    }

    #[test]
    pub fn voronoi_1() {
        use std::path::Path;

        let seed = [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16
        ];
        let mut master_rand = rand::rngs::StdRng::from_seed(seed);

        let mut generator = Generator::new(master_rand.clone());

        let config = GeneratorConfig {
            box_size: 500.0,
            num_points: 10,
            ..Default::default()
        };

        let mut cells = generator.gen_voronoi::<CellData>(
            &config
        );

        generator.create_island(&config,
                                &IslandGeneratorSettings::default(),
                                &mut cells);

        generator.save_heightmap_image(&config, &Path::new("/tmp/test.png"), &cells).unwrap();
    }

}






// Generate dt
/*
 This was all experimental code with dt and voronoi
for (n, poly) in d.cells().iter().enumerate() {
    let mut points = poly.points.iter().map(|p| {
        imageproc::drawing::Point::new(p.x.into_inner() as i32,
                                       p.y.into_inner() as i32)
    }).collect::<Vec<_>>();

    if points[0] == points[points.len() - 1] {
        println!("FIRST AND LAST MATCH");
        continue;
    }

    imageproc::drawing::draw_convex_polygon_mut(&mut imgbuf, &points, image::Rgb([0, 0, (n * 10) as u8]));

    // Test jordan curve theorem
    assert!(inside_poly(Point::new(poly.centroid.x(), poly.centroid.y()), poly.points.iter().map(|p| { Point::new(p.x(), p.y()) }).collect::<Vec<_>>().as_slice());
    assert_eq!(false, inside_poly(Point::new(9999.0, 9999.0), poly.points.iter().map(|p| { Point::new(p.x(), p.y()) }).collect::<Vec<_>>().as_slice());
}*/





// Now draw the DT
/*
for triangle in triangles {
    // Draw the triangle between the three points
    let point1 = { (points[triangle.0].0 as f32, points[triangle.0].1 as f32) };
    let point2 = { (points[triangle.1].0 as f32, points[triangle.1].1 as f32) };
    let point3 = { (points[triangle.2].0 as f32, points[triangle.2].1 as f32) };
    imageproc::drawing::draw_line_segment_mut(&mut imgbuf, point1, point2, image::Rgb([0, 255, 0]));
    imageproc::drawing::draw_line_segment_mut(&mut imgbuf, point2, point3, image::Rgb([0, 255, 0]));
    imageproc::drawing::draw_line_segment_mut(&mut imgbuf, point3, point1, image::Rgb([0, 255, 0]));

    // Is the first point of the triangle the cell location?
    //let pixel = imgbuf.get_pixel_mut(point1.0 as u32, point1.1 as u32);
    // *pixel = image::Rgb([255, 0, 0]);
}

use itertools::Itertools;
let (points, regions) = dt.export_voronoi_regions();
for region in regions {
    for n in (0..region.len()) {
        let point1 = { (points[region[n]].0 as f32, points[region[n]].1 as f32) };
        let point2;
        if n == region.len()-1 {
            point2 = { (points[region[0]].0 as f32, points[region[0]].1 as f32) };
        } else {
            point2 = { (points[region[n+1]].0 as f32, points[region[n+1]].1 as f32) };
        }
        imageproc::drawing::draw_line_segment_mut(&mut imgbuf, point1, point2, image::Rgb([50, 0, 0]));
    }
}
*/