#[doc = "Register `CONFIG` reader"]
pub type R = crate::R<ConfigSpec>;
#[doc = "Register `CONFIG` writer"]
pub type W = crate::W<ConfigSpec>;
#[doc = "Bias correction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dercen {
    #[doc = "0: Disabled"]
    Disabled = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<Dercen> for bool {
    #[inline(always)]
    fn from(variant: Dercen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DERCEN` reader - Bias correction"]
pub type DercenR = crate::BitReader<Dercen>;
impl DercenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dercen {
        match self.bits {
            false => Dercen::Disabled,
            true => Dercen::Enabled,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dercen::Disabled
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dercen::Enabled
    }
}
#[doc = "Field `DERCEN` writer - Bias correction"]
pub type DercenW<'a, REG> = crate::BitWriter<'a, REG, Dercen>;
impl<'a, REG> DercenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dercen::Disabled)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dercen::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Bias correction"]
    #[inline(always)]
    pub fn dercen(&self) -> DercenR {
        DercenR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bias correction"]
    #[inline(always)]
    pub fn dercen(&mut self) -> DercenW<'_, ConfigSpec> {
        DercenW::new(self, 0)
    }
}
#[doc = "Configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfigSpec;
impl crate::RegisterSpec for ConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config::R`](R) reader structure"]
impl crate::Readable for ConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`config::W`](W) writer structure"]
impl crate::Writable for ConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONFIG to value 0"]
impl crate::Resettable for ConfigSpec {}
