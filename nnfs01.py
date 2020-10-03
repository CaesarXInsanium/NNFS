import random
inputs = [1,2,3]
weights = [0.2,0.8,-0.5]
class Neuron():
    def __init__(self, input_num):
        
        self.weights = [random.random() for i in range(input_num)]
        self.bias  = random.random()

    def output(self, inputs):
        total = 0
        for i in inputs:
            for j in self.weights:
                total += i * j
        return total + self.bias


neur = Neuron(4)

print(neur.output([1,2,3,4]))