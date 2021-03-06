// x11-rs: Rust bindings for X11 libraries
// The X11 libraries are available under the MIT license.
// These bindings are public domain.

use std::cmp::min;
use std::convert::From;
use std::mem::{
  size_of,
  zeroed,
};
use std::slice::{
  from_raw_parts,
  from_raw_parts_mut,
};

use libc::{
  c_char,
  c_int,
  c_long,
  c_short,
  c_uchar,
  c_uint,
  c_ulong,
  c_ushort,
  c_void,
};


//
// functions
//


#[link(name="X11")]
extern "C" {
  pub fn XActivateScreenSaver (display: *mut Display);
  pub fn XAddConnectionWatch (display: *mut Display, watch: XConnectionWatchProc, client_data: XPointer) -> Status;
  pub fn XAddExtension (display: *mut Display) -> *mut XExtCodes;
  pub fn XAddHost (display: *mut Display, host: *const XHostAddress) -> c_int;
  pub fn XAddHosts (display: *mut Display, hosts: *const XHostAddress, num_hosts: c_int) -> c_int;
  pub fn XAddPixel (image: *mut XImage, value: c_long);
  pub fn XAddToSaveSet (display: *mut Display, window: Window);
  pub fn XAllocClassHint () -> *mut XClassHint;
  pub fn XAllocColor (display: *mut Display, colormap: Colormap, color: *mut XColor) -> Status;
  pub fn XAllPlanes () -> c_ulong;
  pub fn XBitmapBitOrder (display: *mut Display) -> c_int;
  pub fn XBitmapPad (display: *mut Display) -> c_int;
  pub fn XBitmapUnit (display: *mut Display) -> c_int;
  pub fn XBlackPixel (display: *mut Display, screen_num: c_int) -> c_ulong;
  pub fn XBlackPixelOfScreen (screen: *mut Screen) -> c_ulong;
  pub fn XCellsOfScreen (screen: *mut Screen) -> c_int;
  pub fn XCheckMaskEvent (display: *mut Display, event_mask: c_long, event_return: *mut XEvent) -> Bool;
  pub fn XCheckTypedEvent (display: *mut Display, event_type: c_int, event_return: *mut XEvent) -> Bool;
  pub fn XCloseDisplay (display: *mut Display);
  pub fn XCloseIM (im: XIM) -> Status;
  pub fn XCreateColormap (display: *mut Display, window: Window, visual: *mut Visual, alloc: c_int) -> Colormap;
  pub fn XCreateGC (display: *mut Display, drawable: Drawable, valuemask: c_ulong, values: *const XGCValues) -> GC;
  pub fn XCreateIC (im: XIM, ...) -> XIC;
  pub fn XCreateSimpleWindow (display: *mut Display, parent: Window, x: c_int, y: c_int, width: c_uint, height: c_uint, border_width: c_uint, border: c_ulong, background: c_ulong) -> Window;
  pub fn XCreateWindow (display: *mut Display, parent: Window, x: c_int, y: c_int, width: c_uint, height: c_uint, border_width: c_uint, depth: c_int, class: c_int, visual: *mut Visual, attr_mask: c_ulong, attr: *const XSetWindowAttributes) -> Window;
  pub fn XDefaultColormap (display: *mut Display, screen_num: c_int) -> Colormap;
  pub fn XDefaultRootWindow (display: *mut Display) -> Window;
  pub fn XDefaultScreen (display: *mut Display) -> c_int;
  pub fn XDefaultVisual (display: *mut Display, screen_num: c_int) -> *mut Visual;
  pub fn XDefineCursor (display: *mut Display, window: Window, cursor: Cursor);
  pub fn XDestroyIC (ic: XIC);
  pub fn XDestroyWindow (display: *mut Display, window: Window);
  pub fn XDisableAccessControl (display: *mut Display) -> c_int;
  pub fn XDisplayOfIM (im: XIM) -> *mut Display;
  pub fn XDrawRectangle (display: *mut Display, drawable: Drawable, gc: GC, x: c_int, y: c_int, width: c_uint, height: c_uint);
  pub fn XEnableAccessControl (display: *mut Display) -> c_int;
  pub fn XFetchName (display: *mut Display, window: Window, name: *mut *mut c_char) -> Status;
  pub fn XFillRectangle (display: *mut Display, drawable: Drawable, gc: GC, x: c_int, y: c_int, width: c_uint, height: c_uint);
  pub fn XFilterEvent (event: *mut XEvent, window: Window) -> Bool;
  pub fn XFlush (display: *mut Display);
  pub fn XFree (mem: *mut c_void);
  pub fn XFreeColormap (display: *mut Display, colormap: Colormap);
  pub fn XFreeGC (display: *mut Display, gc: GC);
  pub fn XGetClassHint (display: *mut Display, window: Window, class_hints_return: *mut XClassHint) -> Status;
  pub fn XGetGeometry (display: *mut Display, drawable: Drawable, root: *mut Window, x: *mut c_int, y: *mut c_int, width: *mut c_uint, height: *mut c_uint, border_width: *mut c_uint, depth: *mut c_uint) -> Status;
  pub fn XGetIMValues (im: XIM, ...) -> *const c_char;
  pub fn XGetKeyboardMapping (display: *mut Display, keycode: *const KeyCode, keycode_count: c_int, keysyms_returned_per_keycode: *mut c_int)-> *mut KeySym;
  pub fn XGetVisualInfo (display: *mut Display, mask: c_long, template: *const XVisualInfo, nitems: *mut c_int) -> *mut XVisualInfo;
  pub fn XGetWindowAttributes (display: *mut Display, window: Window, attr: *mut XWindowAttributes) -> Status;
  pub fn XGetWindowProperty (display: *mut Display, window: Window, property: Atom, long_offset: c_long, long_length: c_long, delete: Bool, requestedkind: Atom, outkind: *mut Atom, out_format: *mut c_int, out_length: *mut c_ulong, out_remaining: *mut c_ulong, out_data: *mut *mut c_char) -> c_int;
  pub fn XGetWMName (display: *mut Display, window: Window, name: *mut XTextProperty) -> Status;
  pub fn XGrabKey (display: *mut Display, keycode: KeyCode, modifiers: c_uint, window: Window, owner_events: c_int, pointer_mode: c_int, keyboard_mode: c_int);
  pub fn XGrabPointer (display: *mut Display, grab_window: Window, owner_events: Bool, event_mask: c_uint, pointer_mode: c_int, keyboard_mode: c_int, confine_to: Window, cursor: Cursor, time: Time) -> c_int;
  pub fn XHeightOfScreen (screen: *mut Screen) -> c_int;
  pub fn XIMOfIC (ic: XIC) -> XIM;
  pub fn XInitThreads () -> Status;
  pub fn XInternAtom (display: *mut Display, name: *const c_char, only_if_exists: Bool) -> Atom;
  pub fn XkbSetDetectableAutoRepeat (display: *mut Display, detectable: Bool, supported_return: *mut Bool) -> Bool;
  pub fn XKeycodeToKeysym (display: *mut Display, keycode: KeyCode, index: c_int) -> KeySym;
  pub fn XListHosts (display: *mut Display, nhosts_return: *mut c_int, state_return: Bool) -> *mut XHostAddress;
  pub fn XLocaleOfIM (im: XIM) -> *const c_char;
  pub fn XLookupKeysym (key_event: *const XKeyEvent, index: c_int) -> KeySym;
  pub fn XMapRaised (display: *mut Display, window: Window);
  pub fn XMapWindow (display: *mut Display, window: Window);
  pub fn XMoveWindow (display: *mut Display, window: Window, x: c_int, y: c_int);
  pub fn XNextEvent (display: *mut Display, event: *mut XEvent);
  pub fn XOpenDisplay (name: *const c_char) -> *mut Display;
  pub fn XOpenIM (display: *mut Display, db: XrmDatabase, res_name: *const c_char, res_class: *const c_char) -> XIM;
  pub fn XPeekEvent (display: *mut Display, event_return: *mut XEvent);
  pub fn XPending (display: *mut Display) -> c_int;
  pub fn XQueryTree (display: *mut Display, window: Window, root: *mut Window, parent: *mut Window, children: *mut *mut Window, nchildren: *mut c_uint) -> Status;
  pub fn XRefreshKeyboardMapping (event_map: *mut XMappingEvent);
  pub fn XRemoveHost (display: *mut Display, host: *const XHostAddress) -> c_int;
  pub fn XRemoveHosts (display: *mut Display, hosts: *const XHostAddress, num_hosts: c_int) -> c_int;
  pub fn XResizeWindow (display: *mut Display, window: Window, width: c_uint, height: c_uint);
  pub fn XRootWindow (display: *mut Display, screen_num: c_int) -> Window;
  pub fn XScreenCount (display: *mut Display) -> c_int;
  pub fn XScreenOfDisplay (display: *mut Display, screen_num: c_int) -> *mut Screen;
  pub fn XSendEvent (display: *mut Display, window: Window, propagate: Bool, event_mask: c_long, event: *const XEvent) -> Status;
  pub fn XSetAccessControl (display: *mut Display, mode: c_int) -> c_int;
  pub fn XSetClassHint (display: *mut Display, window: Window, class_hints: *const XClassHint);
  pub fn XSetClipRectangles (display: *mut Display, gc: GC, clip_x_origin: c_int, clip_y_origin: c_int, rectangles: *const XRectangle, n: c_int, ordering: c_int);
  pub fn XSetErrorHandler (handler: ErrorHandler) -> ErrorHandler;
  pub fn XSetForeground (display: *mut Display, gc: GC, fg: c_ulong);
  pub fn XSetICFocus (ic: XIC);
  pub fn XSetIMValues (im: XIM, ...) -> *const c_char;
  pub fn XSetWMNormalHints (display: *mut Display, window: Window, hints: *const XSizeHints);
  pub fn XSetWMProtocols (display: *mut Display, window: Window, protocols: *const Atom, count: c_int) -> Status;
  pub fn XStoreName (display: *mut Display, window: Window, name: *const c_char);
  pub fn XStringToKeysym (display: *mut Display, window: Window, string: *const c_char) -> *mut KeySym;
  pub fn XUngrabPointer (display: *mut Display, time: Time);
  pub fn XUnmapWindow (display: *mut Display, window: Window);
  pub fn XUnsetICFocus (ic: XIC);
  pub fn Xutf8LookupString (ic: XIC, event: *const XKeyPressedEvent, buffer_return: *mut c_char, buffer_size: c_int, keysym_return: *mut KeySym, status_return: *mut Status) -> c_int;
  pub fn XVisualIDFromVisual (visual: *mut Visual) -> VisualID;
  pub fn XWarpPointer (display: *mut Display, src_window: Window, dest_window: Window, src_x: c_int, src_y: c_int, src_width: c_uint, src_height: c_uint, dest_x: c_int, dest_y: c_int);
  pub fn XWhitePixel (display: *mut Display, screen_num: c_int) -> c_ulong;
  pub fn XWhitePixelOfScreen (screen: *mut Screen) -> c_ulong;
  pub fn XWidthOfScreen (screen: *mut Screen) -> c_int;
}


