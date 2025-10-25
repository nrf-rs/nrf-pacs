#[doc = "Register `AES_CMAC_SIZE0_KICK` writer"]
pub type W = crate::W<AesCmacSize0KickSpec>;
#[doc = "Force AES CMAC operation with size 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enable {
    #[doc = "0: Normal AES CMAC operation"]
    Disable = 0,
    #[doc = "1: Force CMAC operation with size 0"]
    Enable = 1,
}
impl From<Enable> for bool {
    #[inline(always)]
    fn from(variant: Enable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE` writer - Force AES CMAC operation with size 0."]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG, Enable>;
impl<'a, REG> EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal AES CMAC operation"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Disable)
    }
    #[doc = "Force CMAC operation with size 0"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::Enable)
    }
}
impl W {
    #[doc = "Bit 0 - Force AES CMAC operation with size 0."]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, AesCmacSize0KickSpec> {
        EnableW::new(self, 0)
    }
}
#[doc = "Writing to this address triggers the AES engine to perform a CMAC operation with size 0. The CMAC result can be read from the AES_IV_0 register.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_cmac_size0_kick::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesCmacSize0KickSpec;
impl crate::RegisterSpec for AesCmacSize0KickSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`aes_cmac_size0_kick::W`](W) writer structure"]
impl crate::Writable for AesCmacSize0KickSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AES_CMAC_SIZE0_KICK to value 0"]
impl crate::Resettable for AesCmacSize0KickSpec {}
