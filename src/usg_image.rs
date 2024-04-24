use fltk::{prelude::*, image::PngImage};
use std::fmt;
use crate::consts::*;

#[derive(Clone, Debug)]
pub struct RecCoords{
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

#[derive(Clone, Debug)]
pub struct PointCoords{
    pub x: i32,
    pub y: i32,
}

impl fmt::Display for PointCoords {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

#[derive(Clone, Debug)]
pub struct USGImage{
    pub img: Option<PngImage>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub file_name: Option<String>,
    pub resize_ratio: f32
}

impl USGImage {

    pub fn add_image(&mut self, _file_name: String) {
        
        self.file_name =  Some(_file_name.clone());   
        let mut img = PngImage::load(&_file_name).unwrap();
        let base_width= img.width();
        img.scale(IM_WIDTH, IM_HEIGHT, true, false);
        let scaled_width = img.width();

        self.width = Some(img.width());
        self.height = Some(img.height());
        self.img = Some(img);  
        self.resize_ratio = (scaled_width as f32)/(base_width as f32); 
    }
}