//
// types
//


pub type Atom = XID;
pub type Bool = c_int;
pub type Colormap = XID;
pub type Cursor = XID;
pub type Drawable = XID;
pub type Font = XID;
pub type GC = XID;
pub type KeyCode = c_uchar;
pub type KeySym = XID;
pub type Mask = c_ulong;
pub type Pixmap = XID;
pub type Status = Bool;
pub type Time = c_ulong;
pub type VisualID = XID;
pub type Window = XID;
pub type XIC = *mut XIC_Rec;
pub type XID = c_ulong;
pub type XIM = *mut XIM_Rec;
pub type XPointer = *mut c_void;

#[allow(missing_copy_implementations)]
#[repr(C)]
pub struct Display;

#[allow(missing_copy_implementations)]
#[repr(C)]
pub struct Screen;

#[allow(missing_copy_implementations)]
#[repr(C)]
pub struct Visual;

#[allow(missing_copy_implementations)]
#[repr(C)]
pub struct XIC_Rec;

#[allow(missing_copy_implementations)]
#[repr(C)]
pub struct XIM_Rec;

#[allow(missing_copy_implementations)]
#[repr(C)]
pub struct _XrmHashBucketRec;

pub type AddPixelFn = Option<unsafe extern "C" fn () -> c_int>;
pub type CreateImageFn = Option<unsafe extern "C" fn () -> *mut XImage>;
pub type DestroyImageFn = Option<unsafe extern "C" fn () -> c_int>;
pub type ErrorHandler = Option<unsafe extern "C" fn (*mut Display, *const XErrorEvent) -> c_int>;
pub type GetPixelFn = Option<unsafe extern "C" fn () -> c_ulong>;
pub type PutPixelFn = Option<unsafe extern "C" fn () -> c_int>;
pub type SubImageFn = Option<unsafe extern "C" fn () -> *mut XImage>;
pub type XConnectionWatchProc = Option<unsafe extern "C" fn (*mut Display, XPointer, c_int, Bool, XPointer)>;
pub type XrmDatabase = *mut _XrmHashBucketRec;


