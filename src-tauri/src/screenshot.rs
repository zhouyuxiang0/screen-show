pub fn screenshot() {
    use std::fs::File;
    use std::io::Write;
    use std::mem;
    use winapi::shared::minwindef::LPVOID;
    use winapi::shared::windef::{HDC, HGDIOBJ, RECT};
    use winapi::um::{
        wingdi::{
            CreateCompatibleBitmap, CreateCompatibleDC, CreateDIBSection, DeleteDC, GetDIBits,
            GetObjectW, SelectObject, StretchBlt, BITMAPFILEHEADER, BITMAPINFO, BITMAPINFOHEADER,
            DIB_RGB_COLORS, RGBQUAD, SRCCOPY,
        },
        winnt::HANDLE,
        winuser::{GetDesktopWindow, GetWindowDC, GetWindowRect},
    };
    let mut data: Vec<u8> = vec![];
    unsafe {
        let hwnd = GetDesktopWindow();
        let dc = GetWindowDC(hwnd);
        let cdc = CreateCompatibleDC(0 as HDC);
        let mut rect = RECT {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        };
        GetWindowRect(hwnd, &mut rect);
        let (w, h) = (rect.right / 2, rect.bottom / 2);
        let bm = CreateCompatibleBitmap(dc, w, h);
        SelectObject(cdc, bm as HGDIOBJ);
        StretchBlt(cdc, 0, 0, w, h, dc, 0, 0, w, h, SRCCOPY);
        let buf = vec![0u8; (w * h * 4) as usize];
        GetObjectW(bm as HANDLE, 84, buf.as_ptr() as LPVOID);
        let mut bi = BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biBitCount: 32,
                biWidth: w,
                biHeight: h,
                biPlanes: 1,
                biCompression: 0,
                biSizeImage: 0,

                biClrImportant: 0,
                biClrUsed: 0,
                biSize: 0,
                biXPelsPerMeter: 0,
                biYPelsPerMeter: 0,
            },
            bmiColors: [RGBQUAD {
                rgbBlue: 0,
                rgbGreen: 0,
                rgbRed: 0,
                rgbReserved: 0,
            }; 1],
        };
        bi.bmiHeader.biSize = mem::size_of_val(&bi.bmiHeader) as u32;
        CreateDIBSection(
            cdc,
            &bi,
            DIB_RGB_COLORS,
            buf.as_ptr() as *mut *mut winapi::ctypes::c_void,
            0 as HANDLE,
            0,
        );

        GetDIBits(
            cdc,
            bm,
            0,
            h as u32,
            buf.as_ptr() as LPVOID,
            &mut bi,
            DIB_RGB_COLORS,
        );

        let bif = BITMAPFILEHEADER {
            bfType: ('B' as u16) | (('M' as u16) << 8),
            bfOffBits: 54,
            bfReserved1: 0,
            bfReserved2: 0,
            bfSize: (w * h * 4 + 54) as u32,
        };

        for v in serialize_row(&bif) {
            data.push(*v);
        }
        let bii = BITMAPINFOHEADER {
            biBitCount: 32,
            biSize: 40,
            biWidth: w,
            biHeight: h,
            biPlanes: 1,
            biCompression: 0,
            biSizeImage: (w * h * 4) as u32,
            biClrImportant: 0,
            biClrUsed: 0,
            biXPelsPerMeter: 0,
            biYPelsPerMeter: 0,
        };

        for v in serialize_row(&bii) {
            data.push(*v);
        }

        for v in buf {
            data.push(v);
        }

        DeleteDC(dc);
        DeleteDC(cdc);
    }
    let mut file = File::create("1.bmp").expect("create failed");
    file.write_all(&data[..]).expect("write failed");
}

pub unsafe fn serialize_row<T: Sized>(src: &T) -> &[u8] {
    ::std::slice::from_raw_parts((src as *const T) as *const u8, ::std::mem::size_of::<T>())
}
