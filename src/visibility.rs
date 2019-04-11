use amethyst::core::math::Vector2;
use approx::*;
use num_traits::Float;

#[derive(Eq, PartialEq)]
pub enum Orientation {
    Left,
    Right,
    Collinear,
}

pub struct LineSegment {
    a: Vector2<f64>,
    b: Vector2<f64>,
}

pub struct Ray {
    origin: Vector2<f64>,
    direction: Vector2<f64>,
}

impl Ray {
    pub fn intersects(&self, segment: &LineSegment) -> Option<Vector2<f64>> {
        let ao = self.origin - segment.a;
        let ab = segment.b - segment.a;
        let det = cross(ab, self.direction);
        if ulps_eq!(det, 0.0) {
            let abo = compute_orientation(segment.a, segment.b, self.origin);

            if abo != Orientation::Collinear {
                return None;
            }

            let dist_a = dot(ao, self.direction);
            let dist_b = dot(self.origin - segment.b, self.direction);

            return match (dist_a > 0.0, dist_b > 0.0) {
                (true, true) => None,
                (false, false) if dist_a > dist_b => Some(segment.a),
                (false, false) => Some(segment.b),
                _ => Some(self.origin),
            };
        }

        let u = cross(ao, self.direction) / det;

        if strictly_less(u, 0.0) || strictly_less(1.0, u) {
            return None;
        }

        let t = -cross(ab, ao) / det;
        if ulps_eq!(t, 0.0) || t > 0.0 {
            Some(self.origin + self.direction * t)
        } else {
            None
        }
    }
}

//template<typename Vector, typename InputIterator>
//std::vector<Vector> visibility_polygon(
//Vector point,
//InputIterator begin,
//InputIterator end)
//{
pub fn visibility_polygon() -> Vec<Vector2<f64>> {
//    using segment_type = line_segment < Vector >;
//    using event_type = visibility_event < Vector >;
//    using segment_comparer_type = line_segment_dist_comparer < Vector >;

//    segment_comparer_type cmp_dist { point };
//    std::set < segment_type, segment_comparer_type > state { cmp_dist };
//    std::vector < event_type > events;

//    for (;
//    begin != end; + + begin)
//    {
    for {
        auto segment = *begin;

// Sort line segment endpoints and add them as events
// Skip line segments collinear with the point
        auto pab = compute_orientation(point, segment.a, segment.b);
        if (pab == orientation::collinear)
        {
            continue;
        } else if (pab == orientation::right_turn)
        {
            events.emplace_back(event_type::start_vertex, segment);
            events.emplace_back(
                event_type::end_vertex,
                segment_type { segment.b, segment.a });
        } else {
            events.emplace_back(
                event_type::start_vertex,
                segment_type { segment.b, segment.a });
            events.emplace_back(event_type::end_vertex, segment);
        }

// Initialize state by adding line segments that are intersected
// by vertical ray from the point
        auto a = segment.a, b = segment.b;
        if (a.x > b.x)
        std::swap(a, b);

        auto abp = compute_orientation(a, b, point);
        if (abp == orientation::right_turn &&
            (approx_equal(b.x, point.x) ||
                (a.x < point.x && point.x < b.x)))
        {
            state.insert(segment);
        }
    }

// sort events by angle
    angle_comparer < Vector > cmp_angle { point };
    std::sort(events.begin(), events.end(), [&cmp_angle](auto && a, auto && b)
    {
// if the points are equal, sort end vertices first
        if (approx_equal(a.point(), b.point()))
        return a.; type == event_type::end_vertex &&
        b. type == event_type::start_vertex;
        return cmp_angle(a.point(), b.point());
    });

// find the visibility polygon
    std::vector < Vector > vertices;
    for (auto && event: events)
        {
            if (event. type == event_type::end_vertex)
            state.erase(event.segment);

            if (state.empty())
            {
                vertices.push_back(event.point());
            } else if (cmp_dist(event.segment, *state.begin()))
            {
// Nearest line segment has changed
// Compute the intersection point with this segment
                vec2 intersection;
                ray < Vector > ray { point, event.point() - point };
                auto nearest_segment = *state.begin();
                auto intersects = ray.intersects(nearest_segment, intersection);
                assert(intersects &&
                    "Ray intersects line segment L iff L is in the state");

                if (event. type == event_type::start_vertex)
                {
                    vertices.push_back(intersection);
                    vertices.push_back(event.point());
                }
                else
                {
                    vertices.push_back(event.point());
                    vertices.push_back(intersection);
                }
            }

            if (event. type == event_type::start_vertex)
            state.insert(event.segment);
        }

// remove collinear points
    auto top = vertices.begin();
    for (auto
    it = vertices.begin();
    it != vertices.end(); + + it)
    {
        auto prev = top == vertices.begin()? vertices.end() - 1: top - 1;
        auto next = it + 1 == vertices.end()? vertices.begin(): it + 1;
        if (compute_orientation(*prev, *it, *next) != orientation::collinear)
            * top + + = *it;
    }
    vertices.erase(top, vertices.end());
    return vertices;
}


pub fn compute_orientation(a: Vector2<f64>, b: Vector2<f64>, c: Vector2<f64>) -> Orientation {
    let det = cross(b - a, c - a);
    match (strictly_less(0.0, det), strictly_less(det, 0.0)) {
        (false, true) => Orientation::Right,
        (true, false) => Orientation::Left,
        _ => Orientation::Collinear,
    }
}

pub fn cross(a: Vector2<f64>, b: Vector2<f64>) -> f64 {
    a.x * b.y - a.y * b.x
}

pub fn dot(a: Vector2<f64>, b: Vector2<f64>) -> f64 {
    a.x * b.x + a.y * b.y
}

pub fn strictly_less(a: f64, b: f64) -> bool {
    (a - b).abs() <= a.max(b) * f64::epsilon()
}
