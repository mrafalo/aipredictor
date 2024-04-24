use log::info;
use tensorflow::{Graph, SavedModelBundle, SessionOptions, SessionRunArgs, Tensor};
use std::path::Path;
use crate::consts::*;

#[derive(Debug, Clone)]
pub struct AIModel {
    pub model_path: String,
    pub model_name: String,
    pub model_desc: String,

}

impl AIModel {

    pub fn load_model(&mut self, _model_path:  String, _model_name: String, _model_desc: String) {
        self.model_desc = _model_desc;
        self.model_name = _model_name;
        self.model_path = _model_path;
    }

    pub fn print_model_info(&mut self){

        let mut graph = Graph::new();
        let bundle = SavedModelBundle::load(&SessionOptions::new(), &["serve"], &mut graph, self.model_path.clone()).unwrap();

        let signature = bundle
            .meta_graph_def()
            .get_signature("serving_default")
            .unwrap();

        for t1 in signature.inputs().iter(){
            println!("layer: {}", t1.0);
        }

    }

    pub fn predict(&mut self, _image_path: &str) -> f32 {

        let img = image::open(&Path::new(_image_path)).unwrap();
        info!("image to predict size: {} {}", img.width(), img.height());
        let img = img.to_luma8();
        let img = img.to_vec();
        let mut vec8: Vec<f32> = vec![];

        for elem in img {
            vec8.push(elem as f32 / 255.0)
        }
        let img = vec8;

        let tensor: Tensor<f32> = Tensor::new(&[1, IM_WIDTH_PREV as u64, IM_HEIGHT_PREV as u64, 1])
            .with_values(&img)
            .expect("Can't create tensor");

        info!("input tensor shape: {}", &tensor.shape());
        let input_tensor = tensor;
        let mut graph = Graph::new();

        let bundle = SavedModelBundle::load(&SessionOptions::new(), &["serve"], &mut graph, self.model_path.clone()).unwrap();

        
        let session = &bundle.session;
        
        
        let signature = bundle
            .meta_graph_def()
            .get_signature("serving_default")
            .unwrap();

        let input_name = (signature.inputs()).clone();
        let input_name = input_name.keys().last().unwrap();

        let output_name = signature.outputs();
        let output_name = output_name.keys().last().unwrap();

        let input_info = signature.get_input(input_name).unwrap();
        let output_info = signature.get_output(output_name).unwrap();

        info!("input shape: {}", input_info.shape());
        info!("output shape: {}", output_info.shape());

        // Get input/output ops from graph
        let input_op = graph.operation_by_name_required(&input_info.name().name).unwrap();
        let output_op = graph.operation_by_name_required(&output_info.name().name).unwrap();

        // Manages inputs and outputs for the execution of the graph
        let mut args = SessionRunArgs::new();
        args.add_feed(&input_op, 0, &input_tensor); 
        let out = args.request_fetch(&output_op, 0); 

        // Run model
        session.run(&mut args).expect("Error occurred during calculations");
        info!("run model OK...");
        let out_res1: f32 = args.fetch(out).unwrap()[0];
        info!("Results: {:?}", out_res1);
        
        return out_res1;

    }


}

