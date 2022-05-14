# HLeOs

## Description.
**HLeOs** is an toy operating system programed by Rust language, on amd64 system. \
made by HLe4s, a system programmer and a student in [sslab](https://sslab.ajou.ac.kr/), Ajou Univ.

## Environment.
**HLeOs** can be run on qemu-system-x86_64, and on amd64 PC.
but, It's code have some randomness.. (see init kmalloc,in src/main.c)
so An amd64 PC may not run **HLeOs**

If you want running **HLeOs on your PC. you can use "Win32 disk imager" program.

## Prerequisite
**HLeOs** is programed by Rust language. so you should prepare rust cargo. (nightly version)

## Files

HLeOs/\
├── Cargo.lock\
├── Cargo.toml\
├── Makefile\
├── gdb_init_real_mode.txt\
├── i386-32bit.xml\
├── paging.png\
├── src\
│   ├── etc.rs\
│   ├── hleos\
│   │   ├── asm.rs\
│   │   ├── interrupt\
│   │   │   └── handler.rs\
│   │   ├── interrupt.rs\
│   │   ├── iostream.rs\
│   │   ├── kmalloc.rs\
│   │   ├── thread\
│   │   │   └── jobs.rs\
│   │   ├── thread.rs\
│   │   ├── timer.rs\
│   │   └── vga.rs\
│   ├── hleos.rs\
│   ├── main.rs\
│   ├── std\
│   │   ├── bit_map.rs\
│   │   ├── io.rs\
│   │   └── queue.rs\
│   └── std.rs\
├── target.xml\
└── x86_64-HLeos.json

- paging.png : paging state when kernel started. (before init_stack(), in main.rs)
![paging](https://github.com/HLe4s/HLeOs/blob/main/HLeOs/paging.png?raw=true)
- src : a directory, have source files of **HLeOs**
    - src/main.rs : **HLeOs**'s kernel main
    - src/etc.rs : **HLeOs**'s etc functions..
    - src/hleos : a module about **HLeOs**'s kernel function.
        - src/hleos/asm.rs : rust inline asm code blocks
        - src/hleos/interrupt.rs : funcs about interrupt, GDT, IDT, TSS .. etc
        - src/hleos/interrupt/handler.rs : interrupt and exception handlers
        - src/hleos/iostream.rs : keyboard service. (very limited, waiting based while loop)
        - src/hleos/kmalloc.rs : kernel memory allocate service. (bit_map **must be** cleared to zero)
        - src/hleos/thread.rs : kernel thread api
        - src/hleos/thread/jobs.rs : for kernel thread jobs
        - src/hleos/timer.rs : incompleted module. about timer
        - src/hleos/vga.rs : **HLeOs**'s vga text print api
    - src/std : a module for a user famililar layer of **HLeOs**'s functions.
        - src/std/bit_map.rs : usable bitmap module.
        - src/std/io.rs : for _println!_, _print!_ macro
        - src/std/queue.rs : usable queue module.

## Usage

please refer to Makefile file.

make - build **HLeOs**
make run - run **HLeOs** on qemu-system-x86_64
make gdb - run **HLeOs** on qemu-system-x86_64 and remote debug. on tcp:1234
make debug - debug **HLeOs** on qemu, (pair to make gdb)
make dump - make disasmbled dump file.

and other PHONY are in Makefile. good luck !

## Reference
64비트 멀티코어 OS 원리와 구조 \
very thanks to writer of this book, for writting good book \
<img src="http://image.yes24.com/goods/65061299/XL" width="50%" height="50%"/>

also refered to [AMD64 Architecture Programmer’s Manual Volume 2: System Programming](https://www.amd.com/system/files/TechDocs/24593.pdf)

Deprecated README.md ( ~ 2022.05.13 )
----------------------------------------------

# HLeOs
HLeOs, a educational operating system programed by rust.
-----------------------------------------------------

show some descriptions about this project. in my blog, .. it 's in korean

[myblog](https://hacking-yi.kro.kr "my blog")

### 01. how to debug boot loader in real mode debugging

[ko] Makefile을 보면, gdb-i386과 debug-loader가 있다. 이 두 규칙이 boot loader를 디버깅하기 위한 것이다.
굳이 qemu-system-i386을 사용하는 규칙을 만들어낸 이유는, gdb로 realmode 디버깅을 하는데 있어서,
x86_64로 하면 에러가 났기 때문이다. 그래서 어쩔 수 없지만, realmode 디버깅은 제한적으로 진행해야 했다.

일단 아래의 방식을 따르자.
1. 터미널을 두 개 연다.
2. 한 쪽에서는 "make ; make gdb-i386"을 입력하자.
3. 다른 한 쪽에서는 "make debug-loader"를 입력하자.
4. 디버깅하자. 심볼이 안뜨는데, file 명령어를 치면 깨지더라;;
 ++ symbol 명령어 쓰면 심볼 생긴다.

[en] you can find two rules in Makefile, 'gdb-i386', 'debug-loader' this two rules are used to debug the boot loader in real mode.
I made 'gdb-i386' because, when I debug boot loader in realmode, It makes an error to use 'qemu-system-x86_64'
so I made restrictly the two rules.

How to debug loader
1. open two terminal
2. in one, type "make ; make gdb-i386"
3. in another, type "make debug-loader"
4. debug ! # file command makes error :(
 ++ you can make symbol by using 'symbol' command
