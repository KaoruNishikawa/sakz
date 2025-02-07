use std::fs::File;
use std::io::prelude::Write;

use crate::plotters::Plot;
use crate::saver;

use resvg;
use svg::Document;
use tiny_skia;
use tiny_skia_path;
use usvg;

pub enum CoordinateSystem {
    Cartesian,
    Polar,
    Spherical,
    Cylindrical,
}

pub struct Figure<'a> {
    pub title: String,
    pub coordinate: CoordinateSystem,
    pub dim: usize,
    pub size: (f64, f64),
    pub aspect: Option<f64>,
    pub subfigures: Vec<Figure<'a>>,
    pub plots: Vec<&'a dyn Plot>, // TODO: assign valid type
    pub legend: (),               // TODO: assign valid type
}

impl Default for Figure<'_> {
    fn default() -> Self {
        Self {
            title: "Figure".to_string(),
            coordinate: CoordinateSystem::Cartesian,
            dim: 2,
            size: (800.0, 600.0),
            aspect: None,
            subfigures: Vec::new(),
            plots: Vec::new(),
            legend: (),
        }
    }
}

impl<'a> Figure<'a> {
    pub fn new(title: String, dim: usize, coordinate: CoordinateSystem) -> Self {
        if dim != 2 && dim != 3 {
            panic!("Only 2D and 3D plots are supported.");
        }

        let default = Self::default();
        Self {
            title,
            coordinate,
            dim,
            ..default
        }
    }

    fn tree(&self) -> Document {
        let mut tree = Document::new();
        for subfigure in &self.subfigures {
            let subfigure_tree = subfigure.tree();
            tree = tree.add(subfigure_tree);
        }
        for plot in &self.plots {
            tree = tree.add(plot.render(self.size, self.aspect));
        }
        tree
    }

    fn render(&self) -> String {
        self.tree().to_string()
    }

    fn rasterize(&self, include_alpha: bool) -> Vec<u8> {
        let width = self.size.0 as u32;
        let height = self.size.1 as u32;

        let svg_image = self.render();
        let tree = usvg::Tree::from_str(&svg_image, &usvg::Options::default()).unwrap();
        let mut pixmap = tiny_skia::Pixmap::new(width, height).unwrap();
        let mut pixmap = pixmap.as_mut();
        resvg::render(&tree, tiny_skia_path::Transform::identity(), &mut pixmap);

        let mut image_data = pixmap.data_mut().to_owned();

        if !include_alpha {
            // NOTE: Convert to dimmed colors, based on alpha value
            for i in (0..image_data.len()).step_by(4) {
                let r = image_data[i];
                let g = image_data[i + 1];
                let b = image_data[i + 2];
                let a = image_data[i + 3] as f64 / 255.0;

                if a < 1.0 {
                    image_data[i] = (r as f64 * a + 255.0 * (1.0 - a)).round() as u8;
                    image_data[i + 1] = (g as f64 * a + 255.0 * (1.0 - a)).round() as u8;
                    image_data[i + 2] = (b as f64 * a + 255.0 * (1.0 - a)).round() as u8;
                }
            }

            // NOTE: Remove alpha channel
            let mut idx = -1;
            image_data.retain(|_| {
                idx += 1;
                return idx % 4 != 3;
            })
        }

        image_data
    }

    pub fn plot(&mut self, plot: &'a dyn Plot) -> &mut Self {
        self.plots.push(plot);
        self
    }

    pub fn add_subfigure(
        mut self,
        title: String,
        dim: usize,
        coordinate: CoordinateSystem,
    ) -> Self {
        let new_figure = Figure::new(title, dim, coordinate);
        self.subfigures.push(new_figure);
        self
    }

    pub fn save(&self, path: &str) -> Result<(), std::io::Error> {
        let path_upper = path.to_uppercase();
        let extension = path_upper.split('.').last();

        let mut file = File::create(path)?;

        let width = self.size.0 as u32;
        let height = self.size.1 as u32;

        match extension {
            Some("JPG") | Some("JPEG") => {
                let image_data = self.rasterize(false);
                saver::save_jpeg(&mut file, image_data, width, height)?;
            }
            Some("PNG") => {
                let image_data = self.rasterize(true);
                saver::save_png(&mut file, image_data, width, height)?;
            }
            Some("WEBP") => {
                let image_data = self.rasterize(true);
                saver::save_webp(&mut file, image_data, width, height)?;
            }
            Some("PDF") => {
                let svg_image = self.render();
                saver::save_pdf(&mut file, svg_image)?;
            }
            Some("SVG") | None => {
                let svg_image = self.render();
                file.write_all(svg_image.as_bytes())?;
            }
            _ => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Unsupported file format '{}'", extension.unwrap_or("")),
                ));
            }
        }

        Ok(())
    }
}
