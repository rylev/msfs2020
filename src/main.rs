mod bindings;

use std::ffi::{c_void, CStr, CString};

macro_rules! success {
    ($hr:expr) => {
        let hr = $hr;
        if hr != 0 {
            return Err(hr);
        }
    };
}

fn main() {
    let sc = SimConnect::new(CString::new("Example").unwrap().as_c_str())
        .expect("Could not correct handle to SimConnect");
    sc.associate_brakes().unwrap();

    loop {
        unsafe {
            bindings::SimConnect_CallDispatch(
                sc.handle.as_ptr(),
                Some(callback),
                std::ptr::null_mut(),
            )
        };
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

unsafe extern "C" fn callback(
    data: *mut bindings::SIMCONNECT_RECV,
    cb_data: bindings::DWORD,
    context: *mut c_void,
) {
    match (*data).dwID as i32 {
        bindings::SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_OPEN => {
            println!("Open!");
        }
        bindings::SIMCONNECT_RECV_ID_SIMCONNECT_RECV_ID_EVENT => println!("Event!"),
        id => println!("Unrecognized dwID {}", id),
    }
}

pub struct SimConnect {
    pub handle: std::ptr::NonNull<c_void>,
}

impl SimConnect {
    pub fn new(name: &CStr) -> Result<Self, bindings::HRESULT> {
        let mut handle = std::ptr::null_mut();
        success!(unsafe {
            bindings::SimConnect_Open(
                &mut handle,
                name.as_ptr(),
                std::ptr::null_mut(),
                0,
                std::ptr::null_mut(),
                0,
            )
        });
        Ok(Self {
            handle: std::ptr::NonNull::new(handle)
                .expect("ERROR: SimConnect_Open returned null pointer on success"),
        })
    }

    fn associate_brakes(&self) -> Result<(), i32> {
        let event = Event::Brakes;
        let name = CString::new("brakes").unwrap();
        success!(unsafe {
            bindings::SimConnect_MapClientEventToSimEvent(
                self.handle.as_ptr(),
                event as u32,
                name.as_ptr(),
            )
        });
        let group = Group::Group0;
        success!(unsafe {
            bindings::SimConnect_AddClientEventToNotificationGroup(
                self.handle.as_ptr(),
                group as u32,
                event as u32,
                0,
            )
        });
        success!(unsafe {
            bindings::SimConnect_SetNotificationGroupPriority(self.handle.as_ptr(), group as u32, 1)
        });

        Ok(())
    }
}

#[derive(Copy, Clone)]
#[repr(u32)]
enum Event {
    Brakes,
}

#[derive(Copy, Clone)]
#[repr(u32)]
enum Group {
    Group0,
}

impl Drop for SimConnect {
    fn drop(&mut self) {
        let _ = unsafe { bindings::SimConnect_Close(self.handle.as_ptr()) };
    }
}
