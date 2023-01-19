/*
  PicoGamepad

  Turn a Raspberry Pico 2040 into an HID gamepad

  Supports:
  128 Buttons
  8 Analog axes
  4 Hat switches

  created 28 June 2021
  by Jake Wilkinson (RealRobots)

  This example code is in the public domain.

  https://www.gitlab.com/realrobots/PicoGamepad
*/
#include <PicoGamepad.h>

PicoGamepad gamepad;

// 16 bit integer for holding input values
int val;

const int XPOT = 26;
const int YPOT = 27;

const int BUTTON1 = 28;

const int potPins[] = { XPOT, YPOT };
const int numOfPins = 2;

void setup() {
  Serial.begin(115200);

  pinMode(LED_BUILTIN, OUTPUT);

  for (int i = 0; i < numOfPins; i+= 1 ) {
    pinMode(potPins[i], INPUT);
  }

  // Button on pin
  pinMode(BUTTON1, INPUT_PULLUP);
}

// Map analog 0-1023 value from pin to max HID range -32767 - 32767
int mapPotVal(int val) {
  return map(val, 0, 1023, -32767, 32767);
}

void loop() {

  // Get input value from Pico analog pin
  val = mapPotVal(analogRead(XPOT));
  gamepad.SetX(val);

  // Repeat with Y pin
  val = mapPotVal(analogRead(YPOT));
  gamepad.SetY(val);

//  gamepad.SetZ(val);
//  gamepad.SetRx(val);
//  gamepad.SetRy(val);
//  gamepad.SetRz(val);
//  gamepad.SetS0(val);
//  gamepad.SetS1(val);

  // Set button 0 of 128 by reading button on digital pin 28
  gamepad.SetButton(0, !digitalRead(28));

  // Set hat direction, 4 hats available. direction is clockwise 0=N 1=NE 2=E 3=SE 4=S 5=SW 6=W 7=NW 8=CENTER
  // gamepad.SetHat(0, 8);


  // Send all inputs via HID
  // Nothing is send to your computer until this is called.
  gamepad.send_update();

  // Flash the LED just for fun
  //digitalWrite(LED_BUILTIN, !digitalRead(LED_BUILTIN));
  //delay(1000);
  delay(1);
}
