pub use rects::{BoundingRect, Point2D};

mod rects;

/// A Common shape.
pub trait Shape {
    /// Returns the bounding rectangle of this shape.
    fn bounding_rect(&self) -> BoundingRect;
    /// Calculates the intersection with another shape if the shapes intersect.
    fn intersection(&self, other: &Self) -> Option<BoundingRect> {
        self.bounding_rect().intersect(&other.bounding_rect())
    }
}

/// Shapes intersection summary.
#[derive(Debug, Clone, PartialEq)]
pub struct Intersection {
    /// Shapes intersection area.
    pub area: BoundingRect,
    /// First shape index.
    pub a_idx: usize,
    /// Second shape index.
    pub b_idx: usize,
}

/// Searches for intersecting shapes in the specified list.
/// 
/// Note that this method uses a naive O(n^2) method to find shapes intersection.
pub fn list_intersections<S: Shape>(objects: &[S]) -> Vec<Intersection> {
    let mut intersections = Vec::new();
    for i in 0..objects.len() {
        for j in (i + 1)..objects.len() {
            if let Some(area) = objects[i].intersection(&objects[j]) {
                intersections.push(Intersection {
                    area,
                    a_idx: i,
                    b_idx: j,
                })
            }
        }
    }

    intersections
}

#[test]
fn test_objects_intersection() {
    struct TestShape {
        x: f32,
        y: f32,
        w: f32,
        h: f32,
    }

    impl Shape for TestShape {
        fn bounding_rect(&self) -> BoundingRect {
            BoundingRect::from_points(
                Point2D {
                    x: self.x,
                    y: self.y,
                },
                Point2D {
                    x: self.x + self.w,
                    y: self.y + self.h,
                },
            )
        }
    }

    let objects = vec![
        TestShape {
            x: 1.0,
            y: 1.0,
            w: 4.0,
            h: 4.0,
        },
        TestShape {
            x: 2.0,
            y: 2.0,
            w: 1.0,
            h: 1.0,
        },
        TestShape {
            x: 3.0,
            y: -1.0,
            w: 2.0,
            h: 6.0,
        },
        TestShape {
            x: -2.0,
            y: -5.0,
            w: 1.0,
            h: 1.0,
        },
    ];

    let expected = [
        Intersection {
            area: BoundingRect {
                from: Point2D { x: 2.0, y: 2.0 },
                to: Point2D { x: 3.0, y: 3.0 },
            },
            a_idx: 0,
            b_idx: 1,
        },
        Intersection {
            area: BoundingRect {
                from: Point2D { x: 3.0, y: 1.0 },
                to: Point2D { x: 5.0, y: 5.0 },
            },
            a_idx: 0,
            b_idx: 2,
        },
    ];

    let actual = list_intersections(&objects);
    assert_eq!(actual, expected);
}
