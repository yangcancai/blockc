#import "HbFfiPlugin.h"
#if __has_include(<hb_ffi/hb_ffi-Swift.h>)
#import <hb_ffi/hb_ffi-Swift.h>
#else
// Support project import fallback if the generated compatibility header
// is not copied when this plugin is created as a library.
// https://forums.swift.org/t/swift-static-libraries-dont-copy-generated-objective-c-header/19816
#import "hb_ffi-Swift.h"
#endif

@implementation HbFfiPlugin
+ (void)registerWithRegistrar:(NSObject<FlutterPluginRegistrar>*)registrar {
  [SwiftHbFfiPlugin registerWithRegistrar:registrar];
}
@end
