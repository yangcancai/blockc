/// bindings for `libhb`

import 'dart:ffi';
import 'dart:io';
import 'package:ffi/ffi.dart' as ffi;
import 'package:ffi/ffi.dart';

// ignore_for_file: unused_import, camel_case_types, non_constant_identifier_names
final DynamicLibrary _dl = _open();
/// Reference to the Dynamic Library, it should be only used for low-level access
final DynamicLibrary dl = _dl;
DynamicLibrary _open() {
  if (Platform.isMacOS) return DynamicLibrary.open('../../target/debug/libhb_ffi.dylib');
  if (Platform.isAndroid) return DynamicLibrary.open('libhb_ffi.so');
  if (Platform.isIOS) return DynamicLibrary.executable();
  throw UnsupportedError('This platform is not supported.');
}

/// C struct `Ws`.
class Ws extends Opaque{
  // static Pointer<Ws> allocate() {
  //   return ffi.calloc.allocate<Ws>(0);
  // }
  // static Ws from(int ptr) {
  //   return Pointer<Ws>.fromAddress(ptr);
  // }
}
class Market extends Struct {
  external Pointer<ffi.Utf8> ch;
  @Int64()
  external int ts; 
  @Double()
  external double ask;
  @Double()
  external double ask_size;
  @Double()
  external double bid;
  @Double()
  external double bid_size;
  @Int64()
  external int quote_time;
  external Pointer<ffi.Utf8> symbol;
}
void free_market(Market m){
  _free_market(m);
}
final _free_market_Dart _free_market = _dl.lookupFunction<_free_market_C, _free_market_Dart>('free_market');
typedef _free_market_C = Void Function(Market m);
typedef _free_market_Dart = void Function(Market m);

/// C function `get_market_point`.
Pointer<Market> get_market_point(
  Pointer<Ws> ws,
  Pointer<ffi.Utf8> ch,
) {
  return _get_market_point(ws, ch);
}
final _get_market_point_Dart _get_market_point= _dl.lookupFunction<_get_market_point_C, _get_market_point_Dart>('get_market_point');
typedef _get_market_point_C = Pointer<Market> Function(
  Pointer<Ws> ws,
  Pointer<ffi.Utf8> ch,
);
typedef _get_market_point_Dart = Pointer<Market> Function(
  Pointer<Ws> ws,
  Pointer<ffi.Utf8> ch,
);
/// C function `get_market`.
Market get_market(
  Pointer<Ws> ws,
  Pointer<ffi.Utf8> ch,
) {
  return _get_market(ws, ch);
}
final _get_market_Dart _get_market= _dl.lookupFunction<_get_market_C, _get_market_Dart>('get_market');
typedef _get_market_C = Market Function(
  Pointer<Ws> ws,
  Pointer<ffi.Utf8> ch,
);
typedef _get_market_Dart = Market Function(
  Pointer<Ws> ws,
  Pointer<ffi.Utf8> ch,
);



/// C function `error_message_utf8`.
int error_message_utf8(
  Pointer<ffi.Utf8> buf,
  int length,
) {
  return _error_message_utf8(buf, length);
}
final _error_message_utf8_Dart _error_message_utf8 = _dl.lookupFunction<_error_message_utf8_C, _error_message_utf8_Dart>('error_message_utf8');
typedef _error_message_utf8_C = Int32 Function(
  Pointer<ffi.Utf8> buf,
  Int32 length,
);
typedef _error_message_utf8_Dart = int Function(
  Pointer<ffi.Utf8> buf,
  int length,
);

/// C function `is_alive`.
int is_alive(
  Pointer<Ws> ws,
) {
  return _is_alive(ws);
}
final _is_alive_Dart _is_alive = _dl.lookupFunction<_is_alive_C, _is_alive_Dart>('is_alive');
typedef _is_alive_C = Int32 Function(
  Pointer<Ws> ws,
);
typedef _is_alive_Dart = int Function(
  Pointer<Ws> ws,
);

/// C function `last_error_length`.
int last_error_length() {
  return _last_error_length();
}
final _last_error_length_Dart _last_error_length = _dl.lookupFunction<_last_error_length_C, _last_error_length_Dart>('last_error_length');
typedef _last_error_length_C = Int32 Function();
typedef _last_error_length_Dart = int Function();

/// C function `get_symbols`
int get_symbols(
  int port,
  Pointer<ffi.Utf8> url){
    return _get_symbols(port, url);
}
final _get_symbols_Dart _get_symbols = _dl.lookupFunction<_get_symbols_C, _get_symbols_Dart>('get_symbols');
typedef _get_symbols_C = Int32 Function(
  Int64 port,
  Pointer<ffi.Utf8> url,
);
typedef _get_symbols_Dart = int Function(
  int port,
  Pointer<ffi.Utf8> url,
);
/// C function `get_symbols`.
int load_page(
  int port,
  Pointer<ffi.Utf8> url,
) {
  return _load_page(port, url);
}
final _load_page_Dart _load_page = _dl.lookupFunction<_load_page_C, _load_page_Dart>('load_page');
typedef _load_page_C = Int32 Function(
  Int64 port,
  Pointer<ffi.Utf8> url,
);
typedef _load_page_Dart = int Function(
  int port,
  Pointer<ffi.Utf8> url,
);

/// C function `start_timer`.
int start_timer(
  int port,
) {
  return _start_timer(port);
}
final _start_timer_Dart _start_timer = _dl.lookupFunction<_start_timer_C, _start_timer_Dart>('start_timer');
typedef _start_timer_C = Int32 Function(
  Int64 port,
);
typedef _start_timer_Dart = int Function(
  int port,
);

/// C function `start_ws`.
Pointer<Ws> start_ws(
  Pointer<ffi.Utf8> url,
) {
  return _start_ws(url);
}
final _start_ws_Dart _start_ws = _dl.lookupFunction<_start_ws_C, _start_ws_Dart>('start_ws');
typedef _start_ws_C = Pointer<Ws> Function(
  Pointer<ffi.Utf8> url,
);
typedef _start_ws_Dart = Pointer<Ws> Function(
  Pointer<ffi.Utf8> url,
);
/// C function `stop_ws`.
void stop_ws(
  Pointer<Ws> ws,
) {
  _stop_ws(ws);
}
final _stop_ws_Dart _stop_ws = _dl.lookupFunction<_stop_ws_C, _stop_ws_Dart>('stop_ws');
typedef _stop_ws_C = Void Function(
  Pointer<Ws> ws,
);
typedef _stop_ws_Dart = void Function(
  Pointer<Ws> ws,
);

/// Binding to `allo-isolate` crate
void store_dart_post_cobject(
  Pointer<NativeFunction<Int8 Function(Int64, Pointer<Dart_CObject>)>> ptr,
) {
  _store_dart_post_cobject(ptr);
}
final _store_dart_post_cobject_Dart _store_dart_post_cobject = _dl.lookupFunction<_store_dart_post_cobject_C, _store_dart_post_cobject_Dart>('store_dart_post_cobject');
typedef _store_dart_post_cobject_C = Void Function(
  Pointer<NativeFunction<Int8 Function(Int64, Pointer<Dart_CObject>)>> ptr,
);
typedef _store_dart_post_cobject_Dart = void Function(
  Pointer<NativeFunction<Int8 Function(Int64, Pointer<Dart_CObject>)>> ptr,
);
