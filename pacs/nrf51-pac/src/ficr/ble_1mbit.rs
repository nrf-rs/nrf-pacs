#[doc = "Register `BLE_1MBIT[%s]` reader"]
pub struct R(crate::R<BLE_1MBIT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLE_1MBIT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLE_1MBIT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLE_1MBIT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Override values for the OVERRIDEn registers in RADIO for BLE_1Mbit mode.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ble_1mbit](index.html) module"]
pub struct BLE_1MBIT_SPEC;
impl crate::RegisterSpec for BLE_1MBIT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ble_1mbit::R](R) reader structure"]
impl crate::Readable for BLE_1MBIT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BLE_1MBIT[%s]
to value 0xffff_ffff"]
impl crate::Resettable for BLE_1MBIT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
