#[doc = "Register `AES_CMAC_INIT` writer"]
pub type W = crate::W<AesCmacInitSpec>;
#[doc = "Generate K1 and K2 for the AES-CMAC operations.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enable {
    #[doc = "1: Initialize AES-CMAC operations."]
    Enable = 1,
}
impl From<Enable> for bool {
    #[inline(always)]
    fn from(variant: Enable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE` writer - Generate K1 and K2 for the AES-CMAC operations."]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG, Enable>;
impl<'a, REG> EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Initialize AES-CMAC operations."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Enable)
    }
}
impl W {
    #[doc = "Bit 0 - Generate K1 and K2 for the AES-CMAC operations."]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, AesCmacInitSpec> {
        EnableW::new(self, 0)
    }
}
#[doc = "Writing to this address triggers the AES engine to generate K1 and K2 for AES-CMAC operations.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_cmac_init::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesCmacInitSpec;
impl crate::RegisterSpec for AesCmacInitSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`aes_cmac_init::W`](W) writer structure"]
impl crate::Writable for AesCmacInitSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AES_CMAC_INIT to value 0"]
impl crate::Resettable for AesCmacInitSpec {}
