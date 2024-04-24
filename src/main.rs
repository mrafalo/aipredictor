use fltk::{
    app, button::Button, draw::{draw_point, draw_rect, set_draw_color, set_line_style, LineStyle}, enums::{Color, Event}, frame::Frame, image::PngImage, output, prelude::*, surface::ImageSurface, text::SimpleTerminal, window::Window
};

use std::cell::RefCell;
use std::rc::Rc;
use rfd::FileDialog;
use walkdir::WalkDir;
use std::path::Path;
use image::imageops;
use log::info;

use models::{ai_model::AIModel, *};
use crate::consts::*;
use crate::image_repo::ImageRepo;
use crate::usg_image::*;
use crate::ai_model_repo::AIModelRepo;

fn frame2_redraw(_img: Option<image::RgbaImage>, _surf:Rc<RefCell<ImageSurface>>, _frame: Frame){

    let mut f = _frame.clone();
    let surf = _surf.clone();
    let surf = surf.borrow_mut();

    ImageSurface::push_current(&surf);
    if _img  != None {

        let img = Some(PngImage::load(&("tmp.png".to_string())).unwrap());
        let mut img = img.unwrap();

        img.draw(0, 0, f.w(), f.h());

        info!("resized image | width: {}, height: {}", img.width(), img.height() );
    }

    ImageSurface::pop_current();
    f.redraw();
}


fn frame1_redraw(_annotator: ImageRepo, _surf:Rc<RefCell<ImageSurface>>, _frame: Frame){

    let mut annotator_clone = _annotator.clone();
    let mut f = _frame.clone();
    let surf = _surf.clone();
    let surf = surf.borrow_mut();

    ImageSurface::push_current(&surf);
 
    annotator_clone.update_current_img();

    let img = annotator_clone.get_current_img();

    if !img.is_none() {
        let mut img = img.unwrap();
        
        img.draw(f.x(), f.y(), f.w(), f.h());

        ImageSurface::pop_current();
        f.redraw();
    }
}

