#[doc = "Register `HEADERMASK` reader"]
pub type R = crate::R<HeadermaskSpec>;
#[doc = "Register `HEADERMASK` writer"]
pub type W = crate::W<HeadermaskSpec>;
#[doc = "Field `HEADERMASK` reader - Header (S0) mask"]
pub type HeadermaskR = crate::FieldReader;
#[doc = "Field `HEADERMASK` writer - Header (S0) mask"]
pub type HeadermaskW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Header (S0) mask"]
    #[inline(always)]
    pub fn headermask(&self) -> HeadermaskR {
        HeadermaskR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Header (S0) mask"]
    #[inline(always)]
    pub fn headermask(&mut self) -> HeadermaskW<'_, HeadermaskSpec> {
        HeadermaskW::new(self, 0)
    }
}
#[doc = "Header (S0) mask.\n\nYou can [`read`](crate::Reg::read) this register and get [`headermask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`headermask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HeadermaskSpec;
impl crate::RegisterSpec for HeadermaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`headermask::R`](R) reader structure"]
impl crate::Readable for HeadermaskSpec {}
#[doc = "`write(|w| ..)` method takes [`headermask::W`](W) writer structure"]
impl crate::Writable for HeadermaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HEADERMASK to value 0xe3"]
impl crate::Resettable for HeadermaskSpec {
    const RESET_VALUE: u32 = 0xe3;
}
