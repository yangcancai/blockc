import 'dart:async';
import 'dart:ffi';
import 'dart:io';

import 'package:hb/ffi.dart';
import 'package:hb/hb.dart';
import 'package:ffi/ffi.dart';
void main(List<String> arguments) async {
  Hb hb = Hb();
  Hb.setup();
  // hb.startTimer();
  Pointer<Ws> ws = start_ws("wss://api.huobi.pro/ws".toNativeUtf8());
  // Pointer<Ws> ws = start_ws("ws://127.0.0.1:2794".toNativeUtf8());
  var alive = is_alive(ws);
  print("alive = $alive");
  var alive1 = is_alive(ws);
  print("alive = $alive1");
  print('Hello world!');
  sleep(Duration(seconds:1));
  Market m = get_market(ws, "market.btcusdt.bbo".toNativeUtf8());
  print("ask ${m.ask}");
  print("ask_size ${m.ask_size}");
  print("bid ${m.bid}");
  print("bid_size ${m.bid_size}");
  print("ch ${m.ch.toDartString()}");
  print("quote_time ${m.quote_time}");
  print("ts ${m.ts}");
  print("symbol ${m.symbol.toDartString()}");
  free_market(m);
  // print("market ${m.ref.ts}");
  // print("market ${m.ref.ch.toDartString()}");
  final rs = await hb.getSymbols("https://api.huobi.pro/v1/common/symbols"); 
  print("rs ==== $rs");
  print("==== ");
  stop_ws(ws) ;
}
