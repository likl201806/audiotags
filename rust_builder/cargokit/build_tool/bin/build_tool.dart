/// This is copied from Cargokit (which is the official way to use it currently)
/// Details: https://fzyzcjy.github.io/flutter_rust_bridge/manual/integrate/builtin

// ignore: avoid_relative_lib_imports
import '../lib/src/build_tool.dart' as build_tool;

void main(List<String> arguments) {
  build_tool.runMain(arguments);
}
