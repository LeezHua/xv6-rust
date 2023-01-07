[33mcommit 5049e019a80b863469e97a5b22d0eadb7dbd1919[m[33m ([m[1;36mHEAD -> [m[1;32mriscv[m[33m, [m[1;31morigin/riscv[m[33m)[m
Author: lihua <lihua19832@163.com>
Date:   Sat Dec 31 12:55:28 2022 +0800

    make clean

 fs.img               |   Bin [31m2048000[m -> [32m0[m bytes
 kernel/bio.d         |     3 [31m-[m
 kernel/bio.o         |   Bin [31m36848[m -> [32m0[m bytes
 kernel/console.d     |     5 [31m-[m
 kernel/console.o     |   Bin [31m43344[m -> [32m0[m bytes
 kernel/entry.o       |   Bin [31m1248[m -> [32m0[m bytes
 kernel/exec.d        |     3 [31m-[m
 kernel/exec.o        |   Bin [31m59288[m -> [32m0[m bytes
 kernel/file.d        |     3 [31m-[m
 kernel/file.o        |   Bin [31m60600[m -> [32m0[m bytes
 kernel/fs.d          |     3 [31m-[m
 kernel/fs.o          |   Bin [31m172816[m -> [32m0[m bytes
 kernel/kalloc.d      |     2 [31m-[m
 kernel/kalloc.o      |   Bin [31m21528[m -> [32m0[m bytes
 kernel/kernel        |   Bin [31m264064[m -> [32m0[m bytes
 kernel/kernel.asm    | 12991 [31m---------------------------------------------------------------------------------------------------------------------------------------------------------[m
 kernel/kernel.sym    |   267 [31m----[m
 kernel/kernelvec.o   |   Bin [31m1328[m -> [32m0[m bytes
 kernel/log.d         |     3 [31m-[m
 kernel/log.o         |   Bin [31m50056[m -> [32m0[m bytes
 kernel/main.d        |     2 [31m-[m
 kernel/main.o        |   Bin [31m13224[m -> [32m0[m bytes
 kernel/pipe.d        |     3 [31m-[m
 kernel/pipe.o        |   Bin [31m44624[m -> [32m0[m bytes
 kernel/plic.d        |     2 [31m-[m
 kernel/plic.o        |   Bin [31m13552[m -> [32m0[m bytes
 kernel/printf.d      |     5 [31m-[m
 kernel/printf.o      |   Bin [31m41336[m -> [32m0[m bytes
 kernel/proc.d        |     3 [31m-[m
 kernel/proc.o        |   Bin [31m165672[m -> [32m0[m bytes
 kernel/sleeplock.d   |     3 [31m-[m
 kernel/sleeplock.o   |   Bin [31m23016[m -> [32m0[m bytes
 kernel/spinlock.d    |     3 [31m-[m
 kernel/spinlock.o    |   Bin [31m35104[m -> [32m0[m bytes
 kernel/start.d       |     2 [31m-[m
 kernel/start.o       |   Bin [31m23976[m -> [32m0[m bytes
 kernel/string.d      |     1 [31m-[m
 kernel/string.o      |   Bin [31m41040[m -> [32m0[m bytes
 kernel/swtch.o       |   Bin [31m976[m -> [32m0[m bytes
 kernel/syscall.d     |     3 [31m-[m
 kernel/syscall.o     |   Bin [31m40208[m -> [32m0[m bytes
 kernel/sysfile.d     |     4 [31m-[m
 kernel/sysfile.o     |   Bin [31m120656[m -> [32m0[m bytes
 kernel/sysproc.d     |     3 [31m-[m
 kernel/sysproc.o     |   Bin [31m30232[m -> [32m0[m bytes
 kernel/trampoline.o  |   Bin [31m1304[m -> [32m0[m bytes
 kernel/trap.d        |     3 [31m-[m
 kernel/trap.o        |   Bin [31m64600[m -> [32m0[m bytes
 kernel/uart.d        |     3 [31m-[m
 kernel/uart.o        |   Bin [31m32056[m -> [32m0[m bytes
 kernel/virtio_disk.d |     3 [31m-[m
 kernel/virtio_disk.o |   Bin [31m63056[m -> [32m0[m bytes
 kernel/vm.d          |     2 [31m-[m
 kernel/vm.o          |   Bin [31m139632[m -> [32m0[m bytes
 mkfs/mkfs            |   Bin [31m21512[m -> [32m0[m bytes
 user/_cat            |   Bin [31m32848[m -> [32m0[m bytes
 user/_echo           |   Bin [31m31696[m -> [32m0[m bytes
 user/_forktest       |   Bin [31m15832[m -> [32m0[m bytes
 user/_grep           |   Bin [31m36224[m -> [32m0[m bytes
 user/_grind          |   Bin [31m47536[m -> [32m0[m bytes
 user/_init           |   Bin [31m32200[m -> [32m0[m bytes
 user/_kill           |   Bin [31m31664[m -> [32m0[m bytes
 user/_ln             |   Bin [31m31480[m -> [32m0[m bytes
 user/_ls             |   Bin [31m34792[m -> [32m0[m bytes
 user/_mkdir          |   Bin [31m31712[m -> [32m0[m bytes
 user/_rm             |   Bin [31m31704[m -> [32m0[m bytes
 user/_sh             |   Bin [31m54144[m -> [32m0[m bytes
 user/_stressfs       |   Bin [31m32592[m -> [32m0[m bytes
 user/_usertests      |   Bin [31m180488[m -> [32m0[m bytes
 user/_wc             |   Bin [31m33800[m -> [32m0[m bytes
 user/_zombie         |   Bin [31m31064[m -> [32m0[m bytes
 user/cat.asm         |  1429 [31m-----------------[m
 user/cat.d           |     1 [31m-[m
 user/cat.o           |   Bin [31m15864[m -> [32m0[m bytes
 user/cat.sym         |    65 [31m-[m
 user/echo.asm        |  1352 [31m----------------[m
 user/echo.d          |     1 [31m-[m
 user/echo.o          |   Bin [31m9504[m -> [32m0[m bytes
 user/echo.sym        |    63 [31m-[m
 user/forktest.asm    |   810 [31m----------[m
 user/forktest.d      |     1 [31m-[m
 user/forktest.o      |   Bin [31m16720[m -> [32m0[m bytes
 user/grep.asm        |  1608 [31m-------------------[m
 user/grep.d          |     1 [31m-[m
 user/grep.o          |   Bin [31m38680[m -> [32m0[m bytes
 user/grep.sym        |    68 [31m-[m
 user/grind.asm       |  2550 [31m------------------------------[m
 user/grind.d         |     3 [31m-[m
 user/grind.o         |   Bin [31m94512[m -> [32m0[m bytes
 user/grind.sym       |    69 [31m-[m
 user/init.asm        |  1401 [31m-----------------[m
 user/init.d          |     2 [31m-[m
 user/init.o          |   Bin [31m13000[m -> [32m0[m bytes
 user/init.sym        |    64 [31m-[m
 user/initcode        |   Bin [31m52[m -> [32m0[m bytes
 user/initcode.asm    |    41 [31m-[m
 user/initcode.d      |     1 [31m-[m
 user/initcode.o      |   Bin [31m3360[m -> [32m0[m bytes
 user/initcode.out    |   Bin [31m2064[m -> [32m0[m bytes
 user/kill.asm        |  1337 [31m----------------[m
 user/kill.d          |     1 [31m-[m
 user/kill.o          |   Bin [31m9128[m -> [32m0[m bytes
 user/kill.sym        |    63 [31m-[m
 user/ln.asm          |  1333 [31m----------------[m
 user/ln.d            |     1 [31m-[m
 user/ln.o            |   Bin [31m8592[m -> [32m0[m bytes
 user/ln.sym          |    63 [31m-[m
 user/ls.asm          |  1605 [31m-------------------[m
 user/ls.d            |     1 [31m-[m
 user/ls.o            |   Bin [31m27872[m -> [32m0[m bytes
 user/ls.sym          |    66 [31m-[m
 user/mkdir.asm       |  1350 [31m----------------[m
 user/mkdir.d         |     1 [31m-[m
 user/mkdir.o         |   Bin [31m9832[m -> [32m0[m bytes
 user/mkdir.sym       |    63 [31m-[m
 user/printf.d        |     2 [31m-[m
 user/printf.o        |   Bin [31m42928[m -> [32m0[m bytes
 user/rm.asm          |  1350 [31m----------------[m
 user/rm.d            |     1 [31m-[m
 user/rm.o            |   Bin [31m9824[m -> [32m0[m bytes
 user/rm.sym          |    63 [31m-[m
 user/sh.asm          |  2724 [31m--------------------------------[m
 user/sh.d            |     1 [31m-[m
 user/sh.o            |   Bin [31m126752[m -> [32m0[m bytes
 user/sh.sym          |    84 [31m-[m
 user/stressfs.asm    |  1404 [31m-----------------[m
 user/stressfs.d      |     2 [31m-[m
 user/stressfs.o      |   Bin [31m14256[m -> [32m0[m bytes
 user/stressfs.sym    |    63 [31m-[m
 user/ulib.d          |     2 [31m-[m
 user/ulib.o          |   Bin [31m53168[m -> [32m0[m bytes
 user/umalloc.d       |     2 [31m-[m
 user/umalloc.o       |   Bin [31m24360[m -> [32m0[m bytes
 user/usertests.asm   | 10492 [31m---------------------------------------------------------------------------------------------------------------------------[m
 user/usertests.d     |     3 [31m-[m
 user/usertests.o     |   Bin [31m825584[m -> [32m0[m bytes
 user/usertests.sym   |   141 [31m--[m
 user/usys.S          |   107 [31m--[m
 user/usys.d          |     1 [31m-[m
 user/usys.o          |   Bin [31m7928[m -> [32m0[m bytes
 user/wc.asm          |  1495 [31m------------------[m
 user/wc.d            |     1 [31m-[m
 user/wc.o            |   Bin [31m22768[m -> [32m0[m bytes
 user/wc.sym          |    65 [31m-[m
 user/zombie.asm      |  1311 [31m----------------[m
 user/zombie.d        |     1 [31m-[m
 user/zombie.o        |   Bin [31m6016[m -> [32m0[m bytes
 user/zombie.sym      |    63 [31m-[m
 148 files changed, 48117 deletions(-)

[33mcommit 69b243808cc379e41e6a32cc6f45ccf3fe24a1ed[m
Author: lihua <lihua19832@163.com>
Date:   Sat Dec 31 12:54:54 2022 +0800

    clone xv6-riscv

 .gitignore           |     1 [31m-[m
 Cargo.lock           |     7 [31m-[m
 Cargo.toml           |    12 [31m-[m
 LICENSE              |    24 [32m+[m
 Makefile             |   179 [32m++[m[31m-[m
 README               |    49 [32m+[m
 README.md            |     1 [31m-[m
 cbindgen.toml        |     0
 fs.img               |   Bin [31m0[m -> [32m2048000[m bytes
 fun.h                |     7 [31m-[m
 kernel/bio.c         |   153 [32m++[m
 kernel/bio.d         |     3 [32m+[m
 kernel/bio.o         |   Bin [31m0[m -> [32m36848[m bytes
 kernel/buf.h         |    12 [32m+[m
 kernel/console.c     |   192 [32m+++[m
 kernel/console.d     |     5 [32m+[m
 kernel/console.o     |   Bin [31m0[m -> [32m43344[m bytes
 kernel/defs.h        |   189 [32m+++[m
 kernel/elf.h         |    42 [32m+[m
 kernel/entry.S       |    21 [32m+[m
 kernel/entry.o       |   Bin [31m0[m -> [32m1248[m bytes
 kernel/exec.c        |   166 [32m++[m
 kernel/exec.d        |     3 [32m+[m
 kernel/exec.o        |   Bin [31m0[m -> [32m59288[m bytes
 kernel/fcntl.h       |     5 [32m+[m
 kernel/file.c        |   182 [32m+++[m
 kernel/file.d        |     3 [32m+[m
 kernel/file.h        |    40 [32m+[m
 kernel/file.o        |   Bin [31m0[m -> [32m60600[m bytes
 kernel/fs.c          |   697 [32m+++++++++[m
 kernel/fs.d          |     3 [32m+[m
 kernel/fs.h          |    60 [32m+[m
 kernel/fs.o          |   Bin [31m0[m -> [32m172816[m bytes
 kernel/kalloc.c      |    82 [32m+[m
 kernel/kalloc.d      |     2 [32m+[m
 kernel/kalloc.o      |   Bin [31m0[m -> [32m21528[m bytes
 kernel/kernel        |   Bin [31m0[m -> [32m264064[m bytes
 kernel/kernel.asm    | 12991 [32m+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++[m
 kernel/kernel.ld     |    44 [32m+[m
 kernel/kernel.sym    |   267 [32m++++[m
 kernel/kernelvec.S   |   124 [32m++[m
 kernel/kernelvec.o   |   Bin [31m0[m -> [32m1328[m bytes
 kernel/log.c         |   236 [32m+++[m
 kernel/log.d         |     3 [32m+[m
 kernel/log.o         |   Bin [31m0[m -> [32m50056[m bytes
 kernel/main.c        |    45 [32m+[m
 kernel/main.d        |     2 [32m+[m
 kernel/main.o        |   Bin [31m0[m -> [32m13224[m bytes
 kernel/memlayout.h   |    67 [32m+[m
 kernel/param.h       |    13 [32m+[m
 kernel/pipe.c        |   130 [32m++[m
 kernel/pipe.d        |     3 [32m+[m
 kernel/pipe.o        |   Bin [31m0[m -> [32m44624[m bytes
 kernel/plic.c        |    47 [32m+[m
 kernel/plic.d        |     2 [32m+[m
 kernel/plic.o        |   Bin [31m0[m -> [32m13552[m bytes
 kernel/printf.c      |   135 [32m++[m
 kernel/printf.d      |     5 [32m+[m
 kernel/printf.o      |   Bin [31m0[m -> [32m41336[m bytes
 kernel/proc.c        |   683 [32m++++++++[m
 kernel/proc.d        |     3 [32m+[m
 kernel/proc.h        |   107 [32m++[m
 kernel/proc.o        |   Bin [31m0[m -> [32m165672[m bytes
 kernel/ramdisk.c     |    45 [32m+[m
 kernel/riscv.h       |   363 [32m+++++[m
 kernel/sleeplock.c   |    55 [32m+[m
 kernel/sleeplock.d   |     3 [32m+[m
 kernel/sleeplock.h   |    10 [32m+[m
 kernel/sleeplock.o   |   Bin [31m0[m -> [32m23016[m bytes
 kernel/spinlock.c    |   110 [32m++[m
 kernel/spinlock.d    |     3 [32m+[m
 kernel/spinlock.h    |     9 [32m+[m
 kernel/spinlock.o    |   Bin [31m0[m -> [32m35104[m bytes
 kernel/start.c       |    89 [32m++[m
 kernel/start.d       |     2 [32m+[m
 kernel/start.o       |   Bin [31m0[m -> [32m23976[m bytes
 kernel/stat.h        |    11 [32m+[m
 kernel/string.c      |   107 [32m++[m
 kernel/string.d      |     1 [32m+[m
 kernel/string.o      |   Bin [31m0[m -> [32m41040[m bytes
 kernel/swtch.S       |    42 [32m+[m
 kernel/swtch.o       |   Bin [31m0[m -> [32m976[m bytes
 kernel/syscall.c     |   147 [32m++[m
 kernel/syscall.d     |     3 [32m+[m
 kernel/syscall.h     |    22 [32m+[m
 kernel/syscall.o     |   Bin [31m0[m -> [32m40208[m bytes
 kernel/sysfile.c     |   505 [32m++++++[m
 kernel/sysfile.d     |     4 [32m+[m
 kernel/sysfile.o     |   Bin [31m0[m -> [32m120656[m bytes
 kernel/sysproc.c     |    91 [32m++[m
 kernel/sysproc.d     |     3 [32m+[m
 kernel/sysproc.o     |   Bin [31m0[m -> [32m30232[m bytes
 kernel/trampoline.S  |   151 [32m++[m
 kernel/trampoline.o  |   Bin [31m0[m -> [32m1304[m bytes
 kernel/trap.c        |   221 [32m+++[m
 kernel/trap.d        |     3 [32m+[m
 kernel/trap.o        |   Bin [31m0[m -> [32m64600[m bytes
 kernel/types.h       |    10 [32m+[m
 kernel/uart.c        |   190 [32m+++[m
 kernel/uart.d        |     3 [32m+[m
 kernel/uart.o        |   Bin [31m0[m -> [32m32056[m bytes
 kernel/virtio.h      |    96 [32m++[m
 kernel/virtio_disk.c |   327 [32m++++[m
 kernel/virtio_disk.d |     3 [32m+[m
 kernel/virtio_disk.o |   Bin [31m0[m -> [32m63056[m bytes
 kernel/vm.c          |   439 [32m++++++[m
 kernel/vm.d          |     2 [32m+[m
 kernel/vm.o          |   Bin [31m0[m -> [32m139632[m bytes
 main.c               |    18 [31m-[m
 mkfs/mkfs            |   Bin [31m0[m -> [32m21512[m bytes
 mkfs/mkfs.c          |   301 [32m++++[m
 src/lib.rs           |    18 [31m-[m
 user/_cat            |   Bin [31m0[m -> [32m32848[m bytes
 user/_echo           |   Bin [31m0[m -> [32m31696[m bytes
 user/_forktest       |   Bin [31m0[m -> [32m15832[m bytes
 user/_grep           |   Bin [31m0[m -> [32m36224[m bytes
 user/_grind          |   Bin [31m0[m -> [32m47536[m bytes
 user/_init           |   Bin [31m0[m -> [32m32200[m bytes
 user/_kill           |   Bin [31m0[m -> [32m31664[m bytes
 user/_ln             |   Bin [31m0[m -> [32m31480[m bytes
 user/_ls             |   Bin [31m0[m -> [32m34792[m bytes
 user/_mkdir          |   Bin [31m0[m -> [32m31712[m bytes
 user/_rm             |   Bin [31m0[m -> [32m31704[m bytes
 user/_sh             |   Bin [31m0[m -> [32m54144[m bytes
 user/_stressfs       |   Bin [31m0[m -> [32m32592[m bytes
 user/_usertests      |   Bin [31m0[m -> [32m180488[m bytes
 user/_wc             |   Bin [31m0[m -> [32m33800[m bytes
 user/_zombie         |   Bin [31m0[m -> [32m31064[m bytes
 user/cat.asm         |  1429 [32m+++++++++++++++++[m
 user/cat.c           |    43 [32m+[m
 user/cat.d           |     1 [32m+[m
 user/cat.o           |   Bin [31m0[m -> [32m15864[m bytes
 user/cat.sym         |    65 [32m+[m
 user/echo.asm        |  1352 [32m++++++++++++++++[m
 user/echo.c          |    19 [32m+[m
 user/echo.d          |     1 [32m+[m
 user/echo.o          |   Bin [31m0[m -> [32m9504[m bytes
 user/echo.sym        |    63 [32m+[m
 user/forktest.asm    |   810 [32m++++++++++[m
 user/forktest.c      |    56 [32m+[m
 user/forktest.d      |     1 [32m+[m
 user/forktest.o      |   Bin [31m0[m -> [32m16720[m bytes
 user/grep.asm        |  1608 [32m+++++++++++++++++++[m
 user/grep.c          |   106 [32m++[m
 user/grep.d          |     1 [32m+[m
 user/grep.o          |   Bin [31m0[m -> [32m38680[m bytes
 user/grep.sym        |    68 [32m+[m
 user/grind.asm       |  2550 [32m++++++++++++++++++++++++++++++[m
 user/grind.c         |   351 [32m+++++[m
 user/grind.d         |     3 [32m+[m
 user/grind.o         |   Bin [31m0[m -> [32m94512[m bytes
 user/grind.sym       |    69 [32m+[m
 user/init.asm        |  1401 [32m+++++++++++++++++[m
 user/init.c          |    54 [32m+[m
 user/init.d          |     2 [32m+[m
 user/init.o          |   Bin [31m0[m -> [32m13000[m bytes
 user/init.sym        |    64 [32m+[m
 user/initcode        |   Bin [31m0[m -> [32m52[m bytes
 user/initcode.S      |    28 [32m+[m
 user/initcode.asm    |    41 [32m+[m
 user/initcode.d      |     1 [32m+[m
 user/initcode.o      |   Bin [31m0[m -> [32m3360[m bytes
 user/initcode.out    |   Bin [31m0[m -> [32m2064[m bytes
 user/kill.asm        |  1337 [32m++++++++++++++++[m
 user/kill.c          |    17 [32m+[m
 user/kill.d          |     1 [32m+[m
 user/kill.o          |   Bin [31m0[m -> [32m9128[m bytes
 user/kill.sym        |    63 [32m+[m
 user/ln.asm          |  1333 [32m++++++++++++++++[m
 user/ln.c            |    15 [32m+[m
 user/ln.d            |     1 [32m+[m
 user/ln.o            |   Bin [31m0[m -> [32m8592[m bytes
 user/ln.sym          |    63 [32m+[m
 user/ls.asm          |  1605 [32m+++++++++++++++++++[m
 user/ls.c            |    86 [32m++[m
 user/ls.d            |     1 [32m+[m
 user/ls.o            |   Bin [31m0[m -> [32m27872[m bytes
 user/ls.sym          |    66 [32m+[m
 user/mkdir.asm       |  1350 [32m++++++++++++++++[m
 user/mkdir.c         |    23 [32m+[m
 user/mkdir.d         |     1 [32m+[m
 user/mkdir.o         |   Bin [31m0[m -> [32m9832[m bytes
 user/mkdir.sym       |    63 [32m+[m
 user/printf.c        |   113 [32m++[m
 user/printf.d        |     2 [32m+[m
 user/printf.o        |   Bin [31m0[m -> [32m42928[m bytes
 user/rm.asm          |  1350 [32m++++++++++++++++[m
 user/rm.c            |    23 [32m+[m
 user/rm.d            |     1 [32m+[m
 user/rm.o            |   Bin [31m0[m -> [32m9824[m bytes
 user/rm.sym          |    63 [32m+[m
 user/sh.asm          |  2724 [32m++++++++++++++++++++++++++++++++[m
 user/sh.c            |   494 [32m++++++[m
 user/sh.d            |     1 [32m+[m
 user/sh.o            |   Bin [31m0[m -> [32m126752[m bytes
 user/sh.sym          |    84 [32m+[m
 user/stressfs.asm    |  1404 [32m+++++++++++++++++[m
 user/stressfs.c      |    49 [32m+[m
 user/stressfs.d      |     2 [32m+[m
 user/stressfs.o      |   Bin [31m0[m -> [32m14256[m bytes
 user/stressfs.sym    |    63 [32m+[m
 user/ulib.c          |   147 [32m++[m
 user/ulib.d          |     2 [32m+[m
 user/ulib.o          |   Bin [31m0[m -> [32m53168[m bytes
 user/umalloc.c       |    90 [32m++[m
 user/umalloc.d       |     2 [32m+[m
 user/umalloc.o       |   Bin [31m0[m -> [32m24360[m bytes
 user/user.h          |    41 [32m+[m
 user/user.ld         |    36 [32m+[m
 user/usertests.asm   | 10492 [32m+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++[m
 user/usertests.c     |  3101 [32m+++++++++++++++++++++++++++++++++++++[m
 user/usertests.d     |     3 [32m+[m
 user/usertests.o     |   Bin [31m0[m -> [32m825584[m bytes
 user/usertests.sym   |   141 [32m++[m
 user/usys.S          |   107 [32m++[m
 user/usys.d          |     1 [32m+[m
 user/usys.o          |   Bin [31m0[m -> [32m7928[m bytes
 user/usys.pl         |    38 [32m+[m
 user/wc.asm          |  1495 [32m++++++++++++++++++[m
 user/wc.c            |    54 [32m+[m
 user/wc.d            |     1 [32m+[m
 user/wc.o            |   Bin [31m0[m -> [32m22768[m bytes
 user/wc.sym          |    65 [32m+[m
 user/zombie.asm      |  1311 [32m++++++++++++++++[m
 user/zombie.c        |    14 [32m+[m
 user/zombie.d        |     1 [32m+[m
 user/zombie.o        |   Bin [31m0[m -> [32m6016[m bytes
 user/zombie.sym      |    63 [32m+[m
 228 files changed, 60169 insertions(+), 75 deletions(-)

[33mcommit f907b25364d59273e11bcf691a2b424bde716fb9[m[33m ([m[1;31morigin/master[m[33m, [m[1;31morigin/HEAD[m[33m, [m[1;32mmaster[m[33m)[m
Author: lihua <lihua19832@163.com>
Date:   Sat Dec 31 12:10:44 2022 +0800

    clean

 main   | Bin [31m16072[m -> [32m0[m bytes
 main.o | Bin [31m1888[m -> [32m0[m bytes
 2 files changed, 0 insertions(+), 0 deletions(-)

[33mcommit 10842c11ba943ba3f74c31f9537a6d8deef9259a[m
Author: lihua <lihua19832@163.com>
Date:   Fri Dec 30 22:24:59 2022 +0800

    try coding with rust and c together

 .gitignore    |   1 [32m+[m
 Cargo.lock    |   7 [32m+++++++[m
 Cargo.toml    |  12 [32m++++++++++++[m
 Makefile      |  16 [32m++++++++++++++++[m
 cbindgen.toml |   0
 fun.h         |   7 [32m+++++++[m
 main          | Bin [31m0[m -> [32m16072[m bytes
 main.c        |  18 [32m++++++++++++++++++[m
 main.o        | Bin [31m0[m -> [32m1888[m bytes
 src/lib.rs    |  18 [32m++++++++++++++++++[m
 10 files changed, 79 insertions(+)

[33mcommit 0c8eb88c94fec7c9078a7f00d3c808c4b9b98ba1[m
Author: LeezHua <87840988+LeezHua@users.noreply.github.com>
Date:   Fri Dec 30 22:21:31 2022 +0800

    Initial commit

 README.md | 1 [32m+[m
 1 file changed, 1 insertion(+)
