#[cfg(desktop_os)]
#[macro_export]
macro_rules! vulkust_start {
    ($App:ident) => {
        fn main() {
            use std::sync::{Arc, RwLock};
            use $crate::core::application::Application as CoreAppTrait;
            use $crate::render::engine::Engine as RenderEngine;
            use $crate::system::os::application::Application as OsApp;
            let core_app: Arc<RwLock<CoreAppTrait>> = Arc::new(RwLock::new($App::new()));
            let os_app = Arc::new(RwLock::new(OsApp::new(core_app.clone())));
            OsApp::initialize(&os_app);
            let renderer = Arc::new(RwLock::new(RenderEngine::new(core_app.clone(), &os_app)));
            let renderer_w = Arc::downgrade(&renderer);
            vxresult!(renderer.write()).set_myself(renderer_w);
            vxresult!(os_app.write()).set_renderer(renderer.clone());
            {
                let mut core_app = vxresult!(core_app.write());
                core_app.set_os_app(os_app.clone());
                core_app.set_renderer(renderer);
                core_app.initialize();
            }
            vxresult!(os_app.read()).run();
        }
    };
}

// todo core app set os app renderer initialize
#[cfg(target_os = "android")]
#[macro_export]
macro_rules! vulkust_start {
    ($App:ident) => {
        #[allow(dead_code)]
        #[no_mangle]
        pub unsafe extern "C" fn vulkust_on_create(
            activity: *mut $crate::system::android::activity::ANativeActivity,
            saved_state: *mut $crate::libc::c_void,
            saved_state_size: $crate::libc::size_t,
        ) {
            use std::sync::{Arc, RwLock};
            use $crate::core::application::Application as CoreAppTrait;
            let core_app: Arc<RwLock<CoreAppTrait>> = Arc::new(RwLock::new($App::new()));
            $crate::system::android::glue::android_app_create(
                activity,
                saved_state,
                saved_state_size,
                core_app,
            );
        }
    };
}

#[cfg(target_os = "ios")]
#[macro_export]
macro_rules! vulkust_start {
    ($App:ident) => {
        #[allow(dead_code)]
        #[no_mangle]
        pub extern "C" fn vulkust_allocate() -> *mut ::std::os::raw::c_void {
            use std::mem::transmute;
            use std::sync::{Arc, RwLock};
            use $crate::core::application::Application as CoreAppTrait;
            use $crate::system::os::application::Application as OsApp;
            let core_app: Arc<RwLock<CoreAppTrait>> = Arc::new(RwLock::new($App::new()));
            let os_app = Arc::new(RwLock::new(OsApp::new(core_app)));
            let os_app_clone = Arc::downgrade(&os_app);
            vxresult!(os_app.write()).set_itself(os_app_clone);
            unsafe { transmute(Box::into_raw(Box::new(os_app))) }
        }
    };
}

#[cfg(not(target_os = "android"))]
#[macro_export]
macro_rules! vxlogi {
    ($fmt:expr) => {
        println!("{}", format!("Vulkust information message in file: {} line: {} {}", file!(), line!(), $fmt));
    };
    ($fmt:expr, $($arg:tt)*) => {
        println!("{}", format!("Vulkust information message in file: {} line: {} {}", file!(), line!(), format!($fmt, $($arg)*)));
    };
}

#[cfg(not(target_os = "android"))]
#[macro_export]
macro_rules! vxloge {
    ($fmt:expr) => {
        eprintln!("{}", format!("Vulkust error message in file: {} line: {} {}", file!(), line!(), $fmt));
    };
    ($fmt:expr, $($arg:tt)*) => {
        eprintln!("{}", format!("Vulkust error message in file: {} line: {} {}", file!(), line!(), format!($fmt, $($arg)*)));
    };
}

#[cfg(not(target_os = "android"))]
#[macro_export]
macro_rules! vxlogf {
    ($fmt:expr) => (
        panic!("{}", format!("Vulkust fatal message in file: {} line: {} {}", file!(), line!(), $fmt));
    );
    ($fmt:expr, $($arg:tt)*) => {
        panic!("{}", format!("Vulkust fatal message in file: {} line: {} {}", file!(), line!(), format!($fmt, $($arg)*)));
    };
}

#[cfg(target_os = "android")]
#[macro_export]
macro_rules! vxlogi {
    ($fmt:expr) => {
        $crate::system::android::log::print(
            $crate::system::android::log::Priority::Info, &format!(
            "Vulkust Information MSG in file: {} line: {} {}", file!(), line!(), format!($fmt)));
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::system::android::log::print(
            $crate::system::android::log::Priority::Info, &format!(
            "Vulkust Information MSG in file: {} line: {} {}", file!(), line!(),
            format!($fmt, $($arg)*)));
    };
}

#[cfg(target_os = "android")]
#[macro_export]
macro_rules! vxloge {
    ($fmt:expr) => {
        $crate::system::android::log::print(
            $crate::system::android::log::Priority::Error, &format!(
            "Vulkust Error MSG in file: {} line: {} {}", file!(), line!(), format!($fmt)));
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::system::android::log::print(
            $crate::system::android::log::Priority::Error, &format!(
            "Vulkust Error MSG in file: {} line: {} {}", file!(), line!(),
            format!($fmt, $($arg)*)));
    };
}

#[cfg(target_os = "android")]
#[macro_export]
macro_rules! vxlogf {
    ($fmt:expr) => ({
        $crate::system::android::log::print(
            $crate::system::android::log::Priority::Fatal, &format!(
            "Vulkust Fatal MSG in file: {} line: {} {}", file!(), line!(), format!($fmt)));
        panic!("Terminated!");
    });
    ($fmt:expr, $($arg:tt)*) => ({
        $crate::system::android::log::print(
            $crate::system::android::log::Priority::Fatal,
            &format!("Vulkust Fatal MSG in file: {} line: {} {}", file!(), line!(),
            format!($fmt, $($arg)*)));
        panic!("Terminated!");
    });
}

#[macro_export]
macro_rules! vxunwrap {
    ($e:expr) => {
        match $e {
            Some(v) => v,
            None => vxlogf!("Unwrap failed!"),
        }
    };
}

#[macro_export]
macro_rules! vxresult {
    ($e:expr) => {
        match $e {
            Ok(v) => v,
            Err(e) => vxlogf!("Unwrap failed! {:?}", e),
        }
    };
}

#[macro_export]
macro_rules! vxunimplemented {
    () => {
        vxlogf!("Not implemented")
    };
}

#[macro_export]
macro_rules! vxunexpected {
    () => {
        vxlogf!("Unexpected")
    };
}

#[macro_export]
macro_rules! vxtodo {
    () => {
        vxloge!("TODO")
    };
}

macro_rules! vxflagcheck {
    ($f:expr, $b:expr) => {
        $f & $b == $b
    };
}
