import random
import numpy as np

class Tensor():
    """
    Attempt at creating a class to hold multi dimensional arrays using only python lists
    """
    def __init__(self, shape: list):
        self.tensor  = self.create_tensor(shape)
        self.tensor_shape = shape


    def create_tensor(self, shape):
        length = len(shape)
        def recur(index=0):
            if index != length -1:

                return [recur(index+1) for i in range(shape[index])]

            return [0 for i in range(shape[index])]
        return recur()
    def __str__(self):
        return "Tensor: "+str(self.tensor) + "\tShape: " + str(self.tensor_shape)

    @staticmethod
    def dot(tensor1 , tensor2):
        raise NotImplementedError


class PyMatrix(Tensor):
    def __init__(self, x_, y_):
        super(PyMatrix, self).__init__([x_, y_])
        self.shape = (x_, y_)
    def dot(self, x , y):
        if x.shape != x.shape:
            
        for i in range()
    
  

if __name__ =="__main__":
    m = PyMatrix(2,3)
    d = PyMatrix(3,4)
    f = PyMatrix(3,2)
    PyMatrix.dot(m,f)
    print(m)