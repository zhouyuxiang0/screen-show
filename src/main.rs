use windows::{
    core::*,
    Win32::Foundation::*,
    Win32::Graphics::Gdi::ValidateRect,
    Win32::UI::Input::KeyboardAndMouse::RegisterHotKey,
    Win32::{
        System::LibraryLoader::GetModuleHandleW,
        UI::{
            Controls::{LoadIconMetric, LIM_SMALL},
            Input::KeyboardAndMouse::{MOD_ALT, MOD_NOREPEAT},
            Shell::{Shell_NotifyIconW, NIF_GUID, NIF_ICON, NIF_TIP, NIM_ADD, NOTIFYICONDATAW},
            WindowsAndMessaging::*,
        },
    },
};

fn main() -> Result<()> {
    unsafe {
        let instance = GetModuleHandleW(None)?;
        debug_assert!(instance.0 != 0);

        let window_class = w!("window");

        let wc = WNDCLASSW {
            hCursor: LoadCursorW(None, IDC_ARROW)?,
            hInstance: instance,
            lpszClassName: window_class,

            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wndproc),
            ..Default::default()
        };
        let atom = RegisterClassW(&wc);
        debug_assert!(atom != 0);
        let hwnd = CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            window_class,
            w!("test 标题"),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            None,
            None,
            instance,
            None,
        );
        let guid = GUID {
            data1: 0x23977b55,
            data2: 0x10e0,
            data3: 0x4041,
            data4: [0xb8, 0x62, 0xb1, 0x95, 0x41, 0x96, 0x36, 0x69],
        };
        let nid = NOTIFYICONDATAW {
            hWnd: hwnd,
            uFlags: NIF_ICON | NIF_TIP | NIF_GUID,
            guidItem: guid,
            ..Default::default()
        };
        LoadIconMetric(HMODULE::default(), IDI_ASTERISK, LIM_SMALL)?;
        if Shell_NotifyIconW(NIM_ADD, &nid) == BOOL(0) {
            println!("{:?}", GetLastError());
        }
        // ALT + E
        let _ = RegisterHotKey(hwnd, 1, MOD_ALT | MOD_NOREPEAT, 0x45);
        let mut message = MSG::default();

        while GetMessageA(&mut message, None, 0, 0).into() {
            DispatchMessageA(&message);
        }

        Ok(())
    }
}

extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match message {
            WM_PAINT => {
                println!("WM_PAINT");
                ValidateRect(window, None);
                LRESULT(0)
            }
            WM_DESTROY => {
                println!("WM_DESTROY");
                PostQuitMessage(0);
                LRESULT(0)
            }
            WM_HOTKEY => {
                MessageBoxW(window, w!("test"), w!("标题"), MB_OK);
                LRESULT(0)
            }
            WM_CREATE => LRESULT(0),
            _ => DefWindowProcA(window, message, wparam, lparam),
        }
    }
}
