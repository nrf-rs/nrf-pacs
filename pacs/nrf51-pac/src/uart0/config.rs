#[doc = "Register `CONFIG` reader"]
pub type R = crate::R<ConfigSpec>;
#[doc = "Register `CONFIG` writer"]
pub type W = crate::W<ConfigSpec>;
#[doc = "Hardware flow control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hwfc {
    #[doc = "0: Hardware flow control disabled."]
    Disabled = 0,
    #[doc = "1: Hardware flow control enabled."]
    Enabled = 1,
}
impl From<Hwfc> for bool {
    #[inline(always)]
    fn from(variant: Hwfc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HWFC` reader - Hardware flow control."]
pub type HwfcR = crate::BitReader<Hwfc>;
impl HwfcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hwfc {
        match self.bits {
            false => Hwfc::Disabled,
            true => Hwfc::Enabled,
        }
    }
    #[doc = "Hardware flow control disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Hwfc::Disabled
    }
    #[doc = "Hardware flow control enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Hwfc::Enabled
    }
}
#[doc = "Field `HWFC` writer - Hardware flow control."]
pub type HwfcW<'a, REG> = crate::BitWriter<'a, REG, Hwfc>;
impl<'a, REG> HwfcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Hardware flow control disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Hwfc::Disabled)
    }
    #[doc = "Hardware flow control enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Hwfc::Enabled)
    }
}
#[doc = "Include parity bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Parity {
    #[doc = "0: Parity bit excluded."]
    Excluded = 0,
    #[doc = "7: Parity bit included."]
    Included = 7,
}
impl From<Parity> for u8 {
    #[inline(always)]
    fn from(variant: Parity) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Parity {
    type Ux = u8;
}
impl crate::IsEnum for Parity {}
#[doc = "Field `PARITY` reader - Include parity bit."]
pub type ParityR = crate::FieldReader<Parity>;
impl ParityR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Parity> {
        match self.bits {
            0 => Some(Parity::Excluded),
            7 => Some(Parity::Included),
            _ => None,
        }
    }
    #[doc = "Parity bit excluded."]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        *self == Parity::Excluded
    }
    #[doc = "Parity bit included."]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        *self == Parity::Included
    }
}
#[doc = "Field `PARITY` writer - Include parity bit."]
pub type ParityW<'a, REG> = crate::FieldWriter<'a, REG, 3, Parity>;
impl<'a, REG> ParityW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Parity bit excluded."]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut crate::W<REG> {
        self.variant(Parity::Excluded)
    }
    #[doc = "Parity bit included."]
    #[inline(always)]
    pub fn included(self) -> &'a mut crate::W<REG> {
        self.variant(Parity::Included)
    }
}
impl R {
    #[doc = "Bit 0 - Hardware flow control."]
    #[inline(always)]
    pub fn hwfc(&self) -> HwfcR {
        HwfcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Include parity bit."]
    #[inline(always)]
    pub fn parity(&self) -> ParityR {
        ParityR::new(((self.bits >> 1) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Hardware flow control."]
    #[inline(always)]
    pub fn hwfc(&mut self) -> HwfcW<'_, ConfigSpec> {
        HwfcW::new(self, 0)
    }
    #[doc = "Bits 1:3 - Include parity bit."]
    #[inline(always)]
    pub fn parity(&mut self) -> ParityW<'_, ConfigSpec> {
        ParityW::new(self, 1)
    }
}
#[doc = "Configuration of parity and hardware flow control register.\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfigSpec;
impl crate::RegisterSpec for ConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config::R`](R) reader structure"]
impl crate::Readable for ConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`config::W`](W) writer structure"]
impl crate::Writable for ConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONFIG to value 0"]
impl crate::Resettable for ConfigSpec {}
