use std::ptr::addr_of;

use windows::{
    core::*,
    Win32::{Foundation::*, Graphics::Gdi::*, UI::WindowsAndMessaging::*},
};

pub const WIDTH: usize = 1024;
pub const HEIGHT: usize = 768;
pub(crate) static mut SCRBUFF: [u32; WIDTH * HEIGHT * 4] = [0; WIDTH * HEIGHT * 4];

macro_rules! pcwstr {
    ($($arg:tt)*) => {
        PCWSTR::from_raw(widestring::u16str!($($arg)*).as_ptr() as _)
    };
}

unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    if msg == WM_CLOSE {
        DestroyWindow(hwnd).unwrap();
    } else if msg == WM_DESTROY {
        PostQuitMessage(0);
    }

    DefWindowProcW(hwnd, msg, wparam, lparam)
}

pub unsafe fn create_window() {
    SCRBUFF.iter_mut().for_each(|x| *x = 0xFFFFFFFF);

    let mut wc: WNDCLASSW = std::mem::zeroed();
    wc.style = CS_CLASSDC;
    wc.lpfnWndProc = Some(window_proc);
    wc.lpszClassName = pcwstr!("LayGround");
    wc.hCursor = LoadCursorW(None, IDC_ARROW).unwrap();
    RegisterClassW(&wc);

    let hwnd: HWND = CreateWindowExW(
        WINDOW_EX_STYLE::default(),
        wc.lpszClassName,
        pcwstr!("LayGround"),
        WS_OVERLAPPED | WS_SYSMENU | WS_MINIMIZEBOX | WS_VISIBLE,
        CW_USEDEFAULT,
        CW_USEDEFAULT,
        WIDTH.try_into().unwrap(),
        HEIGHT.try_into().unwrap(),
        None,
        None,
        None,
        None,
    )
    .unwrap();

    SetTimer(HWND::default(), 0, 16, None);

    let mut exit = false;
    dbg!("done");
    while !exit {
        let mut msg = MSG::default();
        while PeekMessageW(&mut msg, HWND::default(), 0, 0, PM_REMOVE) == TRUE {
            if msg.message == WM_QUIT {
                exit = true;
            }

            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }

        let mut bmi: BITMAPINFOHEADER = std::mem::zeroed();
        bmi.biSize = std::mem::size_of::<BITMAPINFOHEADER>() as u32;
        bmi.biWidth = WIDTH as i32;
        bmi.biHeight = -(HEIGHT as i32);
        bmi.biPlanes = 1;
        bmi.biBitCount = 32;

        let dc: HDC = GetDC(hwnd);
        SetDIBitsToDevice(
            dc,
            0,
            0,
            WIDTH.try_into().unwrap(),
            HEIGHT.try_into().unwrap(),
            0,
            0,
            0,
            HEIGHT.try_into().unwrap(),
            addr_of!(SCRBUFF) as *const _ as _,
            &bmi as *const _ as _,
            DIB_RGB_COLORS,
        );
        ReleaseDC(hwnd, dc);

        super::real_main();
    }
}
