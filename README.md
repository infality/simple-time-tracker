# Simple Time Tracker
A simple time tracker written in Rust using the [Iced](https://github.com/hecrj/iced) GUI library.

## Features

* Save tracked time along with a description or add it to an existing entry
* Split tracked time onto multiple actions
* Dark / Light mode
* Stores data persistently

## TODO

- [ ] Releases (currently only windows binaries are uploaded in [Github Actions](https://github.com/infality/simple-time-tracker/actions/workflows/rust.yml))
- [ ] Icon for dark mode button (and maybe for the other buttons)
- [ ] Copy more than only the description?
- [ ] Display error messages (Waiting for overlay functionality or toast messages)
- [ ] Handle and display possible SQLite errors
- [ ] Tab movement (Waiting for https://github.com/hecrj/iced/issues/489)
