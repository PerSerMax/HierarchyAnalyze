mod misc;
use misc::*;

fn main() {
    let n = 27;
    let countries = read_file("data.txt");
    let mut analyze = Analyze::new(countries);
    let d = analyze.cluster_n_times(n);
    analyze.print();
    println!("{d}")
}