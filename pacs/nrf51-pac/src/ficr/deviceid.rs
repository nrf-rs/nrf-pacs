#[doc = "Register `DEVICEID[%s]` reader"]
pub type R = crate::R<DeviceidSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Device identifier.\n\nYou can [`read`](crate::Reg::read) this register and get [`deviceid::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DeviceidSpec;
impl crate::RegisterSpec for DeviceidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`deviceid::R`](R) reader structure"]
impl crate::Readable for DeviceidSpec {}
#[doc = "`reset()` method sets DEVICEID[%s] to value 0xffff_ffff"]
impl crate::Resettable for DeviceidSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
