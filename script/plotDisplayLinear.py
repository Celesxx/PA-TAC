import ctypes
import numpy as np
import matplotlib.pyplot as plt

def plotDisplay(X_train, y_train, model_init, mlp_model, n_features):
    # Définir les limites de la grille
    x_min, x_max = X_train[:,0].min() - 0.1, X_train[:,0].max() + 0.1
    y_min, y_max = X_train[:,1].min() - 0.1, X_train[:,1].max() + 0.1
    step = 0.01

    xx, yy = np.meshgrid(np.arange(x_min, x_max, step), np.arange(y_min, y_max, step))
    grid_points = np.c_[xx.ravel(), yy.ravel()]

    # Effectuer des prédictions pour chaque point de la grille
    grid_predictions = []
    for point in grid_points:
        point_array = np.array(point, dtype=np.float64)
        prediction = np.zeros(1, dtype=np.float64)
        mlp_model.LM_predict(
            model_init,
            point_array.ctypes.data_as(ctypes.POINTER(ctypes.c_double)),
            1,  # Le nombre d'échantillons est 1 car nous prédisons pour un seul point
            n_features,
            prediction.ctypes.data_as(ctypes.POINTER(ctypes.c_double))
        )
        grid_predictions.append(prediction[0])  # Append the scalar value instead of a list

    grid_predictions = np.array(grid_predictions)

    # Vérifier si y_train est un vecteur simple ou un vecteur de vecteurs
    if y_train.ndim == 1:
        class_0 = X_train[y_train < 0]
        class_1 = X_train[y_train > 0]
    else:
        class_0 = X_train[y_train[:, 0] < 0]
        class_1 = X_train[y_train[:, 0] > 0]

    # Tracé des points d'entraînement avec des couleurs différentes pour chaque classe
    plt.scatter(class_0[:, 0], class_0[:, 1], color='blue', edgecolor='k', label='Classe 0')
    plt.scatter(class_1[:, 0], class_1[:, 1], color='red', edgecolor='k', label='Classe 1')

    # Tracer la séparation des classes
    contour = grid_predictions.reshape(xx.shape)
    plt.contourf(xx, yy, contour, levels=[-np.inf, 0, np.inf], colors=['blue', 'red'], alpha=0.5)

    plt.legend()
    plt.show()