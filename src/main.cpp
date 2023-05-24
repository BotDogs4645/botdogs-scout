#include <iostream>
#include <new>
#include <string>
#include <vector>
#include <fstream>

#include "../API/TBA.hpp"


int main() {

    std::ofstream JSONOut("output.json");

    TBA_Training_API test("hghEo2woJZ3zZcXpfkmcO2noUM6ohn7SlHo89YAFaC5kgxTXSEGgrXMsMpSVyhpf");
    std::vector<string> eventCodes;
    eventCodes.push_back("2023ilch");
    JSONOut << test.to_MATLAB_Serializable(4646, eventCodes, "blue");
    
    JSONOut.close();

    // TBA tba("hghEo2woJZ3zZcXpfkmcO2noUM6ohn7SlHo89YAFaC5kgxTXSEGgrXMsMpSVyhpf");
    // std::cout << tba.getMatches(4645, "2023ilch"); 
}