from fido2.hid import CtapHidDevice


devs = list(CtapHidDevice.list_devices())

dev = devs[0]

print("# import me:")
print("> from send_ctap import dev")
print("# and try:")
print("> dev.call(0x75)")

if __name__ == "__main__":
    print(dev.call(0x75))
    print(dev.call(0x76))
    print(dev.call(0x77))
    print(dev.call(0x78))
    print(dev.call(0x79))



