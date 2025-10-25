#[doc = "Register `AHBM_HMASTLOCK` reader"]
pub type R = crate::R<AhbmHmastlockSpec>;
#[doc = "Register `AHBM_HMASTLOCK` writer"]
pub type W = crate::W<AhbmHmastlockSpec>;
#[doc = "Field `AHB_HMASTLOCK` reader - The AHB HMASTLOCK value."]
pub type AhbHmastlockR = crate::BitReader;
#[doc = "Field `AHB_HMASTLOCK` writer - The AHB HMASTLOCK value."]
pub type AhbHmastlockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The AHB HMASTLOCK value."]
    #[inline(always)]
    pub fn ahb_hmastlock(&self) -> AhbHmastlockR {
        AhbHmastlockR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The AHB HMASTLOCK value."]
    #[inline(always)]
    pub fn ahb_hmastlock(&mut self) -> AhbHmastlockW<'_, AhbmHmastlockSpec> {
        AhbHmastlockW::new(self, 0)
    }
}
#[doc = "This register holds AHB HMASTLOCK value\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbm_hmastlock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbm_hmastlock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AhbmHmastlockSpec;
impl crate::RegisterSpec for AhbmHmastlockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbm_hmastlock::R`](R) reader structure"]
impl crate::Readable for AhbmHmastlockSpec {}
#[doc = "`write(|w| ..)` method takes [`ahbm_hmastlock::W`](W) writer structure"]
impl crate::Writable for AhbmHmastlockSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHBM_HMASTLOCK to value 0"]
impl crate::Resettable for AhbmHmastlockSpec {}
