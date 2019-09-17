use std::fmt;

#[derive(Debug)]
struct City {
    name: &'static str,
    lat: f32,
    lng: f32,
}

impl fmt::Display for City {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let lat_caption = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lng_caption = if self.lng >= 0.0 { 'E' } else { 'W' };

        write!(f, "{city}: {lat:0.3}°{lat_caption} {lng:0.3}°{lng_caption}",
            city=self.name,
            lat=self.lat.abs(),
            lat_caption=lat_caption,
            lng=self.lng.abs(),
            lng_caption=lng_caption)
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RGB ({0}, {1}, {2}) 0x{0:02X}{1:02X}{2:02X}", self.red, self.green, self.blue)
    }
}

pub fn run() {
    let cities = [
        City { name: "Dublin", lat: 53.347778, lng: -6.259722 },
        City { name: "Oslo", lat: 59.95, lng: 10.75 },
        City { name: "Vancouver", lat: 49.25, lng: -123.1 },
    ];

    for city in cities.iter() {
        println!("{}", city);
    }

    let colors = [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ];

    for color in colors.iter() {
        println!("{}", color);
    }
}