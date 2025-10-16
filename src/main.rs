fn main() {
    let time_conv = [
        "plancktijd",
        "seconde",
        "min",
        "kwartier",
        "uur",
        "etmaal",
        "weekend",
        "trimester",
        "tropisch jaar",
    ];

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
        "lichtetmaal",
        "lichtweekend",
        "lichttrimester",
        "lichttropisch jaar",
    ];

    let time_s = 90.0;
    for op in time_conv {
        println!("{}s = {} {}", time_s, convert_seconds(time_s, op).unwrap(), op);
    }

    println!();

    let dist_m = 125.0;
    for op in dist_conv {
        println!("{}m = {} {}", dist_m, convert_meters(dist_m, op).unwrap(), op);
    }

    println!();

    let v = 10.0;
    let v2 = convert_speed(v, "meter", "uur").unwrap();
    println!("{} m/s = {} m/u", v, v2);

    let a = 10.0;
    let a2 = convert_acceleration(a, "meter", "uur").unwrap();
    println!("{} m/s^2 = {} m/u^2", a, a2);
}

pub fn convert_seconds(t: f64, to: &str) -> Option<f64> {
    match to {
        "plancktijd"            => Some(t * 10f64.powi(44) / 5.39124),
        "seconde"               => Some(t),
        "min"                   => Some(t / 60.),
        "kwartier"              => Some(t / 900.),
        "uur"                   => Some(t / 3600.),
        "etmaal"                => Some(t / 86400.),
        "weekend"               => Some(t / 172800.),
        "trimester"             => Some(t / 2880000.),
        "tropisch jaar"         => Some(t / 31556926.08),
        _                       => None
    }
}

pub fn convert_meters(x: f64, to: &str) -> Option<f64>{
    match to {
        "plancklengte" => Some(x * 10f64.powi(35) / 1.616255),
        "angstrom"     => Some(x * 1000000000.),
        "meter"        => Some(x),
        "inch"         => Some(x / 0.0254),
        "zeemijl"      => Some(x / 1852.),
        "AE"           => Some(x / 149597870700.),
        "lichtjaar"    => Some(x / 9460730472580800.),
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

pub fn convert_speed(v: f64, dist: &str, time: &str) -> Option<f64> {
    let d = convert_meters(1., dist)?;
    let t = convert_seconds(1., time)?;
    Some(v * d / t)
}

pub fn convert_acceleration(a: f64, dist: &str, time: &str) -> Option<f64> {
    let d = convert_meters(1., dist)?;
    let t = convert_seconds(1., time)?;
    Some(a * d / t.powi(2))
}
