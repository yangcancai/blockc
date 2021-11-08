import 'dart:ffi';

import 'package:scrap/ffi.dart';
import 'package:scrap/scrap.dart';
import 'package:ffi/ffi.dart';
void main(List<String> arguments) {
  Scrap hb = Scrap();
  Scrap.setup();
  hb.startTimer();
  // Pointer<Ws> ws = start_ws("ws://192.168.3.6:2794".toNativeUtf8());
  Pointer<Ws> ws = start_ws_ssl("wss://api.huobi.pro/ws".toNativeUtf8());
  print('Hello world!');
}
