#[doc = "Register `OVERRIDEEN` reader"]
pub type R = crate::R<OverrideenSpec>;
#[doc = "Override default values for NRF_1Mbit mode.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nrf1mbit {
    #[doc = "0: Override the default values for NRF_1Mbit mode."]
    Override = 0,
    #[doc = "1: Do not override the default values for NRF_1Mbit mode."]
    NotOverride = 1,
}
impl From<Nrf1mbit> for bool {
    #[inline(always)]
    fn from(variant: Nrf1mbit) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NRF_1MBIT` reader - Override default values for NRF_1Mbit mode."]
pub type Nrf1mbitR = crate::BitReader<Nrf1mbit>;
impl Nrf1mbitR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nrf1mbit {
        match self.bits {
            false => Nrf1mbit::Override,
            true => Nrf1mbit::NotOverride,
        }
    }
    #[doc = "Override the default values for NRF_1Mbit mode."]
    #[inline(always)]
    pub fn is_override(&self) -> bool {
        *self == Nrf1mbit::Override
    }
    #[doc = "Do not override the default values for NRF_1Mbit mode."]
    #[inline(always)]
    pub fn is_not_override(&self) -> bool {
        *self == Nrf1mbit::NotOverride
    }
}
#[doc = "Override default values for BLE_1Mbit mode.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ble1mbit {
    #[doc = "0: Override the default values for BLE_1Mbit mode."]
    Override = 0,
    #[doc = "1: Do not override the default values for BLE_1Mbit mode."]
    NotOverride = 1,
}
impl From<Ble1mbit> for bool {
    #[inline(always)]
    fn from(variant: Ble1mbit) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BLE_1MBIT` reader - Override default values for BLE_1Mbit mode."]
pub type Ble1mbitR = crate::BitReader<Ble1mbit>;
impl Ble1mbitR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ble1mbit {
        match self.bits {
            false => Ble1mbit::Override,
            true => Ble1mbit::NotOverride,
        }
    }
    #[doc = "Override the default values for BLE_1Mbit mode."]
    #[inline(always)]
    pub fn is_override(&self) -> bool {
        *self == Ble1mbit::Override
    }
    #[doc = "Do not override the default values for BLE_1Mbit mode."]
    #[inline(always)]
    pub fn is_not_override(&self) -> bool {
        *self == Ble1mbit::NotOverride
    }
}
impl R {
    #[doc = "Bit 0 - Override default values for NRF_1Mbit mode."]
    #[inline(always)]
    pub fn nrf_1mbit(&self) -> Nrf1mbitR {
        Nrf1mbitR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - Override default values for BLE_1Mbit mode."]
    #[inline(always)]
    pub fn ble_1mbit(&self) -> Ble1mbitR {
        Ble1mbitR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Radio calibration override enable.\n\nYou can [`read`](crate::Reg::read) this register and get [`overrideen::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OverrideenSpec;
impl crate::RegisterSpec for OverrideenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`overrideen::R`](R) reader structure"]
impl crate::Readable for OverrideenSpec {}
#[doc = "`reset()` method sets OVERRIDEEN to value 0xffff_ffff"]
impl crate::Resettable for OverrideenSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
