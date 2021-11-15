import 'dart:ffi';

import 'package:flutter/material.dart';
import 'package:hb/hb.dart';
import 'package:hb/ffi.dart';
import 'package:ffi/ffi.dart';

void main() => runApp(MyApp());

class MyApp extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Flutter Demo',
      theme: ThemeData(
        primarySwatch: Colors.blue,
      ),
      home: MyHomePage(title: 'Flutter Demo Home Page'),
    );
  }
}

class MyHomePage extends StatefulWidget {
  MyHomePage({Key? key, required this.title}) : super(key: key);
  final String title;
  @override
  _MyHomePageState createState() => _MyHomePageState();
}

class _MyHomePageState extends State<MyHomePage> {
  int _counter = 0;
  Hb hb = Hb();
  late Pointer<Ws> ws;
  @override
  void initState() {
    super.initState();
    Hb.setup();
    hb.startTimer();
    // ws = start_ws("wss://api.huobi.pro/ws".toNativeUtf8());
    ws = start_ws("ws://127.0.0.1:2794".toNativeUtf8());
    if (ws == nullptr){
       print("start_ws null");
    }else{
      is_alive(ws);
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text(widget.title),
      ),
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Text(
              'You have pushed the button this many times:',
            ),
            Text(
              '$_counter',
              style: Theme.of(context).textTheme.headline4,
            ),
            const SizedBox(height: 100),
            RaisedButton(
              color: Colors.greenAccent,
              child: Text(
                'Hb rust-lang.org',
                style: TextStyle(
                  color: Colors.white,
                ),
              ),
              onPressed: _showWebPage,
            )
          ],
        ),
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: _incrementCounter,
        tooltip: 'Increment',
        child: Icon(Icons.add),
      ),
    );
  }

  void _incrementCounter() {
    setState(() {
      _counter = _counter + 1;
     print(is_alive(ws));
      // _counter = adder.add(_counter, 1);
    });
  }
  void _startWs(){
    // ws = start_ws("ws://192.168.3.6:2794".toNativeUtf8());
    ws = start_ws("wss://api.huobi.pro/ws".toNativeUtf8());
    if (ws == nullptr){
      print("start_ws null");
    }
  }
  void _showWebPage() async {
    final html = await hb.loadPage('https://www.rust-lang.org/');
    // final html = "kkk";
    showModalBottomSheet(
      context: context,
      isScrollControlled: true,
      isDismissible: true,
      builder: (context) => SingleChildScrollView(
        child: Text(html),
      ),
    );
  }
}
