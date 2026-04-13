/// Returns true if microphone access is already granted.
pub fn is_granted() -> bool {
    #[cfg(target_os = "macos")]
    {
        check_macos_status() == 3 // AVAuthorizationStatusAuthorized
    }
    #[cfg(not(target_os = "macos"))]
    { true }
}

/// Shows the native permission dialog if status is NotDetermined.
pub fn request_if_needed() {
    #[cfg(target_os = "macos")]
    request_macos();
}

#[cfg(target_os = "macos")]
fn check_macos_status() -> i64 {
    use objc2::runtime::AnyClass;
    use objc2::msg_send;
    use objc2_foundation::NSString;

    unsafe {
        let Some(cls) = AnyClass::get(c"AVCaptureDevice") else { return -1 };
        let nsstring_cls = AnyClass::get(c"NSString").unwrap();
        let media_type: objc2::rc::Retained<NSString> =
            msg_send![nsstring_cls, stringWithUTF8String: b"soun\0".as_ptr() as *const i8];
        msg_send![cls, authorizationStatusForMediaType: &*media_type]
    }
}

#[cfg(target_os = "macos")]
fn request_macos() {
    use block2::RcBlock;
    use objc2::runtime::{AnyClass, Bool};
    use objc2::msg_send;
    use objc2_foundation::NSString;

    unsafe {
        let Some(cls) = AnyClass::get(c"AVCaptureDevice") else { return };

        let nsstring_cls = AnyClass::get(c"NSString").unwrap();
        let media_type: objc2::rc::Retained<NSString> =
            msg_send![nsstring_cls, stringWithUTF8String: b"soun\0".as_ptr() as *const i8];

        let status: i64 = msg_send![cls, authorizationStatusForMediaType: &*media_type];
        if status != 0 {
            return; // already decided
        }

        let block = RcBlock::new(move |_: Bool| {});
        let _: () = msg_send![
            cls,
            requestAccessForMediaType: &*media_type,
            completionHandler: &*block
        ];
    }
}
