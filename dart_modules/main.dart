import 'dart:html';
import 'A.dart';
import 'B.dart';

void a() {
  A:a();
}

void b() {
  B:b();
}

void main() {
  window.console.info("in `main.dart`");
}
