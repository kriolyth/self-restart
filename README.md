# self-restart

## Description

This is a small program that can auto-update itself with a new version and restart.

## Build

`cargo build --bins` will create two executables, `self-restart.exe` and `self-restart-updated.exe`

## Run

Start `self-restart.exe`:
```
Hello, world! My PID is 9336
Press (q)uit, (u)pdate, (r)estart, (c)reate another instance here
```

Press `u` to update the executable with the other one, `r` to restart the program.

As a result the other program should start in the same console window:
```
Keypress: u
Prepare for update: "C:\\Projects\\self-restart\\target\\debug\\self-restart.exe" to be replaced with "C:\\Projects\\self-restart\\target\\debug\\self-restart-temp.bin"
Keypress: r
Hello, updated world! My PID is 12440
```

