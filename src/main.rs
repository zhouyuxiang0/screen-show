use std::result::Result;
use windows::{
    core::*,
    Win32::Foundation::*,
    Win32::Graphics::Gdi::ValidateRect,
    Win32::UI::WindowsAndMessaging::*,
    Win32::{Graphics::Gdi::HBRUSH, System::LibraryLoader::GetModuleHandleA},
};

fn main() -> Result<(), Error> {
    unsafe {
        let instance = GetModuleHandleA(PCSTR::null())?;
        debug_assert!(!instance.is_invalid());

        let window_class = s!("window");
        let hcursor = HCURSOR::default();
        let wc = WNDCLASSA {
            hCursor: hcursor,
            hInstance: instance,
            lpszClassName: window_class,
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wndproc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hIcon: HICON::default(),
            hbrBackground: HBRUSH::default(),
            lpszMenuName: PCSTR::null(),
        };

        let atom = RegisterClassA(&wc);
        debug_assert!(atom != 0);

        CreateWindowExA(
            WINDOW_EX_STYLE::default(),
            window_class,
            s!("截图"),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            HWND::default(),
            HMENU::default(),
            instance,
            Option::None,
        );

        let mut message = std::mem::zeroed();

        while GetMessageA(&mut message, HWND::default(), 0, 0) != BOOL(0) {
            DispatchMessageA(&message);
        }
    }
    Ok(())
}

extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match message {
            WM_PAINT => {
                println!("WM_PAINT");
                ValidateRect(window, Option::None);
                LRESULT(0)
            }
            WM_DESTROY => {
                println!("WM_DESTROY");
                PostQuitMessage(0);
                LRESULT(0)
            }
            _ => DefWindowProcA(window, message, wparam, lparam),
        }
    }
}
