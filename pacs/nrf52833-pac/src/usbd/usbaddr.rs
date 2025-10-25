#[doc = "Register `USBADDR` reader"]
pub type R = crate::R<UsbaddrSpec>;
#[doc = "Field `ADDR` reader - Device USB address"]
pub type AddrR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:6 - Device USB address"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new((self.bits & 0x7f) as u8)
    }
}
#[doc = "Device USB address\n\nYou can [`read`](crate::Reg::read) this register and get [`usbaddr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsbaddrSpec;
impl crate::RegisterSpec for UsbaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usbaddr::R`](R) reader structure"]
impl crate::Readable for UsbaddrSpec {}
#[doc = "`reset()` method sets USBADDR to value 0"]
impl crate::Resettable for UsbaddrSpec {}
