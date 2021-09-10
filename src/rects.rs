use std::mem;

use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct Point2D {
    pub x: f32,
    pub y: f32,
}

/// Bounding rectangle.
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct BoundingRect {
    /// Top left corner of the rectangle.
    pub(crate) from: Point2D,
    /// Bottom right corner of the rectangle.
    pub(crate) to: Point2D,
}

impl BoundingRect {
    /// Creates rectangle from the specified points. Points can be specified in any order.
    pub fn from_points(a: Point2D, b: Point2D) -> BoundingRect {
        let (ax, bx) = if a.x < b.x { (a.x, b.x) } else { (b.x, a.x) };
        let (ay, by) = if a.y < b.y { (a.y, b.y) } else { (b.y, a.y) };

        Self {
            from: Point2D { x: ax, y: ay },
            to: Point2D { x: bx, y: by },
        }
    }
}

// Calculates the intersection of the lines.
fn lines_intersection(mut a: (f32, f32), mut b: (f32, f32)) -> Option<(f32, f32)> {
    if b.0 < a.0 {
        mem::swap(&mut a, &mut b);
    }

    let a2 = Some(b.0).filter(|b0| b0 < &a.1)?;
    let b2 = if a.1 < b.1 { a.1 } else { b.1 };

    Some((a2, b2))
}

impl BoundingRect {
    /// Calculates the intersection with another rectangle if the rectangles intersect.
    pub fn intersect(&self, other: &BoundingRect) -> Option<BoundingRect> {
        let (x1, x2) = lines_intersection((self.from.x, self.to.x), (other.from.x, other.to.x))?;
        let (y1, y2) = lines_intersection((self.from.y, self.to.y), (other.from.y, other.to.y))?;

        Some(BoundingRect::from_points(
            Point2D { x: x1, y: y1 },
            Point2D { x: x2, y: y2 },
        ))
    }
}

#[test]
fn test_lines_intersection() {
    let cases = vec![
        (
            (3.0, 7.0),
            (6.0, 10.0),
            Some((6.0, 7.0)),
            "a > b: has intersections",
        ),
        ((3.0, 7.0), (8.0, 10.0), None, "a > b: no intersections"),
        (
            (3.0, 10.0),
            (4.0, 6.0),
            Some((4.0, 6.0)),
            "a > b: line 'a' completely contains line 'b'",
        ),
    ];

    for case in cases {
        assert_eq!(
            lines_intersection(case.0, case.1),
            case.2,
            "Test case \"{}\" has been failed",
            case.3
        );
        assert_eq!(
            lines_intersection(case.1, case.0),
            case.2,
            "Test case \"{}\" (inverted) has been failed",
            case.3
        );
    }
}

#[test]
fn test_rects_intersection() {
    fn assert_rect_is_valid(rect: &BoundingRect) {
        assert!(rect.from.x < rect.to.x && rect.from.y < rect.to.y);
    }

    let cases = vec![
        (
            BoundingRect {
                from: Point2D { x: 1.0, y: 1.0 },
                to: Point2D { x: 5.0, y: 5.0 },
            },
            BoundingRect {
                from: Point2D { x: 3.0, y: 2.0 },
                to: Point2D { x: 6.0, y: 7.0 },
            },
            Some(BoundingRect {
                from: Point2D { x: 3.0, y: 2.0 },
                to: Point2D { x: 5.0, y: 5.0 },
            }),
            "intersection",
        ),
        (
            BoundingRect {
                from: Point2D { x: 1.0, y: 1.0 },
                to: Point2D { x: 10.0, y: 10.0 },
            },
            BoundingRect {
                from: Point2D { x: 3.0, y: 3.0 },
                to: Point2D { x: 5.0, y: 5.0 },
            },
            Some(BoundingRect {
                from: Point2D { x: 3.0, y: 3.0 },
                to: Point2D { x: 5.0, y: 5.0 },
            }),
            "'b' contains 'a'",
        ),
        (
            BoundingRect {
                from: Point2D { x: 1.0, y: 1.0 },
                to: Point2D { x: 5.0, y: 5.0 },
            },
            BoundingRect {
                from: Point2D { x: 6.0, y: 2.0 },
                to: Point2D { x: 7.0, y: 7.0 },
            },
            None,
            "no intersection by x",
        ),
        (
            BoundingRect {
                from: Point2D { x: 1.0, y: 1.0 },
                to: Point2D { x: 5.0, y: 5.0 },
            },
            BoundingRect {
                from: Point2D { x: 3.0, y: 6.0 },
                to: Point2D { x: 6.0, y: 7.0 },
            },
            None,
            "no intersection by y",
        ),
        (
            BoundingRect {
                from: Point2D { x: 1.0, y: 1.0 },
                to: Point2D { x: 5.0, y: 5.0 },
            },
            BoundingRect {
                from: Point2D { x: 6.0, y: 2.0 },
                to: Point2D { x: 7.0, y: 7.0 },
            },
            None,
            "no intersections",
        ),
        (
            BoundingRect {
                from: Point2D { x: 1.0, y: 1.0 },
                to: Point2D { x: 5.0, y: 5.0 },
            },
            BoundingRect {
                from: Point2D { x: 1.0, y: 1.0 },
                to: Point2D { x: 5.0, y: 5.0 },
            },
            Some(BoundingRect {
                from: Point2D { x: 1.0, y: 1.0 },
                to: Point2D { x: 5.0, y: 5.0 },
            }),
            "same rect",
        ),
    ];

    for case in cases {
        assert_rect_is_valid(&case.0);
        assert_rect_is_valid(&case.1);

        assert_eq!(
            case.0.intersect(&case.1),
            case.2,
            "Test case \"{}\" has been failed",
            case.3
        );
        assert_eq!(
            case.1.intersect(&case.0),
            case.2,
            "Test case \"{}\" (inverted) has been failed",
            case.3
        );
    }
}
