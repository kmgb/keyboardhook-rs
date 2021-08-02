use winapi::shared::windef::HHOOK;
use winapi::shared::minwindef::{WPARAM, LPARAM, LRESULT, UINT};
use winapi::um::winuser;
use winapi::ctypes::c_int;

use std::convert::TryFrom;

pub fn run_hook() {
    std::thread::spawn(|| {
        let hook = setup_hook();
        message_loop();
        remove_hook(hook);
    });
}

fn setup_hook() -> HHOOK {
    unsafe {
        let hook = winuser::SetWindowsHookExA(winuser::WH_KEYBOARD_LL, Some(callback), std::ptr::null_mut(), 0);

        if hook.is_null() {
            panic!("Windows hook null return");
        }

        println!("Successfully hooked keyboard");

        hook
    }
}

fn remove_hook(hook: HHOOK) {
    unsafe {
        let result = winuser::UnhookWindowsHookEx(hook);

        if result == 0 {
            panic!("Windows unhook non-zero return");
        }

        println!("Successfully unhooked keyboard");
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
                println!("Keydown: {}", (*info).vkCode);
            },

            winuser::WM_KEYUP
            | winuser::WM_SYSKEYUP => {
                let info: winuser::PKBDLLHOOKSTRUCT = std::mem::transmute(l_param);
                println!("Keyup: {}", (*info).vkCode);
            },

            _ => (),
        }
    }

    winuser::CallNextHookEx(std::ptr::null_mut(), code, w_param, l_param)
}