#

参考文档：

https://os.phil-opp.com/minimal-rust-kernel/

记录

```bash


bootimage工具开始使用cargo xbuild编译内核

cargo bootimage 编译

bootimage工具执行了三个步骤：
    编译我们的内核为一个ELF（Executable and Linkable Format）文件；
    编译引导程序为独立的可执行文件；
    将内核ELF文件按字节拼接（append by bytes）到引导程序的末端。

qemu:

qemu-system-x86_64 -drive format=raw,file=target/x86_64-blog_os/debug/bootimage-blog_os.bin
```
