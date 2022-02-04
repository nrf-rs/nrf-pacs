#[doc = "Register `OVERRIDEEN` reader"]
pub struct R(crate::R<OVERRIDEEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OVERRIDEEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OVERRIDEEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OVERRIDEEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Override default values for NRF_1Mbit mode.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NRF_1MBIT_A {
    #[doc = "0: Override the default values for NRF_1Mbit mode."]
    OVERRIDE = 0,
    #[doc = "1: Do not override the default values for NRF_1Mbit mode."]
    NOTOVERRIDE = 1,
}
impl From<NRF_1MBIT_A> for bool {
    #[inline(always)]
    fn from(variant: NRF_1MBIT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NRF_1MBIT` reader - Override default values for NRF_1Mbit mode."]
pub struct NRF_1MBIT_R(crate::FieldReader<bool, NRF_1MBIT_A>);
impl NRF_1MBIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NRF_1MBIT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NRF_1MBIT_A {
        match self.bits {
            false => NRF_1MBIT_A::OVERRIDE,
            true => NRF_1MBIT_A::NOTOVERRIDE,
        }
    }
    #[doc = "Checks if the value of the field is `OVERRIDE`"]
    #[inline(always)]
    pub fn is_override(&self) -> bool {
        **self == NRF_1MBIT_A::OVERRIDE
    }
    #[doc = "Checks if the value of the field is `NOTOVERRIDE`"]
    #[inline(always)]
    pub fn is_not_override(&self) -> bool {
        **self == NRF_1MBIT_A::NOTOVERRIDE
    }
}
impl core::ops::Deref for NRF_1MBIT_R {
    type Target = crate::FieldReader<bool, NRF_1MBIT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Override default values for BLE_1Mbit mode.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BLE_1MBIT_A {
    #[doc = "0: Override the default values for BLE_1Mbit mode."]
    OVERRIDE = 0,
    #[doc = "1: Do not override the default values for BLE_1Mbit mode."]
    NOTOVERRIDE = 1,
}
impl From<BLE_1MBIT_A> for bool {
    #[inline(always)]
    fn from(variant: BLE_1MBIT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BLE_1MBIT` reader - Override default values for BLE_1Mbit mode."]
pub struct BLE_1MBIT_R(crate::FieldReader<bool, BLE_1MBIT_A>);
impl BLE_1MBIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BLE_1MBIT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLE_1MBIT_A {
        match self.bits {
            false => BLE_1MBIT_A::OVERRIDE,
            true => BLE_1MBIT_A::NOTOVERRIDE,
        }
    }
    #[doc = "Checks if the value of the field is `OVERRIDE`"]
    #[inline(always)]
    pub fn is_override(&self) -> bool {
        **self == BLE_1MBIT_A::OVERRIDE
    }
    #[doc = "Checks if the value of the field is `NOTOVERRIDE`"]
    #[inline(always)]
    pub fn is_not_override(&self) -> bool {
        **self == BLE_1MBIT_A::NOTOVERRIDE
    }
}
impl core::ops::Deref for BLE_1MBIT_R {
    type Target = crate::FieldReader<bool, BLE_1MBIT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Override default values for NRF_1Mbit mode."]
    #[inline(always)]
    pub fn nrf_1mbit(&self) -> NRF_1MBIT_R {
        NRF_1MBIT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 3 - Override default values for BLE_1Mbit mode."]
    #[inline(always)]
    pub fn ble_1mbit(&self) -> BLE_1MBIT_R {
        BLE_1MBIT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
#[doc = "Radio calibration override enable.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [overrideen](index.html) module"]
pub struct OVERRIDEEN_SPEC;
impl crate::RegisterSpec for OVERRIDEEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [overrideen::R](R) reader structure"]
impl crate::Readable for OVERRIDEEN_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OVERRIDEEN to value 0xffff_ffff"]
impl crate::Resettable for OVERRIDEEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
