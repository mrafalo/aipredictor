
use std::borrow::BorrowMut;
use log::info;
use std::fs::read_dir;

use crate::ai_model::AIModel;

#[derive(Debug, Clone)]
pub struct AIModelRepo {
    pub models: Option<Vec<AIModel>>,
    pub model_results: Option<Vec<f32>>
}


impl AIModelRepo {

    pub fn get_models_info(&mut self) -> String{

        let mut res = String::from("");
        let m_tmp = self.models.borrow_mut().clone().unwrap();

        for m in m_tmp{
            res = res + m.model_name.as_str() + "\n" + m.model_desc.as_str() + "\n" + m.model_path.as_str() + "\n";
            res = res + "--------------\n";

        }
        
        return res;

    }

    pub fn get_models_count(&mut self) -> usize{

        let tmp =  self.clone().models;
        let mut res = 0;

        if !(tmp.is_none()){
        
            res = tmp.unwrap().len();
            println!("res: {:?}", res);

        }
        return res;
    }

    pub fn is_empty(&mut self) -> bool{

        let res = (*self.models.borrow_mut()).clone().is_none();
        
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
            
            let a:AIModel = AIModel { model_path: String::from(file_path), model_name: String::from(file_name.to_string_lossy()), model_desc: String::from("todo")};
            tmp.push(a);
        }

       *self.models.borrow_mut() = Some(tmp);   
    }

}

