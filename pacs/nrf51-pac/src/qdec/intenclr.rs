#[doc = "Register `INTENCLR` reader"]
pub type R = crate::R<IntenclrSpec>;
#[doc = "Register `INTENCLR` writer"]
pub type W = crate::W<IntenclrSpec>;
#[doc = "Disable interrupt on SAMPLERDY event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Samplerdy {
    #[doc = "0: Interrupt disabled."]
    Disabled = 0,
    #[doc = "1: Interrupt enabled."]
    Enabled = 1,
}
impl From<Samplerdy> for bool {
    #[inline(always)]
    fn from(variant: Samplerdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SAMPLERDY` reader - Disable interrupt on SAMPLERDY event."]
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
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Samplerdy::Disabled
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Samplerdy::Enabled
    }
}
#[doc = "Disable interrupt on SAMPLERDY event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SamplerdyWO {
    #[doc = "1: Disable interrupt on write."]
    Clear = 1,
}
impl From<SamplerdyWO> for bool {
    #[inline(always)]
    fn from(variant: SamplerdyWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SAMPLERDY` writer - Disable interrupt on SAMPLERDY event."]
pub type SamplerdyW<'a, REG> = crate::BitWriter<'a, REG, SamplerdyWO>;
impl<'a, REG> SamplerdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable interrupt on write."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(SamplerdyWO::Clear)
    }
}
#[doc = "Disable interrupt on REPORTRDY event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reportrdy {
    #[doc = "0: Interrupt disabled."]
    Disabled = 0,
    #[doc = "1: Interrupt enabled."]
    Enabled = 1,
}
impl From<Reportrdy> for bool {
    #[inline(always)]
    fn from(variant: Reportrdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REPORTRDY` reader - Disable interrupt on REPORTRDY event."]
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
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Reportrdy::Disabled
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Reportrdy::Enabled
    }
}
#[doc = "Disable interrupt on REPORTRDY event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReportrdyWO {
    #[doc = "1: Disable interrupt on write."]
    Clear = 1,
}
impl From<ReportrdyWO> for bool {
    #[inline(always)]
    fn from(variant: ReportrdyWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REPORTRDY` writer - Disable interrupt on REPORTRDY event."]
pub type ReportrdyW<'a, REG> = crate::BitWriter<'a, REG, ReportrdyWO>;
impl<'a, REG> ReportrdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable interrupt on write."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(ReportrdyWO::Clear)
    }
}
#[doc = "Disable interrupt on ACCOF event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Accof {
    #[doc = "0: Interrupt disabled."]
    Disabled = 0,
    #[doc = "1: Interrupt enabled."]
    Enabled = 1,
}
impl From<Accof> for bool {
    #[inline(always)]
    fn from(variant: Accof) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACCOF` reader - Disable interrupt on ACCOF event."]
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
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Accof::Disabled
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Accof::Enabled
    }
}
#[doc = "Disable interrupt on ACCOF event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AccofWO {
    #[doc = "1: Disable interrupt on write."]
    Clear = 1,
}
impl From<AccofWO> for bool {
    #[inline(always)]
    fn from(variant: AccofWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACCOF` writer - Disable interrupt on ACCOF event."]
pub type AccofW<'a, REG> = crate::BitWriter<'a, REG, AccofWO>;
impl<'a, REG> AccofW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable interrupt on write."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(AccofWO::Clear)
    }
}
impl R {
    #[doc = "Bit 0 - Disable interrupt on SAMPLERDY event."]
    #[inline(always)]
    pub fn samplerdy(&self) -> SamplerdyR {
        SamplerdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Disable interrupt on REPORTRDY event."]
    #[inline(always)]
    pub fn reportrdy(&self) -> ReportrdyR {
        ReportrdyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Disable interrupt on ACCOF event."]
    #[inline(always)]
    pub fn accof(&self) -> AccofR {
        AccofR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Disable interrupt on SAMPLERDY event."]
    #[inline(always)]
    pub fn samplerdy(&mut self) -> SamplerdyW<'_, IntenclrSpec> {
        SamplerdyW::new(self, 0)
    }
    #[doc = "Bit 1 - Disable interrupt on REPORTRDY event."]
    #[inline(always)]
    pub fn reportrdy(&mut self) -> ReportrdyW<'_, IntenclrSpec> {
        ReportrdyW::new(self, 1)
    }
    #[doc = "Bit 2 - Disable interrupt on ACCOF event."]
    #[inline(always)]
    pub fn accof(&mut self) -> AccofW<'_, IntenclrSpec> {
        AccofW::new(self, 2)
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
