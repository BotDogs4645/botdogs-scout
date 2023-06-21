import 'package:botdogs_scout/about/about.dart';
import 'package:botdogs_scout/setup/setup.dart';
import 'package:botdogs_scout/home/home.dart';
import 'package:botdogs_scout/pit/pitScout.dart';
import 'package:botdogs_scout/stats/stats.dart';
import 'package:botdogs_scout/guest/guest.dart';

var appRoutes = {
  '/auth': (context) => const HomePage(),
  '/aboutTeams': (context) => const AboutTeamsPage(),
  '/': (context) => const SetupPage(),
  '/stats': (context) => const StatsPage(),
  '/pit': (context) => const PitScoutPage(),
  '/guest': (context) => const GuestPage(),
};
