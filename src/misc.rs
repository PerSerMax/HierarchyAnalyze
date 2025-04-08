use std::collections::HashMap;
use std::{fs, io, ptr};

#[derive(Debug, Clone)]
pub struct Country  {
    pub name: String,
    pub attrs: Vec<f64>,
}

#[derive(Debug, Clone)]
pub struct Cluster {
    pub countries: Vec<Country>
}

impl Cluster {
    pub fn new() -> Cluster {
        Cluster { countries: Vec::new() }
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
                    lc = &i;
                    rc = &j;
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
    pub fn new(countries: Vec<Country>) -> Analyze {
        let mut clusters: Vec<Cluster> = Vec::new();
        for i in countries {
            let mut c = Cluster::new();
            c.add(i);
            clusters.push(c);
        }
        Analyze { clusters }
    }
    fn range(a: &Vec<f64>, b: &Vec<f64>) -> f64 {
        let mut result = 0.0;
        for i in 0..a.len() {
            result += (a[i] - b[i]).powi(2)
        }
        result
    }
    fn nearest_clusters(&self) -> (&Cluster, &Cluster, f64) {
        let mut min_lc: &Cluster = &self.clusters[0];
        let mut min_rc: &Cluster = &self.clusters[1];
        let mut min_dist: f64 = f64::MAX;
        for lcluster in &self.clusters {
            for rcluster in &self.clusters {
                if ptr::eq(lcluster, rcluster) { continue; }
                let (_lc, _rc, dist) = Cluster::range(lcluster, rcluster);
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
        let mut new_clusters: Vec<Cluster> = Vec::new();
        for i in &self.clusters {
            if ptr::eq(i, lc) || ptr::eq(i, rc) {
                continue;
            }
            new_clusters.push(i.clone());
        }
        new_clusters.push(Cluster::union(lc.clone(), rc.clone()));
        self.clusters = new_clusters;
        dist
    }
    pub fn cluster_n_times(&mut self, n: usize) -> f64{
        let mut result = 0.0;
        for _ in 0..n { result = self.nearest_union(); }
        result
    }
    pub fn print(&self) {
        for i in 0..self.clusters.len() {
            println!("Cluster {}: [", i+1);
            for country in &self.clusters[i].countries {
                println!("\t{},", country.name);
            }
            println!("]");
        }
    }
}
pub fn read_file(filename: &str) -> Vec<Country> {
    let mut countries_map: HashMap<String, Vec<f64>> = HashMap::new();
    let data = fs::read_to_string(filename).unwrap_or_else(|e| {
        match e.kind() {
            io::ErrorKind::NotFound => println!("File not found"),
            io::ErrorKind::PermissionDenied => println!("Permission denied"),
            _ => println!("I/O error: {}", e),
        }
        std::process::exit(1); // Завершаем программу с ошибкой
    });
    for line in data.lines() {
        let mut line = line.trim().split("\t");
        let country_name: String = line.next().unwrap().to_string();
        let country_attrs = line
            .map(|s|
                s.trim()
                    .parse::<f64>()
                    .unwrap())
            .collect();
        countries_map.insert(country_name, country_attrs);
    }
    let mut countries: Vec<Country> = Vec::new();
    for (country, attrs) in countries_map {
        countries.push(Country { name: country, attrs });
    }
    countries
}
