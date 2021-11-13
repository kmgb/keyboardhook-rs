use winapi::shared::windef::HHOOK;
use winapi::shared::minwindef::{WPARAM, LPARAM, LRESULT, UINT};
use winapi::um::winuser::{self, LLKHF_INJECTED, LLKHF_LOWER_IL_INJECTED};
use winapi::ctypes::c_int;

use std::convert::TryFrom;

pub fn run_hook() {
    std::thread::spawn(|| {
        let _hook = WindowsHook::new(winuser::WH_KEYBOARD_LL, Some(callback));
        message_loop();
    });
}

struct WindowsHook {
    hook: HHOOK,
}

impl WindowsHook {
    fn new(id: c_int, callback: winuser::HOOKPROC) -> WindowsHook {
        let hook;
        unsafe {
            hook = winuser::SetWindowsHookExA(id, callback, std::ptr::null_mut(), 0);
        }
        if hook.is_null() {
            unsafe {
                panic!("WindowsHook initialization failed: GetLastError={}", winapi::um::errhandlingapi::GetLastError());
            }
        }

        WindowsHook { hook }
    }
}

impl Drop for WindowsHook {
    fn drop(&mut self) {
        unsafe {
            winuser::UnhookWindowsHookEx(self.hook);
        }
    }
}

fn message_loop() {
    // This function handles the event loop, which is necessary for the hook to function

    let mut msg = winuser::MSG::default();
    unsafe {
        while 0 == winuser::GetMessageA(&mut msg, std::ptr::null_mut(), 0, 0) {
            winuser::TranslateMessage(&msg);
            winuser::DispatchMessageA(&msg);
        }
    }
}

#[allow(dead_code)]
unsafe extern "system" fn callback(code: c_int, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    if code == winuser::HC_ACTION {
        match UINT::try_from(w_param).unwrap() {
            winuser::WM_KEYDOWN
            | winuser::WM_SYSKEYDOWN => {
                let info: winuser::PKBDLLHOOKSTRUCT = std::mem::transmute(l_param);
                println!("Keydown: {} {} {}", (*info).vkCode, if (*info).flags & LLKHF_INJECTED != 0 { "INJECTED" } else { "." }, if (*info).flags & LLKHF_LOWER_IL_INJECTED != 0 { "LOWER INJECTED" } else { "." });
            },

            winuser::WM_KEYUP
            | winuser::WM_SYSKEYUP => {
                let info: winuser::PKBDLLHOOKSTRUCT = std::mem::transmute(l_param);
                println!("Keyup: {} {} {}", (*info).vkCode, if (*info).flags & LLKHF_INJECTED != 0 { "INJECTED" } else { "." }, if (*info).flags & LLKHF_LOWER_IL_INJECTED != 0 { "LOWER INJECTED" } else { "." });
            },

            _ => (),
        }
    }

    winuser::CallNextHookEx(std::ptr::null_mut(), code, w_param, l_param)
}