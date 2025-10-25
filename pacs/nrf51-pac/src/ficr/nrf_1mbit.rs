#[doc = "Register `NRF_1MBIT[%s]` reader"]
pub type R = crate::R<Nrf1mbitSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Override values for the OVERRIDEn registers in RADIO for NRF_1Mbit mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`nrf_1mbit::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Nrf1mbitSpec;
impl crate::RegisterSpec for Nrf1mbitSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nrf_1mbit::R`](R) reader structure"]
impl crate::Readable for Nrf1mbitSpec {}
#[doc = "`reset()` method sets NRF_1MBIT[%s] to value 0xffff_ffff"]
impl crate::Resettable for Nrf1mbitSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
