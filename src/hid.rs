// HID Report Descriptor from http://www.usb.org/developers/hidpage/HUTRR48.pdf
pub const HID_U2F_REPORT_DESCRIPTOR: [u8; 34] = [
    // Usage Page: FIDO Alliance Page (0xF1D0) 
    0x06, 0xd0, 0xf1,

    // Usage: U2F Authenticator Device (0x01)
    0x09, 0x01,

    // Collection: Application
    0xa1, 0x01,

    //     Usage: Input Report Data (0x20)
    0x09, 0x20,

    //     Logical Minimum (0)
    0x15, 0x00,

    //     Logical Maximum (255)
    0x26, 0xff, 0x00,

    //     Report Size (8)
    0x75, 0x08,

    //     Report Count (64)
    0x95, 0x40,

    //     Input (Data, Absolute, Variable)
    0x81, 0x02,

    //     Usage: Input Report Data (0x21)
    0x09, 0x21,

    //     Logical Minimum (0)
    0x15, 0x00,

    //     Logical Maximum (255)
    0x26, 0xff, 0x00,

    //     Report Size (8)
    0x75, 0x08,

    //     Report Count (64)
    0x95, 0x40,

    //     Output (Data, Absolute, Variable)
    0x91, 0x02,

    // End Collection
    0xc0,
];
