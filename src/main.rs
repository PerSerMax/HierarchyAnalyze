mod misc;
use misc::*;
use std::io;
use std::process::exit;

fn main() {
    let countries = read_file("data.txt");
    println!("Введите количество итераций алгоритма (<{}): ", countries.len());

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Не удалось прочитать строку");

    let n = input.trim()
        .parse::<usize>()
        .expect("Пожалуйста введите число");

    if n >= countries.len() {
        println!("Слишком большое число");
        exit(1);
    }

    let mut analyze = Analyze::new(countries);
    let d = analyze.cluster_n_times(n);
    analyze.print();
    println!("Расстояние между последними объединенными кластерами: {d}")
}