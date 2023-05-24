matchesJSON = fileread("output.json");
jsonSerialized = jsondecode(matchesJSON);
data = struct2table(jsonSerialized);
