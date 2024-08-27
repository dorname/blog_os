use x86_64::VirtAddr;
use x86_64::structures::tss::{self, TaskStateSegment};
use lazy_static::lazy_static;
use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector};


pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

// 使用lazy_static宏来创建一个静态的TaskStateSegment实例
lazy_static! {
    // 定义一个静态变量TSS，用于存储任务状态段
    static ref TSS: TaskStateSegment = {
        let mut tss = tss::TaskStateSegment::new();
        // 设置双重故障中断栈表项
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            // 定义栈的大小为20KB
            const STACK_SIZE: usize = 4096 * 5;
            // 定义一个静态的、不安全的栈区域
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];
            // 获取栈的起始虚拟地址
            let stack_start = VirtAddr::from_ptr(unsafe { &STACK });
            // 计算栈的顶部地址
            stack_start + STACK_SIZE
        };
        // 返回配置好的任务状态段实例
        tss
    };
}
struct Selectors {
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}
lazy_static!{
    static ref GDT:(GlobalDescriptorTable,Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));
        (gdt,Selectors{
            code_selector,tss_selector
        })
    };
}


pub fn init(){
    use x86_64::instructions::tables::load_tss;
    use x86_64::instructions::segmentation::{CS, Segment};
    
    GDT.0.load();
    unsafe {
        CS::set_reg(GDT.1.code_selector);
        load_tss(GDT.1.tss_selector);
    }
}