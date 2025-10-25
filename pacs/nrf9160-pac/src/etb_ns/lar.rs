#[doc = "Register `LAR` writer"]
pub type W = crate::W<LarSpec>;
#[doc = "Field `ACCESS_W` writer - A write of 0xC5ACCE55 enables further write access to this device. A write of any value other than 0xC5ACCE55 will have the affect of removing write access."]
pub type AccessWW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - A write of 0xC5ACCE55 enables further write access to this device. A write of any value other than 0xC5ACCE55 will have the affect of removing write access."]
    #[inline(always)]
    pub fn access_w(&mut self) -> AccessWW<'_, LarSpec> {
        AccessWW::new(self, 0)
    }
}
#[doc = "Lock Access Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lar::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LarSpec;
impl crate::RegisterSpec for LarSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`lar::W`](W) writer structure"]
impl crate::Writable for LarSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LAR to value 0"]
impl crate::Resettable for LarSpec {}
