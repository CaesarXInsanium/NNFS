import numpy as np
from numpy.core.multiarray import result_type
import tensorflow as tf

inputs = np.array([1.0, 2.0, 3.0, 2.5])
weights = np.array([0.2, 0.8, -0.5, 1.0])
bias = np.array([2.0])
outputs = np.dot(weights, inputs) + bias
print(outputs)

#how do I do dot product in pure python?
def dotProduct(x, y):
    total=0.0
    for i in x:
        for j in y:
            total += i*j

    return total

result = dotProduct(inputs, weights) + bias
print("Python: ", result)

a = [1,2,3]
b = [2,3,4]

a = np.array([a])
b = np.array([b]).T
print(np.dot(a,b))
inputs = [[1.0, 2.0, 3.0, 2.5],
          [2.0, 5.0, -1.0, 2.0],
          [-1.5, 2.7, 3.3, -0.8]]
weights = [[0.2, 0.8, -0.5, 1.0],
           [0.5, -0.91, 0.26, -0.5],
           [-0.26, -0.27, 0.17, 0.87]]
biases = [2.0, 3.0, 0.5]

layer_outputs = np.dot(inputs, np.array(weights).T) + biases

print(layer_outputs)