#[doc = "Register `INTENSET` reader"]
pub type R = crate::R<IntensetSpec>;
#[doc = "Register `INTENSET` writer"]
pub type W = crate::W<IntensetSpec>;
#[doc = "Write '1' to enable interrupt for event HFCLKSTARTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hfclkstarted {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Hfclkstarted> for bool {
    #[inline(always)]
    fn from(variant: Hfclkstarted) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HFCLKSTARTED` reader - Write '1' to enable interrupt for event HFCLKSTARTED"]
pub type HfclkstartedR = crate::BitReader<Hfclkstarted>;
impl HfclkstartedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hfclkstarted {
        match self.bits {
            false => Hfclkstarted::Disabled,
            true => Hfclkstarted::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Hfclkstarted::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Hfclkstarted::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event HFCLKSTARTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HfclkstartedWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<HfclkstartedWO> for bool {
    #[inline(always)]
    fn from(variant: HfclkstartedWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HFCLKSTARTED` writer - Write '1' to enable interrupt for event HFCLKSTARTED"]
pub type HfclkstartedW<'a, REG> = crate::BitWriter<'a, REG, HfclkstartedWO>;
impl<'a, REG> HfclkstartedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(HfclkstartedWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event LFCLKSTARTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lfclkstarted {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Lfclkstarted> for bool {
    #[inline(always)]
    fn from(variant: Lfclkstarted) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LFCLKSTARTED` reader - Write '1' to enable interrupt for event LFCLKSTARTED"]
pub type LfclkstartedR = crate::BitReader<Lfclkstarted>;
impl LfclkstartedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lfclkstarted {
        match self.bits {
            false => Lfclkstarted::Disabled,
            true => Lfclkstarted::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Lfclkstarted::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Lfclkstarted::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event LFCLKSTARTED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LfclkstartedWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<LfclkstartedWO> for bool {
    #[inline(always)]
    fn from(variant: LfclkstartedWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LFCLKSTARTED` writer - Write '1' to enable interrupt for event LFCLKSTARTED"]
pub type LfclkstartedW<'a, REG> = crate::BitWriter<'a, REG, LfclkstartedWO>;
impl<'a, REG> LfclkstartedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(LfclkstartedWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event DONE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Done {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Done> for bool {
    #[inline(always)]
    fn from(variant: Done) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DONE` reader - Write '1' to enable interrupt for event DONE"]
pub type DoneR = crate::BitReader<Done>;
impl DoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Done {
        match self.bits {
            false => Done::Disabled,
            true => Done::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Done::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Done::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event DONE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DoneWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<DoneWO> for bool {
    #[inline(always)]
    fn from(variant: DoneWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DONE` writer - Write '1' to enable interrupt for event DONE"]
pub type DoneW<'a, REG> = crate::BitWriter<'a, REG, DoneWO>;
impl<'a, REG> DoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(DoneWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event CTTO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctto {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Ctto> for bool {
    #[inline(always)]
    fn from(variant: Ctto) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTTO` reader - Write '1' to enable interrupt for event CTTO"]
pub type CttoR = crate::BitReader<Ctto>;
impl CttoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctto {
        match self.bits {
            false => Ctto::Disabled,
            true => Ctto::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ctto::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ctto::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event CTTO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CttoWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<CttoWO> for bool {
    #[inline(always)]
    fn from(variant: CttoWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTTO` writer - Write '1' to enable interrupt for event CTTO"]
pub type CttoW<'a, REG> = crate::BitWriter<'a, REG, CttoWO>;
impl<'a, REG> CttoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(CttoWO::Set)
    }
}
impl R {
    #[doc = "Bit 0 - Write '1' to enable interrupt for event HFCLKSTARTED"]
    #[inline(always)]
    pub fn hfclkstarted(&self) -> HfclkstartedR {
        HfclkstartedR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write '1' to enable interrupt for event LFCLKSTARTED"]
    #[inline(always)]
    pub fn lfclkstarted(&self) -> LfclkstartedR {
        LfclkstartedR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Write '1' to enable interrupt for event DONE"]
    #[inline(always)]
    pub fn done(&self) -> DoneR {
        DoneR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write '1' to enable interrupt for event CTTO"]
    #[inline(always)]
    pub fn ctto(&self) -> CttoR {
        CttoR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write '1' to enable interrupt for event HFCLKSTARTED"]
    #[inline(always)]
    pub fn hfclkstarted(&mut self) -> HfclkstartedW<'_, IntensetSpec> {
        HfclkstartedW::new(self, 0)
    }
    #[doc = "Bit 1 - Write '1' to enable interrupt for event LFCLKSTARTED"]
    #[inline(always)]
    pub fn lfclkstarted(&mut self) -> LfclkstartedW<'_, IntensetSpec> {
        LfclkstartedW::new(self, 1)
    }
    #[doc = "Bit 3 - Write '1' to enable interrupt for event DONE"]
    #[inline(always)]
    pub fn done(&mut self) -> DoneW<'_, IntensetSpec> {
        DoneW::new(self, 3)
    }
    #[doc = "Bit 4 - Write '1' to enable interrupt for event CTTO"]
    #[inline(always)]
    pub fn ctto(&mut self) -> CttoW<'_, IntensetSpec> {
        CttoW::new(self, 4)
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
