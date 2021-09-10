use std::{fs::File, path::PathBuf};

use serde::{Deserialize, Serialize};
use sophya_prog_test::{list_intersections, BoundingRect, Point2D, Shape};
use structopt::StructOpt;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Input {
    pub objects: Vec<Object>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Object {
    pub name: String,
    pub width: f32,
    pub height: f32,
    pub x: f32,
    pub y: f32,
    #[serde(default)]
    pub properties: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ObjectArea {
    name: String,
    area: BoundingRect,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ObjectIntersection {
    names: (String, String),
    area: BoundingRect,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Output {
    areas: Vec<ObjectArea>,
    intersections: Vec<ObjectIntersection>,
}

impl Object {
    fn area(&self) -> ObjectArea {
        let rect = BoundingRect::from_points(
            Point2D {
                x: self.x,
                y: self.y,
            },
            Point2D {
                x: self.x + self.width,
                y: self.y + self.height,
            },
        );

        ObjectArea {
            name: self.name.clone(),
            area: rect,
        }
    }
}

impl Shape for ObjectArea {
    fn bounding_rect(&self) -> BoundingRect {
        self.area
    }
}

/// Searches for intersecting objects in the given input file
#[derive(StructOpt)]
struct Args {
    /// Input file (*.json)
    input_file: PathBuf,
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("File {path:?} could to be openned due to: {reason:}")]
    FileReadError {
        path: PathBuf,
        reason: anyhow::Error,
    },
    #[error("Parse error: {reason:}")]
    ParseError { reason: anyhow::Error },
}

impl Args {
    fn run(self) -> Result<Output, Error> {
        let file = File::open(&self.input_file).map_err(|err| Error::FileReadError {
            path: self.input_file,
            reason: err.into(),
        })?;

        let input: Input = serde_json::from_reader(file)
            .map_err(|err| Error::ParseError { reason: err.into() })?;

        let areas = input.objects.iter().map(Object::area).collect::<Vec<_>>();
        let intersections = list_intersections(&areas)
            .iter()
            .map(|x| ObjectIntersection {
                area: x.area,
                names: (areas[x.a_idx].name.clone(), areas[x.b_idx].name.clone()),
            })
            .collect();

        Ok(Output {
            areas,
            intersections,
        })
    }
}

fn main() -> anyhow::Result<()> {
    let output = Args::from_args().run()?;
    println!("{}", serde_json::to_string_pretty(&output).unwrap());
    Ok(())
}
