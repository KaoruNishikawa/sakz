# sakz

[![Test status](https://img.shields.io/github/actions/workflow/status/KaoruNishikawa/sakz/test.yml?branch=main&logo=github&label=Test&style=flat-square)](https://github.com/KaoruNishikawa/sakz/actions)
![License](https://img.shields.io/crates/l/ahash)

Fast and simple plotting library.

## Installation

```shell
cargo add sakz
```

```shell
# If you're using pip:
pip install sakz

# If you're using poetry:
poetry add sakz

# If you're using uv:
uv add sakz
```

## Usage

```rust
use sakz;
use sakz::Plot;

use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let x: Vec<f64> = (0..100).map(|_| rng.gen_range(0.0..10.0)).collect();
    let y: Vec<f64> = (0..100).map(|_| rng.gen_range(0.0..10.0)).collect();
    let data = x.iter().zip(y.iter()).map(|(x, y)| (*x, *y)).collect();

    let mut fig = sakz::Figure::new("Test".to_string(), 2, sakz::CoordinateSystem::Cartesian);
    let mut scatter = sakz::Scatter::new_2d(data);
    scatter.color("#39f").marker_size(3.0);
    _ = fig.plot(&scatter);
    fig.save("example.svg").unwrap();
}
```

---

This library is using [Semantic Versioning](https://semver.org).
