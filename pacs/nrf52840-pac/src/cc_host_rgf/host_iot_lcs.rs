#[doc = "Register `HOST_IOT_LCS` reader"]
pub type R = crate::R<HostIotLcsSpec>;
#[doc = "Register `HOST_IOT_LCS` writer"]
pub type W = crate::W<HostIotLcsSpec>;
#[doc = "Life-cycle state value. This field is write-once per reset.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lcs {
    #[doc = "0: CC310 operates in debug mode"]
    DebugEnable = 0,
    #[doc = "2: CC310 operates in secure mode"]
    Secure = 2,
}
impl From<Lcs> for u8 {
    #[inline(always)]
    fn from(variant: Lcs) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lcs {
    type Ux = u8;
}
impl crate::IsEnum for Lcs {}
#[doc = "Field `LCS` reader - Life-cycle state value. This field is write-once per reset."]
pub type LcsR = crate::FieldReader<Lcs>;
impl LcsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Lcs> {
        match self.bits {
            0 => Some(Lcs::DebugEnable),
            2 => Some(Lcs::Secure),
            _ => None,
        }
    }
    #[doc = "CC310 operates in debug mode"]
    #[inline(always)]
    pub fn is_debug_enable(&self) -> bool {
        *self == Lcs::DebugEnable
    }
    #[doc = "CC310 operates in secure mode"]
    #[inline(always)]
    pub fn is_secure(&self) -> bool {
        *self == Lcs::Secure
    }
}
#[doc = "Field `LCS` writer - Life-cycle state value. This field is write-once per reset."]
pub type LcsW<'a, REG> = crate::FieldWriter<'a, REG, 3, Lcs>;
impl<'a, REG> LcsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CC310 operates in debug mode"]
    #[inline(always)]
    pub fn debug_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Lcs::DebugEnable)
    }
    #[doc = "CC310 operates in secure mode"]
    #[inline(always)]
    pub fn secure(self) -> &'a mut crate::W<REG> {
        self.variant(Lcs::Secure)
    }
}
#[doc = "Read-only field. Indicates if CRYPTOCELL LCS has been successfully configured since last reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LcsIsValid {
    #[doc = "0: Valid LCS not yet retained in the CRYPTOCELL AO power domain"]
    Invalid = 0,
    #[doc = "1: Valid LCS successfully retained in the CRYPTOCELL AO power domain"]
    Valid = 1,
}
impl From<LcsIsValid> for bool {
    #[inline(always)]
    fn from(variant: LcsIsValid) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCS_IS_VALID` reader - Read-only field. Indicates if CRYPTOCELL LCS has been successfully configured since last reset."]
pub type LcsIsValidR = crate::BitReader<LcsIsValid>;
impl LcsIsValidR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LcsIsValid {
        match self.bits {
            false => LcsIsValid::Invalid,
            true => LcsIsValid::Valid,
        }
    }
    #[doc = "Valid LCS not yet retained in the CRYPTOCELL AO power domain"]
    #[inline(always)]
    pub fn is_invalid(&self) -> bool {
        *self == LcsIsValid::Invalid
    }
    #[doc = "Valid LCS successfully retained in the CRYPTOCELL AO power domain"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == LcsIsValid::Valid
    }
}
impl R {
    #[doc = "Bits 0:2 - Life-cycle state value. This field is write-once per reset."]
    #[inline(always)]
    pub fn lcs(&self) -> LcsR {
        LcsR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 8 - Read-only field. Indicates if CRYPTOCELL LCS has been successfully configured since last reset."]
    #[inline(always)]
    pub fn lcs_is_valid(&self) -> LcsIsValidR {
        LcsIsValidR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Life-cycle state value. This field is write-once per reset."]
    #[inline(always)]
    pub fn lcs(&mut self) -> LcsW<'_, HostIotLcsSpec> {
        LcsW::new(self, 0)
    }
}
#[doc = "Controls life-cycle state (LCS) for CRYPTOCELL subsystem\n\nYou can [`read`](crate::Reg::read) this register and get [`host_iot_lcs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_iot_lcs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HostIotLcsSpec;
impl crate::RegisterSpec for HostIotLcsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_iot_lcs::R`](R) reader structure"]
impl crate::Readable for HostIotLcsSpec {}
#[doc = "`write(|w| ..)` method takes [`host_iot_lcs::W`](W) writer structure"]
impl crate::Writable for HostIotLcsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HOST_IOT_LCS to value 0x02"]
impl crate::Resettable for HostIotLcsSpec {
    const RESET_VALUE: u32 = 0x02;
}
