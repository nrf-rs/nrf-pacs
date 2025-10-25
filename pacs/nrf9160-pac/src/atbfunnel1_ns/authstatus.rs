#[doc = "Register `AUTHSTATUS` reader"]
pub type R = crate::R<AuthstatusSpec>;
#[doc = "Register `AUTHSTATUS` writer"]
pub type W = crate::W<AuthstatusSpec>;
#[doc = "Non-secure Invasive Debug\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Nsid {
    #[doc = "0: The feature is not implemented."]
    NotImplemented = 0,
    #[doc = "1: The feature is implemented."]
    Implemented = 1,
}
impl From<Nsid> for u8 {
    #[inline(always)]
    fn from(variant: Nsid) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Nsid {
    type Ux = u8;
}
impl crate::IsEnum for Nsid {}
#[doc = "Field `NSID` reader - Non-secure Invasive Debug"]
pub type NsidR = crate::FieldReader<Nsid>;
impl NsidR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Nsid> {
        match self.bits {
            0 => Some(Nsid::NotImplemented),
            1 => Some(Nsid::Implemented),
            _ => None,
        }
    }
    #[doc = "The feature is not implemented."]
    #[inline(always)]
    pub fn is_not_implemented(&self) -> bool {
        *self == Nsid::NotImplemented
    }
    #[doc = "The feature is implemented."]
    #[inline(always)]
    pub fn is_implemented(&self) -> bool {
        *self == Nsid::Implemented
    }
}
#[doc = "Field `NSID` writer - Non-secure Invasive Debug"]
pub type NsidW<'a, REG> = crate::FieldWriter<'a, REG, 2, Nsid>;
impl<'a, REG> NsidW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The feature is not implemented."]
    #[inline(always)]
    pub fn not_implemented(self) -> &'a mut crate::W<REG> {
        self.variant(Nsid::NotImplemented)
    }
    #[doc = "The feature is implemented."]
    #[inline(always)]
    pub fn implemented(self) -> &'a mut crate::W<REG> {
        self.variant(Nsid::Implemented)
    }
}
#[doc = "Non-secure Non-Invasive Debug\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Nsnid {
    #[doc = "0: The feature is not implemented."]
    NotImplemented = 0,
    #[doc = "1: The feature is implemented."]
    Implemented = 1,
}
impl From<Nsnid> for u8 {
    #[inline(always)]
    fn from(variant: Nsnid) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Nsnid {
    type Ux = u8;
}
impl crate::IsEnum for Nsnid {}
#[doc = "Field `NSNID` reader - Non-secure Non-Invasive Debug"]
pub type NsnidR = crate::FieldReader<Nsnid>;
impl NsnidR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Nsnid> {
        match self.bits {
            0 => Some(Nsnid::NotImplemented),
            1 => Some(Nsnid::Implemented),
            _ => None,
        }
    }
    #[doc = "The feature is not implemented."]
    #[inline(always)]
    pub fn is_not_implemented(&self) -> bool {
        *self == Nsnid::NotImplemented
    }
    #[doc = "The feature is implemented."]
    #[inline(always)]
    pub fn is_implemented(&self) -> bool {
        *self == Nsnid::Implemented
    }
}
#[doc = "Field `NSNID` writer - Non-secure Non-Invasive Debug"]
pub type NsnidW<'a, REG> = crate::FieldWriter<'a, REG, 2, Nsnid>;
impl<'a, REG> NsnidW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The feature is not implemented."]
    #[inline(always)]
    pub fn not_implemented(self) -> &'a mut crate::W<REG> {
        self.variant(Nsnid::NotImplemented)
    }
    #[doc = "The feature is implemented."]
    #[inline(always)]
    pub fn implemented(self) -> &'a mut crate::W<REG> {
        self.variant(Nsnid::Implemented)
    }
}
#[doc = "Secure Invasive Debug\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sid {
    #[doc = "0: The feature is not implemented."]
    NotImplemented = 0,
    #[doc = "1: The feature is implemented."]
    Implemented = 1,
}
impl From<Sid> for u8 {
    #[inline(always)]
    fn from(variant: Sid) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sid {
    type Ux = u8;
}
impl crate::IsEnum for Sid {}
#[doc = "Field `SID` reader - Secure Invasive Debug"]
pub type SidR = crate::FieldReader<Sid>;
impl SidR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sid> {
        match self.bits {
            0 => Some(Sid::NotImplemented),
            1 => Some(Sid::Implemented),
            _ => None,
        }
    }
    #[doc = "The feature is not implemented."]
    #[inline(always)]
    pub fn is_not_implemented(&self) -> bool {
        *self == Sid::NotImplemented
    }
    #[doc = "The feature is implemented."]
    #[inline(always)]
    pub fn is_implemented(&self) -> bool {
        *self == Sid::Implemented
    }
}
#[doc = "Field `SID` writer - Secure Invasive Debug"]
pub type SidW<'a, REG> = crate::FieldWriter<'a, REG, 2, Sid>;
impl<'a, REG> SidW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The feature is not implemented."]
    #[inline(always)]
    pub fn not_implemented(self) -> &'a mut crate::W<REG> {
        self.variant(Sid::NotImplemented)
    }
    #[doc = "The feature is implemented."]
    #[inline(always)]
    pub fn implemented(self) -> &'a mut crate::W<REG> {
        self.variant(Sid::Implemented)
    }
}
#[doc = "Secure Non-Invasive Debug\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Snid {
    #[doc = "0: The feature is not implemented."]
    NotImplemented = 0,
    #[doc = "1: The feature is implemented."]
    Implemented = 1,
}
impl From<Snid> for u8 {
    #[inline(always)]
    fn from(variant: Snid) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Snid {
    type Ux = u8;
}
impl crate::IsEnum for Snid {}
#[doc = "Field `SNID` reader - Secure Non-Invasive Debug"]
pub type SnidR = crate::FieldReader<Snid>;
impl SnidR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Snid> {
        match self.bits {
            0 => Some(Snid::NotImplemented),
            1 => Some(Snid::Implemented),
            _ => None,
        }
    }
    #[doc = "The feature is not implemented."]
    #[inline(always)]
    pub fn is_not_implemented(&self) -> bool {
        *self == Snid::NotImplemented
    }
    #[doc = "The feature is implemented."]
    #[inline(always)]
    pub fn is_implemented(&self) -> bool {
        *self == Snid::Implemented
    }
}
#[doc = "Field `SNID` writer - Secure Non-Invasive Debug"]
pub type SnidW<'a, REG> = crate::FieldWriter<'a, REG, 2, Snid>;
impl<'a, REG> SnidW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The feature is not implemented."]
    #[inline(always)]
    pub fn not_implemented(self) -> &'a mut crate::W<REG> {
        self.variant(Snid::NotImplemented)
    }
    #[doc = "The feature is implemented."]
    #[inline(always)]
    pub fn implemented(self) -> &'a mut crate::W<REG> {
        self.variant(Snid::Implemented)
    }
}
impl R {
    #[doc = "Bits 0:1 - Non-secure Invasive Debug"]
    #[inline(always)]
    pub fn nsid(&self) -> NsidR {
        NsidR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Non-secure Non-Invasive Debug"]
    #[inline(always)]
    pub fn nsnid(&self) -> NsnidR {
        NsnidR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Secure Invasive Debug"]
    #[inline(always)]
    pub fn sid(&self) -> SidR {
        SidR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Secure Non-Invasive Debug"]
    #[inline(always)]
    pub fn snid(&self) -> SnidR {
        SnidR::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Non-secure Invasive Debug"]
    #[inline(always)]
    pub fn nsid(&mut self) -> NsidW<'_, AuthstatusSpec> {
        NsidW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Non-secure Non-Invasive Debug"]
    #[inline(always)]
    pub fn nsnid(&mut self) -> NsnidW<'_, AuthstatusSpec> {
        NsnidW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Secure Invasive Debug"]
    #[inline(always)]
    pub fn sid(&mut self) -> SidW<'_, AuthstatusSpec> {
        SidW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Secure Non-Invasive Debug"]
    #[inline(always)]
    pub fn snid(&mut self) -> SnidW<'_, AuthstatusSpec> {
        SnidW::new(self, 6)
    }
}
#[doc = "Indicates the current level of tracing permitted by the system\n\nYou can [`read`](crate::Reg::read) this register and get [`authstatus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`authstatus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AuthstatusSpec;
impl crate::RegisterSpec for AuthstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`authstatus::R`](R) reader structure"]
impl crate::Readable for AuthstatusSpec {}
#[doc = "`write(|w| ..)` method takes [`authstatus::W`](W) writer structure"]
impl crate::Writable for AuthstatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AUTHSTATUS to value 0"]
impl crate::Resettable for AuthstatusSpec {}
