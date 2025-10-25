#[doc = "Register `INTENCLR` reader"]
pub type R = crate::R<IntenclrSpec>;
#[doc = "Register `INTENCLR` writer"]
pub type W = crate::W<IntenclrSpec>;
#[doc = "Write '1' to disable interrupt for event ENDKSGEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Endksgen {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Endksgen> for bool {
    #[inline(always)]
    fn from(variant: Endksgen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDKSGEN` reader - Write '1' to disable interrupt for event ENDKSGEN"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Endksgen::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Endksgen::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event ENDKSGEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EndksgenWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<EndksgenWO> for bool {
    #[inline(always)]
    fn from(variant: EndksgenWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDKSGEN` writer - Write '1' to disable interrupt for event ENDKSGEN"]
pub type EndksgenW<'a, REG> = crate::BitWriter<'a, REG, EndksgenWO>;
impl<'a, REG> EndksgenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(EndksgenWO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event ENDCRYPT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Endcrypt {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Endcrypt> for bool {
    #[inline(always)]
    fn from(variant: Endcrypt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDCRYPT` reader - Write '1' to disable interrupt for event ENDCRYPT"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Endcrypt::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Endcrypt::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event ENDCRYPT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EndcryptWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<EndcryptWO> for bool {
    #[inline(always)]
    fn from(variant: EndcryptWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDCRYPT` writer - Write '1' to disable interrupt for event ENDCRYPT"]
pub type EndcryptW<'a, REG> = crate::BitWriter<'a, REG, EndcryptWO>;
impl<'a, REG> EndcryptW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(EndcryptWO::Clear)
    }
}
#[doc = "Deprecated intclrfield - Write '1' to disable interrupt for event ERROR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Error> for bool {
    #[inline(always)]
    fn from(variant: Error) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERROR` reader - Deprecated intclrfield - Write '1' to disable interrupt for event ERROR"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Error::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Error::Enabled
    }
}
#[doc = "Deprecated intclrfield - Write '1' to disable interrupt for event ERROR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErrorWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<ErrorWO> for bool {
    #[inline(always)]
    fn from(variant: ErrorWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERROR` writer - Deprecated intclrfield - Write '1' to disable interrupt for event ERROR"]
pub type ErrorW<'a, REG> = crate::BitWriter<'a, REG, ErrorWO>;
impl<'a, REG> ErrorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(ErrorWO::Clear)
    }
}
impl R {
    #[doc = "Bit 0 - Write '1' to disable interrupt for event ENDKSGEN"]
    #[inline(always)]
    pub fn endksgen(&self) -> EndksgenR {
        EndksgenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write '1' to disable interrupt for event ENDCRYPT"]
    #[inline(always)]
    pub fn endcrypt(&self) -> EndcryptR {
        EndcryptR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Deprecated intclrfield - Write '1' to disable interrupt for event ERROR"]
    #[inline(always)]
    pub fn error(&self) -> ErrorR {
        ErrorR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write '1' to disable interrupt for event ENDKSGEN"]
    #[inline(always)]
    pub fn endksgen(&mut self) -> EndksgenW<'_, IntenclrSpec> {
        EndksgenW::new(self, 0)
    }
    #[doc = "Bit 1 - Write '1' to disable interrupt for event ENDCRYPT"]
    #[inline(always)]
    pub fn endcrypt(&mut self) -> EndcryptW<'_, IntenclrSpec> {
        EndcryptW::new(self, 1)
    }
    #[doc = "Bit 2 - Deprecated intclrfield - Write '1' to disable interrupt for event ERROR"]
    #[inline(always)]
    pub fn error(&mut self) -> ErrorW<'_, IntenclrSpec> {
        ErrorW::new(self, 2)
    }
}
#[doc = "Disable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
