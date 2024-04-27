

import 'dart:convert';
import 'package:http/http.dart' as http;

Future<AuthRes> authUser(String phoneNumber) async {
  final response = await http.post(
    Uri.parse('https://dummyjson.com/auth/login'),
    headers: <String, String>{
      'Content-Type': 'application/json; charset=UTF-8',
    },
    body: jsonEncode(<String, String>{
      'username': phoneNumber,
      'password': '0lelplR',
      'expiresInMins': 30.toString()
    }),
  );

  if (response.statusCode == 200) {
    return AuthRes.fromJson(jsonDecode(response.body));
  } else {
    throw Exception('Failed to authenticate user');
  }
}


class AuthRes {
  final String authToken;

  AuthRes({required this.authToken});

  factory AuthRes.fromJson(Map<String, dynamic> json) {
    return switch(json){
      {
        'token': String authToken,
      } =>
      AuthRes(
        authToken: authToken,
      ),
      _ => throw Exception('Failed to parse AuthRes'),
    };
  }
}