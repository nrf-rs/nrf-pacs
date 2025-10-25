#[doc = "Register `DEVICEADDR[%s]` reader"]
pub type R = crate::R<DeviceaddrSpec>;
#[doc = "Field `DEVICEADDR` reader - 48 bit device address"]
pub type DeviceaddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 48 bit device address"]
    #[inline(always)]
    pub fn deviceaddr(&self) -> DeviceaddrR {
        DeviceaddrR::new(self.bits)
    }
}
#[doc = "Description collection: Device address n\n\nYou can [`read`](crate::Reg::read) this register and get [`deviceaddr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DeviceaddrSpec;
impl crate::RegisterSpec for DeviceaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`deviceaddr::R`](R) reader structure"]
impl crate::Readable for DeviceaddrSpec {}
#[doc = "`reset()` method sets DEVICEADDR[%s] to value 0xffff_ffff"]
impl crate::Resettable for DeviceaddrSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
