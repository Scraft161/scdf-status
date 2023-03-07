# SCDF-status

A simple minimalistic statusbar for dwm written in pure rust

---

SCDF-status was born out of a need for a good statusbar that is minimal and just works.
SCDF-status works by setting the root window name through `Xlib` which is provided by the system and thus will work with any window manager that reads the statusbar text from the root window name.

The codebase is written with the intent of being "stupid simple" to avoid unintentional fuck-ups.

---

To-do:

- [ ] Integrate proper (non compiled-in) configuration
   - [ ] read from toml
- [ ] Add customization through command line arguments (Clap)
- [ ] Add support for CPU usage (not just system load)
