import struct
from evdev import InputDevice, ecodes

# 替换为实际的 event 设备路径
device_path = "/dev/input/event1"  # 改为实际的鼠标设备路径
output_path = "/dev/hidg1"  # 输出文件路径

# 初始化报文结构长度和默认值
RELA_MOUSE_REPORT_LENGTH = 5
EMPTY_REPORT = bytearray([0] * RELA_MOUSE_REPORT_LENGTH)

try:
    # 打开鼠标输入设备
    mouse = InputDevice(device_path)
    print(f"Listening to device: {mouse.name}")

    with open(output_path, 'wb') as output:
        for event in mouse.read_loop():
            if event.type == ecodes.EV_REL:  # 只处理鼠标相对移动事件
                report = bytearray(EMPTY_REPORT)  # 初始化空报文

                if event.code == ecodes.REL_X:  # X 轴移动
                    report[1] = event.value & 0xFF  # 确保只取低 8 位
                elif event.code == ecodes.REL_Y:  # Y 轴移动
                    report[2] = event.value & 0xFF  # 确保只取低 8 位

                # 将报文写入 /dev/hidg1
                output.write(report)
                output.flush()

except FileNotFoundError:
    print(f"Device {device_path} or {output_path} not found. Ensure you have the correct paths.")
except PermissionError:
    print("Permission denied. Try running the script as root.")
except Exception as e:
    print(f"An unexpected error occurred: {e}")
