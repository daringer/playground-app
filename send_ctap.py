from fido2.hid import CtapHidDevice

def get_dev():
    devs = list(CtapHidDevice.list_devices())
    return devs[0]

print("# import me:")
print("> from send_ctap import get_dev")
print("# and try:")
print("> get_dev().call(0x75)")

if __name__ == "__main__":
    dev = get_dev()
    print(dev.call(0x75))
    print(dev.call(0x76))
    print(dev.call(0x77))
    print(dev.call(0x78))
    print(dev.call(0x79))



