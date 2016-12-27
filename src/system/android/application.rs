use std::process::exit;

use super::super::application::Application as SysApp;
use super::activity::ANativeActivity;
use super::rect::{
    ARect,
};
use super::input::{
    AInputQueue,
};
use super::window::{
    ANativeWindow,
};

pub struct Application {}

impl Application {
    pub fn on_start(&mut self, activity: *mut ANativeActivity) {
        logdbg!(format!("Activity {:?} on_start.", activity));
    }
    pub fn on_resume(&mut self, activity: *mut ANativeActivity) {
        logdbg!(format!("Activity {:?} on_resume.", activity));
    }
    pub fn on_save_instance_state(&mut self, activity: *mut ANativeActivity, size: *mut usize) {
        logdbg!(format!("Activity {:?}   {:?} on_save_instance_state.", activity, size));
    }
    pub fn on_pause(&mut self, activity: *mut ANativeActivity) {
        logdbg!(format!("Activity {:?} on_pause.", activity));
    }
    pub fn on_stop(&mut self, activity: *mut ANativeActivity) {
        logdbg!(format!("Activity {:?} on_stop.", activity));
    }
    pub fn on_destroy(&mut self, activity: *mut ANativeActivity) {
        logdbg!(format!("Activity {:?} on_destroy.", activity));
        exit(0);
    }
    pub fn on_window_focus_changed(&mut self, activity: *mut ANativeActivity, has_focus: i64) {
        logdbg!(format!("Activity {:?}   {:?} on_window_focus_changed.", activity, has_focus));
    }
    pub fn on_native_window_created(&mut self, activity: *mut ANativeActivity, window: *mut ANativeWindow) {
        logdbg!(format!("Activity {:?}   {:?} on_native_window_created.", activity, window));
    }
    pub fn on_native_window_resized(&mut self, activity: *mut ANativeActivity, window: *mut ANativeWindow) {
        logdbg!(format!("Activity {:?}   {:?} on_native_window_resized.", activity, window));
    }
    pub fn on_native_window_redraw_needed(&mut self, activity: *mut ANativeActivity, window: *mut ANativeWindow) {
        logdbg!(format!("Activity {:?}   {:?} on_native_window_redraw_needed.", activity, window));
    }
    pub fn on_native_window_destroyed(&mut self, activity: *mut ANativeActivity, window: *mut ANativeWindow) {
        logdbg!(format!("Activity {:?}   {:?} on_native_window_destroyed.", activity, window));
    }
    pub fn on_input_queue_created(&mut self, activity: *mut ANativeActivity, queue: *mut AInputQueue) {
        logdbg!(format!("Activity {:?}   {:?} on_input_queue_created.", activity, queue));
    }
    pub fn on_input_queue_destroyed(&mut self, activity: *mut ANativeActivity, queue: *mut AInputQueue) {
        logdbg!(format!("Activity {:?}   {:?} on_input_queue_destroyed.", activity, queue));
    }
    pub fn on_content_rect_changed(&mut self, activity: *mut ANativeActivity, rect: *const ARect) {
        logdbg!(format!("Activity {:?}   {:?} on_content_rect_changed.", activity, rect));
    }
    pub fn on_configuration_changed(&mut self, activity: *mut ANativeActivity) {
        logdbg!(format!("Activity {:?} on_configuration_changed.", activity));
    }
    pub fn on_low_memory(&mut self, activity: *mut ANativeActivity) {
        logdbg!(format!("Activity {:?} on_low_memory.", activity));
    }
}

impl SysApp for Application {}
