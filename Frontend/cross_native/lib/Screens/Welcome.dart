
import 'package:cross_native/Utlis/helpers.dart';
import 'package:flutter/material.dart';
import 'package:google_fonts/google_fonts.dart';

class Welcome extends StatefulWidget {
  const Welcome({super.key});
  @override
  State<Welcome> createState() => _WelcomeState();
}

class _WelcomeState extends State<Welcome> {

  void _logInClick() {
    Navigator.pushNamed(context, '/login');
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        children: <Widget>[
          Container(
            margin: const EdgeInsets.only(top: 20),
            child: Image.asset('assets/images/st_ic_welcome_screen.png'),
          ),
          Container(
            margin: const EdgeInsets.only(top: 29),
            child: Text(
              'Welcome',
              textDirection: TextDirection.ltr,
              style: GoogleFonts.poppins(
                fontWeight: FontWeight.w500,
                fontSize: 24.0,
              ),
            ),
          ),
          Container(
            margin: const EdgeInsets.only(top: 12),
            child: Text(
              'Have a better sharing experience',
              textDirection: TextDirection.ltr,
              style: GoogleFonts.poppins(
                fontWeight: FontWeight.w400,
                fontSize: 16.0,
                color: "#A0A0A0".toColor(),
              ),
            ),
          ),
          const Spacer(),
          letsStartBtn(),
        ],
      ),
    );
  }


  Widget letsStartBtn() {
  return Container(
    margin: const EdgeInsets.only(left: 26, top: 0, right: 27, bottom: 20),
    child: TextButton(
      style : TextButton.styleFrom(
        minimumSize: const Size.fromHeight(54),
        backgroundColor: "#008955".toColor(),
        shape: RoundedRectangleBorder(
          borderRadius: BorderRadius.circular(8),
        ),
      ),
      onPressed: _logInClick,
      child: Text(
          "Let's Start",
        style: TextStyle(
          color: "#FFFFFF".toColor(),
        ),
      ),
    ),
  );
}
}
