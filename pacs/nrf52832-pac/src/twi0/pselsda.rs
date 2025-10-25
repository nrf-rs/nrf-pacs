#[doc = "Register `PSELSDA` reader"]
pub type R = crate::R<PselsdaSpec>;
#[doc = "Register `PSELSDA` writer"]
pub type W = crate::W<PselsdaSpec>;
#[doc = "Pin number configuration for TWI SDA signal\n\nValue on reset: 4294967295"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Pselsda {
    #[doc = "4294967295: Disconnect"]
    Disconnected = 4294967295,
}
impl From<Pselsda> for u32 {
    #[inline(always)]
    fn from(variant: Pselsda) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pselsda {
    type Ux = u32;
}
impl crate::IsEnum for Pselsda {}
#[doc = "Field `PSELSDA` reader - Pin number configuration for TWI SDA signal"]
pub type PselsdaR = crate::FieldReader<Pselsda>;
impl PselsdaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pselsda> {
        match self.bits {
            4294967295 => Some(Pselsda::Disconnected),
            _ => None,
        }
    }
    #[doc = "Disconnect"]
    #[inline(always)]
    pub fn is_disconnected(&self) -> bool {
        *self == Pselsda::Disconnected
    }
}
#[doc = "Field `PSELSDA` writer - Pin number configuration for TWI SDA signal"]
pub type PselsdaW<'a, REG> = crate::FieldWriter<'a, REG, 32, Pselsda>;
impl<'a, REG> PselsdaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Disconnect"]
    #[inline(always)]
    pub fn disconnected(self) -> &'a mut crate::W<REG> {
        self.variant(Pselsda::Disconnected)
    }
}
impl R {
    #[doc = "Bits 0:31 - Pin number configuration for TWI SDA signal"]
    #[inline(always)]
    pub fn pselsda(&self) -> PselsdaR {
        PselsdaR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Pin number configuration for TWI SDA signal"]
    #[inline(always)]
    pub fn pselsda(&mut self) -> PselsdaW<'_, PselsdaSpec> {
        PselsdaW::new(self, 0)
    }
}
#[doc = "Pin select for SDA\n\nYou can [`read`](crate::Reg::read) this register and get [`pselsda::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pselsda::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PselsdaSpec;
impl crate::RegisterSpec for PselsdaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pselsda::R`](R) reader structure"]
impl crate::Readable for PselsdaSpec {}
#[doc = "`write(|w| ..)` method takes [`pselsda::W`](W) writer structure"]
impl crate::Writable for PselsdaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSELSDA to value 0xffff_ffff"]
impl crate::Resettable for PselsdaSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
