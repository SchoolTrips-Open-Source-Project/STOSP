import 'package:cross_native/Components/BackBtn.dart';
import 'package:cross_native/Utlis/Colors.dart';
import 'package:cross_native/Utlis/helpers.dart';
import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:google_fonts/google_fonts.dart';

import '../Components/GeneralInput.dart';

class CreateProfile extends StatefulWidget {
  const CreateProfile({super.key});
  @override
  State<CreateProfile> createState() => _CreateProfileState();
}

class _CreateProfileState extends State<CreateProfile> {
  String fullName = "";
  String email = "";

  void _backBtnClicked() {
    Navigator.pop(context);
  }

  void _typingFullName(String fullName) {
    this.fullName = fullName;
  }

  void _typingEmail(String email) {}

  @override
  Widget build(BuildContext context) {
    return SafeArea(
        child: Scaffold(
            body: Container(
      margin: const EdgeInsets.fromLTRB(0, 8, 0, 8),
      color: "#FFFFFF".toColor(),
      child: Column(
        children: [
          Row(
            children: [
              Flexible(
                flex: 1,
                child: backBtn(_backBtnClicked),
              ),
              Flexible(
                flex: 1,
                child: Center(
                  child: Text(
                    "Profile",
                    style: GoogleFonts.poppins(
                      fontWeight: FontWeight.w500,
                      fontSize: 18,
                    ),
                  ),
                ),
              ),
              const Flexible(
                flex: 1,
                child: Text(""),
              )
            ],
          ),
          Container(
            padding: const EdgeInsets.only(top: 30),
            child: CircleAvatar(
              backgroundColor: "#D0D0D0".toColor(),
              radius: 60,
            ),
          ),
          inputBox("Full Name", _typingFullName),
          inputBox("Email", _typingEmail),
        ],
      ),
    )));
  }
}
