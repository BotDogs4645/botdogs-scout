import 'package:flutter/material.dart';
import 'package:flutter/src/widgets/framework.dart';
import 'package:flutter/src/widgets/placeholder.dart';
import 'package:botdogs_scout/shared/teamData.dart';
import 'package:botdogs_scout/routes.dart';

class HomePage extends StatelessWidget {
  const HomePage({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text("Team 4645 - Scouting App"),
      ),
      body: Padding(
        padding: const EdgeInsets.all(25),
        child: Align(
          alignment: Alignment.center,
          child: Column(children: [
            FloatingActionButton.extended(
              onPressed: () => {Navigator.pushNamed(context, "/auth")},
              label: const Text("Sign In"),
            ),
            const Divider(
              color: Colors.transparent,
              height: 20,
            ),
            FloatingActionButton.extended(
              onPressed: () => {Navigator.pushNamed(context, "/guest")},
              label: const Text("Continue as guest (Statistics Only)"),
            )
          ]),
        ),
      ),
    );
  }
}
