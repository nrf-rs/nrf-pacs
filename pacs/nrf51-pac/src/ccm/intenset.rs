#[doc = "Register `INTENSET` reader"]
pub type R = crate::R<IntensetSpec>;
#[doc = "Register `INTENSET` writer"]
pub type W = crate::W<IntensetSpec>;
#[doc = "Enable interrupt on ENDKSGEN event.\n\nValue on reset: 0"]
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
#[doc = "Field `ENDKSGEN` reader - Enable interrupt on ENDKSGEN event."]
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
#[doc = "Enable interrupt on ENDKSGEN event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EndksgenWO {
    #[doc = "1: Enable interrupt on write."]
    Set = 1,
}
impl From<EndksgenWO> for bool {
    #[inline(always)]
    fn from(variant: EndksgenWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDKSGEN` writer - Enable interrupt on ENDKSGEN event."]
pub type EndksgenW<'a, REG> = crate::BitWriter<'a, REG, EndksgenWO>;
impl<'a, REG> EndksgenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable interrupt on write."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(EndksgenWO::Set)
    }
}
#[doc = "Enable interrupt on ENDCRYPT event.\n\nValue on reset: 0"]
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
#[doc = "Field `ENDCRYPT` reader - Enable interrupt on ENDCRYPT event."]
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
#[doc = "Enable interrupt on ENDCRYPT event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EndcryptWO {
    #[doc = "1: Enable interrupt on write."]
    Set = 1,
}
impl From<EndcryptWO> for bool {
    #[inline(always)]
    fn from(variant: EndcryptWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDCRYPT` writer - Enable interrupt on ENDCRYPT event."]
pub type EndcryptW<'a, REG> = crate::BitWriter<'a, REG, EndcryptWO>;
impl<'a, REG> EndcryptW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable interrupt on write."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(EndcryptWO::Set)
    }
}
#[doc = "Enable interrupt on ERROR event.\n\nValue on reset: 0"]
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
#[doc = "Field `ERROR` reader - Enable interrupt on ERROR event."]
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
#[doc = "Enable interrupt on ERROR event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErrorWO {
    #[doc = "1: Enable interrupt on write."]
    Set = 1,
}
impl From<ErrorWO> for bool {
    #[inline(always)]
    fn from(variant: ErrorWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERROR` writer - Enable interrupt on ERROR event."]
pub type ErrorW<'a, REG> = crate::BitWriter<'a, REG, ErrorWO>;
impl<'a, REG> ErrorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable interrupt on write."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(ErrorWO::Set)
    }
}
impl R {
    #[doc = "Bit 0 - Enable interrupt on ENDKSGEN event."]
    #[inline(always)]
    pub fn endksgen(&self) -> EndksgenR {
        EndksgenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable interrupt on ENDCRYPT event."]
    #[inline(always)]
    pub fn endcrypt(&self) -> EndcryptR {
        EndcryptR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable interrupt on ERROR event."]
    #[inline(always)]
    pub fn error(&self) -> ErrorR {
        ErrorR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable interrupt on ENDKSGEN event."]
    #[inline(always)]
    pub fn endksgen(&mut self) -> EndksgenW<'_, IntensetSpec> {
        EndksgenW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable interrupt on ENDCRYPT event."]
    #[inline(always)]
    pub fn endcrypt(&mut self) -> EndcryptW<'_, IntensetSpec> {
        EndcryptW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable interrupt on ERROR event."]
    #[inline(always)]
    pub fn error(&mut self) -> ErrorW<'_, IntensetSpec> {
        ErrorW::new(self, 2)
    }
}
#[doc = "Interrupt enable set register.\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntensetSpec;
impl crate::RegisterSpec for IntensetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intenset::R`](R) reader structure"]
impl crate::Readable for IntensetSpec {}
#[doc = "`write(|w| ..)` method takes [`intenset::W`](W) writer structure"]
impl crate::Writable for IntensetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTENSET to value 0"]
impl crate::Resettable for IntensetSpec {}
