import ctypes
import numpy as np
from django.http import JsonResponse
from django.views import View
from django.views.decorators.csrf import csrf_exempt
from django.utils.decorators import method_decorator
import json
from tqdm.notebook import tqdm
import channels.layers
from asgiref.sync import async_to_sync

class mlpView(View):
    model_instance = None
    dataset = None

    @method_decorator(csrf_exempt)
    def dispatch(self, *args, **kwargs):
        return super().dispatch(*args, **kwargs)
    
    def post(self, request, *args, **kwargs):
        if request.path_info == '/api/initialisation/':
            return self.initialisation(request)
        elif request.path_info == '/api/train/':
            if self.model_instance:
                return self.train(request)
            else:
                return JsonResponse({'error': 'Modèle non initialisé'}, status=400)
        return JsonResponse({'error': 'Invalid action'}, status=400)

    def get(self, request, *args, **kwargs):
        if request.path_info == '/api/status/':
            if self.model_instance:
                return JsonResponse({'status': 'Modèle initialisé'})
            else:
                return JsonResponse({'status': 'Modèle non initialisé'})
        return JsonResponse({'error': 'Invalid action'}, status=400)

    def load_datasets(self):
        with open('path_to/datasets.json', 'r') as f:
            datasets = json.load(f)
        return datasets

    def initialisation(self, request):
        try:
            data = json.loads(request.body.decode('utf-8'))
        except json.JSONDecodeError as e:
            return JsonResponse({'error': f'Invalid JSON: {e}'}, status=400)

        try:
            learning_rate = data['parametres'][0]['value']
            neuronnes = data['parametres'][1]['value']
            self.dataset_name = data['dataset']

            parameter = np.array(neuronnes, dtype=np.uintp)
            parameter_ptr = parameter.ctypes.data_as(ctypes.POINTER(ctypes.c_size_t))
            size = parameter.size

            mlp_model = ctypes.CDLL("path_to_your_so_file/libmlp_classification.so")

            mlp_model.mlpInit.argtypes = [ctypes.POINTER(ctypes.c_size_t), ctypes.c_size_t, ctypes.c_double]
            mlp_model.mlpInit.restype = ctypes.c_void_p

            self.model_instance = mlp_model.mlpInit(parameter_ptr, size, learning_rate)
            return JsonResponse({'message': 'Modèle initialisé avec succès'})
        
        except KeyError as e:
            return JsonResponse({'error': f'Paramètre manquant: {e}'}, status=400)
        except Exception as e:
            return JsonResponse({'error': str(e)}, status=500)

    def train(self, request):
        try:
            data = json.loads(request.body.decode('utf-8'))
        except json.JSONDecodeError as e:
            return JsonResponse({'error': f'Invalid JSON: {e}'}, status=400)

        try:
            PROGRESS_CALLBACK = ctypes.CFUNCTYPE(None, ctypes.c_int, ctypes.c_double)
            
            mlp_model = ctypes.CDLL("path_to_your_so_file/libmlp_classification.so")
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

            datasets = self.load_datasets()
            dataset = next((ds for ds in datasets['datasets'] if ds['name'] == self.dataset_name), None)
            if dataset is None:
                return JsonResponse({'error': 'Dataset not found'}, status=400)

            X_train = np.array(dataset['X'], dtype=np.double)
            y_train = np.array(dataset['Y'], dtype=np.double)

            param_dict = {param['label']: param for param in data['parametres']}
            epochs = param_dict['Epochs']['value']
            batch_size = param_dict['Batch size']['value']
            classification = param_dict['Classification']['value']
            log_enable = param_dict['Log']['enable']
            callback_interval = param_dict['Callback']['value']
            checkpoint_enable = False
            checkpoint_interval = 0
            log_tag = None

            progress_bar = tqdm(total=epochs, desc="Training Progress")
            def progress_callback(epoch, loss):
                progress_bar.update(callback_interval)
                progress_bar.set_postfix(loss=loss)

                channel_layer = channels.layers.get_channel_layer()
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

            callback_func = PROGRESS_CALLBACK(progress_callback)

            mlp_model.mlpTrain(
                self.model_instance,
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
                log_tag.encode() if log_tag else None,
            )
            return JsonResponse({'message': 'Training completed'})
        
        except KeyError as e:
            return JsonResponse({'error': f'Paramètre manquant: {e}'}, status=400)
        except Exception as e:
            return JsonResponse({'error': str(e)}, status=500)
