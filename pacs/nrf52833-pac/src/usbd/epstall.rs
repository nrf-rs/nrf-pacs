#[doc = "Register `EPSTALL` writer"]
pub type W = crate::W<EpstallSpec>;
#[doc = "Field `EP` writer - Select endpoint number"]
pub type EpW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Selects IN or OUT endpoint\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Io {
    #[doc = "0: Selects OUT endpoint"]
    Out = 0,
    #[doc = "1: Selects IN endpoint"]
    In = 1,
}
impl From<Io> for bool {
    #[inline(always)]
    fn from(variant: Io) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IO` writer - Selects IN or OUT endpoint"]
pub type IoW<'a, REG> = crate::BitWriter<'a, REG, Io>;
impl<'a, REG> IoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Selects OUT endpoint"]
    #[inline(always)]
    pub fn out(self) -> &'a mut crate::W<REG> {
        self.variant(Io::Out)
    }
    #[doc = "Selects IN endpoint"]
    #[inline(always)]
    pub fn in_(self) -> &'a mut crate::W<REG> {
        self.variant(Io::In)
    }
}
#[doc = "Stall selected endpoint\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stall {
    #[doc = "0: Don't stall selected endpoint"]
    UnStall = 0,
    #[doc = "1: Stall selected endpoint"]
    Stall = 1,
}
impl From<Stall> for bool {
    #[inline(always)]
    fn from(variant: Stall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STALL` writer - Stall selected endpoint"]
pub type StallW<'a, REG> = crate::BitWriter<'a, REG, Stall>;
impl<'a, REG> StallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Don't stall selected endpoint"]
    #[inline(always)]
    pub fn un_stall(self) -> &'a mut crate::W<REG> {
        self.variant(Stall::UnStall)
    }
    #[doc = "Stall selected endpoint"]
    #[inline(always)]
    pub fn stall(self) -> &'a mut crate::W<REG> {
        self.variant(Stall::Stall)
    }
}
impl W {
    #[doc = "Bits 0:2 - Select endpoint number"]
    #[inline(always)]
    pub fn ep(&mut self) -> EpW<'_, EpstallSpec> {
        EpW::new(self, 0)
    }
    #[doc = "Bit 7 - Selects IN or OUT endpoint"]
    #[inline(always)]
    pub fn io(&mut self) -> IoW<'_, EpstallSpec> {
        IoW::new(self, 7)
    }
    #[doc = "Bit 8 - Stall selected endpoint"]
    #[inline(always)]
    pub fn stall(&mut self) -> StallW<'_, EpstallSpec> {
        StallW::new(self, 8)
    }
}
#[doc = "STALL endpoints\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`epstall::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EpstallSpec;
impl crate::RegisterSpec for EpstallSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`epstall::W`](W) writer structure"]
impl crate::Writable for EpstallSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EPSTALL to value 0"]
impl crate::Resettable for EpstallSpec {}