fn main() {
    std::env::set_var("RUST_LOG", "info");    
    env_logger::init();


    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut wind = Window::new(0, 0, WIDTH, HEIGHT, "Image AI predictor");

    let mut frame2 = Frame::default().with_size(IM_WIDTH_PREV, IM_HEIGHT_PREV).with_pos(IM_WIDTH + 10, 0);
    let mut frame1 = Frame::default().with_size(IM_WIDTH, IM_HEIGHT).with_pos(0, 0);

    let mut but_select =      Button::new(IM_WIDTH + 10, 190 + 0 * (BUTON_HEIGHT + BUTON_SPACING), BUTTON_WIDTH, BUTON_HEIGHT, "Load images");
    let mut but_prev =        Button::new(IM_WIDTH + 10, 190 + 1 * (BUTON_HEIGHT + BUTON_SPACING), BUTTON_WIDTH, BUTON_HEIGHT, "Prev");
    let mut but_next =        Button::new(IM_WIDTH + 10, 190 + 2 * (BUTON_HEIGHT + BUTON_SPACING), BUTTON_WIDTH, BUTON_HEIGHT, "Next");
    let mut but_load_models = Button::new(IM_WIDTH + 10, 190 + 3 * (BUTON_HEIGHT + BUTON_SPACING), BUTTON_WIDTH, BUTON_HEIGHT, "Load models");
    let mut but_detect =      Button::new(IM_WIDTH + 10, 190 + 4 * (BUTON_HEIGHT + BUTON_SPACING), BUTTON_WIDTH, BUTON_HEIGHT, "Detect");
    let mut but_exit =        Button::new(IM_WIDTH + 10, 190 + 11 * (BUTON_HEIGHT + BUTON_SPACING), BUTTON_WIDTH, BUTON_HEIGHT, "Exit");



    let mut bottom_terminal = SimpleTerminal::default().with_pos(0, IM_HEIGHT+30).with_size(IM_WIDTH, 150);
    let status_disp = output::Output::default().with_pos(0, IM_HEIGHT).with_size(IM_WIDTH - 100, 30);
    let mut position_disp = output::Output::default().with_pos(IM_WIDTH-100, IM_HEIGHT).with_size(100, 30);
 

    //let tmp_model = AIModel { model_path: None, model_name: None, model_desc: None };
    let tmp_models: Vec<AIModel> = Vec::new();
    //tmp_models.push(tmp_model);
    let tmp_results: Vec<f32> = Vec::new();
    let models_repo: AIModelRepo = AIModelRepo{models: Rc::from(RefCell::from(tmp_models)), model_results: Rc::from(RefCell::from(tmp_results))};

    //models.load_models();
    frame1.set_color(Color::Red);
    frame2.set_color(Color::Blue);
    bottom_terminal.set_color(Color::Black);
    bottom_terminal.set_text_color(Color::Green);
    
    let im: USGImage = USGImage { img: None, width: None, height: None, file_name: None, resize_ratio: 0.0 };
    let mut imgs: Vec<USGImage> = Vec::new();
    imgs.push(im);
    let imgs2 = Rc::from(RefCell::from(imgs));
    let annotator: ImageRepo = ImageRepo { current_pos: Rc::from(RefCell::from(0)), current_img: Rc::from(RefCell::from(None)), images: imgs2};

    let mut image_paths = Vec::new();

    wind.end();
    wind.show();


    let surf1 = ImageSurface::new(frame1.width(), frame1.height(), false);
    ImageSurface::push_current(&surf1);
    ImageSurface::pop_current();
    let surf1 = Rc::from(RefCell::from(surf1));

    let surf2 = ImageSurface::new(frame2.width(), frame2.height(), false);
    ImageSurface::push_current(&surf2);
    ImageSurface::pop_current();
    let surf2 = Rc::from(RefCell::from(surf2));

    frame2.draw({
        let surf = surf2.clone();
        move |f| {

            let surf = surf.borrow();
            let mut img = surf.image().unwrap();
            img.draw_ext(f.x(), f.y(), f.w(), f.h(),0,0);
        }

    });


    frame1.draw({
        let surf = surf1.clone();
        move |f| {
      
            let surf = surf.borrow();
            let mut img = surf.image().unwrap();
            img.draw(f.x(), f.y(), f.w(), f.h());
        }
    });

    frame1.handle({
        wind.redraw();
        let mut annotator_clone = annotator.clone();
        let mut x = 0;
        let mut y = 0;
        let s1 = surf1.clone();
        let s2 = surf2.clone();
        let f2 = frame2.clone();

        move |f, ev| {
            let s1 = s1.borrow_mut();
            match ev {
                Event::Move => {
                    let mouse_coords = app::event_coords();
                    let txt = format!("x={}, y={}", mouse_coords.0, mouse_coords.1);
                    position_disp.set_value(&txt);

                    true
                }

                Event::Push => {
                    let button_clicked = app::event_button();
                    if button_clicked == 1 {
                        ImageSurface::push_current(&s1);

                        let coords = app::event_coords();
                        x = coords.0;
                        y = coords.1;
                        set_line_style(LineStyle::Solid, 2);
                        set_draw_color(Color::Red);
                        draw_point(x, y);
                        ImageSurface::pop_current();
                        f.redraw();
                    }

                    true
                }

                Event::Released =>
                {
                    ImageSurface::push_current(&s1);
                    
                    let coords = app::event_coords();
                    
                    let img_path = annotator_clone.get_current_path().unwrap();
                    let (w, h) = (((coords.0-x) as u32), ((coords.1-y) as u32));

                    let img = image::open(Path::new(&img_path)).unwrap();
                    let mut img = img.resize(IM_WIDTH as u32, IM_HEIGHT as u32, imageops::FilterType::Nearest);
                    let sub_img = imageops::crop(&mut img, x as u32, y as u32, w, h);

                    let tmp = sub_img.to_image();

                    let tmp = imageops::resize(&tmp,IM_WIDTH_PREV as u32, IM_HEIGHT_PREV as u32, imageops::FilterType::Nearest);
                    tmp.save(&Path::new("tmp.png")).unwrap();  
                    
                    set_line_style(LineStyle::Dot, 2);
                    set_draw_color(Color::Red);
                    draw_rect(x, y, coords.0-x, coords.1-y);
                    
                    ImageSurface::pop_current();
                    f.redraw();
                    
                    frame2_redraw(Some(tmp), s2.clone(), f2.clone());

                    true
                }

                Event::Drag => {
                    if !annotator_clone.get_current_img().is_none() {

                        ImageSurface::push_current(&s1);
                        
                        set_draw_color(Color::Red);
                        set_line_style(LineStyle::Solid, 2);

                        let coords = app::event_coords();

                        annotator_clone.get_current_img().unwrap().draw(f.x(), f.y(), f.w(), f.h());
                        draw_rect(x, y, coords.0-x, coords.1-y);

                        ImageSurface::pop_current();
                        f.redraw();
                    }

                    true
                }
                _ => false,
            }
        }
    });

    but_select.set_callback({
        let mut annotator_clone = annotator.clone();
        let mut but_next_clone = but_next.clone();
       
        move |_| {
            let new_folder_path = FileDialog::new().set_directory(".").pick_folder();
            if !new_folder_path.is_none(){
                let source_path = new_folder_path.unwrap();
                image_paths.clear();

                for e in WalkDir::new(&source_path).into_iter().filter_map(|e| e.ok()) {
                
                    let file_path = String::from(e.path().to_str().unwrap());
                    let file_path_clone = file_path.clone();

                    if file_path_clone.to_lowercase().ends_with(".png") {
                        image_paths.push(file_path_clone);
                        
                    }
                }
                annotator_clone.load_images(image_paths.clone());
                annotator_clone.reset_pos();
                annotator_clone.update_current_img();
    
                but_next_clone.do_callback();
            }
            
        }

    });

    but_next.set_callback({ 
        let mut status_disp = status_disp.clone();
        let  f = frame1.clone();
        let mut annotator_clone = annotator.clone();
        let surf = surf1.clone();
        move |_| {    

            if !annotator_clone.is_empty() {
                annotator_clone.increment_pos();
                frame1_redraw(annotator_clone.clone(), surf.clone(), f.clone());

                let txt = format!("{}", annotator_clone.get_current_img_info_text());
               
                status_disp.set_value(&txt);

            }
        }
    });

    but_prev.set_callback({
        let f = frame1.clone();
        let mut annotator_clone = annotator.clone();
        let surf = surf1.clone();
        let mut status_disp = status_disp.clone();
        move |_| {

            if !annotator_clone.is_empty() {
                annotator_clone.decrement_pos();
                frame1_redraw(annotator_clone.clone(), surf.clone(), f.clone());
                let txt = format!("{}", annotator_clone.get_current_img_info_text());
                status_disp.set_value(&txt);
            }
        }
    });

    but_exit.set_callback({

        move |_| {  

            std::process::exit(0);     
        }
        
    });

    but_load_models.set_callback({
        let mut models_repo_clone = models_repo.clone();

        move |_| {

            let new_folder_path = FileDialog::new().set_directory(".").pick_folder();

            if !new_folder_path.is_none(){
                let source_path = new_folder_path.unwrap();
                models_repo_clone.load_models(source_path.to_str().unwrap());
            }

        }
    });

    but_detect.set_callback({

        let mut bottom_terminal = bottom_terminal.clone();
        let mut models_repo_clone = models_repo.clone();
        let mut annotator_clone = annotator.clone();

        move |_| {
            if !(models_repo_clone.is_empty()) & !(annotator_clone.is_empty()){
                let txt = models_repo_clone.predict_multi_models();
                bottom_terminal.append(&txt)
            }


        }
    });

    app.run().unwrap();

}

