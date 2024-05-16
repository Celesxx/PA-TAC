import ctypes
import numpy as np


linear_model = ctypes.CDLL('modele/linear/target/release/liblinear_classification.so')

linear_model.LM_init.argtypes = [
    ctypes.c_double,
    np.ctypeslib.ndpointer(dtype=np.float64, ndim=1, flags='C_CONTIGUOUS'),
    ctypes.c_size_t,
    ctypes.c_double
]
linear_model.LM_init.restype = ctypes.POINTER(ctypes.c_void_p)

linear_model.LM_free.argtypes = [ctypes.POINTER(ctypes.c_void_p)]

linear_model.LM_train.argtypes = [
    ctypes.POINTER(ctypes.c_void_p),
    np.ctypeslib.ndpointer(dtype=np.float64, ndim=1, flags='C_CONTIGUOUS'),
    np.ctypeslib.ndpointer(dtype=np.float64, ndim=1, flags='C_CONTIGUOUS'),
    ctypes.c_size_t,
    ctypes.c_size_t,
    ctypes.c_size_t
]

learning_rate = 0.01
weights = np.array([0.1, -0.2, 0.3, -0.4], dtype=np.float64, order='C')
bias = 0.5
epochs = 1000

# Initialisation du model
model = linear_model.LM_init(learning_rate, weights, weights.size, bias)

# XOR test
x_train = np.array([
    [0.0, 0.0],
    [0.0, 1.0],
    [1.0, 0.0]
], dtype=np.float64, order='C')
x_train_flatten = x_train.flatten()
y_train = np.array([-1.0, 1.0, 1.0], dtype=np.float64, order='C')

print(f"flatten vector {x_train}")
print(f"x shape : {x_train.shape[0]}")
print(f"x shape : {x_train.shape[1]}")



# Nombre d'époques d'entraînement

# Entrainement du modèle
try:
    linear_model.LM_train(model, x_train_flatten, y_train, x_train.shape[0], x_train.shape[1], epochs)
    print("Entraînement terminé avec succès")
except Exception as e:
    print(f"Erreur lors de l'entraînement : {e}")

# Libération du modèle
linear_model.LM_free(model)
