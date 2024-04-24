use fltk::image::PngImage;
use std::cell::RefCell;
use std::rc::Rc;
use std::path::Path;

use crate::usg_image::*;

#[derive(Clone, Debug)]
pub struct ImageViewer {
    pub current_pos: Rc<RefCell<usize>>,
    pub current_img:Rc<RefCell<Option<PngImage>>>,
    pub images: Rc<RefCell<Vec<USGImage>>>,
}

impl ImageViewer {
    
    pub fn load_images(&mut self, _paths:  Vec<String>) {

        let mut tmp: Vec<USGImage> = Vec::new();

        for p in _paths
        {
            let mut a:USGImage = USGImage { img: None, width: (None), height: (None), file_name: (None), resize_ratio:0.0};
            a.add_image(p);
            tmp.push(a);
        }
        *self.images.borrow_mut() = tmp;   

    }

    pub fn get_current_img(&mut self) -> Option<PngImage>{

        let res = (*self.current_img.borrow_mut()).clone();
        return res;

    }

    pub fn is_empty(&mut self) -> bool{

        let res = (*self.current_img.borrow_mut()).clone().is_none();
        
        return res;

    }

    pub fn get_current_path(&mut self) -> Option<String>{

        let pos = *self.current_pos.borrow();
        let tmp = (*self.images.borrow_mut())[pos].clone();
        let res = tmp.file_name;

        return res;

    }

    pub fn update_current_img(&mut self) {

        let p =  *self.current_pos.borrow();
        let k = (*self.images.borrow().clone())[p].img.clone();
        
        *self.current_img.borrow_mut() = k;
 
    }

    pub fn increment_pos(&mut self) {

        let size = (*self.images.borrow().clone()).len();
        let pos = *self.current_pos.borrow();

        if pos < size - 1 { 
            *self.current_pos.borrow_mut() += 1;
        }

    }

    pub fn reset_pos(&mut self) {

         *self.current_pos.borrow_mut() = 0;

    }

    pub fn decrement_pos(&mut self) {

        let pos = *self.current_pos.borrow();

        if pos > 0  { 
            *self.current_pos.borrow_mut() -= 1;
        }
        
    }

    pub fn get_current_img_info_text(&self) -> String{

        let pos = *self.current_pos.borrow();
        let size = (*self.images.borrow().clone()).len();
        let all_images = (*self.images.borrow_mut()).clone();
        let curr_image = all_images[pos].clone();
        let img_path = curr_image.file_name.unwrap();
        let filename = Path::new(&img_path).file_name().unwrap();
        let filename = format!("{:?}", filename).replace("\"", "");

        let res = format!("{}/{} | pos: {} width:{}, height: {} | {:?}",pos+1, size, pos, curr_image.width.unwrap(), curr_image.height.unwrap(), &filename).replace("\"", "");


        return res;
    }


}
