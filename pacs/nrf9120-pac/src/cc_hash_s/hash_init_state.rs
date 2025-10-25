#[doc = "Register `HASH_INIT_STATE` writer"]
pub type W = crate::W<HashInitStateSpec>;
#[doc = "Enable loading of data to initial state registers. Digest/IV for HASH/AES_MAC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Load {
    #[doc = "0: Disable loading of data to initial state registers."]
    Disable = 0,
    #[doc = "1: Enable loading of data to initial state registers."]
    Enable = 1,
}
impl From<Load> for bool {
    #[inline(always)]
    fn from(variant: Load) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOAD` writer - Enable loading of data to initial state registers. Digest/IV for HASH/AES_MAC."]
pub type LoadW<'a, REG> = crate::BitWriter<'a, REG, Load>;
impl<'a, REG> LoadW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable loading of data to initial state registers."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Load::Disable)
    }
    #[doc = "Enable loading of data to initial state registers."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Load::Enable)
    }
}
impl W {
    #[doc = "Bit 0 - Enable loading of data to initial state registers. Digest/IV for HASH/AES_MAC."]
    #[inline(always)]
    pub fn load(&mut self) -> LoadW<'_, HashInitStateSpec> {
        LoadW::new(self, 0)
    }
}
#[doc = "Configure HASH engine initial state registers.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_init_state::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HashInitStateSpec;
impl crate::RegisterSpec for HashInitStateSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hash_init_state::W`](W) writer structure"]
impl crate::Writable for HashInitStateSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HASH_INIT_STATE to value 0"]
impl crate::Resettable for HashInitStateSpec {}
