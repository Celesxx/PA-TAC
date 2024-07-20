from django.urls import path
from .views.mlpView  import mlpView

urlpatterns = [
    path('initialisation/', mlpView.as_view(), name='mlp_initialisation'),
    path('test/', mlpView.as_view(), name='test'),
    path('status/', mlpView.as_view(), name='modele_status'),
]
