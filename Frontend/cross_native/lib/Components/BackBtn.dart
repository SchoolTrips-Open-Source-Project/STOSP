

import 'package:cross_native/Utlis/helpers.dart';
import 'package:flutter/material.dart';
import 'package:google_fonts/google_fonts.dart';

Widget backBtn(void Function() backBtnClicked){
  return InkWell(
    onTap: backBtnClicked,
    child: Row(children: [
      Container(
        margin: const EdgeInsets.all( 8),
        child: Image.asset(
          'assets/images/st_ic_angle_left.png',
          height: 24,
          width: 24,
        ),
      ),
      Text(
        "Back",
        style: GoogleFonts.poppins(
          fontWeight: FontWeight.w400,
          fontSize: 16,
          color: "#414141".toColor(),
        ),
      ),
    ]),
  );
}