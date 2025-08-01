// This file is automatically generated, so please do not edit it.
// @generated by `flutter_rust_bridge`@ 2.11.1.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../frb_generated.dart';
import 'error.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'picture.dart';
import 'tag.dart';

// These functions are ignored because they are not marked as `pub`: `get_file`

Future<Tag> read({required String path}) =>
    RustLib.instance.api.crateApiApiRead(path: path);

Future<void> write({required String path, required Tag data}) =>
    RustLib.instance.api.crateApiApiWrite(path: path, data: data);
