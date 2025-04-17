# 2025s-rcore Lab

## 编程作业

引入虚拟内存后，需要注意在内核/用户态之间的地址转换

## 简答题

1.
    - `PPN`: 指向下一级页表的物理地址或最终的物理页帧
    - `RSW`: 交由 OS 使用
    - `flags`: 一些标志位
2.
    1. `Instruction page fault` `Instruction access fault` `Load page fault` `Store/AMO page fault` `Store/AMO page fault` `Load access fault` `Store/AMO access fault` 
    2.
        - `scause`：异常原因
        - `stval`：保存导致缺页异常的虚拟地址
        - `sepc`：当前指令的下一条指令地址
        - `stvec`：`trap handler` 入口地址
    3. Lazy策略一定不会比直接加载策略慢，并且可能会提升性能，因为可能会有些页面被加载后并没有进行访问就被释放或替代了，这样可以避免很多无用的加载。
    4. 10GB / 4KB * 8B = 20M
    5. 一开始不分配物理页，访问缺页时会触发缺页异常，在 `trap handler` 中分配内存
    6. 标志位 `V` 置为 `0`
3.
    1. 修改 `satp`
    2. PTE 的 `U` 标志位置 `0`
    3. 在内核和用户态之间转换时不需要更换页表，也就不需要跳板，可以像之前一样直接切换上下文
    4. 内核/用户空间切换，切换任务；切换任务时

## 荣誉准则

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 **以下各位** 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

    *无*

2. 此外，我也参考了 **以下资料** ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

    [RISC-V Sv39 虚拟内存总结](https://zhuanlan.zhihu.com/p/626899526)  
    [rCore-Tutorial-Book-v3 3.6.0-alpha.1 文档](https://rcore-os.cn/rCore-Tutorial-Book-v3/chapter4/index.html)  
    [The RISC-V Instruction Set Manual: Volume II: Privileged Architecture](https://riscv.github.io/riscv-isa-manual/snapshot/privileged)  

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。
我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。
我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。
我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。
我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。

