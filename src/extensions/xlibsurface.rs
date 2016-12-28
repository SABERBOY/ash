use prelude::*;
use std::ptr;
use std::mem;
use instance::Instance;
use entry::Entry;
use vk;
use std::ffi::CStr;

pub struct XlibSurface {
    pub handle: vk::Instance,
    pub xlib_surface_fn: vk::XlibSurfaceFn,
}

impl XlibSurface {
    pub fn new(entry: &Entry, instance: &Instance) -> Result<XlibSurface, String> {
        let surface_fn = vk::XlibSurfaceFn::load(|name| {
            unsafe {
                mem::transmute(entry.get_instance_proc_addr(instance.handle(), name.as_ptr()))
            }
        })?;
        Ok(XlibSurface {
            handle: instance.handle(),
            xlib_surface_fn: surface_fn,
        })
    }

    pub fn name() -> &'static CStr{
        CStr::from_bytes_with_nul(b"VK_KHR_xlib_surface\0").expect("Wrong extension string")
    }

    pub fn create_xlib_surface_khr(&self,
                                   create_info: &vk::XlibSurfaceCreateInfoKHR)
                                   -> VkResult<vk::SurfaceKHR> {
        unsafe {
            let mut surface = mem::uninitialized();
            let err_code = self.xlib_surface_fn
                .create_xlib_surface_khr(self.handle, create_info, ptr::null(), &mut surface);
            match err_code {
                vk::Result::Success => Ok(surface),
                _ => Err(err_code),
            }
        }
    }
}