//
// event structures
//


#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XEvent {
  pub pad: [c_long; 24],
}

impl XEvent {
  pub fn kind (&self) -> c_int {
    unsafe {
      *(self as *const XEvent as *const c_int)
    }
  }
}

impl From<XAnyEvent> for XEvent {
  fn from (e: XAnyEvent) -> XEvent {
    unsafe { transmute_union(&e) }
  }
}

impl From<XButtonEvent> for XEvent {
  fn from (e: XButtonEvent) -> XEvent {
    unsafe { transmute_union(&e) }
  }
}

impl From<XCirculateEvent> for XEvent {
  fn from (e: XCirculateEvent) -> XEvent {
    unsafe { transmute_union(&e) }
  }
}

impl From<XCirculateRequestEvent> for XEvent {
  fn from (e: XCirculateRequestEvent) -> XEvent {
    unsafe { transmute_union(&e) }
  }
}

impl From<XClientMessageEvent> for XEvent {
  fn from (e: XClientMessageEvent) -> XEvent {
    unsafe { transmute_union(&e) }
  }
}

impl From<XColormapEvent> for XEvent {
  fn from (e: XColormapEvent) -> XEvent {
    unsafe { transmute_union(&e) }
  }
}

impl From<XConfigureEvent> for XEvent {
  fn from (e: XConfigureEvent) -> XEvent {
    unsafe { transmute_union(&e) }
  }
}

impl From<XConfigureRequestEvent> for XEvent {
  fn from (e: XConfigureRequestEvent) -> XEvent {
    unsafe { transmute_union(&e) }
  }
}

impl From<XCreateWindowEvent> for XEvent {
  fn from (e: XCreateWindowEvent) -> XEvent {
    unsafe { transmute_union(&e) }
  }
}

impl From<XCrossingEvent> for XEvent {
  fn from (e: XCrossingEvent) -> XEvent {
    unsafe { transmute_union(&e) }
  }
}

impl From<XDestroyWindowEvent> for XEvent {
  fn from (e: XDestroyWindowEvent) -> XEvent {
    unsafe { transmute_union(&e) }
  }
}

impl From<XErrorEvent> for XEvent {
  fn from (e: XErrorEvent) -> XEvent {
    unsafe { transmute_union(&e) }
  }
}

impl From<XExposeEvent> for XEvent {
  fn from (e: XExposeEvent) -> XEvent {
    unsafe { transmute_union(&e) }
  }
}

impl From<XFocusChangeEvent> for XEvent {
  fn from (e: XFocusChangeEvent) -> XEvent {
    unsafe { transmute_union(&e) }
  }
}

impl From<XGraphicsExposeEvent> for XEvent {
  fn from (e: XGraphicsExposeEvent) -> XEvent {
    unsafe { transmute_union(&e) }
  }
}

impl From<XGravityEvent> for XEvent {
  fn from (e: XGravityEvent) -> XEvent {
    unsafe { transmute_union(&e) }
  }
}

impl From<XKeyEvent> for XEvent {
  fn from (e: XKeyEvent) -> XEvent {
    unsafe { transmute_union(&e) }
  }
}

impl From<XKeymapEvent> for XEvent {
  fn from (e: XKeymapEvent) -> XEvent {
    unsafe { transmute_union(&e) }
  }
}

impl From<XMapEvent> for XEvent {
  fn from (e: XMapEvent) -> XEvent {
    unsafe { transmute_union(&e) }
  }
}

impl From<XMappingEvent> for XEvent {
  fn from (e: XMappingEvent) -> XEvent {
    unsafe { transmute_union(&e) }
  }
}

impl From<XMotionEvent> for XEvent {
  fn from (e: XMotionEvent) -> XEvent {
    unsafe { transmute_union(&e) }
  }
}

impl From<XNoExposeEvent> for XEvent {
  fn from (e: XNoExposeEvent) -> XEvent {
    unsafe { transmute_union(&e) }
  }
}

impl From<XPropertyEvent> for XEvent {
  fn from (e: XPropertyEvent) -> XEvent {
    unsafe { transmute_union(&e) }
  }
}

impl From<XReparentEvent> for XEvent {
  fn from (e: XReparentEvent) -> XEvent {
    unsafe { transmute_union(&e) }
  }
}

impl From<XResizeRequestEvent> for XEvent {
  fn from (e: XResizeRequestEvent) -> XEvent {
    unsafe { transmute_union(&e) }
  }
}

impl From<XSelectionClearEvent> for XEvent {
  fn from (e: XSelectionClearEvent) -> XEvent {
    unsafe { transmute_union(&e) }
  }
}

impl From<XSelectionEvent> for XEvent {
  fn from (e: XSelectionEvent) -> XEvent {
    unsafe { transmute_union(&e) }
  }
}

impl From<XSelectionRequestEvent> for XEvent {
  fn from (e: XSelectionRequestEvent) -> XEvent {
    unsafe { transmute_union(&e) }
  }
}

impl From<XUnmapEvent> for XEvent {
  fn from (e: XUnmapEvent) -> XEvent {
    unsafe { transmute_union(&e) }
  }
}

impl From<XVisibilityEvent> for XEvent {
  fn from (e: XVisibilityEvent) -> XEvent {
    unsafe { transmute_union(&e) }
  }
}

