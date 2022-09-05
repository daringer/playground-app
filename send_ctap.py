from fido2.hid import CtapHidDevice


devs = list(CtapHidDevice.list_devices())

dev = devs[0]


print(dev.call(0x75))


print(dev.call(0x76))



