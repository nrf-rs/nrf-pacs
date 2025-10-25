#[doc = "Register `CLAIMSET` reader"]
pub type R = crate::R<ClaimsetSpec>;
#[doc = "Register `CLAIMSET` writer"]
pub type W = crate::W<ClaimsetSpec>;
#[doc = "Field `CLAIMSET` reader - This claim tag bit is implemented"]
pub type ClaimsetR = crate::FieldReader;
#[doc = "Field `CLAIMSET` writer - This claim tag bit is implemented"]
pub type ClaimsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - This claim tag bit is implemented"]
    #[inline(always)]
    pub fn claimset(&self) -> ClaimsetR {
        ClaimsetR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - This claim tag bit is implemented"]
    #[inline(always)]
    pub fn claimset(&mut self) -> ClaimsetW<'_, ClaimsetSpec> {
        ClaimsetW::new(self, 0)
    }
}
#[doc = "Claim Tag Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`claimset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`claimset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClaimsetSpec;
impl crate::RegisterSpec for ClaimsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`claimset::R`](R) reader structure"]
impl crate::Readable for ClaimsetSpec {}
#[doc = "`write(|w| ..)` method takes [`claimset::W`](W) writer structure"]
impl crate::Writable for ClaimsetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLAIMSET to value 0x0f"]
impl crate::Resettable for ClaimsetSpec {
    const RESET_VALUE: u32 = 0x0f;
}
