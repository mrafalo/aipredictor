
use std::{cell::RefCell, rc::Rc};
use chrono::{DateTime, Utc};
use log::info;
use std::fs::read_dir;

use crate::ai_model::AIModel;

#[derive(Debug, Clone)]
pub struct AIModelRepo {
    pub model_results: Rc<RefCell<Vec<f32>>>,
    pub models: Rc<RefCell<Vec<AIModel>>>
}


impl AIModelRepo {

    pub fn get_models_info(&mut self) -> String{

        let mut res = String::from("");
        let m_tmp = (*self.models.borrow_mut()).clone();
        for m in m_tmp{
            res = res + m.model_name.unwrap().as_str() + "\n" + m.model_desc.unwrap().as_str() + "\n" + m.model_path.unwrap().as_str() + "\n";
            res = res + "--------------\n";

        }
        return res;
    }

    pub fn get_models_count(&mut self) -> usize{

        let tmp = (*self.models.borrow_mut()).clone();
   
        return tmp.len();
    }

    pub fn is_empty(&mut self) -> bool{

        let mut res = true;

        if self.get_models_count() > 0
        {
            res = false
        }
        return res;

    }

    pub fn load_models(&mut self, _model_path: &str) {

        let mut tmp: Vec<AIModel> = Vec::new();

        let paths = read_dir(_model_path).unwrap();

        for e in paths {
            let path = e.unwrap();

            let file_path = String::from(path.path().to_str().unwrap());
           
            let file_name = path.file_name();
            info!("model: {}", file_name.to_string_lossy());
            
            let a:AIModel = AIModel { model_path: Some(String::from(file_path)), model_name: Some(String::from(file_name.to_string_lossy())), model_desc: Some(String::from("todo"))};
            tmp.push(a);
        }

       info!("loaded {} models", tmp.len());  
       *self.models.borrow_mut() = tmp; 

       
    }


    pub fn predict_multi_models(&mut self) -> String{

        let models = (*self.models.borrow_mut()).clone();
        let mut txt:String = String::from("predictions:\n");

        for mut m in models.into_iter(){

            let resm = m.predict("tmp.png");
            let p3 = format!("{:.1$}\n", resm, 3);                
            let now: DateTime<Utc> = Utc::now();
            let p1 = now.format("%Y-%m-%d %T").to_string();
            let p2 = String::from(m.model_name.unwrap().as_str().to_owned());

            txt = format!("{txt}{p1}| {p2}: {p3}");


        }

        return txt;

    }

}

