#[doc = "Register `CURRENTLOADCTRL` reader"]
pub type R = crate::R<CurrentloadctrlSpec>;
#[doc = "Field `CURRENTLOADCTRL` reader - Current value driven to the NFC Load Control"]
pub type CurrentloadctrlR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - Current value driven to the NFC Load Control"]
    #[inline(always)]
    pub fn currentloadctrl(&self) -> CurrentloadctrlR {
        CurrentloadctrlR::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "Current value driven to the NFC Load Control\n\nYou can [`read`](crate::Reg::read) this register and get [`currentloadctrl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CurrentloadctrlSpec;
impl crate::RegisterSpec for CurrentloadctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`currentloadctrl::R`](R) reader structure"]
impl crate::Readable for CurrentloadctrlSpec {}
#[doc = "`reset()` method sets CURRENTLOADCTRL to value 0"]
impl crate::Resettable for CurrentloadctrlSpec {}
