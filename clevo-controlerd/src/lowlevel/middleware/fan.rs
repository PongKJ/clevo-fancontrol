use crate::lowlevel::accessor::ec;
use once_cell::sync::OnceCell;

const EC_CPU_FAN_RPM_HI_ADDR: u8 = 0xD0;
const EC_CPU_FAN_RPM_LO_ADDR: u8 = 0xD1;
const EC_GPU_FAN_RPM_HI_ADDR: u8 = 0xD2;
const EC_GPU_FAN_RPM_LO_ADDR: u8 = 0xD3;
const EC_SET_FAN_SPEED_CMD: u8 = 0x99;
const EC_SET_FAN_AUTO_ADDR: u8 = 0xFF;

pub struct Fan {
    ec: ec::EcAccessor,
}

pub enum FanIndex {
    CPU = 1,
    GPU = 2,
}

impl Default for Fan {
    fn default() -> Self {
        Self::new()
    }
}

impl Fan {
    fn new() -> Self {
        let ec = ec::EcAccessor::new();
        Fan { ec }
    }

    pub fn get_instance() -> &'static Self {
        static INSTANCE: OnceCell<Fan> = OnceCell::new();
        INSTANCE.get_or_init(Fan::new)
    }

    pub fn get_fan_rpm(&self, index: FanIndex) -> u16 {
        let hi;
        let lo;
        match index {
            FanIndex::CPU => {
                hi = self.ec.read_byte(EC_CPU_FAN_RPM_HI_ADDR);
                lo = self.ec.read_byte(EC_CPU_FAN_RPM_LO_ADDR);
            }
            FanIndex::GPU => {
                hi = self.ec.read_byte(EC_GPU_FAN_RPM_HI_ADDR);
                lo = self.ec.read_byte(EC_GPU_FAN_RPM_LO_ADDR);
            }
        }
        let rpm = ((hi as u16) << 8) | (lo as u16);
        if rpm == 0 {
            0
        } else {
            (2156220u32 / rpm as u32) as u16
        }
    }

    pub fn set_fan_speed(&self, duty: f32, index: FanIndex) {
        assert!(
            (0.0..=1.0).contains(&duty),
            "Duty cycle must be between 0.0 and 1.0"
        );
        self.ec
            .cmd_write(EC_SET_FAN_SPEED_CMD, index as u8, (duty * 255.0) as u8);
    }

    pub fn set_fan_auto(&self, index: FanIndex) {
        self.ec
            .cmd_write(EC_SET_FAN_SPEED_CMD, EC_SET_FAN_AUTO_ADDR, index as u8);
    }
}
