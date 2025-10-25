#[doc = "Register `INTENCLR` reader"]
pub type R = crate::R<IntenclrSpec>;
#[doc = "Register `INTENCLR` writer"]
pub type W = crate::W<IntenclrSpec>;
#[doc = "Disable interrupt on ENDECB event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Endecb {
    #[doc = "0: Interrupt disabled."]
    Disabled = 0,
    #[doc = "1: Interrupt enabled."]
    Enabled = 1,
}
impl From<Endecb> for bool {
    #[inline(always)]
    fn from(variant: Endecb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDECB` reader - Disable interrupt on ENDECB event."]
pub type EndecbR = crate::BitReader<Endecb>;
impl EndecbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Endecb {
        match self.bits {
            false => Endecb::Disabled,
            true => Endecb::Enabled,
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Endecb::Disabled
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Endecb::Enabled
    }
}
#[doc = "Disable interrupt on ENDECB event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EndecbWO {
    #[doc = "1: Disable interrupt on write."]
    Clear = 1,
}
impl From<EndecbWO> for bool {
    #[inline(always)]
    fn from(variant: EndecbWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDECB` writer - Disable interrupt on ENDECB event."]
pub type EndecbW<'a, REG> = crate::BitWriter<'a, REG, EndecbWO>;
impl<'a, REG> EndecbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable interrupt on write."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(EndecbWO::Clear)
    }
}
#[doc = "Disable interrupt on ERRORECB event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errorecb {
    #[doc = "0: Interrupt disabled."]
    Disabled = 0,
    #[doc = "1: Interrupt enabled."]
    Enabled = 1,
}
impl From<Errorecb> for bool {
    #[inline(always)]
    fn from(variant: Errorecb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRORECB` reader - Disable interrupt on ERRORECB event."]
pub type ErrorecbR = crate::BitReader<Errorecb>;
impl ErrorecbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Errorecb {
        match self.bits {
            false => Errorecb::Disabled,
            true => Errorecb::Enabled,
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Errorecb::Disabled
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Errorecb::Enabled
    }
}
#[doc = "Disable interrupt on ERRORECB event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErrorecbWO {
    #[doc = "1: Disable interrupt on write."]
    Clear = 1,
}
impl From<ErrorecbWO> for bool {
    #[inline(always)]
    fn from(variant: ErrorecbWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRORECB` writer - Disable interrupt on ERRORECB event."]
pub type ErrorecbW<'a, REG> = crate::BitWriter<'a, REG, ErrorecbWO>;
impl<'a, REG> ErrorecbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable interrupt on write."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(ErrorecbWO::Clear)
    }
}
impl R {
    #[doc = "Bit 0 - Disable interrupt on ENDECB event."]
    #[inline(always)]
    pub fn endecb(&self) -> EndecbR {
        EndecbR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Disable interrupt on ERRORECB event."]
    #[inline(always)]
    pub fn errorecb(&self) -> ErrorecbR {
        ErrorecbR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Disable interrupt on ENDECB event."]
    #[inline(always)]
    pub fn endecb(&mut self) -> EndecbW<'_, IntenclrSpec> {
        EndecbW::new(self, 0)
    }
    #[doc = "Bit 1 - Disable interrupt on ERRORECB event."]
    #[inline(always)]
    pub fn errorecb(&mut self) -> ErrorecbW<'_, IntenclrSpec> {
        ErrorecbW::new(self, 1)
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
