use std::collections::HashMap;
use std::{fs, io};

#[derive(Debug, Clone)]
pub struct Country {
    pub name: String,
    pub attrs: Vec<f64>,
}

#[derive(Debug, Clone)]
pub struct Cluster {
    pub countries: Vec<Country>,
}

impl Cluster {
    pub fn new() -> Cluster {
        Cluster {
            countries: Vec::new(),
        }
    }
    pub fn add(&mut self, country: Country) {
        self.countries.push(country);
    }
    pub fn range<'a>(a: &'a Cluster, b: &'a Cluster) -> (&'a Country, &'a Country, f64) {
        let mut min: f64 = f64::MAX;
        let mut lc: &Country = &a.countries[0];
        let mut rc: &Country = &b.countries[0];
        for i in &a.countries {
            for j in &b.countries {
                let d = Analyze::range(&i.attrs, &j.attrs);
                if d < min {
                    min = d;
                    lc = i;
                    rc = j;
                }
            }
        }
        (lc, rc, min)
    }
    pub fn union(mut a: Cluster, mut b: Cluster) -> Cluster {
        let mut c = Cluster::new();
        c.countries.append(&mut a.countries);
        c.countries.append(&mut b.countries);
        c
    }
}

#[derive(Debug)]
pub struct Analyze {
    pub clusters: Vec<Cluster>,
}

impl Analyze {
    pub fn new(mut countries: Vec<Country>, standartize: bool) -> Analyze {
        if standartize {
            std_countries(&mut countries);
        }
        let mut clusters: Vec<Cluster> = Vec::new();
        for i in countries {
            let mut c = Cluster::new();
            c.add(i);
            clusters.push(c);
        }
        Analyze { clusters }
    }
    fn range(a: &[f64], b: &[f64]) -> f64 {
        let mut result = 0.0;
        for i in 0..a.len() {
            result += (a[i] - b[i]).powi(2)
        }
        result
    }
    fn nearest_clusters(&self) -> (usize, usize, f64) {
        let mut min_lc = 0;
        let mut min_rc = 1;
        let mut min_dist: f64 = f64::MAX;
        for lcluster in 0..self.clusters.len() {
            for rcluster in lcluster + 1..self.clusters.len() {
                let (_lc, _rc, dist) =
                    Cluster::range(&self.clusters[lcluster], &self.clusters[rcluster]);
                if dist < min_dist {
                    min_dist = dist;
                    min_lc = lcluster;
                    min_rc = rcluster;
                }
            }
        }
        (min_lc, min_rc, min_dist)
    }
    fn nearest_union(&mut self) -> f64 {
        let (lc, rc, dist) = self.nearest_clusters();
        let new_cluster = Cluster::union(self.clusters[lc].clone(), self.clusters[rc].clone());
        self.clusters[lc] = new_cluster;
        self.clusters.remove(rc);
        dist
    }
    pub fn cluster_n_times(&mut self, n: usize) -> f64 {
        let mut result = 0.0;
        for _ in 0..n {
            result = self.nearest_union();
            print!("{result:.2}, ")
        }
        println!();
        result
    }
    pub fn print(&self) {
        for i in 0..self.clusters.len() {
            println!("Cluster {}: [", i + 1);
            for country in &self.clusters[i].countries {
                print!("\t{}: ", country.name);
                for j in &country.attrs {
                    print!("{:.1}, ", j);
                }
                println!();
            }
            println!("]");
        }
    }
}

pub fn avg(v: &[f64]) -> f64 {
    v.iter().sum::<f64>() / v.len() as f64
}

pub fn std_val(v: &[f64]) -> f64 {
    let avg = avg(v);
    (v.iter().map(|x| (x - avg).powi(2)).sum::<f64>() / v.len() as f64).sqrt()
}

pub fn std_countries(v: &mut [Country]) {
    if v.is_empty() || v[0].attrs.is_empty() {
        return;
    }
    let num_attrs = v[0].attrs.len();
    let num_countries = v.len();
    let mut columns: Vec<Vec<f64>> = vec![Vec::with_capacity(num_countries); num_attrs];
    for country in v.iter() {
        for (i, val) in country.attrs.iter().enumerate() {
            columns[i].push(*val);
        }
    }
    for i in 0..num_attrs {
        let avg_i = avg(&columns[i]);
        let std_i = std_val(&columns[i]);
        for j in 0..num_countries {
            if std_i == 0.0 {
                v[j].attrs[i] = 0.0;
            } else {
                v[j].attrs[i] = (v[j].attrs[i] - avg_i) / std_i;
            }
        }
    }
}

pub fn read_file(filename: &str, sep: &str) -> Vec<Country> {
    let mut countries_map: HashMap<String, Vec<f64>> = HashMap::new();
    let data = fs::read_to_string(filename).unwrap_or_else(|e| {
        match e.kind() {
            io::ErrorKind::NotFound => println!("File not found"),
            io::ErrorKind::PermissionDenied => println!("Permission denied"),
            _ => println!("I/O error: {}", e),
        }
        std::process::exit(1); // Завершаем программу с ошибкой
    });
    for (idx, line) in data.lines().enumerate() {
        let mut line = line.trim().split(sep);
        let country_name: String = line.next().unwrap().to_string();
        line.next();
        let country_attrs = line
            .map(|s| {
                s.trim().parse::<f64>().unwrap_or_else(|e| {
                    println!("Ошибка {e} на строке {idx} в этой штуке {s}");
                    std::process::exit(1);
                })
            })
            .collect();
        countries_map.insert(country_name, country_attrs);
    }
    let mut countries: Vec<Country> = Vec::new();
    for (country, attrs) in countries_map {
        countries.push(Country {
            name: country,
            attrs,
        });
    }
    countries
}
