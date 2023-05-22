matchesJSON = fileread("matches.json");
jsonSerialized = jsondecode(matchesJSON);
data = struct2table(jsonSerialized);
scoresTable = struct2table(data.score_breakdown);