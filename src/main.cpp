#include <iostream>
#include <new>
#include <string>
#include <vector>
#include <fstream>
#include <ranges>

#include "../API/TBA.hpp"


int main() {

    std::ofstream JSONOut("testData.json");

    TBA_Training_API test("hghEo2woJZ3zZcXpfkmcO2noUM6ohn7SlHo89YAFaC5kgxTXSEGgrXMsMpSVyhpf");
    std::vector<string> eventCodes;
    eventCodes.push_back("2023mnmi2");
    JSONOut << test.toClassificationSerializable(2451, eventCodes, "blue");
    JSONOut.close();

    // std::ofstream JSONOut2; 

    // JSONOut2.open("testData.json", std::ios_base::app);
    // JSONOut2 << test.toClassificationSerializable(4645, {"2023ilch", "2023ilpe"}, "blue");
    
    // JSONOut2.close();

}