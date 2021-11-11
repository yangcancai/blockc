import 'dart:ffi';

import 'package:hb/ffi.dart';
import 'package:hb/hb.dart';
import 'package:ffi/ffi.dart';
void main(List<String> arguments) {
  Hb hb = Hb();
  Hb.setup();
  hb.startTimer();
  Pointer<Ws> ws = start_ws("wss://api.huobi.pro/ws".toNativeUtf8());
  // Pointer<Ws> ws = start_ws("ws://127.0.0.1:2794".toNativeUtf8());
  var alive = is_alive(ws);
  print("alive = $alive");
  var alive1 = is_alive(ws);
  print("alive = $alive1");
  print('Hello world!');
  // stop_ws(ws) ;
}
