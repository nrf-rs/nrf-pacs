#[doc = "Register `INTENCLR` reader"]
pub type R = crate::R<IntenclrSpec>;
#[doc = "Register `INTENCLR` writer"]
pub type W = crate::W<IntenclrSpec>;
#[doc = "Disable interrupt on ENDKSGEN event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Endksgen {
    #[doc = "0: Interrupt disabled."]
    Disabled = 0,
    #[doc = "1: Interrupt enabled."]
    Enabled = 1,
}
impl From<Endksgen> for bool {
    #[inline(always)]
    fn from(variant: Endksgen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDKSGEN` reader - Disable interrupt on ENDKSGEN event."]
pub type EndksgenR = crate::BitReader<Endksgen>;
impl EndksgenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Endksgen {
        match self.bits {
            false => Endksgen::Disabled,
            true => Endksgen::Enabled,
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Endksgen::Disabled
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Endksgen::Enabled
    }
}
#[doc = "Disable interrupt on ENDKSGEN event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EndksgenWO {
    #[doc = "1: Disable interrupt on write."]
    Clear = 1,
}
impl From<EndksgenWO> for bool {
    #[inline(always)]
    fn from(variant: EndksgenWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDKSGEN` writer - Disable interrupt on ENDKSGEN event."]
pub type EndksgenW<'a, REG> = crate::BitWriter<'a, REG, EndksgenWO>;
impl<'a, REG> EndksgenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable interrupt on write."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(EndksgenWO::Clear)
    }
}
#[doc = "Disable interrupt on ENDCRYPT event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Endcrypt {
    #[doc = "0: Interrupt disabled."]
    Disabled = 0,
    #[doc = "1: Interrupt enabled."]
    Enabled = 1,
}
impl From<Endcrypt> for bool {
    #[inline(always)]
    fn from(variant: Endcrypt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDCRYPT` reader - Disable interrupt on ENDCRYPT event."]
pub type EndcryptR = crate::BitReader<Endcrypt>;
impl EndcryptR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Endcrypt {
        match self.bits {
            false => Endcrypt::Disabled,
            true => Endcrypt::Enabled,
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Endcrypt::Disabled
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Endcrypt::Enabled
    }
}
#[doc = "Disable interrupt on ENDCRYPT event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EndcryptWO {
    #[doc = "1: Disable interrupt on write."]
    Clear = 1,
}
impl From<EndcryptWO> for bool {
    #[inline(always)]
    fn from(variant: EndcryptWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDCRYPT` writer - Disable interrupt on ENDCRYPT event."]
pub type EndcryptW<'a, REG> = crate::BitWriter<'a, REG, EndcryptWO>;
impl<'a, REG> EndcryptW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable interrupt on write."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(EndcryptWO::Clear)
    }
}
#[doc = "Disable interrupt on ERROR event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    #[doc = "0: Interrupt disabled."]
    Disabled = 0,
    #[doc = "1: Interrupt enabled."]
    Enabled = 1,
}
impl From<Error> for bool {
    #[inline(always)]
    fn from(variant: Error) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERROR` reader - Disable interrupt on ERROR event."]
pub type ErrorR = crate::BitReader<Error>;
impl ErrorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Error {
        match self.bits {
            false => Error::Disabled,
            true => Error::Enabled,
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Error::Disabled
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Error::Enabled
    }
}
#[doc = "Disable interrupt on ERROR event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErrorWO {
    #[doc = "1: Disable interrupt on write."]
    Clear = 1,
}
impl From<ErrorWO> for bool {
    #[inline(always)]
    fn from(variant: ErrorWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERROR` writer - Disable interrupt on ERROR event."]
pub type ErrorW<'a, REG> = crate::BitWriter<'a, REG, ErrorWO>;
impl<'a, REG> ErrorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable interrupt on write."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(ErrorWO::Clear)
    }
}
impl R {
    #[doc = "Bit 0 - Disable interrupt on ENDKSGEN event."]
    #[inline(always)]
    pub fn endksgen(&self) -> EndksgenR {
        EndksgenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Disable interrupt on ENDCRYPT event."]
    #[inline(always)]
    pub fn endcrypt(&self) -> EndcryptR {
        EndcryptR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Disable interrupt on ERROR event."]
    #[inline(always)]
    pub fn error(&self) -> ErrorR {
        ErrorR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Disable interrupt on ENDKSGEN event."]
    #[inline(always)]
    pub fn endksgen(&mut self) -> EndksgenW<'_, IntenclrSpec> {
        EndksgenW::new(self, 0)
    }
    #[doc = "Bit 1 - Disable interrupt on ENDCRYPT event."]
    #[inline(always)]
    pub fn endcrypt(&mut self) -> EndcryptW<'_, IntenclrSpec> {
        EndcryptW::new(self, 1)
    }
    #[doc = "Bit 2 - Disable interrupt on ERROR event."]
    #[inline(always)]
    pub fn error(&mut self) -> ErrorW<'_, IntenclrSpec> {
        ErrorW::new(self, 2)
    }
}
#[doc = "Interrupt enable clear register.\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenclrSpec;
impl crate::RegisterSpec for IntenclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intenclr::R`](R) reader structure"]
impl crate::Readable for IntenclrSpec {}
#[doc = "`write(|w| ..)` method takes [`intenclr::W`](W) writer structure"]
impl crate::Writable for IntenclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for IntenclrSpec {}
