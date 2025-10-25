#[doc = "Register `CRV` reader"]
pub type R = crate::R<CrvSpec>;
#[doc = "Register `CRV` writer"]
pub type W = crate::W<CrvSpec>;
#[doc = "Field `CRV` reader - Counter reload value in number of cycles of the 32.768 kHz clock"]
pub type CrvR = crate::FieldReader<u32>;
#[doc = "Field `CRV` writer - Counter reload value in number of cycles of the 32.768 kHz clock"]
pub type CrvW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Counter reload value in number of cycles of the 32.768 kHz clock"]
    #[inline(always)]
    pub fn crv(&self) -> CrvR {
        CrvR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Counter reload value in number of cycles of the 32.768 kHz clock"]
    #[inline(always)]
    pub fn crv(&mut self) -> CrvW<'_, CrvSpec> {
        CrvW::new(self, 0)
    }
}
#[doc = "Counter reload value\n\nYou can [`read`](crate::Reg::read) this register and get [`crv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrvSpec;
impl crate::RegisterSpec for CrvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crv::R`](R) reader structure"]
impl crate::Readable for CrvSpec {}
#[doc = "`write(|w| ..)` method takes [`crv::W`](W) writer structure"]
impl crate::Writable for CrvSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CRV to value 0xffff_ffff"]
impl crate::Resettable for CrvSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
