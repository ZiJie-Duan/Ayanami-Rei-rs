use std::os::raw::{c_long, c_ulong};
use std::fs::File;
use std::io::{self, Read};
use std::mem::size_of;

#[repr(C)]
#[derive(Debug)]
struct TimeVal {
    tv_sec: c_long,    // seconds
    tv_usec: c_long,   // microseconds
}

#[repr(C)]
#[derive(Debug)]
struct InputEvent {
    time: TimeVal,     // timestamp
    type_: u16,        // event type
    code: u16,         // event code
    value: i32,        // event value
}

fn init_mouse(path:&str) -> io::Result<()> {
    // 打开设备文件
    let mut file = File::open(path)?;

    // 计算结构体的大小
    let event_size = size_of::<InputEvent>();

    let mut buffer = [0u8; std::mem::size_of::<InputEvent>()];

    loop {
        // 读取一个事件
        match file.read_exact(&mut buffer) {
            Ok(_) => {
                // 将字节数组转换为结构体
                let event = unsafe { std::ptr::read(buffer.as_ptr() as *const InputEvent) };

                // 打印事件信息
                println!(
                    "时间：{}.{}\t类型：{}\t代码：{}\t值：{}",
                    event.time.tv_sec, event.time.tv_usec, event.type_, event.code, event.value
                );
            }
            Err(e) => {
                eprintln!("读取事件时出错：{}", e);
                break;
            }
        }
    }

    Ok(())
}
