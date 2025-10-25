#[doc = "Register `DAI` reader"]
pub type R = crate::R<DaiSpec>;
#[doc = "Field `DAI` reader - Index (n) of device address (see DAB\\[n\\] and DAP\\[n\\]) that obtained an address match."]
pub type DaiR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - Index (n) of device address (see DAB\\[n\\] and DAP\\[n\\]) that obtained an address match."]
    #[inline(always)]
    pub fn dai(&self) -> DaiR {
        DaiR::new((self.bits & 7) as u8)
    }
}
#[doc = "Device address match index.\n\nYou can [`read`](crate::Reg::read) this register and get [`dai::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DaiSpec;
impl crate::RegisterSpec for DaiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dai::R`](R) reader structure"]
impl crate::Readable for DaiSpec {}
#[doc = "`reset()` method sets DAI to value 0"]
impl crate::Resettable for DaiSpec {}
