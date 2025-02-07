use svg::node::element::Group;

pub trait Plot {
    fn render(&self, figsize: (f64, f64), aspect: Option<f64>) -> Group;
    fn bbox(&self) -> (f64, f64, f64, f64);
    fn color(&mut self, color: &str) -> &mut Self
    where
        Self: Sized;
    fn marker_size(&mut self, size: f64) -> &mut Self
    where
        Self: Sized;
    fn xlim(&mut self, min: f64, max: f64) -> &mut Self
    where
        Self: Sized;
    fn ylim(&mut self, min: f64, max: f64) -> &mut Self
    where
        Self: Sized;
}
