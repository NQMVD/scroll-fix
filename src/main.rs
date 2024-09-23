use input::{event::keyboard::Keycode, event::mouse::MouseButton, event::Event, event::EventStream, event::EventType, event::MouseScrollDelta, event::RelMotion};
use std::ptr;
use x11::xlib::{XCloseDisplay, XOpenDisplay, XQueryPointer, XSendEvent, XDefaultRootWindow, Display, KeyPressMask, ButtonPressMask, ButtonReleaseMask, MotionNotify, ButtonPress, ButtonRelease, XFlush, XSync, XEvent, XAnyEvent};

fn main() {
    let mut event_stream = EventStream::new().expect("Failed to create event stream");

    let display = unsafe { XOpenDisplay(ptr::null()) };
    if display.is_null() {
        eprintln!("Unable to open X display");
        return;
    }

    let root = unsafe { XDefaultRootWindow(display) };

    for event in event_stream.iter() {
        match event {
            Ok(Event::MouseScroll(MouseScrollDelta::LineDelta(_, dy))) => {
                correct_scroll_event(display, root, dy);
            }
            _ => {}
        }
    }

    unsafe { XCloseDisplay(display) };
}

fn correct_scroll_event(display: *mut Display, root: u64, dy: f64) {
    static mut LAST_DY: f64 = 0.0;

    unsafe {
        if (dy - LAST_DY).abs() > 5.0 {
            dy = LAST_DY;
        }
        LAST_DY = dy;
    }

    let mut event = XEvent {
        xany: XAnyEvent {
            type: MotionNotify,
            serial: 0,
            send_event: 0,
            display: display,
            window: root,
        },
    };

    unsafe {
        XSendEvent(display, root, 0, ButtonPressMask, &mut event);
        XFlush(display);
    }
}
