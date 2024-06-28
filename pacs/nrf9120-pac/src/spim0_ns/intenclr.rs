#[doc = "Register `INTENCLR` reader"]
pub type R = crate::R<IntenclrSpec>;
#[doc = "Register `INTENCLR` writer"]
pub type W = crate::W<IntenclrSpec>;
#[doc = "Write '1' to disable interrupt for event STOPPED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stopped {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Stopped> for bool {
    #[inline(always)]
    fn from(variant: Stopped) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOPPED` reader - Write '1' to disable interrupt for event STOPPED"]
pub type StoppedR = crate::BitReader<Stopped>;
impl StoppedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stopped {
        match self.bits {
            false => Stopped::Disabled,
            true => Stopped::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Stopped::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Stopped::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event STOPPED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StoppedWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<StoppedWO> for bool {
    #[inline(always)]
    fn from(variant: StoppedWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOPPED` writer - Write '1' to disable interrupt for event STOPPED"]
pub type StoppedW<'a, REG> = crate::BitWriter<'a, REG, StoppedWO>;
impl<'a, REG> StoppedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(StoppedWO::Clear)
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
#[doc = "Write '1' to disable interrupt for event ENDTX\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Endtx {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Endtx> for bool {
    #[inline(always)]
    fn from(variant: Endtx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDTX` reader - Write '1' to disable interrupt for event ENDTX"]
pub type EndtxR = crate::BitReader<Endtx>;
impl EndtxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Endtx {
        match self.bits {
            false => Endtx::Disabled,
            true => Endtx::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Endtx::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Endtx::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event ENDTX\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EndtxWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<EndtxWO> for bool {
    #[inline(always)]
    fn from(variant: EndtxWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDTX` writer - Write '1' to disable interrupt for event ENDTX"]
pub type EndtxW<'a, REG> = crate::BitWriter<'a, REG, EndtxWO>;
impl<'a, REG> EndtxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(EndtxWO::Clear)
    }
}
#[doc = "Write '1' to disable interrupt for event STARTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Started {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Started> for bool {
    #[inline(always)]
    fn from(variant: Started) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STARTED` reader - Write '1' to disable interrupt for event STARTED"]
pub type StartedR = crate::BitReader<Started>;
impl StartedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Started {
        match self.bits {
            false => Started::Disabled,
            true => Started::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Started::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Started::Enabled
    }
}
#[doc = "Write '1' to disable interrupt for event STARTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StartedWO {
    #[doc = "1: Disable"]
    Clear = 1,
}
impl From<StartedWO> for bool {
    #[inline(always)]
    fn from(variant: StartedWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STARTED` writer - Write '1' to disable interrupt for event STARTED"]
pub type StartedW<'a, REG> = crate::BitWriter<'a, REG, StartedWO>;
impl<'a, REG> StartedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(StartedWO::Clear)
    }
}
impl R {
    #[doc = "Bit 1 - Write '1' to disable interrupt for event STOPPED"]
    #[inline(always)]
    pub fn stopped(&self) -> StoppedR {
        StoppedR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Write '1' to disable interrupt for event ENDRX"]
    #[inline(always)]
    pub fn endrx(&self) -> EndrxR {
        EndrxR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Write '1' to disable interrupt for event END"]
    #[inline(always)]
    pub fn end(&self) -> EndR {
        EndR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Write '1' to disable interrupt for event ENDTX"]
    #[inline(always)]
    pub fn endtx(&self) -> EndtxR {
        EndtxR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 19 - Write '1' to disable interrupt for event STARTED"]
    #[inline(always)]
    pub fn started(&self) -> StartedR {
        StartedR::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Write '1' to disable interrupt for event STOPPED"]
    #[inline(always)]
    #[must_use]
    pub fn stopped(&mut self) -> StoppedW<IntenclrSpec> {
        StoppedW::new(self, 1)
    }
    #[doc = "Bit 4 - Write '1' to disable interrupt for event ENDRX"]
    #[inline(always)]
    #[must_use]
    pub fn endrx(&mut self) -> EndrxW<IntenclrSpec> {
        EndrxW::new(self, 4)
    }
    #[doc = "Bit 6 - Write '1' to disable interrupt for event END"]
    #[inline(always)]
    #[must_use]
    pub fn end(&mut self) -> EndW<IntenclrSpec> {
        EndW::new(self, 6)
    }
    #[doc = "Bit 8 - Write '1' to disable interrupt for event ENDTX"]
    #[inline(always)]
    #[must_use]
    pub fn endtx(&mut self) -> EndtxW<IntenclrSpec> {
        EndtxW::new(self, 8)
    }
    #[doc = "Bit 19 - Write '1' to disable interrupt for event STARTED"]
    #[inline(always)]
    #[must_use]
    pub fn started(&mut self) -> StartedW<IntenclrSpec> {
        StartedW::new(self, 19)
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
