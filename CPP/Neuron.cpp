#include <iostream>
#include <vector>
using namespace std;

class Tensor{
    public:
        Tensor(int[] shape){

        }

};

int main(){
    double inputs[3] = {1.0,2.0,3.0};
    double weights[3] = {0.2, 0.8, -0.5};

    double total = 0;
    for (int i = 0; i < 3; i++){
        total += inputs[i] * weights[i];
    }
    cout << total << endl;



}