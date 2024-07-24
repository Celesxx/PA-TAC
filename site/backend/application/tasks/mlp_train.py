import ctypes
import numpy as np
from celery import shared_task
import json
from tqdm.notebook import tqdm
import channels.layers
from asgiref.sync import async_to_sync
# import matplotlib.pyplot as plt
import logging

logger = logging.getLogger(__name__)

@shared_task
def mlp_train(data):
    
    param_dict = {param['label']: param for param in data['parametres']}
    neuronnes = param_dict['Neuronnes']['value']
    dataset_name = param_dict['Neuronnes']['dataset']
    learning_rate = param_dict['Learning rate']['value']
    epochs = param_dict['Epochs']['value']
    batch_size = param_dict['Batch size']['value']
    classification = param_dict['Classification']['enable']
    callback_interval = param_dict['Callback']['value']
    checkpoint_enable = False
    checkpoint_interval = 10
    log_enable = param_dict['Log']['enable']
    log_tag = 'f{dataset_name}-mlp'.encode('utf-8')

    parameter = np.array(neuronnes, dtype=np.uintp)
    parameter_ptr = parameter.ctypes.data_as(ctypes.POINTER(ctypes.c_size_t))
    size = parameter.size

    mlp_model = ctypes.CDLL("../../modele/mlp/target/release/libmlp_classification.so")

    mlp_model.mlpInit.argtypes = [ctypes.POINTER(ctypes.c_size_t), ctypes.c_size_t, ctypes.c_double]
    mlp_model.mlpInit.restype = ctypes.c_void_p

    PROGRESS_CALLBACK = ctypes.CFUNCTYPE(None, ctypes.c_int, ctypes.c_double)
    
    mlp_model.mlpTrain.argtypes = [
        ctypes.c_void_p,
        ctypes.POINTER(ctypes.c_double),
        ctypes.POINTER(ctypes.c_double),
        ctypes.c_size_t,
        ctypes.c_size_t,
        ctypes.c_size_t,
        ctypes.c_size_t,
        ctypes.c_size_t,
        ctypes.c_bool,
        PROGRESS_CALLBACK,
        ctypes.c_size_t,
        ctypes.c_bool,
        ctypes.c_size_t,
        ctypes.c_bool,
        ctypes.c_char_p
    ]

    mlp_model.mlpTrain.restype = None

    model_instance = mlp_model.mlpInit(parameter_ptr, size, learning_rate)
    datasets ={}
    with open('datasets.json', 'r') as f:
        datasets = json.load(f)
        
    print(dataset_name)
    dataset = next((ds for ds in datasets['datasets'] if ds['name'] == dataset_name), None)
    if dataset is None:
        return {'error': 400, 'msg': 'Dataset not found'}

    X_train = np.array(dataset['X'], dtype=np.double)
    y_train = np.array(dataset['Y'], dtype=np.double)

    
    loss_values = []
    # progress_bar = tqdm(total=epochs, desc="Training Progress")
    channel_layer = channels.layers.get_channel_layer()
    logger.info(channel_layer)
    def progress_callback(epoch, loss):
        # progress_bar.update(callback_interval)
        # progress_bar.set_postfix(loss=loss)
        # logger.info(f"Epoch: {epoch}, Loss: {loss}")
        loss_values.append(loss)
        if channel_layer is not None:
            # logger.info(f"Epoch: {epoch}, Loss: {loss}")
            async_to_sync(channel_layer.group_send)(
                'progress_group',
                {
                    'type': 'send_progress',
                    'message': {
                        'epoch': epoch,
                        'loss': loss
                    }
                }
            )
        else:
            logger.error("Channel layer is None")

    callback_func = PROGRESS_CALLBACK(progress_callback)

    result = mlp_model.mlpTrain(
        model_instance,
        X_train.ctypes.data_as(ctypes.POINTER(ctypes.c_double)),
        y_train.ctypes.data_as(ctypes.POINTER(ctypes.c_double)),
        X_train.shape[0],
        X_train.shape[1],
        y_train.shape[1],
        epochs,
        batch_size,
        classification,
        callback_func,
        callback_interval,
        checkpoint_enable,
        checkpoint_interval,
        log_enable,
        log_tag,
    )

    # progress_bar.close()

    predictions_list_train = []
    labels_list_train = []
    for k in range(len(X_train)):
        predictions = np.zeros(y_train.shape[1], dtype=np.float64)
        mlp_model.mlpPredict(
            model_instance,
            X_train[k].ctypes.data_as(ctypes.POINTER(ctypes.c_double)),
            X_train[k].size,
            True,
            predictions.ctypes.data_as(ctypes.POINTER(ctypes.c_double))
        )
        print("Prediction: ", predictions)
        predictions_list_train.append(predictions)
        labels_list_train.append(y_train[k])
    
    predictions_array_train = np.array(predictions_list_train)
    labels_array_train = np.array(labels_list_train)
    accuracy = np.mean(np.argmax(predictions_array_train, axis=1) == np.argmax(labels_array_train, axis=1))
    print(f"Accuracy train data: {accuracy * 100:.2f}%")

    mlp_model.mlpFree(model_instance)

    return {
        'status': 200, 'msg' : 
        'Training completed', 
        'result' : f"label {predictions_array_train} : predicted {labels_array_train}",
        'accuracy': f"Accuracy : {accuracy * 100:.2f}%",
        'data': {
            'prediction': predictions_array_train.tolist(),
            'label': labels_array_train.tolist(),
            'loss': loss_values,
        }
    }
