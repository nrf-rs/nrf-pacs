#[doc = "Register `HOST_CRYPTOKEY_SEL` reader"]
pub type R = crate::R<HostCryptokeySelSpec>;
#[doc = "Register `HOST_CRYPTOKEY_SEL` writer"]
pub type W = crate::W<HostCryptokeySelSpec>;
#[doc = "Select the source of the HW key that is used by the AES engine\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HostCryptokeySel {
    #[doc = "0: Use device root key K_DR from CRYPTOCELL AO power domain"]
    KDr = 0,
    #[doc = "1: Use hard-coded RTL key K_PRTL"]
    KPrtl = 1,
    #[doc = "2: Use provided session key"]
    Session = 2,
}
impl From<HostCryptokeySel> for u8 {
    #[inline(always)]
    fn from(variant: HostCryptokeySel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HostCryptokeySel {
    type Ux = u8;
}
impl crate::IsEnum for HostCryptokeySel {}
#[doc = "Field `HOST_CRYPTOKEY_SEL` reader - Select the source of the HW key that is used by the AES engine"]
pub type HostCryptokeySelR = crate::FieldReader<HostCryptokeySel>;
impl HostCryptokeySelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<HostCryptokeySel> {
        match self.bits {
            0 => Some(HostCryptokeySel::KDr),
            1 => Some(HostCryptokeySel::KPrtl),
            2 => Some(HostCryptokeySel::Session),
            _ => None,
        }
    }
    #[doc = "Use device root key K_DR from CRYPTOCELL AO power domain"]
    #[inline(always)]
    pub fn is_k_dr(&self) -> bool {
        *self == HostCryptokeySel::KDr
    }
    #[doc = "Use hard-coded RTL key K_PRTL"]
    #[inline(always)]
    pub fn is_k_prtl(&self) -> bool {
        *self == HostCryptokeySel::KPrtl
    }
    #[doc = "Use provided session key"]
    #[inline(always)]
    pub fn is_session(&self) -> bool {
        *self == HostCryptokeySel::Session
    }
}
#[doc = "Field `HOST_CRYPTOKEY_SEL` writer - Select the source of the HW key that is used by the AES engine"]
pub type HostCryptokeySelW<'a, REG> = crate::FieldWriter<'a, REG, 2, HostCryptokeySel>;
impl<'a, REG> HostCryptokeySelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Use device root key K_DR from CRYPTOCELL AO power domain"]
    #[inline(always)]
    pub fn k_dr(self) -> &'a mut crate::W<REG> {
        self.variant(HostCryptokeySel::KDr)
    }
    #[doc = "Use hard-coded RTL key K_PRTL"]
    #[inline(always)]
    pub fn k_prtl(self) -> &'a mut crate::W<REG> {
        self.variant(HostCryptokeySel::KPrtl)
    }
    #[doc = "Use provided session key"]
    #[inline(always)]
    pub fn session(self) -> &'a mut crate::W<REG> {
        self.variant(HostCryptokeySel::Session)
    }
}
impl R {
    #[doc = "Bits 0:1 - Select the source of the HW key that is used by the AES engine"]
    #[inline(always)]
    pub fn host_cryptokey_sel(&self) -> HostCryptokeySelR {
        HostCryptokeySelR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Select the source of the HW key that is used by the AES engine"]
    #[inline(always)]
    #[must_use]
    pub fn host_cryptokey_sel(&mut self) -> HostCryptokeySelW<HostCryptokeySelSpec> {
        HostCryptokeySelW::new(self, 0)
    }
}
#[doc = "AES hardware key select\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_cryptokey_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_cryptokey_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HostCryptokeySelSpec;
impl crate::RegisterSpec for HostCryptokeySelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_cryptokey_sel::R`](R) reader structure"]
impl crate::Readable for HostCryptokeySelSpec {}
#[doc = "`write(|w| ..)` method takes [`host_cryptokey_sel::W`](W) writer structure"]
impl crate::Writable for HostCryptokeySelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HOST_CRYPTOKEY_SEL to value 0"]
impl crate::Resettable for HostCryptokeySelSpec {
    const RESET_VALUE: u32 = 0;
}
