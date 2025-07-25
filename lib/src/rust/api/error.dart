// This file is automatically generated, so please do not edit it.
// @generated by `flutter_rust_bridge`@ 2.11.1.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:freezed_annotation/freezed_annotation.dart' hide protected;
part 'error.freezed.dart';

// These function are ignored because they are on traits that is not defined in current crate (put an empty `#[frb]` on it to unignore): `fmt`, `fmt`

@freezed
sealed class AudioTagsError with _$AudioTagsError implements FrbException {
  const AudioTagsError._();

  const factory AudioTagsError.invalidPath() = AudioTagsError_InvalidPath;
  const factory AudioTagsError.noTags() = AudioTagsError_NoTags;
  const factory AudioTagsError.openFile({
    required String message,
  }) = AudioTagsError_OpenFile;
  const factory AudioTagsError.write({
    required String message,
  }) = AudioTagsError_Write;
}
