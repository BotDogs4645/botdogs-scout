import 'package:botdogs_scout/about/about.dart';
import 'package:botdogs_scout/auth/auth.dart';
import 'package:botdogs_scout/home/home.dart';
import 'package:botdogs_scout/pit/pitScout.dart';
import 'package:botdogs_scout/stats/stats.dart';

var appRoutes = {
  '/': (context) => const HomePage(),
  '/aboutTeams': (context) => const AboutTeamsPage(),
  '/auth': (context) => const AuthPage(),
  '/stats': (context) => const StatsPage(),
  '/pit': (context) => const PitScoutPage(),
};
