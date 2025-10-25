#[doc = "Register `CLAIMCLR` reader"]
pub type R = crate::R<ClaimclrSpec>;
#[doc = "Register `CLAIMCLR` writer"]
pub type W = crate::W<ClaimclrSpec>;
#[doc = "Field `CLAIMCLR` reader - The value present reflects the current setting of the Claim Tag."]
pub type ClaimclrR = crate::FieldReader;
#[doc = "Field `CLAIMCLR` writer - The value present reflects the current setting of the Claim Tag."]
pub type ClaimclrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - The value present reflects the current setting of the Claim Tag."]
    #[inline(always)]
    pub fn claimclr(&self) -> ClaimclrR {
        ClaimclrR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - The value present reflects the current setting of the Claim Tag."]
    #[inline(always)]
    pub fn claimclr(&mut self) -> ClaimclrW<'_, ClaimclrSpec> {
        ClaimclrW::new(self, 0)
    }
}
#[doc = "Claim Tag Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`claimclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`claimclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClaimclrSpec;
impl crate::RegisterSpec for ClaimclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`claimclr::R`](R) reader structure"]
impl crate::Readable for ClaimclrSpec {}
#[doc = "`write(|w| ..)` method takes [`claimclr::W`](W) writer structure"]
impl crate::Writable for ClaimclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLAIMCLR to value 0"]
impl crate::Resettable for ClaimclrSpec {}
