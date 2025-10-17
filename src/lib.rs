use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn get_time_formats() -> Vec<String> {
    ["plancktijd", "seconde", "min", "kwartier", "uur", "dag", "weekend", "trimester", "jaar"]
        .iter()
        .map(|s| s.to_string())
        .collect()
}

#[wasm_bindgen]
pub fn get_distance_formats() -> Vec<String> {
    [
        "plancklengte",
        "angstrom",
        "meter",
        "inch",
        "zeemijl",
        "AE",
        "parsec",
        "lichtseconde",
        "lichtmin",
        "lichtkwartier",
        "lichtuur",
        "lichtdag",
        "lichtweekend",
        "lichttrimester",
        "lichtjaar"
    ]
        .iter()
        .map(|s| s.to_string())
        .collect()
}


#[wasm_bindgen]
pub fn convert_seconds(t: f64, to: &str) -> Option<f64> {
    match to {
        "plancktijd"            => Some(t * 10f64.powi(44) / 5.39124),
        "seconde"               => Some(t),
        "min"                   => Some(t / 60.),
        "kwartier"              => Some(t / 900.),
        "uur"                   => Some(t / 3600.),
        "dag"                   => Some(t / 86400.),
        "weekend"               => Some(t / 172800.),
        "trimester"             => Some(t / 2880000.),
        "jaar"                  => Some(t / 31556926.08),
        _                       => None
    }
}

#[wasm_bindgen]
pub fn convert_meters(x: f64, to: &str) -> Option<f64>{
    match to {
        "plancklengte" => Some(x * 10f64.powi(35) / 1.616255),
        "angstrom"     => Some(x * 1000000000.),
        "meter"        => Some(x),
        "inch"         => Some(x / 0.0254),
        "zeemijl"      => Some(x / 1852.),
        "AE"           => Some(x / 149597870700.),
        "parsec"       => Some(x / (149597870700. * 648000. / std::f64::consts::PI)),
        _              => {
            if to.starts_with("licht") {
                let r = convert_seconds(1., &to[5..])?;
                Some(x * r / 299792458.)
            } else {
                None
            }
        }
    }
}

#[wasm_bindgen]
pub fn convert_speed(v: f64, dist: &str, time: &str) -> Option<f64> {
    let d = convert_meters(1., dist)?;
    let t = convert_seconds(1., time)?;
    Some(v * d / t)
}

#[wasm_bindgen]
pub fn convert_acceleration(a: f64, dist: &str, time: &str) -> Option<f64> {
    let d = convert_meters(1., dist)?;
    let t = convert_seconds(1., time)?;
    Some(a * d / t.powi(2))
}

// sub score {
//     my $snum = "$_[0]";
//     my @spl = split(/\./, $snum);
//     if ($spl[0] == "0") {
//         my ($lead0) = $spl[1] =~ /^(0*)/;
//         return length($lead0) + 1;
//     }
//     return length($spl[0]);
// }

fn score(num: f64) -> usize {
    let leading_0_pennalty = 2;
    let snum = num.to_string();
    let (front, back) = snum.split_once('.').unwrap_or((&snum, ""));
    if front == "0" {
        return back.chars().take_while(|&c| c == '0').count() + leading_0_pennalty;
    }
    return front.len();
}

fn best_result_1<G>(v: f64, formats: Vec<String>, convert: G) -> BestResult
where
    G: Fn(f64, &str) -> Option<f64>,
{
    let mut best_score = std::usize::MAX;
    let mut best_format = String::new();
    let mut best_result = 0.;

    for f in formats {
        let res = convert(v, &f).unwrap();
        let score = score(res);
        if score < best_score {
            best_score = score;
            best_format = f;
            best_result = res;
        }
    }

    BestResult::new(best_result, best_format)
}

fn best_result_2<G>(v: f64, d_fmt: Vec<String>, t_fmt: Vec<String>, convert: G, acc: bool) -> BestResult
where
    G: Fn(f64, &str, &str) -> Option<f64>,
{
    let mut best_score = std::usize::MAX;
    let mut best_format = String::new();
    let mut best_result = 0.;

    for d in d_fmt {
        for t in &t_fmt {
            let res = convert(v, &d, &t).unwrap();
            let score = score(res);
            if score < best_score {
                best_score = score;
                best_format = d.clone();
                best_format.push('/');
                best_format.push_str(&t);
                if acc { best_format.push_str("^2") }
                best_result = res;
            }
        }
    }

    BestResult::new(best_result, best_format)
}
#[wasm_bindgen]
pub struct BestResult {
    pub result: f64,
    #[wasm_bindgen(getter_with_clone)]
    pub unit: String
}

#[wasm_bindgen]
impl BestResult {
    #[wasm_bindgen(constructor)]
    pub fn new(result: f64, unit: String) -> BestResult {
        BestResult { result, unit }
    }
}

#[wasm_bindgen]
pub fn find_best(v: f64, mode: &str) -> BestResult {
    match mode {
        "time" => {
            let time_formats = get_time_formats();
            best_result_1(v, time_formats, convert_seconds)
        }
        "distance" => {
            let dist_formats = get_distance_formats();
            best_result_1(v, dist_formats, convert_meters)
        }
        "speed" => {
            let dist_formats = get_distance_formats();
            let time_formats = get_time_formats();
            best_result_2(v, dist_formats, time_formats, convert_speed, false)
        }
        "acceleration" => {
            let dist_formats = get_distance_formats();
            let time_formats = get_time_formats();
            best_result_2(v, dist_formats, time_formats, convert_acceleration, true)
        }
        _ => BestResult::new(-1., String::from(mode))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn print_time() {
        let time_conv = [
            "plancktijd",
            "seconde",
            "min",
            "kwartier",
            "uur",
            "dag",
            "weekend",
            "trimester",
            "jaar",
        ];

        let time_s = 90.0;
        for op in time_conv {
            println!("{}s = {} {}", time_s, convert_seconds(time_s, op).unwrap(), op);
        }

    }

    #[test]
    fn print_dist() {
        let dist_conv = [
            "plancklengte",
            "angstrom",
            "meter",
            "inch",
            "zeemijl",
            "AE",
            "lichtjaar",
            "parsec",
            "lichtplancktijd",
            "lichtmin",
            "lichtkwartier",
            "lichtuur",
            "lichtdag",
            "lichtweekend",
            "lichttrimester",
            "lichtjaar",
        ];
        let dist_m = 125.0;
        for op in dist_conv {
            println!("{}m = {} {}", dist_m, convert_meters(dist_m, op).unwrap(), op);
        }
    }

    #[test]
    fn print_speed() {
        let v = 10.0;
        let v2 = convert_speed(v, "meter", "uur").unwrap();
        println!("{} m/s = {} m/u", v, v2);
    }

    #[test]
    fn print_acceleration() {
        let a = 10.0;
        let a2 = convert_acceleration(a, "meter", "uur").unwrap();
        println!("{} m/s^2 = {} m/u^2", a, a2);
    }
}
