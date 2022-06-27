import 'dart:html';
import 'dart:js';
import 'package:js/js_util.dart';

import 'A.dart';
import 'B.dart';

void main() {
  setProperty(window, 'a', allowInterop(a));
  setProperty(window, 'b', allowInterop(b));

  window.console.info("in `main.dart`");
}
