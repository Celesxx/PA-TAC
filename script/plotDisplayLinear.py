
import ctypes
import numpy as np
import matplotlib.pyplot as plt
from mpl_toolkits.mplot3d import Axes3D


def plotDisplay(X_train, y_train, predictions, title):
    fig = plt.figure()
    ax = fig.add_subplot(111, projection='3d')
    ax.set_title(title)
    
    # Séparer les classes pour les données d'entraînement
    class_0 = X_train[y_train[:, 0] == 1]
    class_1 = X_train[y_train[:, 1] == 1]
    class_2 = X_train[y_train[:, 2] == 1]

    # Séparer les classes pour les prédictions
    pred_class_0 = X_train[predictions[:, 0] == 1]
    pred_class_1 = X_train[predictions[:, 1] == 1]
    pred_class_2 = X_train[predictions[:, 2] == 1]

    # Tracé des points d'entraînement avec des couleurs différentes pour chaque classe
    ax.scatter(class_0[:, 0], class_0[:, 1], y_train[y_train[:, 0] == 1][:, 0], color='blue', edgecolor='k', label='Classe 0')
    ax.scatter(class_1[:, 0], class_1[:, 1], y_train[y_train[:, 1] == 1][:, 1], color='red', edgecolor='k', label='Classe 1')
    ax.scatter(class_2[:, 0], class_2[:, 1], y_train[y_train[:, 2] == 1][:, 2], color='green', edgecolor='k', label='Classe 2')

    # Tracé des prédictions
    ax.scatter(pred_class_0[:, 0], pred_class_0[:, 1], predictions[predictions[:, 0] == 1][:, 0], color='blue', edgecolor='k', marker='o', s=150, alpha=0.6)
    ax.scatter(pred_class_1[:, 0], pred_class_1[:, 1], predictions[predictions[:, 1] == 1][:, 1], color='red', edgecolor='k', marker='o', s=150, alpha=0.6)
    ax.scatter(pred_class_2[:, 0], pred_class_2[:, 1], predictions[predictions[:, 2] == 1][:, 2], color='green', edgecolor='k', marker='o', s=150, alpha=0.6)

    # Ajouter des étiquettes et une légende
    ax.set_xlabel('X1')
    ax.set_ylabel('X2')
    ax.set_zlabel('Valeurs')
    ax.legend()

    plt.show()