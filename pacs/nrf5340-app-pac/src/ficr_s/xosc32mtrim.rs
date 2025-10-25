#[doc = "Register `XOSC32MTRIM` reader"]
pub type R = crate::R<Xosc32mtrimSpec>;
#[doc = "Field `SLOPE` reader - Slope trim factor on twos complement form"]
pub type SlopeR = crate::FieldReader;
#[doc = "Field `OFFSET` reader - Offset trim factor on integer form"]
pub type OffsetR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:4 - Slope trim factor on twos complement form"]
    #[inline(always)]
    pub fn slope(&self) -> SlopeR {
        SlopeR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - Offset trim factor on integer form"]
    #[inline(always)]
    pub fn offset(&self) -> OffsetR {
        OffsetR::new(((self.bits >> 5) & 0x1f) as u8)
    }
}
#[doc = "XOSC32M capacitor selection trim values\n\nYou can [`read`](crate::Reg::read) this register and get [`xosc32mtrim::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Xosc32mtrimSpec;
impl crate::RegisterSpec for Xosc32mtrimSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xosc32mtrim::R`](R) reader structure"]
impl crate::Readable for Xosc32mtrimSpec {}
#[doc = "`reset()` method sets XOSC32MTRIM to value 0xffff_ffff"]
impl crate::Resettable for Xosc32mtrimSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
