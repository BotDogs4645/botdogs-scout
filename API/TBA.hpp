#ifndef TBA_H
#define TBA_H

#include "cpr/api.h"
#include <functional>
#include <iostream>
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

    void to_MATLAB_Serializable(int teamNum, std::vector<string> eventCodes) {
      std::vector<basic_json<>> events;
      for(string str : eventCodes) {
        events.push_back(getMatches(teamNum, str));
      }
   }

};


#endif