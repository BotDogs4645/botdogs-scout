import 'package:flutter/material.dart';
import 'package:flutter/src/widgets/framework.dart';
import 'package:flutter/src/widgets/placeholder.dart';
import 'package:botdogs_scout/shared/teamData.dart';

class HomePage extends StatelessWidget {
  const HomePage({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
        appBar: AppBar(
          title: const Text("Hello"),
        ),
        body: Card(
          child: Row(children: [
            Column(
              children: [Text('Team #: $teamNum')],
            )
          ]),
        ));
  }
}
