use windows::Win32::{
    Foundation::{GetLastError, HWND},
    Graphics::Gdi::{
        BI_RGB, BITMAPINFO, BITMAPINFOHEADER, BitBlt, CreateCompatibleBitmap, CreateCompatibleDC,
        DESKTOPHORZRES, DESKTOPVERTRES, DIB_RGB_COLORS, DeleteDC, DeleteObject, GetDC, GetDIBits,
        GetDeviceCaps, HBITMAP, HDC, ReleaseDC, SRCCOPY, SelectObject,
    },
    UI::WindowsAndMessaging::GetDesktopWindow,
};

#[derive(Debug)]
pub enum CaptureError {
    DeviceContextError(&'static str),
    CompatibleDcError(&'static str),
    BitmapError(&'static str),
    ProcessError(&'static str),
}

impl std::fmt::Display for CaptureError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CaptureError::DeviceContextError(msg) => write!(f, "Device context error: {}", msg),
            CaptureError::CompatibleDcError(msg) => write!(f, "Compatible DC error: {}", msg),
            CaptureError::BitmapError(msg) => write!(f, "Bitmap error: {}", msg),
            CaptureError::ProcessError(msg) => write!(f, "Process error: {}", msg),
        }
    }
}

impl std::error::Error for CaptureError {}

pub struct DeviceContext {
    hdc: HDC,
    hwnd: Option<HWND>,
}

impl DeviceContext {
    pub fn new(hwnd: Option<HWND>) -> Result<Self, CaptureError> {
        let hdc = unsafe { GetDC(hwnd) };
        if hdc.is_invalid() {
            return Err(CaptureError::DeviceContextError("GetDC failed"));
        }
        Ok(DeviceContext { hdc, hwnd })
    }

    pub fn hdc(&self) -> HDC {
        self.hdc
    }
}

impl Drop for DeviceContext {
    fn drop(&mut self) {
        if !self.hdc.is_invalid() {
            unsafe { ReleaseDC(self.hwnd, self.hdc) };
        }
    }
}

// RAII 封装 CreateCompatibleDC
pub struct CompatibleDeviceContext {
    hdc: HDC,
}

impl CompatibleDeviceContext {
    pub fn new(hdc: Option<HDC>) -> Result<Self, CaptureError> {
        let mem_dc = unsafe { CreateCompatibleDC(hdc) };
        if mem_dc.is_invalid() {
            return Err(CaptureError::CompatibleDcError("CreateCompatibleDC failed"));
        }
        Ok(CompatibleDeviceContext { hdc: mem_dc })
    }

    pub fn hdc(&self) -> HDC {
        self.hdc
    }
}

impl Drop for CompatibleDeviceContext {
    fn drop(&mut self) {
        if !self.hdc.is_invalid() {
            unsafe { _ = DeleteDC(self.hdc) };
        }
    }
}

// RAII 封装 HBITMAP
pub struct Bitmap {
    hbitmap: HBITMAP,
}

impl Bitmap {
    pub fn new(hdc: HDC, width: i32, height: i32) -> Result<Self, CaptureError> {
        let bitmap = unsafe { CreateCompatibleBitmap(hdc, width, height) };
        if bitmap.is_invalid() {
            return Err(CaptureError::BitmapError("CreateCompatibleBitmap failed"));
        }
        Ok(Bitmap { hbitmap: bitmap })
    }

    pub fn hbitmap(&self) -> HBITMAP {
        self.hbitmap
    }
}

impl Drop for Bitmap {
    fn drop(&mut self) {
        if !self.hbitmap.is_invalid() {
            unsafe { _ = DeleteObject(self.hbitmap.into()) };
        }
    }
}

/// 捕获屏幕并返回像素数据
/// `hwnd` 可以指定窗口句柄，如果为 `None` 则捕获整个桌面
/// 返回 `FrameData` 包含像素数据、宽度和高度
pub fn capture_frame_gdi(hwnd: Option<HWND>) -> Result<FrameData, CaptureError> {
    let hwnd = hwnd.unwrap_or(unsafe { GetDesktopWindow() });
    let screen_dc = DeviceContext::new(Some(hwnd))?;

    let (physical_width, physical_height) = unsafe {
        // let logical_width = GetDeviceCaps(Some(screen_dc.hdc()), HORZRES);
        // let logical_height = GetDeviceCaps(Some(screen_dc.hdc()), VERTRES);
        let physical_width = GetDeviceCaps(Some(screen_dc.hdc()), DESKTOPHORZRES);
        let physical_height = GetDeviceCaps(Some(screen_dc.hdc()), DESKTOPVERTRES);
        (physical_width, physical_height)
    };

    let width = physical_width;
    let height = physical_height;

    if width <= 0 || height <= 0 {
        return Err(CaptureError::ProcessError("Invalid screen dimensions"));
    }

    let mem_dc = CompatibleDeviceContext::new(Some(screen_dc.hdc()))?;
    let bitmap = Bitmap::new(screen_dc.hdc(), width, height)?;
    let old_bitmap = unsafe { SelectObject(mem_dc.hdc(), bitmap.hbitmap().into()) };
    if old_bitmap.is_invalid() {
        let error = unsafe { GetLastError() };
        eprintln!("Maybe SelectObject failed, error: {:?}", error);
        return Err(CaptureError::ProcessError(
            "SelectObject failed, mem_dc,bitmap may be invalid",
        ));
    }
    // 执行位图复制
    unsafe {
        let result = BitBlt(
            mem_dc.hdc(),
            0,
            0,
            width,
            height,
            Some(screen_dc.hdc()),
            0,
            0,
            SRCCOPY,
        );
        if result.is_err() {
            return Err(CaptureError::ProcessError("BitBlt failed"));
        }
    }

    let mut bi: BITMAPINFO = unsafe { std::mem::zeroed() };
    bi.bmiHeader.biSize = std::mem::size_of::<BITMAPINFOHEADER>() as u32;
    bi.bmiHeader.biWidth = width;
    bi.bmiHeader.biHeight = -height;
    bi.bmiHeader.biPlanes = 1;
    bi.bmiHeader.biBitCount = 32;
    bi.bmiHeader.biCompression = BI_RGB.0;

    let buffer_size = (width * height * 4) as usize;
    let mut pixel_data: Vec<u8> = vec![0; buffer_size];

    unsafe {
        let result = GetDIBits(
            mem_dc.hdc(),
            bitmap.hbitmap(),
            0,
            height as u32,
            Some(pixel_data.as_mut_ptr() as *mut _),
            &mut bi,
            DIB_RGB_COLORS,
        );
        if result == 0 {
            return Err(CaptureError::ProcessError("GetDIBits failed"));
        }
    }
    unsafe {
        let result = SelectObject(mem_dc.hdc(), old_bitmap);
        if result.is_invalid() {
            let error = GetLastError();
            eprintln!(
                "SelectObject failed after GetDIBits, but continuing, error: {:?}",
                error
            );
        }
    }

    Ok(FrameData {
        pixel_data,
        width,
        height,
    })
}
