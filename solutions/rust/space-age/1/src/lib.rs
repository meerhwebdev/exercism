#[derive(Debug)]
pub struct Duration {
    age:u64
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration {
            age:s
        }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        todo!("convert a duration ({d:?}) to the number of years on this planet for that duration");
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

macro_rules! impl_planet {
    (
        $(
        $struct_name:ident {
            orbital_age:$ordital_age:expr
        }
        ) ,* $(,)?
    )
    =>{
        $(
            impl Planet for $struct_name {
            fn years_during(d: &Duration)-> f64 {
                let age:f64 = d.age as f64  / (60 as f64 * 60 as f64 * 24 as f64 * 365.25 * $ordital_age as f64);
                let rounded_age = (age * 100.0).round() / 100.0;
                rounded_age
            }
            }
        )*
    };
}

impl_planet!{
    Mercury {orbital_age:0.2408467},
    Venus {orbital_age:0.61519726},
    Earth {orbital_age:1.0},
    Mars {orbital_age:1.8808158},
    Jupiter {orbital_age:11.862615},
    Saturn {orbital_age:29.447498},
    Uranus {orbital_age:84.016846},
    Neptune {orbital_age:164.79132}
}