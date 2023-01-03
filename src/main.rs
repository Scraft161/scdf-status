use std::{ffi::CString, thread, time::Duration};

use chrono::{Local, Timelike};
use x11::xlib::{XCloseDisplay, XDefaultScreen, XOpenDisplay, XRootWindow, XStoreName};

extern crate sys_info;

fn main() {
    // Values that depend on previous ones
    let mut sys_load_prev: f64 = -10.0;
    let mut sys_load_since_update_stale = -10.0;

	loop {
		let display = unsafe { XOpenDisplay(std::ptr::null()) };
		let screen = unsafe { XDefaultScreen(display) };
		let root = unsafe { XRootWindow(display, screen) };

		let date = statusbar_date();
		let time = statusbar_time();
		let sys_load = statusbar_system();
        // Accomodate for load diff if we have new data
        let sys_load_since_update = if sys_load_prev == sys_load {
            sys_load_since_update_stale
        } else {
            let sys_load_diff = sys_load - sys_load_prev;
            sys_load_prev = sys_load;
            sys_load_since_update_stale = sys_load_diff;

            sys_load_diff
        };
		let (ram_use, swap_use) = statusbar_ram();

        // Don't show load diff when we don't know it's value
        let sys_load_format = match sys_load_since_update {
            _ if sys_load_since_update == (sys_load - 10.0) => {format!("{sys_load:.01}")},
            _ => {format!("{sys_load:.01} ({sys_load_since_update:+.01})")},
        };

		let status = format!(
			"| SYS: {sys_load_format} | MEM: {ram_use}% / SWP: {swap_use}% | {date} / {time} |"
		);

		let cstatus = CString::new(status).unwrap();

		unsafe {
			XStoreName(display, root, cstatus.as_ptr());

			XCloseDisplay(display);
		}

		thread::sleep(Duration::from_millis(1000));
	}
}

fn statusbar_date() -> String {
	let today = Local::now();
	let formatted_today = today.format("%Y-%b-%d (%a)").to_string();

	formatted_today
}

fn statusbar_time() -> String {
	let now = Local::now();
	let (is_pm, hour) = now.hour12();

	format!(
		"{}:{:02}:{:02} {}",
		hour,
		now.minute(),
		now.second(),
		if is_pm { "PM" } else { "AM" }
	)
}

fn statusbar_system() -> f64 {
	let system_load = sys_info::loadavg().unwrap();

	system_load.one // minute
}

fn statusbar_ram() -> (i32, i32) {
	let mem_use = sys_info::mem_info().unwrap();

	let ram_use_perc = (((mem_use.total - mem_use.avail) as f64 / mem_use.total as f64) * 100.0).round() as i32;
	let swap_use_perc = (((mem_use.swap_total - mem_use.swap_free) as f64 / mem_use.swap_total as f64) * 100.0).round() as i32;

	(
		ram_use_perc,
		swap_use_perc
	)
}
