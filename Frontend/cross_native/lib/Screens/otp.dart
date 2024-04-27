

import 'package:cross_native/Components/BackBtn.dart';
import 'package:cross_native/Utlis/helpers.dart';
import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:flutter/widgets.dart';
import 'package:flutter_otp_text_field/flutter_otp_text_field.dart';
import 'package:google_fonts/google_fonts.dart';

class OTP extends StatefulWidget {
  const OTP({super.key});

  @override
  State<OTP> createState() => _OTPState();
}

class _OTPState extends State<OTP> {
  String otp = "";

  void _backBtnClicked() {
    Navigator.pop(context);
  }

  bool onOTPCodeChanged(String otp) {
    setState(() {
      this.otp = otp;
    });

    if(this.otp.length == 4){
      otpBtnSubmitClicked();
    }
    debugPrint("OTP $otp AND ${this.otp}");
    return true;
  }

  void otpBtnSubmitClicked(){
    Navigator.pushNamed(context, "/createProfile");
  }

  bool _checkOTPBtnEnabled() {
    return   otp.length == 4;
  }

  @override
  Widget build(BuildContext context) {
    return (SafeArea(
        child: Scaffold(
      body: Column(
        children: [
          backBtn(_backBtnClicked),
          Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              Container(
                margin: const EdgeInsets.only(top: 30),
                child: Text(
                  'Phone verification',
                  style: GoogleFonts.poppins(
                    fontWeight: FontWeight.w500,
                    fontSize: 24,
                  ),
                ),
              ),
              Container(
                margin: const EdgeInsets.only(top: 30),
                child: Text(
                  'Enter your OTP code',
                  style: GoogleFonts.poppins(
                    fontWeight: FontWeight.w400,
                    fontSize: 16,
                  ),
                ),
              ),
              OtpTextField(
                margin: const EdgeInsets.only(top: 40, right: 10),
                numberOfFields: 4,
                showFieldAsBox: true,
                autoFocus: true,
                fieldWidth: 50,
                fieldHeight: 48,
                // onCodeChanged: onOTPCodeChanged,
                onSubmit: onOTPCodeChanged,
              ),

              submitOTPBtn(),
            ],
          ),
        ],
      ),
    )));
  }

  Widget submitOTPBtn() {
    return Container(
      margin: const EdgeInsets.only(left: 26, top: 16, right: 27, bottom: 20),
      child: TextButton(
        style: TextButton.styleFrom(
          minimumSize: const Size.fromHeight(54),
          backgroundColor: _checkOTPBtnEnabled() ? "#008955".toColor() :  "#008955".toColor().withOpacity(0.5),
          shape: RoundedRectangleBorder(
            borderRadius: BorderRadius.circular(8),
          ),
        ),
        onPressed:_checkOTPBtnEnabled() ?  otpBtnSubmitClicked : null,
        child: Text(
          "Verify OTP",
          style: TextStyle(
            color: "#FFFFFF".toColor(),
          ),
        ),
      ),
    );
  }
}
