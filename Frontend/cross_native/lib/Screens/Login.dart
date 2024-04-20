import 'dart:math';

import 'package:cross_native/Components/BackBtn.dart';
import 'package:cross_native/Components/GeneralInput.dart';
import 'package:cross_native/Components/PrimaryButton.dart';
import 'package:cross_native/Utlis/Colors.dart';
import 'package:cross_native/Utlis/helpers.dart';
import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:google_fonts/google_fonts.dart';

class Login extends StatefulWidget {
  const Login({super.key});
  @override
  State<Login> createState() => _Login();
}

class _Login extends State<Login> {
  String phone = "";

  void _onBackBtnClicked() {
    Navigator.pop(context);
  }

  void _onPhoneChanged(String phone) {
    setState(() {
      this.phone = phone;
    });
    debugPrint("Phone $phone AND ${this.phone}");
  }

  void _onGetOTPClicked() {
    Navigator.pushNamed(context, '/createProfile');
  }

  bool _checkOTPBtnEnabled() {
    if (phone.length == 10) {
      return true;
    }
    return false;
  }

  @override
  Widget build(BuildContext context) {
    return SafeArea(
      child: Scaffold(
        body: Column(
          children: [
            backBtn(_onBackBtnClicked),
            Container(
              margin: const EdgeInsets.only(top: 30),
              padding: const EdgeInsets.only(left: 16, right: 16),
              child: Text(
                'Sign up with your phone number',
                style: GoogleFonts.poppins(
                  fontWeight: FontWeight.w500,
                  fontSize: 24,
                ),
              ),
            ),
            phoneNumberInput(),
            const Spacer(),
            getOTPButton(),
          ],
        ),
      ),
    );
  }

  Widget getOTPButton() {
    return Container(
      margin: const EdgeInsets.only(left: 26, top: 0, right: 27, bottom: 20),
      child: TextButton(
        style: TextButton.styleFrom(
          minimumSize: const Size.fromHeight(54),
          backgroundColor: _checkOTPBtnEnabled() ? "#008955".toColor() :  "#008955".toColor().withOpacity(0.5),
          shape: RoundedRectangleBorder(
            borderRadius: BorderRadius.circular(8),
          ),
        ),
        onPressed: _checkOTPBtnEnabled()?  _onGetOTPClicked : null,
        child: Text(
          "Get OTP",
          style: TextStyle(
            color: "#FFFFFF".toColor(),
          ),
        ),
      ),
    );
  }

  Widget phoneNumberInput() {
    return Container(
      margin: const EdgeInsets.only(left: 16, right: 16, top: 20),
      child: TextField(
        decoration: InputDecoration(
          border: const OutlineInputBorder(
            borderRadius: BorderRadius.all(Radius.circular(8)),
          ),
          hintText: "Phone number",
          hintStyle: TextStyle(
            color: "#D0D0D0".toColor(),
          ),
          contentPadding: const EdgeInsets.all(20),
        ),
        onChanged: _onPhoneChanged,
      ),
    );
  }
}
