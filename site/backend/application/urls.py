from django.urls import path
from .views  import mlpView, taskResult

urlpatterns = [
    # path('mlp/initialisation/', mlpView.as_view(), name='mlp_initialisation'),
    # path('mlp/train/', mlpView.as_view(), name='mlp_train'),
    path('mlp/', mlpView.as_view(), name='mlp'),
    path('status/', mlpView.as_view(), name='modele_status'),
    path('task_result/<task_id>/', taskResult.as_view(), name='task_result'),
]
