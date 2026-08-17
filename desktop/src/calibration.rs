use opencv::{core, imgproc, prelude::*};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    enum_map::Enum,
    serde::Serialize,
    serde::Deserialize,
    strum::EnumIter,
)]
pub enum ColorClass {
    White,
    Yellow,
    Red,
    Orange,
    Blue,
    Green,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    enum_map::Enum,
    serde::Serialize,
    serde::Deserialize,
    strum::EnumIter,
)]
pub enum Location {
    Topleft,
    Topright,
    Botleft,
    Botright,
}

type LocationMap = enum_map::EnumMap<Location, [i32; 2]>;
type ColorMap = enum_map::EnumMap<ColorClass, [f32; 3]>;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CalibrationData {
    colors: ColorMap,
    locations: LocationMap,
}

impl CalibrationData {
    pub fn lookup_closest_color(&self, color_bgr: core::Vec3b) -> ColorClass {
        let color_lab = bgr_to_lab(color_bgr);
        let (k, _v) = self
            .colors
            .iter()
            .min_by(|&(_i, a), &(_j, b)| {
                distance_squared(*a, color_lab).total_cmp(&distance_squared(*b, color_lab))
            })
            .unwrap();
        k
    }
    pub fn get_color(&self, color: ColorClass) -> core::Vec3b {
        lab_to_bgr(self.colors[color])
    }
    pub fn set_color(&mut self, color: ColorClass, color_bgr: core::Vec3b) {
        self.colors[color] = bgr_to_lab(color_bgr);
    }
    pub fn get_location(&mut self, location: Location) -> core::Point2i {
        core::Point2i::from_vec2(self.locations[location].into())
    }
    pub fn set_location(&mut self, location: Location, p: core::Point2i) {
        self.locations[location] = p.to_vec2().0
    }
}

fn bgr_to_lab(bgr_u8: core::Vec3b) -> [f32; 3] {
    let bgr_float = core::Vec3f::from_array(bgr_u8.map(|i| i as f32 / 255.0));
    let bgr_float = &[bgr_float];
    let bgr_mat = core::Mat::new_rows_cols_with_data(1, 1, bgr_float).unwrap();
    let mut lab_mat = core::Mat::default();
    imgproc::cvt_color(&bgr_mat, &mut lab_mat, imgproc::COLOR_BGR2Lab, 3).unwrap();
    let lab_float = *lab_mat.at_2d::<core::Vec3f>(0, 0).unwrap();
    lab_float.0
}
fn lab_to_bgr(lab_float: [f32; 3]) -> core::Vec3b {
    let lab_float = &[core::Vec3f::from_array(lab_float)];
    let lab_mat = core::Mat::new_rows_cols_with_data(1, 1, lab_float).unwrap();
    let mut bgr_mat = core::Mat::default();
    imgproc::cvt_color(&lab_mat, &mut bgr_mat, imgproc::COLOR_Lab2BGR, 3).unwrap();
    let bgr_float = *bgr_mat.at_2d::<core::Vec3f>(0, 0).unwrap();
    let bgr_u8 = core::Vec3b::from(bgr_float.map(|i| (i * 255.0) as _));
    bgr_u8
}

fn distance_squared(a: [f32; 3], b: [f32; 3]) -> f32 {
    let diff = [0, 1, 2].map(|i| a[i] - b[i]);
    let diff_sq = diff.map(|i| i * i);
    let dot = diff_sq.iter().sum();
    dot
}