#[test]
fn xevent_size_test () {
  assert!(size_of::<XEvent>() >= size_of::<XAnyEvent>());
  assert!(size_of::<XEvent>() >= size_of::<XButtonEvent>());
  assert!(size_of::<XEvent>() >= size_of::<XCirculateEvent>());
  assert!(size_of::<XEvent>() >= size_of::<XCirculateRequestEvent>());
  assert!(size_of::<XEvent>() >= size_of::<XClientMessageEvent>());
  assert!(size_of::<XEvent>() >= size_of::<XColormapEvent>());
  assert!(size_of::<XEvent>() >= size_of::<XConfigureEvent>());
  assert!(size_of::<XEvent>() >= size_of::<XConfigureRequestEvent>());
  assert!(size_of::<XEvent>() >= size_of::<XCreateWindowEvent>());
  assert!(size_of::<XEvent>() >= size_of::<XCrossingEvent>());
  assert!(size_of::<XEvent>() >= size_of::<XDestroyWindowEvent>());
  assert!(size_of::<XEvent>() >= size_of::<XErrorEvent>());
  assert!(size_of::<XEvent>() >= size_of::<XExposeEvent>());
  assert!(size_of::<XEvent>() >= size_of::<XFocusChangeEvent>());
  assert!(size_of::<XEvent>() >= size_of::<XGraphicsExposeEvent>());
  assert!(size_of::<XEvent>() >= size_of::<XGravityEvent>());
  assert!(size_of::<XEvent>() >= size_of::<XKeyEvent>());
  assert!(size_of::<XEvent>() >= size_of::<XKeymapEvent>());
  assert!(size_of::<XEvent>() >= size_of::<XMapEvent>());
  assert!(size_of::<XEvent>() >= size_of::<XMappingEvent>());
  assert!(size_of::<XEvent>() >= size_of::<XMapRequestEvent>());
  assert!(size_of::<XEvent>() >= size_of::<XMotionEvent>());
  assert!(size_of::<XEvent>() >= size_of::<XNoExposeEvent>());
  assert!(size_of::<XEvent>() >= size_of::<XPropertyEvent>());
  assert!(size_of::<XEvent>() >= size_of::<XReparentEvent>());
  assert!(size_of::<XEvent>() >= size_of::<XResizeRequestEvent>());
  assert!(size_of::<XEvent>() >= size_of::<XSelectionClearEvent>());
  assert!(size_of::<XEvent>() >= size_of::<XSelectionEvent>());
  assert!(size_of::<XEvent>() >= size_of::<XSelectionRequestEvent>());
  assert!(size_of::<XEvent>() >= size_of::<XUnmapEvent>());
  assert!(size_of::<XEvent>() >= size_of::<XVisibilityEvent>());
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XAnyEvent {
  pub kind: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub window: Window,
}

impl From<XEvent> for XAnyEvent {
  fn from (e: XEvent) -> XAnyEvent {
    unsafe { transmute_union(&e) }
  }
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XButtonEvent {
  pub kind: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub window: Window,
  pub root: Window,
  pub subwindow: Window,
  pub time: Time,
  pub x: c_int,
  pub y: c_int,
  pub x_root: c_int,
  pub y_root: c_int,
  pub state: c_uint,
  pub button: c_uint,
  pub same_screen: Bool,
}
pub type XButtonPressedEvent = XButtonEvent;
pub type XButtonReleasedEvent = XButtonEvent;

impl From<XEvent> for XButtonEvent {
  fn from (e: XEvent) -> XButtonEvent {
    unsafe { transmute_union(&e) }
  }
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XCirculateEvent {
  pub kind: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub event: Window,
  pub window: Window,
  pub place: c_int,
}

impl From<XEvent> for XCirculateEvent {
  fn from (e: XEvent) -> XCirculateEvent {
    unsafe { transmute_union(&e) }
  }
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XCirculateRequestEvent {
  pub kind: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub parent: Window,
  pub window: Window,
  pub place: c_int,
}

impl From<XEvent> for XCirculateRequestEvent {
  fn from (e: XEvent) -> XCirculateRequestEvent {
    unsafe { transmute_union(&e) }
  }
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XClientMessageEvent {
  pub kind: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub window: Window,
  pub message_type: Atom,
  pub format: c_int,
  pub data: ClientMessageData,
}

impl From<XEvent> for XClientMessageEvent {
  fn from (e: XEvent) -> XClientMessageEvent {
    unsafe { transmute_union(&e) }
  }
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XColormapEvent {
  pub kind: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub window: Window,
  pub colormap: Colormap,
  pub new: Bool,
  pub state: c_int,
}

impl From<XEvent> for XColormapEvent {
  fn from (e: XEvent) -> XColormapEvent {
    unsafe { transmute_union(&e) }
  }
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XConfigureEvent {
  pub kind: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub event: Window,
  pub window: Window,
  pub x: c_int,
  pub y: c_int,
  pub width: c_int,
  pub height: c_int,
  pub border_width: c_int,
  pub above: Window,
  pub override_redirect: Bool,
}

impl From<XEvent> for XConfigureEvent {
  fn from (e: XEvent) -> XConfigureEvent {
    unsafe { transmute_union(&e) }
  }
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XConfigureRequestEvent {
  pub kind: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub parent: Window,
  pub window: Window,
  pub x: c_int,
  pub y: c_int,
  pub width: c_int,
  pub height: c_int,
  pub border_width: c_int,
  pub above: Window,
  pub detail: c_int,
  pub value_mask: c_ulong,
}

impl From<XEvent> for XConfigureRequestEvent {
  fn from (e: XEvent) -> XConfigureRequestEvent {
    unsafe { transmute_union(&e) }
  }
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XCreateWindowEvent {
  pub kind: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub parent: Window,
  pub window: Window,
  pub x: c_int,
  pub y: c_int,
  pub width: c_int,
  pub height: c_int,
  pub border_width: c_int,
  pub override_redirect: Bool,
}

impl From<XEvent> for XCreateWindowEvent {
  fn from (e: XEvent) -> XCreateWindowEvent {
    unsafe { transmute_union(&e) }
  }
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XCrossingEvent {
  pub kind: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub window: Window,
  pub root: Window,
  pub subwindow: Window,
  pub time: Time,
  pub x: c_int,
  pub y: c_int,
  pub x_root: c_int,
  pub y_root: c_int,
  pub mode: c_int,
  pub detail: c_int,
  pub same_screen: Bool,
  pub focus: Bool,
  pub state: c_uint,
}
pub type XEnterWindowEvent = XCrossingEvent;
pub type XLeaveWindowEvent = XCrossingEvent;

impl From<XEvent> for XCrossingEvent {
  fn from (e: XEvent) -> XCrossingEvent {
    unsafe { transmute_union(&e) }
  }
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XDestroyWindowEvent {
  pub kind: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub event: Window,
  pub window: Window,
}

impl From<XEvent> for XDestroyWindowEvent {
  fn from (e: XEvent) -> XDestroyWindowEvent {
    unsafe { transmute_union(&e) }
  }
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XErrorEvent {
  pub kind: c_int,
  pub display: *mut Display,
  pub serial: c_ulong,
  pub error_code: c_uchar,
  pub request_code: c_uchar,
  pub minor_code: c_uchar,
  pub resourceid: XID,
}

impl From<XEvent> for XErrorEvent {
  fn from (e: XEvent) -> XErrorEvent {
    unsafe { transmute_union(&e) }
  }
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XExposeEvent {
  pub kind: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub window: Window,
  pub x: c_int,
  pub y: c_int,
  pub width: c_int,
  pub height: c_int,
  pub count: c_int,
}

impl From<XEvent> for XExposeEvent {
  fn from (e: XEvent) -> XExposeEvent {
    unsafe { transmute_union(&e) }
  }
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XFocusChangeEvent {
  pub kind: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub window: Window,
  pub mode: c_int,
  pub detail: c_int,
}
pub type XFocusInEvent = XFocusChangeEvent;
pub type XFocusOutEvent = XFocusChangeEvent;

impl From<XEvent> for XFocusChangeEvent {
  fn from (e: XEvent) -> XFocusChangeEvent {
    unsafe { transmute_union(&e) }
  }
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XGraphicsExposeEvent {
  pub kind: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub drawable: Drawable,
  pub x: c_int,
  pub y: c_int,
  pub width: c_int,
  pub height: c_int,
  pub count: c_int,
  pub major_code: c_int,
  pub minor_code: c_int,
}

impl From<XEvent> for XGraphicsExposeEvent {
  fn from (e: XEvent) -> XGraphicsExposeEvent {
    unsafe { transmute_union(&e) }
  }
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XGravityEvent {
  pub kind: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub event: Window,
  pub window: Window,
  pub x: c_int,
  pub y: c_int,
}

impl From<XEvent> for XGravityEvent {
  fn from (e: XEvent) -> XGravityEvent {
    unsafe { transmute_union(&e) }
  }
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XKeyEvent {
  pub kind: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub window: Window,
  pub root: Window,
  pub subwindow: Window,
  pub time: Time,
  pub x: c_int,
  pub y: c_int,
  pub x_root: c_int,
  pub y_root: c_int,
  pub state: c_uint,
  pub keycode: c_uint,
  pub same_screen: Bool,
}
pub type XKeyPressedEvent = XKeyEvent;
pub type XKeyReleasedEvent = XKeyEvent;

impl From<XEvent> for XKeyEvent {
  fn from (e: XEvent) -> XKeyEvent {
    unsafe { transmute_union(&e) }
  }
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XKeymapEvent {
  pub kind: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub window: Window,
  pub key_vector: [c_char; 32],
}

impl From<XEvent> for XKeymapEvent {
  fn from (e: XEvent) -> XKeymapEvent {
    unsafe { transmute_union(&e) }
  }
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XMapEvent {
  pub kind: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub event: Window,
  pub window: Window,
  pub override_redirect: Bool,
}

impl From<XEvent> for XMapEvent {
  fn from (e: XEvent) -> XMapEvent {
    unsafe { transmute_union(&e) }
  }
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XMappingEvent {
  pub kind: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub event: Window,
  pub request: c_int,
  pub first_keycode: c_int,
  pub count: c_int,
}

impl From<XEvent> for XMappingEvent {
  fn from (e: XEvent) -> XMappingEvent {
    unsafe { transmute_union(&e) }
  }
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XMapRequestEvent {
  pub kind: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub parent: Window,
  pub window: Window,
}

impl From<XEvent> for XMapRequestEvent {
  fn from (e: XEvent) -> XMapRequestEvent {
    unsafe { transmute_union(&e) }
  }
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XMotionEvent {
  pub kind: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub window: Window,
  pub root: Window,
  pub subwindow: Window,
  pub time: Time,
  pub x: c_int,
  pub y: c_int,
  pub x_root: c_int,
  pub y_root: c_int,
  pub state: c_uint,
  pub is_hint: c_char,
  pub same_screen: Bool,
}
pub type XPointerMovedEvent = XMotionEvent;

impl From<XEvent> for XMotionEvent {
  fn from (e: XEvent) -> XMotionEvent {
    unsafe { transmute_union(&e) }
  }
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XNoExposeEvent {
  pub kind: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub drawable: Drawable,
  pub major_code: c_int,
  pub minor_code: c_int,
}

impl From<XEvent> for XNoExposeEvent {
  fn from (e: XEvent) -> XNoExposeEvent {
    unsafe { transmute_union(&e) }
  }
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XPropertyEvent {
  pub kind: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub window: Window,
  pub atom: Atom,
  pub time: Time,
  pub state: c_int,
}

impl From<XEvent> for XPropertyEvent {
  fn from (e: XEvent) -> XPropertyEvent {
    unsafe { transmute_union(&e) }
  }
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XReparentEvent {
  pub kind: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub event: Window,
  pub window: Window,
  pub parent: Window,
  pub x: c_int,
  pub y: c_int,
  pub override_redirect: Bool,
}

impl From<XEvent> for XReparentEvent {
  fn from (e: XEvent) -> XReparentEvent {
    unsafe { transmute_union(&e) }
  }
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XResizeRequestEvent {
  pub kind: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub window: Window,
  pub width: c_int,
  pub height: c_int,
}

impl From<XEvent> for XResizeRequestEvent {
  fn from (e: XEvent) -> XResizeRequestEvent {
    unsafe { transmute_union(&e) }
  }
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XSelectionClearEvent {
  pub kind: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub window: Window,
  pub selection: Atom,
  pub time: Time,
}

impl From<XEvent> for XSelectionClearEvent {
  fn from (e: XEvent) -> XSelectionClearEvent {
    unsafe { transmute_union(&e) }
  }
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XSelectionEvent {
  pub kind: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub requestor: Window,
  pub selection: Atom,
  pub target: Atom,
  pub property: Atom,
  pub time: Time,
}

impl From<XEvent> for XSelectionEvent {
  fn from (e: XEvent) -> XSelectionEvent {
    unsafe { transmute_union(&e) }
  }
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XSelectionRequestEvent {
  pub kind: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub owner: Window,
  pub requestor: Window,
  pub selection: Atom,
  pub target: Atom,
  pub property: Atom,
  pub time: Time,
}

impl From<XEvent> for XSelectionRequestEvent {
  fn from (e: XEvent) -> XSelectionRequestEvent {
    unsafe { transmute_union(&e) }
  }
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XUnmapEvent {
  pub kind: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub event: Window,
  pub window: Window,
  pub from_configure: Bool,
}

impl From<XEvent> for XUnmapEvent {
  fn from (e: XEvent) -> XUnmapEvent {
    unsafe { transmute_union(&e) }
  }
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XVisibilityEvent {
  pub kind: c_int,
  pub serial: c_ulong,
  pub send_event: Bool,
  pub display: *mut Display,
  pub window: Window,
  pub state: c_int,
}

impl From<XEvent> for XVisibilityEvent {
  fn from (e: XEvent) -> XVisibilityEvent {
    unsafe { transmute_union(&e) }
  }
}


//
// other structures
//


#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XClassHint {
  pub res_name: *mut c_char,
  pub res_class: *mut c_char,
}

#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XColor {
  pub pixel: c_ulong,
  pub red: c_ushort,
  pub green: c_ushort,
  pub blue: c_ushort,
  pub flags: c_char,
  pub pad: c_char,
}

#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XExtCodes {
  pub extension: c_int,
  pub major_opcode: c_int,
  pub first_event: c_int,
  pub first_error: c_int,
}

#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XGCValues {
  pub function: c_int,
  pub plane_mask: c_ulong,
  pub foreground: c_ulong,
  pub background: c_ulong,
  pub line_width: c_int,
  pub line_style: c_int,
  pub cap_style: c_int,
  pub join_style: c_int,
  pub fill_style: c_int,
  pub fill_rule: c_int,
  pub arc_mode: c_int,
  pub tile: Pixmap,
  pub stipple: Pixmap,
  pub ts_x_origin: c_int,
  pub ts_y_origin: c_int,
  pub font: Font,
  pub subwindow_mode: c_int,
  pub graphics_exposures: Bool,
  pub clip_x_origin: c_int,
  pub clip_y_origin: c_int,
  pub clip_mask: Pixmap,
  pub dash_offset: c_int,
  pub dashes: c_char,
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XHostAddress {
  pub family: c_int,
  pub length: c_int,
  pub address: *const c_char,
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XImage {
  pub width: c_int,
  pub height: c_int,
  pub xoffset: c_int,
  pub format: c_int,
  pub data: *mut c_char,
  pub byte_order: c_int,
  pub bitmap_unity: c_int,
  pub bitmap_bit_order: c_int,
  pub bitmap_pad: c_int,
  pub depth: c_int,
  pub bytes_per_line: c_int,
  pub bits_per_pixel: c_int,
  pub red_mask: c_ulong,
  pub green_mask: c_ulong,
  pub blue_mask: c_ulong,
  pub obdata: XPointer,
  pub funcs: ImageFns,
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XModifierKeymap {
  pub max_keypermod: c_int,
  pub modifiermap: *mut KeyCode,
}

#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XRectangle {
  pub x: c_short,
  pub y: c_short,
  pub width: c_ushort,
  pub height: c_ushort,
}

#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XSetWindowAttributes {
  pub background_pixmap: Pixmap,
  pub background_pixel: c_ulong,
  pub border_pixmap: Pixmap,
  pub border_pixel: c_ulong,
  pub bit_gravity: c_int,
  pub win_gravity: c_int,
  pub backing_store: c_int,
  pub backing_planes: c_ulong,
  pub backing_pixel: c_ulong,
  pub save_under: Bool,
  pub event_mask: c_long,
  pub do_not_propagate_mask: c_long,
  pub override_redirect: Bool,
  pub colormap: Colormap,
  pub cursor: Cursor,
}

#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XSizeHints {
  pub flags: c_long,
  pub x: c_int,
  pub y: c_int,
  pub width: c_int,
  pub height: c_int,
  pub min_width: c_int,
  pub min_height: c_int,
  pub max_width: c_int,
  pub max_height: c_int,
  pub width_inc: c_int,
  pub height_inc: c_int,
  pub min_aspect: AspectRatio,
  pub max_aspect: AspectRatio,
  pub base_width: c_int,
  pub base_height: c_int,
  pub win_gravity: c_int,
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XTextProperty {
  pub value: *mut c_uchar,
  pub encoding: Atom,
  pub format: c_int,
  pub nitems: c_ulong,
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XVisualInfo {
  pub visual: *mut Visual,
  pub visualid: VisualID,
  pub screen: c_int,
  pub depth: c_uint,
  pub class: c_int,
  pub red_mask: c_ulong,
  pub green_mask: c_ulong,
  pub blue_mask: c_ulong,
  pub colormap_size: c_int,
  pub bits_per_rgb: c_int,
}

#[allow(raw_pointer_derive)]
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct XWindowAttributes {
  pub x: c_int,
  pub y: c_int,
  pub width: c_int,
  pub height: c_int,
  pub border_width: c_int,
  pub depth: c_int,
  pub visual: *mut Visual,
  pub root: Window,
  pub class: c_int,
  pub bit_gravity: c_int,
  pub win_gravity: c_int,
  pub backing_store: c_int,
  pub backing_planes: c_ulong,
  pub backing_pixel: c_ulong,
  pub save_under: Bool,
  pub colormap: Colormap,
  pub map_installed: Bool,
  pub map_state: c_int,
  pub all_event_masks: c_long,
  pub your_event_mask: c_long,
  pub do_not_propagate_mask: c_long,
  pub override_redirect: Bool,
  pub screen: *mut Screen,
}


//
// anonymous structures
//


#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct AspectRatio {
  pub x: c_int,
  pub y: c_int,
}

#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct ClientMessageData {
  longs: [c_long; 5],
}

impl ClientMessageData {
  pub fn get_byte (&self, index: usize) -> c_char {
    unsafe {
      from_raw_parts(&self.longs[0] as *const c_long as *const c_char, 20)[index]
    }
  }

  pub fn get_long (&self, index: usize) -> c_long {
    self.longs[index]
  }

  pub fn get_short (&self, index: usize) -> c_short {
    unsafe {
      from_raw_parts(&self.longs[0] as *const c_long as *const c_short, 10)[index]
    }
  }

  pub fn set_byte (&mut self, index: usize, value: c_char) {
    unsafe {
      from_raw_parts_mut(&mut self.longs[0] as *mut c_long as *mut c_char, 20)[index] = value;
    }
  }

  pub fn set_long (&mut self, index: usize, value: c_long) {
    self.longs[index] = value;
  }

  pub fn set_short (&mut self, index: usize, value: c_short) {
    unsafe {
      from_raw_parts_mut(&mut self.longs[0] as *mut c_long as *mut c_short, 10)[index] = value;
    }
  }
}

#[test]
fn client_message_size_test () {
  assert!(size_of::<ClientMessageData>() >= size_of::<[c_char; 20]>());
  assert!(size_of::<ClientMessageData>() >= size_of::<[c_short; 10]>());
}

#[derive(Copy)]
#[repr(C)]
pub struct ImageFns {
  pub create_image: CreateImageFn,
  pub destroy_image: DestroyImageFn,
  pub get_pixel: GetPixelFn,
  pub put_pixel: PutPixelFn,
  pub sub_image: SubImageFn,
  pub add_pixel: AddPixelFn,
}

impl Clone for ImageFns {
  fn clone (&self) -> ImageFns {
    *self
  }
}

impl PartialEq for ImageFns {
  fn eq (&self, rhs: &ImageFns) -> bool {
    unsafe { mem_eq(self, rhs) }
  }
}


//
// constants
//


// allocate colormap
pub const AllocNone: c_int = 0;
pub const AllocAll: c_int = 1;

// atoms
pub const XA_PRIMARY: Atom = 1;
pub const XA_SECONDARY: Atom = 2;
pub const XA_ARC: Atom = 3;
pub const XA_ATOM: Atom = 4;
pub const XA_BITMAP: Atom = 5;
pub const XA_CARDINAL: Atom = 6;
pub const XA_COLORMAP: Atom = 7;
pub const XA_CURSOR: Atom = 8;
pub const XA_CUT_BUFFER0: Atom = 9;
pub const XA_CUT_BUFFER1: Atom = 10;
pub const XA_CUT_BUFFER2: Atom = 11;
pub const XA_CUT_BUFFER3: Atom = 12;
pub const XA_CUT_BUFFER4: Atom = 13;
pub const XA_CUT_BUFFER5: Atom = 14;
pub const XA_CUT_BUFFER6: Atom = 15;
pub const XA_CUT_BUFFER7: Atom = 16;
pub const XA_DRAWABLE: Atom = 17;
pub const XA_FONT: Atom = 18;
pub const XA_INTEGER: Atom = 19;
pub const XA_PIXMAP: Atom = 20;
pub const XA_POINT: Atom = 21;
pub const XA_RECTANGLE: Atom = 22;
pub const XA_RESOURCE_MANAGER: Atom = 23;
pub const XA_RGB_COLOR_MAP: Atom = 24;
pub const XA_RGB_BEST_MAP: Atom = 25;
pub const XA_RGB_BLUE_MAP: Atom = 26;
pub const XA_RGB_DEFAULT_MAP: Atom = 27;
pub const XA_RGB_GRAY_MAP: Atom = 28;
pub const XA_RGB_GREEN_MAP: Atom = 29;
pub const XA_RGB_RED_MAP: Atom = 30;
pub const XA_STRING: Atom = 31;
pub const XA_VISUALID: Atom = 32;
pub const XA_WINDOW: Atom = 33;
pub const XA_WM_COMMAND: Atom = 34;
pub const XA_WM_HINTS: Atom = 35;
pub const XA_WM_CLIENT_MACHINE: Atom = 36;
pub const XA_WM_ICON_NAME: Atom = 37;
pub const XA_WM_ICON_SIZE: Atom = 38;
pub const XA_WM_NAME: Atom = 39;
pub const XA_WM_NORMAL_HINTS: Atom = 40;
pub const XA_WM_SIZE_HINTS: Atom = 41;
pub const XA_WM_ZOOM_HINTS: Atom = 42;
pub const XA_MIN_SPACE: Atom = 43;
pub const XA_NORM_SPACE: Atom = 44;
pub const XA_MAX_SPACE: Atom = 45;
pub const XA_END_SPACE: Atom = 46;
pub const XA_SUPERSCRIPT_X: Atom = 47;
pub const XA_SUPERSCRIPT_Y: Atom = 48;
pub const XA_SUBSCRIPT_X: Atom = 49;
pub const XA_SUBSCRIPT_Y: Atom = 50;
pub const XA_UNDERLINE_POSITION: Atom = 51;
pub const XA_UNDERLINE_THICKNESS: Atom = 52;
pub const XA_STRIKEOUT_ASCENT: Atom = 53;
pub const XA_STRIKEOUT_DESCENT: Atom = 54;
pub const XA_ITALIC_ANGLE: Atom = 55;
pub const XA_X_HEIGHT: Atom = 56;
pub const XA_QUAD_WIDTH: Atom = 57;
pub const XA_WEIGHT: Atom = 58;
pub const XA_POINT_SIZE: Atom = 59;
pub const XA_RESOLUTION: Atom = 60;
pub const XA_COPYRIGHT: Atom = 61;
pub const XA_NOTICE: Atom = 62;
pub const XA_FONT_NAME: Atom = 63;
pub const XA_FAMILY_NAME: Atom = 64;
pub const XA_FULL_NAME: Atom = 65;
pub const XA_CAP_HEIGHT: Atom = 66;
pub const XA_WM_CLASS: Atom = 67;
pub const XA_WM_TRANSIENT_FOR: Atom = 68;

// boolean values
pub const False: Bool = 0;
pub const True: Bool = 1;

// clip rect ordering
pub const Unsorted: c_int = 0;
pub const YSorted: c_int = 1;
pub const YXSorted: c_int = 2;
pub const YXBanded: c_int = 3;

// color component mask
pub const DoRed: c_char = 1;
pub const DoGreen: c_char = 2;
pub const DoBlue: c_char = 4;

// error codes
pub const Success: c_int = 0;
pub const BadRequest: c_int = 1;
pub const BadValue: c_int = 2;
pub const BadWindow: c_int = 3;
pub const BadPixmap: c_int = 4;
pub const BadAtom: c_int = 5;
pub const BadCursor: c_int = 6;
pub const BadFont: c_int = 7;
pub const BadMatch: c_int = 8;
pub const BadDrawable: c_int = 9;
pub const BadAccess: c_int = 10;
pub const BadAlloc: c_int = 11;
pub const BadColor: c_int = 12;
pub const BadGC: c_int = 13;
pub const BadIDChoice: c_int = 14;
pub const BadName: c_int = 15;
pub const BadLength: c_int = 16;
pub const BadImplementation: c_int = 17;
pub const FirstExtensionError: c_int = 128;
pub const LastExtensionError: c_int = 255;

// event kinds
pub const KeyPress: c_int = 2;
pub const KeyRelease: c_int = 3;
pub const ButtonPress: c_int = 4;
pub const ButtonRelease: c_int = 5;
pub const MotionNotify: c_int = 6;
pub const EnterNotify: c_int = 7;
pub const LeaveNotify: c_int = 8;
pub const FocusIn: c_int = 9;
pub const FocusOut: c_int = 10;
pub const KeymapNotify: c_int = 11;
pub const Expose: c_int = 12;
pub const GraphicsExpose: c_int = 13;
pub const NoExpose: c_int = 14;
pub const VisibilityNotify: c_int = 15;
pub const CreateNotify: c_int = 16;
pub const DestroyNotify: c_int = 17;
pub const UnmapNotify: c_int = 18;
pub const MapNotify: c_int = 19;
pub const MapRequest: c_int = 20;
pub const ReparentNotify: c_int = 21;
pub const ConfigureNotify: c_int = 22;
pub const ConfigureRequest: c_int = 23;
pub const GravityNotify: c_int = 24;
pub const ResizeRequest: c_int = 25;
pub const CirculateNotify: c_int = 26;
pub const CirculateRequest: c_int = 27;
pub const PropertyNotify: c_int = 28;
pub const SelectionClear: c_int = 29;
pub const SelectionRequest: c_int = 30;
pub const SelectionNotify: c_int = 31;
pub const ColormapNotify: c_int = 32;
pub const ClientMessage: c_int = 33;
pub const MappingNotify: c_int = 34;

// event mask
pub const NoEventMask: c_long = 0;
pub const KeyPressMask: c_long = 0x0000_0001;
pub const KeyReleaseMask: c_long = 0x0000_0002;
pub const ButtonPressMask: c_long = 0x0000_0004;
pub const ButtonReleaseMask: c_long = 0x0000_0008;
pub const EnterWindowMask: c_long = 0x0000_0010;
pub const LeaveWindowMask: c_long = 0x0000_0020;
pub const PointerMotionMask: c_long = 0x0000_0040;
pub const PointerMotionHintMask: c_long = 0x0000_0080;
pub const Button1MotionMask: c_long = 0x0000_0100;
pub const Button2MotionMask: c_long = 0x0000_0200;
pub const Button3MotionMask: c_long = 0x0000_0400;
pub const Button4MotionMask: c_long = 0x0000_0800;
pub const Button5MotionMask: c_long = 0x0000_1000;
pub const ButtonMotionMask: c_long = 0x0000_2000;
pub const KeymapStateMask: c_long = 0x0000_4000;
pub const ExposureMask: c_long = 0x0000_8000;
pub const VisibilityChangeMask: c_long = 0x0001_0000;
pub const StructureNotifyMask: c_long = 0x0002_0000;
pub const ResizeRedirectMask: c_long = 0x0004_0000;
pub const SubstructureNotifyMask: c_long = 0x0008_0000;
pub const SubstructureRedirectMask: c_long = 0x0010_0000;
pub const FocusChangeMask: c_long = 0x0020_0000;
pub const PropertyChangeMask: c_long = 0x0040_0000;
pub const ColormapChangeMask: c_long = 0x0080_0000;
pub const OwnerGrabButtonMask: c_long = 0x0100_0000;

// grab modes
pub const GrabModeSync: c_int = 0;
pub const GrabModeAsync: c_int = 1;

// grab status
pub const GrabSuccess: c_int = 0;
pub const AlreadyGrabbed: c_int = 1;
pub const GrabInvalidTime: c_int = 2;
pub const GrabNotViewable: c_int = 3;
pub const GrabFrozen: c_int = 4;

// map state
pub const IsUnmapped: c_int = 0;
pub const IsUnviewable: c_int = 1;
pub const IsViewable: c_int = 2;

// mouse buttons
pub const Button1: c_int = 1;
pub const Button2: c_int = 2;
pub const Button3: c_int = 3;
pub const Button4: c_int = 4;
pub const Button5: c_int = 5;

// size hints mask
pub const USPosition: c_long = 0x0001;
pub const USSize: c_long = 0x0002;
pub const PPosition: c_long = 0x0004;
pub const PSize: c_long = 0x0008;
pub const PMinSize: c_long = 0x0010;
pub const PMaxSize: c_long = 0x0020;
pub const PResizeInc: c_long = 0x0040;
pub const PAspect: c_long = 0x0080;
pub const PBaseSize: c_long = 0x0100;
pub const PWinGravity: c_long = 0x0200;
pub const PAllHints: c_long = PPosition | PSize | PMinSize | PMaxSize | PResizeInc | PAspect;

// time constants
pub const CurrentTime: Time = 0;

// visual class
pub const StaticGray: c_int = 0;
pub const GrayScale: c_int = 1;
pub const StaticColor: c_int = 2;
pub const PseudoColor: c_int = 3;
pub const TrueColor: c_int = 4;
pub const DirectColor: c_int = 5;

// visual info mask
pub const VisualNoMask: c_long = 0x0000;
pub const VisualIDMask: c_long = 0x0001;
pub const VisualScreenMask: c_long = 0x0002;
pub const VisualDepthMask: c_long = 0x0004;
pub const VisualClassMask: c_long = 0x0008;
pub const VisualRedMaskMask: c_long = 0x0010;
pub const VisualGreenMaskMask: c_long = 0x0020;
pub const VisualBlueMaskMask: c_long = 0x0040;
pub const VisualColormapSizeMask: c_long = 0x0080;
pub const VisualBitsPerRGBMask: c_long = 0x0100;
pub const VisualAllMask: c_long = 0x01ff;

// window attributes
pub const CWBackPixmap: c_ulong = 0x0001;
pub const CWBackPixel: c_ulong = 0x0002;
pub const CWBorderPixmap: c_ulong = 0x0004;
pub const CWBorderPixel: c_ulong = 0x0008;
pub const CWBitGravity: c_ulong = 0x0010;
pub const CWWinGravity: c_ulong = 0x0020;
pub const CWBackingStore: c_ulong = 0x0040;
pub const CWBackingPlanes: c_ulong = 0x0080;
pub const CWBackingPixel: c_ulong = 0x0100;
pub const CWOverrideRedirect: c_ulong = 0x0200;
pub const CWSaveUnder: c_ulong = 0x0400;
pub const CWEventMask: c_ulong = 0x0800;
pub const CWDontPropagate: c_ulong = 0x1000;
pub const CWColormap: c_ulong = 0x2000;
pub const CWCursor: c_ulong = 0x4000;

// window classes
pub const InputOutput: c_int = 1;
pub const InputOnly: c_int = 2;

// XCreateIC values
pub const XIMPreeditArea: c_int = 0x0001;
pub const XIMPreeditCallbacks: c_int = 0x0002;
pub const XIMPreeditPosition: c_int = 0x0004;
pub const XIMPreeditNothing: c_int = 0x0008;
pub const XIMPreeditNone: c_int = 0x0010;
pub const XIMStatusArea: c_int = 0x0100;
pub const XIMStatusCallbacks: c_int = 0x0200;
pub const XIMStatusNothing: c_int = 0x0400;
pub const XIMStatusNone: c_int = 0x0800;


//
// private functions
//


unsafe fn mem_eq<T: Sized> (a: &T, b: &T) -> bool {
  let a_addr = a as *const T as usize;
  let b_addr = b as *const T as usize;

  for i in (0..size_of::<T>()) {
    if *((a_addr + i) as *const u8) != *((b_addr + i) as *const u8) {
      return false;
    }
  }

  return true;
}

unsafe fn transmute_union<I, O> (input: &I) -> O
  where I : Sized, O : Sized
{
  let mut output: O = zeroed();
  let copy_len = min(size_of::<I>(), size_of::<O>());

  for i in 0..copy_len {
    *((&mut output as *mut O as usize + i) as *mut u8) = *((input as *const I as usize + i) as *const u8);
  }

  return output;
}
