#include <iostream>
#include <new>
#include <string>
#include <vector>
#include "../API/TBA.hpp"


int main() {
    TBA_Training_API test("hghEo2woJZ3zZcXpfkmcO2noUM6ohn7SlHo89YAFaC5kgxTXSEGgrXMsMpSVyhpf");
    std::vector<string> eventCodes{"2023ilch"};
    test.to_MATLAB_Serializable(4646, eventCodes);
    // TBA tba("hghEo2woJZ3zZcXpfkmcO2noUM6ohn7SlHo89YAFaC5kgxTXSEGgrXMsMpSVyhpf");
    // std::cout << tba.getMatches(4645, "2023ilch"); 
}