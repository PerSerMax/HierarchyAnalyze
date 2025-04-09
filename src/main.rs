mod misc;
use misc::*;
use std::io;
use std::process::exit;
use std::time;

fn main() {
    let countries = read_file("data.txt", "\t");
    println!(
        "Введите количество итераций алгоритма (<{}): ",
        countries.len()
    );

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Не удалось прочитать строку");

    let begin_time = time::Instant::now();
    let n = input
        .trim()
        .parse::<usize>()
        .expect("Пожалуйста введите число");

    if n >= countries.len() {
        println!("Слишком большое число");
        exit(1);
    }

    let mut analyze = Analyze::new(countries, true);
    let d = analyze.cluster_n_times(n);

    analyze.print();

    println!("Расстояние между последними объединенными кластерами: {d}");
    let end_time = time::Instant::now();
    println!(
        "Время выполнения: {} секунд!\nА питон так сможет?? ХАХАХАХАХА",
        end_time.duration_since(begin_time).as_secs_f64()
    );
}
