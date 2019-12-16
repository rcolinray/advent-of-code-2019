use std::sync::mpsc;
use std::thread;

use crate::fft::fft;

pub struct Worker {
    output_rx: mpsc::Receiver<Vec<i32>>,
}

impl Worker {
    pub fn start(offset: usize, indices: &Vec<usize>, data: &Vec<i32>) -> Self {
        let (output_tx, output_rx) = mpsc::channel::<Vec<i32>>();

        let local_indices = indices.clone();
        let local_data = data.clone();
        thread::spawn(move || {
            let results = local_indices
                .iter()
                .map(|&index| fft(offset, index, &local_data))
                .collect::<Vec<_>>();

            output_tx
                .send(results)
                .expect("Failed to send results from worker thread");
        });

        Worker { output_rx }
    }

    pub fn get_results(&self) -> Vec<i32> {
        self.output_rx
            .recv()
            .expect("Failed to get results from worker thread")
    }
}
