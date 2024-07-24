from celery.result import AsyncResult
from django.http import JsonResponse
from django.views import View
from django.views.decorators.csrf import csrf_exempt
from django.utils.decorators import method_decorator

class taskResult(View):

    @method_decorator(csrf_exempt)
    def dispatch(self, *args, **kwargs):
        return super().dispatch(*args, **kwargs)
    
    def get(self, request, task_id):
        result = AsyncResult(task_id)
        if result.state == 'PENDING':
            response = {
                'state': result.state,
                'status': 'Pending...'
            }
        elif result.state != 'FAILURE':
            response = {
                'state': result.state,
                'result': result.result,
            }
            if 'graph' in result.result:
                response['graph'] = result.result['graph']
        else:
            response = {
                'state': result.state,
                'status': str(result.info),
            }
        return JsonResponse(response)
