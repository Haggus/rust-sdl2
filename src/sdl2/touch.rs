use std::ptr;

use sys::touch as ll;

pub type Finger = ll::Finger;
pub type TouchDevice = ll::TouchDevice;

pub fn num_touch_devices() -> i32 {
    unsafe { ll::SDL_GetNumTouchDevices() }
}

pub fn touch_device(index: i32) -> TouchDevice {
    unsafe { ll::SDL_GetTouchDevice(index) }
}

pub fn num_touch_fingers(touch: TouchDevice) -> i32 {
    unsafe { ll::SDL_GetNumTouchFingers(touch) }
}

pub fn touch_finger(touch: TouchDevice, index: i32) -> Option<Finger> {
    let raw = unsafe { ll::SDL_GetTouchFinger(touch, index) };

    if raw == ptr::null_mut() {
        None
    } else {
        unsafe { Some(*raw) }
    }
}
