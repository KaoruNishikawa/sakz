use core::f64;

use svg::node::element::Group;

use super::common::Plot;
use super::utils;

pub struct Scatter {
    data: Vec<(f64, f64, f64)>,
    color: String,
    marker_size: f64,
    xlim: (f64, f64),
    ylim: (f64, f64),
}

impl Default for Scatter {
    fn default() -> Self {
        Self {
            data: vec![],
            color: "black".to_string(),
            marker_size: 5.0,
            xlim: (f64::NEG_INFINITY, f64::INFINITY),
            ylim: (f64::NEG_INFINITY, f64::INFINITY),
        }
    }
}

impl Scatter {
    pub fn new_2d(data: Vec<(f64, f64)>) -> Self {
        let default = Self::default();
        Self {
            data: data.iter().map(|(x, y)| (*x, *y, 0.0)).collect(),
            ..default
        }
    }

    pub fn new_3d(data: Vec<(f64, f64, f64)>) -> Self {
        let default = Self::default();
        Self { data, ..default }
    }
}

impl Plot for Scatter {
    fn render(&self, figsize: (f64, f64), aspect: Option<f64>) -> Group {
        let (scale, shift) = utils::get_scale_factors(figsize, self.bbox(), aspect);

        let mut scatter_plot = Group::new();
        // TODO: Implement reducer from 3D to 2D
        for (x, y, _) in &self.data {
            let circle = svg::node::element::Circle::new()
                .set("cx", *x * scale.0 + shift.0)
                .set("cy", *y * scale.1 + shift.1)
                .set("r", self.marker_size)
                .set("fill", self.color.clone());
            scatter_plot = scatter_plot.add(circle);
        }

        // TODO: This bbox is just for debugging, so remove this later
        // let (min_x, max_x, min_y, max_y) = self.bbox();
        // let rect = svg::node::element::Rectangle::new()
        //     .set("x", min_x * scale.0 + shift.0)
        //     .set("y", min_y * scale.1 + shift.1)
        //     .set("width", (max_x - min_x) * scale.0)
        //     .set("height", (max_y - min_y) * scale.1)
        //     .set("fill", "none")
        //     .set("stroke", "black")
        //     .set("stroke-width", 1);
        // scatter_plot = scatter_plot.add(rect);

        scatter_plot
    }

    fn bbox(&self) -> (f64, f64, f64, f64) {
        // NOTE: Approach bounding values from opposite extremes
        let mut min_x = f64::INFINITY;
        let mut max_x = f64::NEG_INFINITY;
        let mut min_y = f64::INFINITY;
        let mut max_y = f64::NEG_INFINITY;

        for (x, y, _) in &self.data {
            min_x = min_x.min(*x);
            max_x = max_x.max(*x);
            min_y = min_y.min(*y);
            max_y = max_y.max(*y);
        }
        // NOTE: At this point, min_* should be less than or equal to max_*,
        // unless data is empty

        // NOTE: Clip to xlim and ylim
        min_x = min_x.max(self.xlim.0);
        max_x = max_x.min(self.xlim.1);
        min_y = min_y.max(self.ylim.0);
        max_y = max_y.min(self.ylim.1);

        // NOTE: Keep margin
        let margin = 0.1;
        let mut dx = (max_x - min_x) * margin;
        let mut dy = (max_y - min_y) * margin;
        dx = if dx.is_finite() { dx } else { 1.0 };
        dy = if dy.is_finite() { dy } else { 1.0 };

        (min_x - dx, max_x + dx, min_y - dy, max_y + dy)
    }

    fn color(&mut self, color: &str) -> &mut Self
    where
        Self: Sized,
    {
        self.color = color.to_string();
        self
    }

    fn marker_size(&mut self, size: f64) -> &mut Self
    where
        Self: Sized,
    {
        self.marker_size = size;
        self
    }

    fn xlim(&mut self, min: f64, max: f64) -> &mut Self
    where
        Self: Sized,
    {
        if min > max {
            panic!("min must be less than or equal to max");
        }
        self.xlim = (min, max);
        self
    }

    fn ylim(&mut self, min: f64, max: f64) -> &mut Self
    where
        Self: Sized,
    {
        if min > max {
            panic!("min must be less than or equal to max");
        }
        self.ylim = (min, max);
        self
    }
}
