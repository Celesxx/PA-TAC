import ctypes
import numpy as np
import matplotlib.pyplot as plt

def plotDisplay(X_train, y_train, model_init, mlp_model):
    x_min, x_max = X_train[:,0].min() - 0.1, X_train[:,0].max() + 0.1
    y_min, y_max = X_train[:,1].min() - 0.1, X_train[:,1].max() + 0.1
    step = 0.01

    xx, yy = np.meshgrid(np.arange(x_min, x_max, step), np.arange(y_min, y_max, step))
    grid_points = np.c_[xx.ravel(), yy.ravel()]

    num_classes = 1
    grid_predictions = []
    # print(grid_points)
    for point in grid_points:
        num_classes = y_train.shape[1] if y_train.ndim > 1 else 1
        
        prediction = np.zeros(num_classes, dtype=np.float64)
        mlp_model.mlpPredict(
            model_init,
            point.ctypes.data_as(ctypes.POINTER(ctypes.c_double)),
            len(point),
            True,
            prediction.ctypes.data_as(ctypes.POINTER(ctypes.c_double))
        )
        if y_train.ndim == 1:
            grid_predictions.append(prediction.tolist())
        else:
            grid_predictions.append(prediction)
        # print("Sample", X_train[k], ", predictions =", prediction)


    grid_predictions = np.array(grid_predictions)
    # print(grid_predictions)



    if num_classes == 1:
        print("ndim = 1")
        class_0 = X_train[y_train[:, 0] < 0]
        class_1 = X_train[y_train[:, 0] > 0]

        plt.scatter(class_0[:, 0], class_0[:, 1], color='blue', edgecolor='k', label='Classe 0')
        plt.scatter(class_1[:, 0], class_1[:, 1], color='red', edgecolor='k', label='Classe 1')

        contour = grid_predictions[:, 0].reshape(xx.shape)
        plt.contourf(xx, yy, contour, levels=[-np.inf, 0, np.inf], colors=['blue', 'red'], alpha=0.5)
    
    else:
        print("ndim = multi")
        colors = ['blue', 'red', 'green']
        labels = ['Classe A', 'Classe B', 'Classe C']
        for i in range(num_classes):
            class_i = X_train[np.argmax(y_train, axis=1) == i]
            plt.scatter(class_i[:, 0], class_i[:, 1], color=colors[i], edgecolor='k', label=labels[i])

        # Tracer la séparation des classes pour la classification multi-classe
        contour = np.argmax(grid_predictions, axis=1).reshape(xx.shape)
        plt.contourf(xx, yy, contour, levels=np.arange(num_classes+1)-0.5, colors=colors, alpha=0.5)

    plt.legend()
    plt.show()