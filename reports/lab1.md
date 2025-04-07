# 2025s-rcore Lab1

## 编程作业

系统调用需要实现指定地址的读写和记录功能。地址读写可以直接通过裸指针实现，有一个坑就是目前还没有 MMU，kernel space 和
app 共享地址空间，因此只需要直接读写传进来的地址就 OK 了 ~~（一开始以为传进来的是相对于 app 的地址，自己还加了个
`app_i_base offset` ORZ ）~~ 。调用记录直接在 `TaskManagerInner` 新开了一个数组做记录，index 作为 syscall ID, value
记录次数。这里系统调用数量不多而且分布稀疏，空间利用率不高，但是正经 OS 里 syscall ID 应该是连续的 XD。

## 简答题

1. RustSBI version 0.4.0-alpha.1
    `ch2b_bad_address.rs` 访问了非法地址内核打印 log：`[kernel] PageFault in application, bad addr = 0x0, bad instruction = 0x804003a4, kernel killed it.`, 对应 Trap：
    ```rust
    Trap::Exception(Exception::StoreFault) | Trap::Exception(Exception::StorePageFault) => {
        println!("[kernel] PageFault in application, bad addr = {:#x}, bad instruction = {:#x}, kernel killed it.", stval, cx.sepc);
        exit_current_and_run_next();
    }
    ```
    `ch2b_bad_{instructions,register}.rs` 属于在 U 态使用 S 态特权指令，访问 S 态寄存器，属于非法指令。内核打印 log：`[kernel] IllegalInstruction in application, kernel killed it.`，对应 Trap：
    ```rust
    Trap::Exception(Exception::IllegalInstruction) => {
        println!("[kernel] IllegalInstruction in application, kernobel killed it.");
        exit_current_and_run_next();
    }
    ```

2.
    1. 内核栈；从 Trap 返回时恢复上下文；通过特殊构造的 Trap 上下文启动 app
    2. 从栈帧中先通过 `t0~2` 恢复了 `sstatus`，`sepc`，`sscratch`，我们要先恢复 CSR 再恢复通用寄存器，这样我们使用的三个临时寄存器 才能被正确恢复。
    3. `x2` 是 `sp` 此时 `sp` 指向内核栈，应该保存的是用户栈 `sscratch`；`x4` 是 `tp` 没有被使用到
    4. sp->kernel stack, sscratch->user stack
    5. `sret`，该指令具体完成以下功能：
        - CPU 会将当前的特权级按照 `sstatus` 的 `SPP` 字段设置为 U 或者 S ；
        - CPU 会跳转到 `sepc` 寄存器指向的那条指令，然后继续执行。
    6. sp->kernel stack, sscratch->user stack
    7. 在 ch3 中是通过 timer 中断进入的，另外还有 `ecall` 等方式

## 荣誉准则

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 **以下各位** 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

    *无*

2. 此外，我也参考了 **以下资料** ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

    [Rust 程序设计语言-不安全 Rust](https://kaisery.github.io/trpl-zh-cn/ch20-01-unsafe-rust.html)  
    [RISC-V CPU 设计（2）：RISC-V 特权指令架构](https://tinylab.org/cpu-design-part1-riscv-privilleged-instruction/)  
    [The RISC-V Instruction Set Manual: Volume II: Privileged Architecture](https://riscv.github.io/riscv-isa-manual/snapshot/privileged/#_csr_listing)

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。
我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。
我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。
我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。
我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。
