#[doc = "Register `DEBUGCTRL` reader"]
pub type R = crate::R<DebugctrlSpec>;
#[doc = "Register `DEBUGCTRL` writer"]
pub type W = crate::W<DebugctrlSpec>;
#[doc = "Configure CPU flash patch and breakpoint (FPB) unit behavior\n\nValue on reset: 255"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cpufpben {
    #[doc = "255: Enable CPU FPB unit (default behavior)"]
    Enabled = 255,
    #[doc = "0: Disable CPU FPB unit. Writes into the FPB registers will be ignored."]
    Disabled = 0,
}
impl From<Cpufpben> for u8 {
    #[inline(always)]
    fn from(variant: Cpufpben) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cpufpben {
    type Ux = u8;
}
impl crate::IsEnum for Cpufpben {}
#[doc = "Field `CPUFPBEN` reader - Configure CPU flash patch and breakpoint (FPB) unit behavior"]
pub type CpufpbenR = crate::FieldReader<Cpufpben>;
impl CpufpbenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cpufpben> {
        match self.bits {
            255 => Some(Cpufpben::Enabled),
            0 => Some(Cpufpben::Disabled),
            _ => None,
        }
    }
    #[doc = "Enable CPU FPB unit (default behavior)"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Cpufpben::Enabled
    }
    #[doc = "Disable CPU FPB unit. Writes into the FPB registers will be ignored."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Cpufpben::Disabled
    }
}
#[doc = "Field `CPUFPBEN` writer - Configure CPU flash patch and breakpoint (FPB) unit behavior"]
pub type CpufpbenW<'a, REG> = crate::FieldWriter<'a, REG, 8, Cpufpben>;
impl<'a, REG> CpufpbenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Enable CPU FPB unit (default behavior)"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cpufpben::Enabled)
    }
    #[doc = "Disable CPU FPB unit. Writes into the FPB registers will be ignored."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cpufpben::Disabled)
    }
}
impl R {
    #[doc = "Bits 8:15 - Configure CPU flash patch and breakpoint (FPB) unit behavior"]
    #[inline(always)]
    pub fn cpufpben(&self) -> CpufpbenR {
        CpufpbenR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - Configure CPU flash patch and breakpoint (FPB) unit behavior"]
    #[inline(always)]
    pub fn cpufpben(&mut self) -> CpufpbenW<'_, DebugctrlSpec> {
        CpufpbenW::new(self, 8)
    }
}
#[doc = "Processor debug control\n\nYou can [`read`](crate::Reg::read) this register and get [`debugctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debugctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DebugctrlSpec;
impl crate::RegisterSpec for DebugctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debugctrl::R`](R) reader structure"]
impl crate::Readable for DebugctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`debugctrl::W`](W) writer structure"]
impl crate::Writable for DebugctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEBUGCTRL to value 0xffff_ffff"]
impl crate::Resettable for DebugctrlSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
