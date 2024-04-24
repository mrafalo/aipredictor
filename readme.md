The program was created in Rust, using the following libraries:
* chrono = "0.4.38"
* config = "0.13.3"
* config-file = "0.2.3"
* csv = "1.2.1"
* env_logger = "0.11.3"
* fltk = "1.3.34"
* image = "0.24.5"
* imageproc = "0.23.0"
* log = "0.4.20"
* rfd = "0.11.2"
* serde = "1.0"
* tensorflow = "0.21.0"
* walkdir = "2.3.3"

Note, that keras models must be saved in *SavedModel* format ([link](https://www.tensorflow.org/tutorials/keras/save_and_load)). 

The program's GUI consists of one window with the following buttons:
* Load images - select folder containing png images; images are then loaded 
* Next - show next image 
* Prev - show previous image
* Load models - select folder containing tensorflow models in *SavedModel* format; models are loaded
* Detect - selected region of image is transformed (resized, and rescaled) to model input shape
* Exit - exits program