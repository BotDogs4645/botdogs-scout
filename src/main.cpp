#include <iostream>
#include <new>
#include <string>
#include <vector>
#include <fstream>

#include "../API/TBA.hpp"


int main() {

    std::ofstream JSONOut("testData.json");

    TBA_Training_API test("hghEo2woJZ3zZcXpfkmcO2noUM6ohn7SlHo89YAFaC5kgxTXSEGgrXMsMpSVyhpf");
    std::vector<string> eventCodes;
    eventCodes.push_back("2023mnmi2");
    // eventCodes.push_back("2023ilch");
    // eventCodes.push_back("2023ilpe");
    JSONOut << test.to_MATLAB_Serializable(2451, eventCodes, "blue");
    
    JSONOut.close();
}