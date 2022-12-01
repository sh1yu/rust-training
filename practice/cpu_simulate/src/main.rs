/// 模拟一个 CPU 的运行，哈弗结构，指令和数据的RAM分开
// https://mp.weixin.qq.com/s/tbVSfcsIyujUVKRQdz3lGg

// 核心部件：
// PC计数器，指示程序和数据存取位置
// RAM 存储 8 bit 信息的存储器，根据 w 信号为 1 写入当前数据，w 为 0 表示读取。类似 RAM，但只能存储 8 bit 信息。
// 寄存器 存储 8 bit 信息的存储器，根据 w 信号为 1 写入当前数据，w 为 0 表示读取。类似 RAM，但只能存储 8 bit 信息。
// 加法器，完成两数加减法运算，sub 为 1 时表示减法，ci 为 1 时表示进位。这个器件是核心器件，用于构成 ALU（算数逻辑单元）
// 21选择器，相当于单刀双掷开关，根据 s21 信号，决定 8 bit 输出来自或左或右 8 bit 输入端

// 整个数据通路从程序计数器 pc 开始，计数器从 0 开始输出数字 0，1，2，3，4……。
// 指令 RAM 和数据 RAM 中分别存储程序代码和数据。
// RAM 采用数字表示的位置访问、存储数据。根据计数器地址 0，1，2之类，将 RAM 中的数据分别放入指令寄存器 IR 和数据寄存器 DR。
// 寄存器相当于容器、变量，存储了 RAM 给它的数据。
// 指令寄存器中的指令码解码产生 CPU 控制指令，这些 0 和 1 分别表示低电平和高电平信号，
// 而电平信号则控制诸如加法器进位与否，是否打开减法，是否使能寄存器写入，选择 21选择器哪一个输入作输出，是否重置计数器，等等。
// 所以，指令其实是控制 CPU 各部件协同工作的电信号。
// 数据寄存器中的数据分别走向加法器 adder 来进行加法、减法运算后流向 21选择器，也可能直接流向 21选择器等待选择。
// 21选择器选择后，数据进入累加寄存器 AC 。累加器的数据根据 ac 信号是否为高电平 1 ，来决定写入与否。
// AC累加器的数据会参与下次计算或者根据 w 信号存入数据 RAM 中。
// 至此，我们完成了一次计算，程序计数器加 1，然后执行下一次计算。
// 如果本条指令是跳转指令的话，将跳转目的地址直接赋值给程序计数器，程序重新地址开始执行。

// 指令
// 0x18 Load   dr=1, ac=1
// 0x19 Add    dr=1, ac=1, s21=1
// 0x1d Sub    dr=1, ac=1, s21=1, sub=1
// 0x30 Jmp    dr=1, pre=1
// 0x31 Jz     dr=1, pre=1, s21=1
// 0x00 Hlt
// 0x02 Store  w=1

fn main() {
    let ramc: [u8; 7] = [0x18, 0x19, 0x1d, 0x02, 0x31, 0x30, 0x00]; //初始化代码
    let mut ramd: [u8; 6] = [10, 2, 3, 0xff, 0x06, 0x02]; // 初始化数据
    cpu(&ramc, &mut ramd);

    let ramc: [u8; 5] = [0x18, 0x1d, 0x31, 0x30, 0x00];
    let mut ramd: [u8; 4] = [5, 1, 0x04, 0x01];
    cpu(&ramc, &mut ramd);
}

fn cpu(ramc: &[u8], ramd: &mut [u8]) {
    //pc 计数器从 0 开始，无限循环。
    let mut pc = 0;
    // 新建寄存器
    let (mut irc, mut drc, mut acc) = (register(), register(), register());
    // 初始化寄存器
    irc(0, true);
    drc(0, true);
    acc(0, true);

    loop {
        irc(ramc[pc], true); // 指令读写
        if irc(0, false) == 0 {
            //HLT信号
            break;
        }

        let (pre, dr, ac, sub, w, s21) = command_decode(irc(0, false));
        drc(ramd[pc], dr); //数据读写
        let r = adder(acc(0, false), drc(0, false), 0, sub); //加法器自动加法
        acc(b8_21selector(drc(0, false), r, s21), ac); // 选择器选择后，累加寄存器读写

        //根据 w 信号，数据写入 RAM
        if w {
            ramd[pc] = acc(0, false)
        }

        // Jz 指令跳转
        let zf = acc(0, false) == 0;
        if pre && zf && s21 {
            pc = drc(0, false) as usize;
        } else {
            pc += 1;
        }

        // 无条件跳转
        if pre && !s21 {
            pc = drc(0, false) as usize;
        }

        print!("{} ", acc(0, false));
    }
    println!(".");
}

fn adder(a: u8, b: u8, ci: u8, sub: bool) -> u8 {
    match sub {
        true => a - b + ci,
        false => a + b + ci,
    }
}

fn register() -> impl FnMut(u8, bool) -> u8 {
    let mut temp = 0;
    // temp被捕获并被移动到了闭包中，以后每次调用这个闭包时temp就相当于一个内部的持久化静态变量
    move |data: u8, w: bool| {
        if w {
            temp = data;
        }
        return temp;
    }
}

fn b8_21selector(a: u8, b: u8, sel: bool) -> u8 {
    match sel {
        true => b,
        false => a,
    }
}

// 指令解码: pre, dr, ac, sub, w, s21
fn command_decode(data: u8) -> (bool, bool, bool, bool, bool, bool) {
    (
        data & 0x20 != 0,
        data & 0x10 != 0,
        data & 0x08 != 0,
        data & 0x04 != 0,
        data & 0x02 != 0,
        data & 0x01 != 0,
    )
}
