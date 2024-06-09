use crate::mlp::MlpModel;
use std::os::raw::{c_int, c_double};


pub type ProgressCallback = extern "C" fn(epoch: c_int, loss: c_double);



// _________________________________ Init _________________________________
#[no_mangle]
pub extern "C" fn mlpInit(neural_size: *const usize, len: usize, learning_rate: f64) -> *mut MlpModel 
{
    assert!(!neural_size.is_null(), "Please select a correct configuration");

    let neuron_matrix: Vec<usize> = unsafe 
    {
        std::slice::from_raw_parts(neural_size, len).to_vec()
    };
    Box::into_raw(Box::new(MlpModel::init(neuron_matrix, learning_rate)))
}



// _________________________________ Train _________________________________
#[no_mangle]
pub extern "C" fn mlpTrain(
    model: *mut MlpModel,
    X: *const f64,
    y: *const f64,
    n_samples: usize,
    n_features: usize,
    n_classes: usize,
    epochs: usize,
    batch_size: usize,
    is_classification: bool,
    callback: ProgressCallback,
    callback_interval: usize
)
{
    let model = unsafe { &mut *model };
    let X = unsafe { std::slice::from_raw_parts(X, n_samples * n_features) };
    let y = unsafe { std::slice::from_raw_parts(y, n_samples * n_classes) };
    let X: Vec<Vec<f64>> = X.chunks_exact(n_features).map(|chunk| chunk.to_vec()).collect();
    let y: Vec<Vec<f64>> = y.chunks_exact(n_classes).map(|chunk| chunk.to_vec()).collect();

    model.train(&X, &y, epochs, batch_size, is_classification, callback, callback_interval);
}



// _________________________________ Predict _________________________________

#[no_mangle]
pub extern "C" fn mlpPredict(
    model: *const MlpModel,
    inputs: *const f64,
    n_features: usize,
    is_classification: bool,
    predictions: *mut f64
) 
{
    assert!(!model.is_null(), "Model pointer is null");
    assert!(!inputs.is_null(), "Inputs pointer is null");
    assert!(!predictions.is_null(), "Predictions pointer is null");

    let model = unsafe { &*model };
    let inputs = unsafe { std::slice::from_raw_parts(inputs, n_features) };
    let result = model.predict(inputs, is_classification);

    for (i, &value) in result.iter().enumerate() 
    {
        unsafe {
            *predictions.add(i) = value;
        }
    }
}


// // _________________________________ Forward _________________________________
// #[no_mangle]
// pub extern "C" fn mlpForward(
//     model: *mut MlpModel,
//     inputs_ptr: *const f64,
//     n_inputs: usize,
//     is_classification: bool
// ) -> *mut f64 
// {
//     let model = unsafe { &mut *model };
//     let inputs = unsafe { std::slice::from_raw_parts(inputs_ptr, n_inputs) };

//     let activations = model.forward(inputs, is_classification);
//     let last_activations = activations.last().unwrap();

//     let boxed_activations = last_activations.clone().into_boxed_slice();
//     Box::into_raw(boxed_activations) as *mut f64
// }




// // _________________________________ Backward _________________________________
// #[no_mangle]
// pub extern "C" fn mlpBackward(
//     model: *mut MlpModel,
//     activations_ptr: *const f64,
//     target_ptr: *const f64,
//     n_layers: usize,
//     is_classification: bool
// ) -> f64 
// {
//     let model = unsafe { &mut *model };
//     let activations: Vec<Vec<f64>> = (0..n_layers).map(|i| {
//         let ptr = unsafe { activations_ptr.add(i * n_layers) };
//         let slice = unsafe { std::slice::from_raw_parts(ptr, n_layers) };
//         slice.to_vec()
//     }).collect();
//     let target = unsafe { std::slice::from_raw_parts(target_ptr, n_layers) };

//     model.backward(&activations, &target.to_vec(), is_classification)
// }




// // _________________________________ Update _________________________________
// #[no_mangle]
// pub extern "C" fn mlpUpdateWeights(model: *mut MlpModel, learning_rate: f64) {
//     let model = unsafe { &mut *model };
//     model.optimizer.learning_rate = learning_rate;
// }




// _________________________________ Free _________________________________
#[no_mangle]
pub extern "C" fn mlpFree(model: *mut MlpModel) {
    if !model.is_null() {
        unsafe { Box::from_raw(model); }
    }
}
