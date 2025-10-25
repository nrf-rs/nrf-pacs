#[doc = "Register `INTENSET` reader"]
pub type R = crate::R<IntensetSpec>;
#[doc = "Register `INTENSET` writer"]
pub type W = crate::W<IntensetSpec>;
#[doc = "Write '1' to Enable interrupt for ENDECB event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Endecb {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Endecb> for bool {
    #[inline(always)]
    fn from(variant: Endecb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDECB` reader - Write '1' to Enable interrupt for ENDECB event"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Endecb::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Endecb::Enabled
    }
}
#[doc = "Write '1' to Enable interrupt for ENDECB event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EndecbWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<EndecbWO> for bool {
    #[inline(always)]
    fn from(variant: EndecbWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDECB` writer - Write '1' to Enable interrupt for ENDECB event"]
pub type EndecbW<'a, REG> = crate::BitWriter<'a, REG, EndecbWO>;
impl<'a, REG> EndecbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(EndecbWO::Set)
    }
}
#[doc = "Write '1' to Enable interrupt for ERRORECB event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errorecb {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Errorecb> for bool {
    #[inline(always)]
    fn from(variant: Errorecb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRORECB` reader - Write '1' to Enable interrupt for ERRORECB event"]
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
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Errorecb::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Errorecb::Enabled
    }
}
#[doc = "Write '1' to Enable interrupt for ERRORECB event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErrorecbWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<ErrorecbWO> for bool {
    #[inline(always)]
    fn from(variant: ErrorecbWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRORECB` writer - Write '1' to Enable interrupt for ERRORECB event"]
pub type ErrorecbW<'a, REG> = crate::BitWriter<'a, REG, ErrorecbWO>;
impl<'a, REG> ErrorecbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(ErrorecbWO::Set)
    }
}
impl R {
    #[doc = "Bit 0 - Write '1' to Enable interrupt for ENDECB event"]
    #[inline(always)]
    pub fn endecb(&self) -> EndecbR {
        EndecbR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write '1' to Enable interrupt for ERRORECB event"]
    #[inline(always)]
    pub fn errorecb(&self) -> ErrorecbR {
        ErrorecbR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write '1' to Enable interrupt for ENDECB event"]
    #[inline(always)]
    pub fn endecb(&mut self) -> EndecbW<'_, IntensetSpec> {
        EndecbW::new(self, 0)
    }
    #[doc = "Bit 1 - Write '1' to Enable interrupt for ERRORECB event"]
    #[inline(always)]
    pub fn errorecb(&mut self) -> ErrorecbW<'_, IntensetSpec> {
        ErrorecbW::new(self, 1)
    }
}
#[doc = "Enable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
