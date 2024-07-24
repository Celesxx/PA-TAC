# import ctypes
# import numpy as np
from django.http import JsonResponse
from django.views import View
from django.views.decorators.csrf import csrf_exempt
from django.utils.decorators import method_decorator
import json
# from tqdm.notebook import tqdm
import channels.layers
from asgiref.sync import async_to_sync
from application.tasks.mlp_train import mlp_train

class mlpView(View):

    @method_decorator(csrf_exempt)
    def dispatch(self, *args, **kwargs):
        return super().dispatch(*args, **kwargs)
    
    def post(self, request, *args, **kwargs):
        # if request.path_info == '/api/mlp/initialisation/':
        #     return self.initialisation(request)
        # elif request.path_info == '/api/mlp/train/':
        #     return self.train(request)
        if request.path_info == '/api/mlp/':
            return self.mlp(request)
        return JsonResponse({'error': 'Invalid action'}, status=400)

    def get(self, request, *args, **kwargs):
        if request.path_info == '/api/status/':
            # if self.model_instance:
            return JsonResponse({'status': 'Modèle initialisé'})
            # else:
                # return JsonResponse({'status': 'Modèle non initialisé'})
        return JsonResponse({'error': 'Invalid action'}, status=400)

    # def initialisation(self, request):
    #     try:
    #         data = json.loads(request.body.decode('utf-8'))
    #     except json.JSONDecodeError as e:
    #         return JsonResponse({'error': f'Invalid JSON: {e}'}, status=400)

    #     try:
    #         param_dict = {param['label']: param for param in data['parametres']}
    #         neuronnes = param_dict['Neuronnes']['value']
    #         dataset_name = param_dict['Neuronnes']['dataset']
    #         learning_rate = param_dict['Learning rate']['value']

    #         parameter = np.array(neuronnes, dtype=np.uintp)
    #         parameter_ptr = parameter.ctypes.data_as(ctypes.POINTER(ctypes.c_size_t))
    #         size = parameter.size

    #         mlp_model = ctypes.CDLL("../../modele/mlp/target/release/libmlp_classification.so")

    #         mlp_model.mlpInit.argtypes = [ctypes.POINTER(ctypes.c_size_t), ctypes.c_size_t, ctypes.c_double]
    #         mlp_model.mlpInit.restype = ctypes.c_void_p

    #         model_instance = mlp_model.mlpInit(parameter_ptr, size, learning_rate)
    #         return JsonResponse({'message': 'Modèle initialisé avec succès', 'data': {'model_instance': model_instance, 'neuronnes': neuronnes, 'dataset_name': dataset_name}})
        
    #     except KeyError as e:
    #         return JsonResponse({'error': f'Paramètre manquant: {e}'}, status=400)
    #     except Exception as e:
    #         return JsonResponse({'error': str(e)}, status=500)


    # def train(self, request):
    #     try:
    #         data = json.loads(request.body.decode('utf-8'))
    #     except json.JSONDecodeError as e:
    #         return JsonResponse({'error': f'Invalid JSON: {e}'}, status=400)

    #     try:
    #         print("test2")
    #         result = mlp_train.delay(data)
    #         return JsonResponse({'message': 'Training started', 'task_id': result.id})
        
    #     except KeyError as e:
    #         return JsonResponse({'error': f'Paramètre manquant: {e}'}, status=400)
    #     except Exception as e:
    #         return JsonResponse({'error': str(e)}, status=500)
        

    def mlp(self, request):
        try:
            data = json.loads(request.body.decode('utf-8'))
        except json.JSONDecodeError as e:
            return JsonResponse({'error': f'Invalid JSON: {e}'}, status=400)

        try:
            result = mlp_train.delay(data)
            return JsonResponse({'message': 'Training started', 'task_id': result.id})
        
        except KeyError as e:
            return JsonResponse({'error': f'Paramètre manquant: {e}'}, status=400)
        except Exception as e:
            return JsonResponse({'error': str(e)}, status=500)