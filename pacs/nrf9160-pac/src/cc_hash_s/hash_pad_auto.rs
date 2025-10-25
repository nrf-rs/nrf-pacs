#[doc = "Register `HASH_PAD_AUTO` writer"]
pub type W = crate::W<HashPadAutoSpec>;
#[doc = "Enable automatic padding in hardware.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hwpad {
    #[doc = "0: Do not enable automatic hardware padding."]
    Disable = 0,
    #[doc = "1: Enable automatic hardware padding."]
    Enable = 1,
}
impl From<Hwpad> for bool {
    #[inline(always)]
    fn from(variant: Hwpad) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HWPAD` writer - Enable automatic padding in hardware."]
pub type HwpadW<'a, REG> = crate::BitWriter<'a, REG, Hwpad>;
impl<'a, REG> HwpadW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not enable automatic hardware padding."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Hwpad::Disable)
    }
    #[doc = "Enable automatic hardware padding."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Hwpad::Enable)
    }
}
impl W {
    #[doc = "Bit 0 - Enable automatic padding in hardware."]
    #[inline(always)]
    pub fn hwpad(&mut self) -> HwpadW<'_, HashPadAutoSpec> {
        HwpadW::new(self, 0)
    }
}
#[doc = "Configure the HASH engine to automatically pad data at the end of the DMA transfer to complete the digest operation.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_pad_auto::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HashPadAutoSpec;
impl crate::RegisterSpec for HashPadAutoSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hash_pad_auto::W`](W) writer structure"]
impl crate::Writable for HashPadAutoSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HASH_PAD_AUTO to value 0"]
impl crate::Resettable for HashPadAutoSpec {}
