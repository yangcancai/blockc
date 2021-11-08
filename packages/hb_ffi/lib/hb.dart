import 'dart:async';
import 'dart:ffi';
import 'dart:isolate';
import 'package:ffi/ffi.dart';
import 'package:isolate/ports.dart';

import 'ffi.dart' as native;

class Hb {
  static setup() {
    native.store_dart_post_cobject(NativeApi.postCObject);
    print("Hb Setup Done");
  }
int startTimer(){
  final interactiveCppRequests = ReceivePort()
    ..listen((data) {
      print('Received: ${data} from Rust ');
    });
  final int nativePort = interactiveCppRequests.sendPort.nativePort;
  return  native.start_timer(nativePort);
}
  Future<String> loadPage(String url) {
    var urlPointer = url.toNativeUtf8();
    final completer = Completer<String>();
    final sendPort = singleCompletePort(completer);
    final res = native.load_page(
      sendPort.nativePort,
      urlPointer,
    );
    if (res != 1) {
      _throwError();
    }
    return completer.future;
  }

  void _throwError() {
    final length = native.last_error_length();
    final Pointer<Utf8> message = calloc.allocate(length);
    native.error_message_utf8(message, length);
    final error = message.toDartString();
    print(error);
    throw error;
  }
}
