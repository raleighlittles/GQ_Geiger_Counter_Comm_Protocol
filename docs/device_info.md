# Serial port settings

From Picocom:

```
port is        : /dev/ttyUSB0
flowcontrol    : none
baudrate is    : 115200
parity is      : none
databits are   : 8
stopbits are   : 1
escape is      : C-a
local echo is  : no
noinit is      : no
noreset is     : no
hangup is      : no
nolock is      : no
send_cmd is    : sz -vv
receive_cmd is : rz -vv -E
imap is        : spchex,tabhex,crhex,lfhex,8bithex,nrmhex,
omap is        : 
emap is        : crcrlf,delbs,
logfile is     : none
initstring     : none
exit_after is  : not set
exit is        : no
```

To connect to serial port:

```bash
$ sudo picocom --baud 115200 --parity n --flow n
```

Then to write commands, from another terminal, do:

```bash
$ echo -ne '<GETSERIAL>>' > /dev/ttyUSB0
```

On my device, returns: 61814�

or as raw bytes: [36][01][31][38][31][34][b8]
Convert this to ascii to get result

On the actual device screen (About -> Serial Number), I see:

57 61 48 48 36 35
36 01 31 38 31 34

# Gyroscope

Gyro data from standing up: 254, 176, 62, 208, 4, 176, 170, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0

Gyro data from lying down: 0, 64, 255, 144, 191, 48, 170, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0

# Serial number

On my device:

```
57 61 48 48 36 35
36 01 31 38 31 34
```

Without formatting: 576148483635360131383134
Length: 24

# Device info (`GETVER`)

Returns raw bytes: [71, 77, 67, 45, 53, 48, 48, 43, 82, 101, 32, 50, 46, 52, 50]

Length: 15

Parses to: 'GMC-500+Re 2.42'