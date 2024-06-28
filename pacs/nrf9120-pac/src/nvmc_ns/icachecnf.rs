#[doc = "Register `ICACHECNF` reader"]
pub type R = crate::R<IcachecnfSpec>;
#[doc = "Register `ICACHECNF` writer"]
pub type W = crate::W<IcachecnfSpec>;
#[doc = "Cache enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cacheen {
    #[doc = "0: Disable cache. Invalidates all cache entries."]
    Disabled = 0,
    #[doc = "1: Enable cache"]
    Enabled = 1,
}
impl From<Cacheen> for bool {
    #[inline(always)]
    fn from(variant: Cacheen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CACHEEN` reader - Cache enable"]
pub type CacheenR = crate::BitReader<Cacheen>;
impl CacheenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cacheen {
        match self.bits {
            false => Cacheen::Disabled,
            true => Cacheen::Enabled,
        }
    }
    #[doc = "Disable cache. Invalidates all cache entries."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Cacheen::Disabled
    }
    #[doc = "Enable cache"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Cacheen::Enabled
    }
}
#[doc = "Field `CACHEEN` writer - Cache enable"]
pub type CacheenW<'a, REG> = crate::BitWriter<'a, REG, Cacheen>;
impl<'a, REG> CacheenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable cache. Invalidates all cache entries."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cacheen::Disabled)
    }
    #[doc = "Enable cache"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cacheen::Enabled)
    }
}
#[doc = "Cache profiling enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cacheprofen {
    #[doc = "0: Disable cache profiling"]
    Disabled = 0,
    #[doc = "1: Enable cache profiling"]
    Enabled = 1,
}
impl From<Cacheprofen> for bool {
    #[inline(always)]
    fn from(variant: Cacheprofen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CACHEPROFEN` reader - Cache profiling enable"]
pub type CacheprofenR = crate::BitReader<Cacheprofen>;
impl CacheprofenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cacheprofen {
        match self.bits {
            false => Cacheprofen::Disabled,
            true => Cacheprofen::Enabled,
        }
    }
    #[doc = "Disable cache profiling"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Cacheprofen::Disabled
    }
    #[doc = "Enable cache profiling"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Cacheprofen::Enabled
    }
}
#[doc = "Field `CACHEPROFEN` writer - Cache profiling enable"]
pub type CacheprofenW<'a, REG> = crate::BitWriter<'a, REG, Cacheprofen>;
impl<'a, REG> CacheprofenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable cache profiling"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cacheprofen::Disabled)
    }
    #[doc = "Enable cache profiling"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cacheprofen::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Cache enable"]
    #[inline(always)]
    pub fn cacheen(&self) -> CacheenR {
        CacheenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Cache profiling enable"]
    #[inline(always)]
    pub fn cacheprofen(&self) -> CacheprofenR {
        CacheprofenR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Cache enable"]
    #[inline(always)]
    #[must_use]
    pub fn cacheen(&mut self) -> CacheenW<IcachecnfSpec> {
        CacheenW::new(self, 0)
    }
    #[doc = "Bit 8 - Cache profiling enable"]
    #[inline(always)]
    #[must_use]
    pub fn cacheprofen(&mut self) -> CacheprofenW<IcachecnfSpec> {
        CacheprofenW::new(self, 8)
    }
}
#[doc = "I-code cache configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icachecnf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icachecnf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcachecnfSpec;
impl crate::RegisterSpec for IcachecnfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icachecnf::R`](R) reader structure"]
impl crate::Readable for IcachecnfSpec {}
#[doc = "`write(|w| ..)` method takes [`icachecnf::W`](W) writer structure"]
impl crate::Writable for IcachecnfSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICACHECNF to value 0"]
impl crate::Resettable for IcachecnfSpec {
    const RESET_VALUE: u32 = 0;
}
