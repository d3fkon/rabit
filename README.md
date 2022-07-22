# Rabit - The Habit Tracker

Rabit is a terminal based digital habit tracker written in rust. It helps you track all your habits without lifting a finger.

### Features

1. Vim like motions - `hjkl`
2. Modal - `add`, `edit` and `delete`
3. Themable (Almost)

### Install

Currenly, `rabit` is under development, and the only way to install it is by building the binary

```bash
$ git clone https://github.com/d3fkon/rabit
$ cd rabit

$ cargo build --release

$ cp target/release/rabit /usr/bin  # Copy the binary into one of your $PATH dirs, or run from this dir
```

---

### Usage

 <img width="586" alt="image" src="https://user-images.githubusercontent.com/23007190/179391679-f611d16d-e6b1-4b1d-95cc-7e285ab9dc9e.png">

#### Navigation

1. Use `hjkl` for moving around the grid
2. Press `<SPC>` to mark or unmark a habit for the day

#### Add a habit

1. Enter command mode by pressing `:`
2. Add your habit `add {HABIT_NAME}`

Habit Types:

1. BIT - Your normal boolean type
2. COUNT - Type where you can count the number of times you performed the habit
3. ALPHA - Type where you can enter a single CHAR to track the habit your performed

#### Edit a habit

1. Enter command mode by pressing `:`
2. Edit your habit by ID `edit {HABIT_ID} {NEW_HABIT_NAME}`

#### Delete a habit

1. Enter command mode by pressing `:`
2. Delete your habit by ID `delete {HABIT_ID}`

---

### Things to do

- [ ] Locking habit marking for only for _Today_
- [ ] Allow habits with longer names
- [x] Make the TUI look more cute (?????)
- [ ] Config file to change colors and characters on the UI
- [x] Add different inputs for a task. Beyond just true false

---
