#[doc = "Register `INTENCLR` reader"]
pub type R = crate::R<IntenclrSpec>;
#[doc = "Register `INTENCLR` writer"]
pub type W = crate::W<IntenclrSpec>;
#[doc = "Write '1' to disable interrupt for event END\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum End {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<End> for bool {
    #[inline(always)]
    fn from(variant: End) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `END` reader - Write '1' to disable interrupt for event END"]
pub type EndR = crate::BitReader<End>;
impl EndR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> End {
        match self.bits {
            false => End::Disabled,
            true => End::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == End::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == End::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event END\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EndWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<EndWO> for bool {
    #[inline(always)]
    fn from(variant: EndWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `END` writer - Write '1' to disable interrupt for event END"]
pub type EndW<'a, REG> = crate::BitWriter<'a, REG, EndWO>;
impl<'a, REG> EndW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(EndWO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event ENDRX\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Endrx {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Endrx> for bool {
    #[inline(always)]
    fn from(variant: Endrx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDRX` reader - Write '1' to disable interrupt for event ENDRX"]
pub type EndrxR = crate::BitReader<Endrx>;
impl EndrxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Endrx {
        match self.bits {
            false => Endrx::Disabled,
            true => Endrx::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Endrx::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Endrx::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event ENDRX\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EndrxWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<EndrxWO> for bool {
    #[inline(always)]
    fn from(variant: EndrxWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDRX` writer - Write '1' to disable interrupt for event ENDRX"]
pub type EndrxW<'a, REG> = crate::BitWriter<'a, REG, EndrxWO>;
impl<'a, REG> EndrxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(EndrxWO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event ACQUIRED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Acquired {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Acquired> for bool {
    #[inline(always)]
    fn from(variant: Acquired) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACQUIRED` reader - Write '1' to disable interrupt for event ACQUIRED"]
pub type AcquiredR = crate::BitReader<Acquired>;
impl AcquiredR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Acquired {
        match self.bits {
            false => Acquired::Disabled,
            true => Acquired::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Acquired::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Acquired::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event ACQUIRED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AcquiredWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<AcquiredWO> for bool {
    #[inline(always)]
    fn from(variant: AcquiredWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACQUIRED` writer - Write '1' to disable interrupt for event ACQUIRED"]
pub type AcquiredW<'a, REG> = crate::BitWriter<'a, REG, AcquiredWO>;
impl<'a, REG> AcquiredW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(AcquiredWO::Clear)
    }
}
impl R {
    #[doc = "Bit 1 - Write '1' to disable interrupt for event END"]
    #[inline(always)]
    pub fn end(&self) -> EndR {
        EndR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Write '1' to disable interrupt for event ENDRX"]
    #[inline(always)]
    pub fn endrx(&self) -> EndrxR {
        EndrxR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 10 - Write '1' to disable interrupt for event ACQUIRED"]
    #[inline(always)]
    pub fn acquired(&self) -> AcquiredR {
        AcquiredR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Write '1' to disable interrupt for event END"]
    #[inline(always)]
    #[must_use]
    pub fn end(&mut self) -> EndW<IntenclrSpec> {
        EndW::new(self, 1)
    }
    #[doc = "Bit 4 - Write '1' to disable interrupt for event ENDRX"]
    #[inline(always)]
    #[must_use]
    pub fn endrx(&mut self) -> EndrxW<IntenclrSpec> {
        EndrxW::new(self, 4)
    }
    #[doc = "Bit 10 - Write '1' to disable interrupt for event ACQUIRED"]
    #[inline(always)]
    #[must_use]
    pub fn acquired(&mut self) -> AcquiredW<IntenclrSpec> {
        AcquiredW::new(self, 10)
    }
}
#[doc = "Disable interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intenclr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intenclr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenclrSpec;
impl crate::RegisterSpec for IntenclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intenclr::R`](R) reader structure"]
impl crate::Readable for IntenclrSpec {}
#[doc = "`write(|w| ..)` method takes [`intenclr::W`](W) writer structure"]
impl crate::Writable for IntenclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for IntenclrSpec {
    const RESET_VALUE: u32 = 0;
}
