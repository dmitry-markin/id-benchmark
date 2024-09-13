use litep2p::PeerId;
use std::{fs, time::Instant};

const DEFAULT_ITERATIONS: u64 = 1_000_000;

fn main() {
    let iterations = std::env::args()
        .skip(1)
        .next()
        .map(|s| str::parse(&s).ok())
        .flatten()
        .unwrap_or(DEFAULT_ITERATIONS);

    let mut phantom = 0u8;

    let begin = Instant::now();

    for _ in 0..iterations {
        let peer_id = PeerId::random();

        phantom = phantom
            ^ peer_id
                .to_bytes()
                .last()
                .expect("`PeerId` to have non-zero length");
    }

    let elapsed = begin.elapsed();
    let elapsed_msec = elapsed.as_millis();

    let rate = iterations as f64 / elapsed.as_secs_f64();

    println!("{elapsed_msec} ms to generate {iterations} IDs");
    println!("{rate:.0} IDs/s");

    let _ = fs::write("/dev/null", "{phantom}");
}
