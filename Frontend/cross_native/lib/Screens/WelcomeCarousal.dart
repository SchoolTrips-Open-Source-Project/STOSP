import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';

class WelcomeCarousal extends StatefulWidget {
  const WelcomeCarousal({super.key});

  @override
  State<WelcomeCarousal> createState() => _WelcomeCarousalState();
}

class _WelcomeCarousalState extends State<WelcomeCarousal> {
  @override
  Widget build(BuildContext context) {
    return SafeArea(child: Scaffold(
      body: Column(
        children: [
          const Placeholder(),
        ],
      ),
    ));
  }
}