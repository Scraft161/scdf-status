use std::{ffi::CString, thread, time::Duration};

use chrono::{Local, Timelike};
use x11::xlib::{XCloseDisplay, XDefaultScreen, XOpenDisplay, XRootWindow, XStoreName};

extern crate sys_info;

fn main() {
	loop {
		let display = unsafe { XOpenDisplay(std::ptr::null()) };
		let screen = unsafe { XDefaultScreen(display) };
		let root = unsafe { XRootWindow(display, screen) };

		let date = statusbar_date();
		let time = statusbar_time();
		let cpu_use = statusbar_cpu();
		let (ram_use, swap_use) = statusbar_ram();

		let status = format!(
			"| CPU: {cpu_use} | MEM: {ram_use}% / SWP: {swap_use}% | {date} / {time} |"
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

fn statusbar_cpu() -> f64 {
	let cpu_usage = sys_info::loadavg().unwrap();

	cpu_usage.one
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
