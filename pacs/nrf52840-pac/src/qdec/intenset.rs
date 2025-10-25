#[doc = "Register `INTENSET` reader"]
pub type R = crate::R<IntensetSpec>;
#[doc = "Register `INTENSET` writer"]
pub type W = crate::W<IntensetSpec>;
#[doc = "Write '1' to enable interrupt for event SAMPLERDY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Samplerdy {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Samplerdy> for bool {
    #[inline(always)]
    fn from(variant: Samplerdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SAMPLERDY` reader - Write '1' to enable interrupt for event SAMPLERDY"]
pub type SamplerdyR = crate::BitReader<Samplerdy>;
impl SamplerdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Samplerdy {
        match self.bits {
            false => Samplerdy::Disabled,
            true => Samplerdy::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Samplerdy::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Samplerdy::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event SAMPLERDY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SamplerdyWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<SamplerdyWO> for bool {
    #[inline(always)]
    fn from(variant: SamplerdyWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SAMPLERDY` writer - Write '1' to enable interrupt for event SAMPLERDY"]
pub type SamplerdyW<'a, REG> = crate::BitWriter<'a, REG, SamplerdyWO>;
impl<'a, REG> SamplerdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(SamplerdyWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event REPORTRDY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reportrdy {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Reportrdy> for bool {
    #[inline(always)]
    fn from(variant: Reportrdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REPORTRDY` reader - Write '1' to enable interrupt for event REPORTRDY"]
pub type ReportrdyR = crate::BitReader<Reportrdy>;
impl ReportrdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Reportrdy {
        match self.bits {
            false => Reportrdy::Disabled,
            true => Reportrdy::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Reportrdy::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Reportrdy::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event REPORTRDY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReportrdyWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<ReportrdyWO> for bool {
    #[inline(always)]
    fn from(variant: ReportrdyWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REPORTRDY` writer - Write '1' to enable interrupt for event REPORTRDY"]
pub type ReportrdyW<'a, REG> = crate::BitWriter<'a, REG, ReportrdyWO>;
impl<'a, REG> ReportrdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(ReportrdyWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event ACCOF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Accof {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Accof> for bool {
    #[inline(always)]
    fn from(variant: Accof) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACCOF` reader - Write '1' to enable interrupt for event ACCOF"]
pub type AccofR = crate::BitReader<Accof>;
impl AccofR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Accof {
        match self.bits {
            false => Accof::Disabled,
            true => Accof::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Accof::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Accof::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event ACCOF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AccofWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<AccofWO> for bool {
    #[inline(always)]
    fn from(variant: AccofWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACCOF` writer - Write '1' to enable interrupt for event ACCOF"]
pub type AccofW<'a, REG> = crate::BitWriter<'a, REG, AccofWO>;
impl<'a, REG> AccofW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(AccofWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event DBLRDY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dblrdy {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Dblrdy> for bool {
    #[inline(always)]
    fn from(variant: Dblrdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBLRDY` reader - Write '1' to enable interrupt for event DBLRDY"]
pub type DblrdyR = crate::BitReader<Dblrdy>;
impl DblrdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dblrdy {
        match self.bits {
            false => Dblrdy::Disabled,
            true => Dblrdy::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dblrdy::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dblrdy::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event DBLRDY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DblrdyWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<DblrdyWO> for bool {
    #[inline(always)]
    fn from(variant: DblrdyWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBLRDY` writer - Write '1' to enable interrupt for event DBLRDY"]
pub type DblrdyW<'a, REG> = crate::BitWriter<'a, REG, DblrdyWO>;
impl<'a, REG> DblrdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(DblrdyWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event STOPPED\n\nValue on reset: 0"]
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
#[doc = "Field `STOPPED` reader - Write '1' to enable interrupt for event STOPPED"]
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
#[doc = "Write '1' to enable interrupt for event STOPPED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StoppedWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<StoppedWO> for bool {
    #[inline(always)]
    fn from(variant: StoppedWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOPPED` writer - Write '1' to enable interrupt for event STOPPED"]
pub type StoppedW<'a, REG> = crate::BitWriter<'a, REG, StoppedWO>;
impl<'a, REG> StoppedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(StoppedWO::Set)
    }
}
impl R {
    #[doc = "Bit 0 - Write '1' to enable interrupt for event SAMPLERDY"]
    #[inline(always)]
    pub fn samplerdy(&self) -> SamplerdyR {
        SamplerdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write '1' to enable interrupt for event REPORTRDY"]
    #[inline(always)]
    pub fn reportrdy(&self) -> ReportrdyR {
        ReportrdyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write '1' to enable interrupt for event ACCOF"]
    #[inline(always)]
    pub fn accof(&self) -> AccofR {
        AccofR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write '1' to enable interrupt for event DBLRDY"]
    #[inline(always)]
    pub fn dblrdy(&self) -> DblrdyR {
        DblrdyR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write '1' to enable interrupt for event STOPPED"]
    #[inline(always)]
    pub fn stopped(&self) -> StoppedR {
        StoppedR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write '1' to enable interrupt for event SAMPLERDY"]
    #[inline(always)]
    pub fn samplerdy(&mut self) -> SamplerdyW<'_, IntensetSpec> {
        SamplerdyW::new(self, 0)
    }
    #[doc = "Bit 1 - Write '1' to enable interrupt for event REPORTRDY"]
    #[inline(always)]
    pub fn reportrdy(&mut self) -> ReportrdyW<'_, IntensetSpec> {
        ReportrdyW::new(self, 1)
    }
    #[doc = "Bit 2 - Write '1' to enable interrupt for event ACCOF"]
    #[inline(always)]
    pub fn accof(&mut self) -> AccofW<'_, IntensetSpec> {
        AccofW::new(self, 2)
    }
    #[doc = "Bit 3 - Write '1' to enable interrupt for event DBLRDY"]
    #[inline(always)]
    pub fn dblrdy(&mut self) -> DblrdyW<'_, IntensetSpec> {
        DblrdyW::new(self, 3)
    }
    #[doc = "Bit 4 - Write '1' to enable interrupt for event STOPPED"]
    #[inline(always)]
    pub fn stopped(&mut self) -> StoppedW<'_, IntensetSpec> {
        StoppedW::new(self, 4)
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
