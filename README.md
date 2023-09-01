# Tools for auditing eBPF-based security tools

This is a small pack of tools was made for finding flaws in eBPF-based security tools.

Contains the following:

Self developed (self_dev):
- wr_syscalls - library that provides friendly interface for making syscalls (doesn't include all syscalls)
- syscaller - CLI-tool for making necessary syscalls (doesn't include all syscalls)
- executor - library that provides friendly interface for starting process
- speed_executor - tool for executing may processes on each CPU core
- enc_executor - allows to run process with large arg, many args; start a process with large path/name; pass valid/invalid UTF-8 characters as arg
- visualizer - tool for visualizing speed execution results

Open Source Software (oss):
- net/l2 (OSS) - sending L2 frames
- net/udp (OSS) - sending UDP packages


## Installation

Build image and push it to your registry or save it locally

```bash
bash build.sh
podman push my-harbor.net/ebpf-bypass/ebpf-bypass
```

## Usage

### speed_executor

```bash
/tools/speed [mode] <count>

# where [mode] can be:
# of - "one file" - tries to open and read one file multiple times (cat /etc/passwd <count> times)
# mf - "many files" - tries to open and read many files (cat 1, cat 2, ..., cat <count>)

```
```bash
# opens 1000 files on each of 8 CPUs (1-1000, 1001-2000, ..., 7001-8000)
/tools/speed mf 8000 
```

### enc_executor

```bash
/tools/enc [mode] <args - depend on mode>

# where [mode] can be:
# ma - "many args" - tries to run process with <n> args
# la - "large arg" - tries to run process with <n>-length arg in one of the next modes:
#     <nr> - "no real" - passes one large meaningless arg (0123456789012...)
#     <rd> - "real data" - passes one large meaningless arg and one short usefull arg (such as /etc/passwd in case of reading files)
# lb - "large binary" - tries to create and run binary with <n>-length name
# lbp - "large binary path" - tries to create and run binary with <n>-length path
# u8 - "utf-8" - tries to run process and pass valid utf-8 string as arg
# ab - "abusing bytes" - tries to run process /bin/echo and pass <b1> <b2> ... <bn> as utf-8 hex-encoded symbols  
```

```bash
/tools/ma 1300
# /bin/cat/ 1 2 3 ... 1300

/tools/la 1200 nr
# /bin/cat 123456789012...

/tools/la 1200 rd
# /bin/cat /etc/passwd 123456789012...

/tools/lbp 100
# /b/b/b/.../b/b/echo "here i am"

/tools/enc ab D2 A2 CE B5 CE B9 CE B9 CF 98
# /bin/echo ҢειιϘ
```

### visualizer


1. Start your ebpf-Based security tools  
2. Run ```enc_executor``` in ```mf``` mode
3. Save detected events to file in format
    ```
    /bin/cat 1
    /bin/cat 3
    /bin/cat 2
    /bin/cat 10
    /bin/cat 7
    ...
    ```
4. Specify file(s) name(s) in visualizer's script
5. Run visualizer script
    ```
    python visualizer.py
    ```

### L2

```
/tools/l2 <dst_mac_part_1> <dst_mac_part_2> <dst_mac_part_3> <dst_mac_part_4> <dst_mac_part_5> <dst_mac_part_6>
```

```
/tools/l2 EC 4B F0 18 B3 82
```

### UDP

```
/tools/udp <dst_addr> <dst_port>
```

```
/tools/udp 10.11.19.2 9071
```
