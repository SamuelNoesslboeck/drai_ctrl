use syunit::*;

pub const PIXEL_PER_MM : f32 = 6.0; 

#[derive(Copy, Clone, serde::Serialize, serde::Deserialize)]
pub struct Line {
    pub p1 : [f32; 2],
    pub p2 : [f32; 2]
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct LinesFile {
    pub contour : Vec<Line>
}

pub fn load_points(path : &str) -> LinesFile {
    serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap()
}

pub fn convert_pixel(pixel : f32) -> Phi {
    Phi(pixel / PIXEL_PER_MM)
}

pub fn convert_line(line : Line) -> [[Phi; 2]; 2] {
    [
        [ convert_pixel(line.p1[0]), convert_pixel(line.p1[1]) ],
        [ convert_pixel(line.p2[0]), convert_pixel(line.p2[1]) ]
    ]
}