use std::{
    ffi::{c_char, c_void},
    mem::{self, size_of},
    ptr::null,
};

use winapi::{
    shared::{
        minwindef::LPVOID,
        windef::{LPRECT, RECT},
    },
    um::{
        wingdi::CreateCompatibleBitmap,
        wingdi::{
            BitBlt, CreateCompatibleDC, GetDIBits, SelectObject, BITMAPINFO, BITMAPINFOHEADER,
            DIB_RGB_COLORS, RGBQUAD, SRCCOPY,
        },
        winuser::{GetDesktopWindow, GetWindowDC, GetWindowRect},
    },
};

pub fn screenshot() {
    unsafe {
        let hwnd = GetDesktopWindow();
        let hdc1 = GetWindowDC(hwnd);
        let hdc2 = CreateCompatibleDC(hdc1);
        let rect = &mut RECT {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        } as *mut RECT;
        GetWindowRect(hwnd, rect);
        let width = (*rect).right - (*rect).left;
        let height = (*rect).bottom - (*rect).top;
        let bitmap = CreateCompatibleBitmap(hdc1, width, height);
        SelectObject(hdc2, bitmap as *mut c_void);
        let bitmap_info = &mut BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biSize: mem::size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: width,
                biHeight: height,
                biPlanes: 1,
                biBitCount: 24,
                biCompression: 0,
                biSizeImage: 0,
                biXPelsPerMeter: 0,
                biYPelsPerMeter: 0,
                biClrUsed: 0,
                biClrImportant: 0,
            },
            bmiColors: [RGBQUAD {
                rgbBlue: 0,
                rgbGreen: 0,
                rgbRed: 0,
                rgbReserved: 0,
            }],
        };
        // let buffer = &mut String::new();
        BitBlt(hdc2, 0, 0, width, height, hdc1, 0, 0, SRCCOPY);
        let flag = GetDIBits(
            hdc2,
            bitmap,
            0,
            height.try_into().unwrap(),
            0 as *mut c_void,
            bitmap_info as *mut BITMAPINFO,
            DIB_RGB_COLORS,
        );
        println!("{}", flag);
        println!(
            "width: {:?}; height: {:?} buffer: {:?}",
            width, height, bitmap_info.bmiHeader.biSizeImage
        );
    }
    println!("------------");
}
