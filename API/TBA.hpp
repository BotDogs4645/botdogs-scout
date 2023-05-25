#ifndef TBA_H
#define TBA_H

#include "cpr/api.h"
#include <functional>
#include <iostream>
#include <regex>
#include <vector>
#include <string>
#include <future>

#define API_URL(endpoint) "https://www.thebluealliance.com/api/v3/" + endpoint
#define AUTH_HEADER_KEY "X-TBA-Auth-Key"

#include <cpr/cpr.h>
#include <json.hpp>



using json = nlohmann::json;
using nlohmann::basic_json;

using std::string;

class TBA {
  public: 
    TBA(string api_key) {
        this->api_key = api_key;
    }
    basic_json<> getStatus() {
      return GET("/status");
    }
    basic_json<> getTeamData(int teamNum) {
      return GET("/team/frc" + std::to_string(teamNum));
    }
    basic_json<> getMatches(int teamNum, string event_id) {
      return GET("/team/frc" + std::to_string(teamNum) + "/event/" + event_id + "/matches");
    }

  private:
    basic_json<> GET(string endpoint) {
     auto resp = cpr::GetAsync(
        cpr::Url{API_URL(endpoint) },
        cpr::Header{{AUTH_HEADER_KEY, api_key}}
        );
      resp.wait();
      return json::parse(resp.get().text);
    }

  protected:
    std::string api_key;
};

class TBA_Training_API : public TBA {
  public:
    TBA_Training_API(string api_key) : TBA(api_key) {}

    basic_json<> toClassificationSerializable(int teamNum, std::vector<string> eventCodes, string alliance) {
      basic_json<> ret = json::array();
      std::vector<basic_json<>> events;
      for(string ptr : eventCodes) {
        events.push_back(getMatches(teamNum, ptr));
      }
      for(int i = 0; i < events.size(); i++) {
        for(int j = 0; j < events[i].size(); j++) {
          json temp;

          temp.emplace("team1", events[i][j]["alliances"][alliance]["team_keys"][0]);
          temp.emplace("team2", events[i][j]["alliances"][alliance]["team_keys"][1]);
          temp.emplace("team3", events[i][j]["alliances"][alliance]["team_keys"][2]);
          temp.emplace("activationBonusAchieved", events[i][j]["score_breakdown"][alliance]["activationBonusAchieved"]);
          temp.emplace("autoDocked", events[i][j]["score_breakdown"][alliance]["autoDocked"]);
          temp.emplace("autoPoints", events[i][j]["score_breakdown"][alliance]["autoPoints"]);
          temp.emplace("autoChargeStationRobot1", events[i][j]["score_breakdown"][alliance]["autoChargeStationRobot1"]);
          temp.emplace("autoChargeStationRobot2", events[i][j]["score_breakdown"][alliance]["autoChargeStationRobot2"]);
          temp.emplace("autoChargeStationRobot3", events[i][j]["score_breakdown"][alliance]["autoChargeStationRobot3"]);
          temp.emplace("teleopPoints",events[i][j]["score_breakdown"][alliance]["teleopPoints"]);
          temp.emplace("chargeStationPoints", events[i][j]["score_breakdown"][alliance]["totalChargeStationPoints"]);
          temp.emplace("autoPoints", events[i][j]["score_breakdown"][alliance]["autoPoints"]);
          temp.emplace("autoNumPieces", events[i][j]["score_breakdown"][alliance]["autoGamePieceCount"]);
          temp.emplace("autoPieceScore", events[i][j]["score_breakdown"][alliance]["autoGamePiecePoints"]);
          temp.emplace("autoMobilityScore", events[i][j]["score_breakdown"][alliance]["autoMobilityPoints"]);
          temp.emplace("chargeStationState", events[i][j]["score_breakdown"][alliance]["autoBridgeState"]);
          

          ret.push_back(temp);
        }
      }
      return ret;
   }

   basic_json<> toRegressionSerializable(int teamNum, std::vector<basic_json<>> prevMatches, string alliance) {
      
   }

};


#endif