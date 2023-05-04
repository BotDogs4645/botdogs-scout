import 'package:botdogs_scout/routes.dart';
import 'package:botdogs_scout/themes.dart';
import 'package:flutter/material.dart';

void main() {
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  // This widget is the root of your application.
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      theme: appThemes,
      routes: appRoutes,
    );
  }
}
