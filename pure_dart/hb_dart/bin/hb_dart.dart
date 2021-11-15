import 'dart:async';
import 'dart:ffi';
import 'dart:io';

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
  sleep(Duration(seconds:1));
  Market m = get_market1(ws, "market.btcusdt.bbo".toNativeUtf8());
  print("market ${m.ask}");
  print("market ${m.ask_size}");
  print("market ${m.bid}");
  print("market ${m.bid_size}");
  // final ch = m.ch.toDartString();
  // print("market ${m.ch.toDartString()}");
  print("market ${m.quote_time}");
  print("market ${m.ts}");
  // print("market ${m.symbol.toDartString()}");
  // print("market ${m.ref.ts}");
  // print("market ${m.ref.ch.toDartString()}");
  stop_ws(ws) ;
}
