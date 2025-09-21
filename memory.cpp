#include <iostream>
using namespace std;

int main() {
    int* arr = new int[3]; // allocate dynamically
    arr[0] = 10;
    arr[1] = 20;
    arr[2] = 30;

    cout << "C++ array allocated" << endl;

    delete[] arr; // manual deallocation
    cout << "C++: memory freed manually" << endl;

    return 0;
}
