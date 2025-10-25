#[doc = "Register `LFRCMODE` reader"]
pub type R = crate::R<LfrcmodeSpec>;
#[doc = "Register `LFRCMODE` writer"]
pub type W = crate::W<LfrcmodeSpec>;
#[doc = "Set LFRC mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    #[doc = "0: Normal mode"]
    Normal = 0,
    #[doc = "1: Ultra-low power mode (ULP)"]
    Ulp = 1,
}
impl From<Mode> for bool {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODE` reader - Set LFRC mode"]
pub type ModeR = crate::BitReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode {
        match self.bits {
            false => Mode::Normal,
            true => Mode::Ulp,
        }
    }
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Mode::Normal
    }
    #[doc = "Ultra-low power mode (ULP)"]
    #[inline(always)]
    pub fn is_ulp(&self) -> bool {
        *self == Mode::Ulp
    }
}
#[doc = "Field `MODE` writer - Set LFRC mode"]
pub type ModeW<'a, REG> = crate::BitWriter<'a, REG, Mode>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Normal)
    }
    #[doc = "Ultra-low power mode (ULP)"]
    #[inline(always)]
    pub fn ulp(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Ulp)
    }
}
#[doc = "Active LFRC mode. This field is read only.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    #[doc = "0: Normal mode"]
    Normal = 0,
    #[doc = "1: Ultra-low power mode (ULP)"]
    Ulp = 1,
}
impl From<Status> for bool {
    #[inline(always)]
    fn from(variant: Status) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STATUS` reader - Active LFRC mode. This field is read only."]
pub type StatusR = crate::BitReader<Status>;
impl StatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Status {
        match self.bits {
            false => Status::Normal,
            true => Status::Ulp,
        }
    }
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Status::Normal
    }
    #[doc = "Ultra-low power mode (ULP)"]
    #[inline(always)]
    pub fn is_ulp(&self) -> bool {
        *self == Status::Ulp
    }
}
#[doc = "Field `STATUS` writer - Active LFRC mode. This field is read only."]
pub type StatusW<'a, REG> = crate::BitWriter<'a, REG, Status>;
impl<'a, REG> StatusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Status::Normal)
    }
    #[doc = "Ultra-low power mode (ULP)"]
    #[inline(always)]
    pub fn ulp(self) -> &'a mut crate::W<REG> {
        self.variant(Status::Ulp)
    }
}
impl R {
    #[doc = "Bit 0 - Set LFRC mode"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - Active LFRC mode. This field is read only."]
    #[inline(always)]
    pub fn status(&self) -> StatusR {
        StatusR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set LFRC mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, LfrcmodeSpec> {
        ModeW::new(self, 0)
    }
    #[doc = "Bit 16 - Active LFRC mode. This field is read only."]
    #[inline(always)]
    pub fn status(&mut self) -> StatusW<'_, LfrcmodeSpec> {
        StatusW::new(self, 16)
    }
}
#[doc = "LFRC mode configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`lfrcmode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfrcmode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfrcmodeSpec;
impl crate::RegisterSpec for LfrcmodeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfrcmode::R`](R) reader structure"]
impl crate::Readable for LfrcmodeSpec {}
#[doc = "`write(|w| ..)` method takes [`lfrcmode::W`](W) writer structure"]
impl crate::Writable for LfrcmodeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LFRCMODE to value 0"]
impl crate::Resettable for LfrcmodeSpec {}
