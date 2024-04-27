import 'package:cross_native/Components/BackBtn.dart';
import 'package:cross_native/Utlis/Colors.dart';
import 'package:cross_native/Utlis/helpers.dart';
import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:google_fonts/google_fonts.dart';


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
    setState(() {
      this.fullName = fullName;
    });
  }

  void _typingEmail(String email) {
    setState(() {
      this.email = email;
    });
  }

  bool _checkSaveBtnEnabled(){
    return fullName.isNotEmpty && email.isNotEmpty;
  }

  void _saveBtnClicked(){
    Navigator.pushNamed(context, "/home");
  }

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
          saveProfileBtn()
        ],
      ),
    )));
  }

  Widget inputBox(String hint, Function(String) onChanged) {
    return Container(
      margin: const EdgeInsets.only(left: 16, right: 16, top: 20),
      child: TextField(
        decoration: InputDecoration(
          border: const OutlineInputBorder(
            borderRadius: BorderRadius.all(Radius.circular(8)),
          ),
          hintText:hint,
          hintStyle: TextStyle(
            color: "#D0D0D0".toColor(),
          ),
          contentPadding: const EdgeInsets.all(20),
        ),
        onChanged: onChanged,
      ),
    );
  }

  Widget saveProfileBtn() {
    return Container(
      margin: const EdgeInsets.only(left: 26, top: 16, right: 27, bottom: 20),
      child: TextButton(
        style: TextButton.styleFrom(
          minimumSize: const Size.fromHeight(54),
          backgroundColor: _checkSaveBtnEnabled() ? "#008955".toColor() :  "#008955".toColor().withOpacity(0.5),
          shape: RoundedRectangleBorder(
            borderRadius: BorderRadius.circular(8),
          ),
        ),
        onPressed:_checkSaveBtnEnabled() ?  _saveBtnClicked : null,
        child: Text(
          "Save",
          style: TextStyle(
            color: "#FFFFFF".toColor(),
          ),
        ),
      ),
    );
  }
}
