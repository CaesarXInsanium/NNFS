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
        return "Tensor:\n"+str(self.tensor) + "\nShape:\n" + str(self.tensor_shape)

    @staticmethod
    def dot(tensor1 , tensor2):
        raise NotImplementedError
  

if __name__ =="__main__":
    t = Tensor([10,10])
    for x in t.tensor:
        print(x)
