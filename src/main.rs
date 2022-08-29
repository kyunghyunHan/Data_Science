use ndarray::prelude::*;
use ndarray_rand::rand;
use ndarray_rand::rand_distr::{StandardNormal, Uniform};
use ndarray_rand::{RandomExt, SamplingStrategy};
use ndarray_stats::histogram::{strategies::Sqrt, GridBuilder};
use ndarray_stats::HistogramExt;
use noisy_float::types::{n64, N64};
use rand::seq::IteratorRandom;

fn main() {
    let arr17 = Array::<f64, _>::random_using((10000, 2), StandardNormal, &mut rand::thread_rng());
    let data = arr17.mapv(|e| n64(e));
    let grid = GridBuilder::<Sqrt<N64>>::from_array(&data).unwrap().build();
    let histogram = data.histogram(grid);
    let histogram_matrix = histogram.counts();
    let data = histogram_matrix.sum_axis(Axis(0));
    let his_data: Vec<(f32, f32)> = data
        .iter()
        .enumerate()
        .map(|(e, i)| (e as f32, *i as f32))
        .collect();
    let file = std::fs::File::create("standard_normal_hist.svg").unwrap();
    let mut graph = poloto::plot("Histogram", "x", "y");
    graph
        .histogram("Stand.Norm.Dist.", his_data)
        .xmarker(0)
        .ymarker(0);
    graph.simple_theme(poloto::upgrade_write(file));

    let arr18 = Array::<f64, _>::random_using((300, 2), StandardNormal, &mut rand::thread_rng());

    let data: Vec<(f64, f64)> = arr18
        .axis_iter(Axis(0))
        .map(|e| {
            let v = e.to_vec();
            (v[0], v[1])
        })
        .collect();
    let x_line = [[-3, 0], [3, 0]];
    let y_line = [[0, -3], [0, 3]];
    let file = std::fs::File::create("standard_normal_scatter.svg").unwrap(); // create file on disk

    let mut graph = poloto::plot("Scatter Plot", "x", "y"); // create graph
    graph.line("", &x_line);
    graph.line("", &y_line);
    graph.scatter("Stand.Norm.Dist.", data).ymarker(0);

    graph.simple_theme(poloto::upgrade_write(file));
}
