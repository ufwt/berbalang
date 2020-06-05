use serde::Serialize;

use crate::configure::Config;
use crate::evolution::{Genome, Phenome};
use crate::fitness::Pareto;
use crate::observer::Window;
use crate::roper::creature::Creature;
use crate::util::count_min_sketch::DecayingSketch;

#[derive(Serialize, Clone, Debug, Default)]
pub struct StatRecord {
    pub counter: usize,
    pub avg_len: f64,
    pub avg_genetic_freq: f64,
    pub avg_scalar_fitness: f64,
    // TODO: how to report on fitness vectors?
    // #[serde(flatten)]
    pub avg_exec_count: f64,
    pub avg_exec_ratio: f64,
    // Fitness scores // TODO find a way to make this more flexible
    pub avg_place_error: f64,
    pub avg_value_error: f64,
    pub avg_crash_count: f64,
}

impl StatRecord {
    fn from_window(window: &Window<Creature>, counter: usize) -> Self {
        let frame = &window.frame;
        log::info!("default report function");
        let avg_len = frame.iter().map(|c| c.len()).sum::<usize>() as f64 / frame.len() as f64;
        let mut sketch = DecayingSketch::default();
        for g in frame.iter() {
            g.record_genetic_frequency(&mut sketch);
        }
        let avg_genetic_freq = frame
            .iter()
            .map(|g| g.measure_genetic_frequency(&sketch))
            .sum::<f64>()
            / frame.len() as f64;
        let avg_scalar_fitness: f64 =
            frame.iter().filter_map(|g| g.scalar_fitness()).sum::<f64>() / frame.len() as f64;
        let fitnesses = frame.iter().filter_map(|g| g.fitness()).collect::<Vec<_>>();
        let fit_vec = Pareto::average(&fitnesses);

        let avg_exec_count: f64 = frame
            .iter()
            .map(|g| g.num_alleles_executed() as f64)
            .sum::<f64>()
            / frame.len() as f64;
        let avg_exec_ratio: f64 =
            frame.iter().map(|g| g.execution_ratio()).sum::<f64>() / frame.len() as f64;

        StatRecord {
            counter,
            avg_len,
            avg_genetic_freq,
            avg_scalar_fitness,
            avg_exec_count,
            avg_exec_ratio,
            // fitness scores
            // TODO: it would be nice if this were less hard-coded
            avg_place_error: fit_vec["place_error"],
            avg_value_error: fit_vec["value_error"],
            avg_crash_count: fit_vec["crash_count"],
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_stat_record_serialization() {
        let record = StatRecord::default();
        let file = std::io::stderr();
        let mut writer = csv::WriterBuilder::new()
            .delimiter(b'\t')
            .terminator(csv::Terminator::Any(b'\n'))
            .has_headers(false)
            .from_writer(file);

        writer
            .serialize(&record)
            .expect("Failed to serialize record!");
        writer.flush().expect("Failed to flush");
    }
}

/// the CSV serializer can't handle IndexMaps, and so it will choke
/// on the Pareto struct. This is where a bit of tinkering will be
/// needed, in the event that the (dynamically determined) pareto format
/// is changed. If a key is missing, for example, the converter will
/// panic, which should make it easy to spot the problem.
#[derive(Serialize, Clone, Debug, Default)]
pub struct Objectives {
    pub place_error: f64,
    pub value_error: f64,
    pub crash_count: f64,
}

impl From<&Pareto<'static>> for Objectives {
    fn from(p: &Pareto<'static>) -> Self {
        Self {
            place_error: p["place_error"],
            value_error: p["value_error"],
            crash_count: p["crashes"],
        }
    }
}

pub fn report_fn(window: &Window<Creature>, counter: usize, _params: &Config) {
    let record = StatRecord::from_window(window, counter);

    log::info!("{:#?}", record);

    window.log_record(record);
    window.dump_soup();
    window.dump_population();

    log::info!("[{}] Reigning champion: {:#?}", counter, window.best);
}