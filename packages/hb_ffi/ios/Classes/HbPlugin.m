#import "HbPlugin.h"
#if __has_include(<hb/hb-Swift.h>)
#import <hb/hb-Swift.h>
#else
// Support project import fallback if the generated compatibility header
// is not copied when this plugin is created as a library.
// https://forums.swift.org/t/swift-static-libraries-dont-copy-generated-objective-c-header/19816
#import "hb-Swift.h"
#endif

@implementation HbPlugin
+ (void)registerWithRegistrar:(NSObject<FlutterPluginRegistrar>*)registrar {
  [SwiftHbPlugin registerWithRegistrar:registrar];
}
@end
