import 'dart:ffi';

import 'package:cross_native/Utlis/helpers.dart';
import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:flutter/widgets.dart';
import 'package:geolocator/geolocator.dart';
import 'package:google_maps_flutter/google_maps_flutter.dart';

class Home extends StatefulWidget {
  const Home({super.key});

  @override
  State<Home> createState() => _HomeState();
}

class _HomeState extends State<Home> {
  LatLng userLocation = LatLng(0.0, 0.0);
  int activeHomeTab = 0;
  late GoogleMapController mapController;

  void getLocation() async {
    LocationPermission permission = await Geolocator.requestPermission();

    if (permission == LocationPermission.denied ||
        permission == LocationPermission.deniedForever) {
      return;
    }

    Position position = await Geolocator.getCurrentPosition(
        desiredAccuracy: LocationAccuracy.high);
    debugPrint("Location: ${position.latitude} ${position.longitude}");

    setState(() {
      userLocation = LatLng(position.latitude, position.longitude);
    });

    mapController.animateCamera(CameraUpdate.newLatLngZoom(
        LatLng(position.latitude, position.longitude), 14));
  }

  @override
  void initState() {
    // TODO: implement initState
    super.initState();
    getLocation();
  }

  _onReCenterClicked() {
    getLocation();
  }

  _onMapCreated(GoogleMapController controller) {
    mapController = controller;
    getLocation();
  }

  _onHomeTabClick(int index) {
    setState(() {
      activeHomeTab = index;
    });
  }

  @override
  Widget build(BuildContext context) {
    return SafeArea(
      child: Scaffold(
        body: Stack(
          children: [
            GoogleMap(
              initialCameraPosition: CameraPosition(
                target: userLocation,
                zoom: 14.4746,
              ),
              markers: {
                Marker(
                  markerId: const MarkerId("userLocation"),
                  position: userLocation,
                  infoWindow: const InfoWindow(title: "Your Location"),
                ),
              },
              onMapCreated: _onMapCreated,
              zoomControlsEnabled: false,
            ),
            Positioned(
              bottom: 16.0,
              right: 16.0,
              child: recenterBtn(),
            ),
          ],
        ),
        bottomNavigationBar: bottomNavBar(),
      ),
    );
  }

  Widget recenterBtn() {
    return IconButton(
      style: ButtonStyle(
        backgroundColor: MaterialStateProperty.all(Colors.white),
        shape: MaterialStateProperty.all(const CircleBorder()),
      ),
      icon: Icon(Icons.gps_fixed),
      onPressed: _onReCenterClicked,
    );
  }

  Widget bottomNavBar() {
    return BottomNavigationBar(
      selectedItemColor: "#08B783".toColor(),
      unselectedItemColor: "#414141".toColor(),
      currentIndex: activeHomeTab,
      onTap: _onHomeTabClick,
      items: const [
        BottomNavigationBarItem(
          icon: Icon(Icons.home),
          label: "Home",
        ),
        BottomNavigationBarItem(
          icon: Icon(Icons.favorite),
          label: "Favourite",
        ),
        BottomNavigationBarItem(
          icon: Icon(Icons.wallet),
          label: "Wallet",
        ),
        BottomNavigationBarItem(
          icon: Icon(Icons.local_offer),
          label: "Offer",
        ),
        BottomNavigationBarItem(
          icon: Icon(Icons.person),
          label: "Person",
        ),
      ],
    );
  }
}
