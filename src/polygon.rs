// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

use core::ops;
use std::f64::consts::PI;
use std::slice::Iter;

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Point(i16, i16);

impl Point {
    fn new(x: i16, y: i16) -> Self
    {
        Point(x, y)
    }

    fn magnitude(&self) -> f64
    {
        f64::sqrt(f64::from(self.0 * self.0 + self.1 * self.1))
    }

    fn dist(&self, other: Self) -> f64
    {
        f64::sqrt(f64::from(
            i16::pow(self.0 - other.0, 2) + i16::pow(self.1 - other.1, 2))
        )
    }
}

impl ops::Add<Point> for Point {
    type Output = Self;
    fn add(self, _rhs: Self) -> Self {
        Self::new(self.0 + _rhs.0, self.1 + _rhs.1)
    }
}

pub struct Polygon {
    points: Vec<Point>,
}

impl Polygon {
    fn new() -> Self {
        Polygon {
            points: Vec::new()
        }
    }

    fn add_point(&mut self, point: Point) {
        self.points.push(point);
    }

    fn left_most_point(&self) -> Option<Point> {
        if !self.points.is_empty() {
            let mut left_point = self.points[0];
            for point in &self.points {
                if point.0 < left_point.0 {
                    left_point = point.clone();
                }
            }
            return Some(left_point);
        }
        return None;
    }

    fn iter(&self) -> Iter<'_, Point> {
        self.points.iter()
    }

    fn perimeter(&self) -> f64 {
        let mut perimeter: f64 = 0.0;
        if !self.points.is_empty() {
            for i in 0..(self.points.len() - 1) {
                perimeter += self.points[i].dist(self.points[i + 1]);
            }
            perimeter += self.points.last().unwrap().dist(*self.points.first().unwrap());
        }
        return perimeter;
    }
}

pub struct Circle {
    center: Point,
    radius: i64,
}

impl Circle {
    fn new(center: Point, radius: i64) -> Self {
        Circle {
            center,
            radius,
        }
    }

    fn perimeter(&self) -> f64 {
        2.0 * PI * self.radius as f64
    }
}

pub enum Shape {
    Polygon(Polygon),
    Circle(Circle),
}

impl Shape {
    fn perimeter(&self) -> f64 {
        use Shape::*;

        match self {
            Polygon(n) => n.perimeter(),
            Circle(n) => n.perimeter()
        }
    }
}

impl From<Polygon> for Shape {
    fn from(value: Polygon) -> Self {
        return Shape::Polygon(value);
    }
}

impl From<Circle> for Shape {
    fn from(value: Circle) -> Self {
        return Shape::Circle(value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn round_two_digits(x: f64) -> f64 {
        (x * 100.0).round() / 100.0
    }

    #[test]
    fn test_point_magnitude() {
        let p1 = Point::new(12, 13);
        assert_eq!(round_two_digits(p1.magnitude()), 17.69);
    }

    #[test]
    fn test_point_dist() {
        let p1 = Point::new(10, 10);
        let p2 = Point::new(14, 13);
        assert_eq!(round_two_digits(p1.dist(p2)), 5.00);
    }

    #[test]
    fn test_point_add() {
        let p1 = Point::new(16, 16);
        let p2 = p1 + Point::new(-4, 3);
        assert_eq!(p2, Point::new(12, 19));
    }

    #[test]
    fn test_polygon_left_most_point() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();
        poly.add_point(p1);
        poly.add_point(p2);
        assert_eq!(poly.left_most_point(), Some(p1));
    }

    #[test]
    fn test_polygon_iter() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();
        poly.add_point(p1);
        poly.add_point(p2);

        let points = poly.iter().cloned().collect::<Vec<_>>();
        assert_eq!(points, vec![Point::new(12, 13), Point::new(16, 16)]);
    }

    #[test]
    fn test_shape_perimeters() {
        let mut poly = Polygon::new();
        poly.add_point(Point::new(12, 13));
        poly.add_point(Point::new(17, 11));
        poly.add_point(Point::new(16, 16));
        let shapes = vec![
            Shape::from(poly),
            Shape::from(Circle::new(Point::new(10, 20), 5)),
        ];
        let perimeters = shapes
            .iter()
            .map(Shape::perimeter)
            .map(round_two_digits)
            .collect::<Vec<_>>();
        assert_eq!(perimeters, vec![15.48, 31.42]);
    }
}

#[allow(dead_code)]
fn main() {}