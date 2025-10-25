#[doc = "Register `PSELSCL` reader"]
pub type R = crate::R<PselsclSpec>;
#[doc = "Register `PSELSCL` writer"]
pub type W = crate::W<PselsclSpec>;
#[doc = "Pin number configuration for TWI SCL signal\n\nValue on reset: 4294967295"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Pselscl {
    #[doc = "4294967295: Disconnect"]
    Disconnected = 4294967295,
}
impl From<Pselscl> for u32 {
    #[inline(always)]
    fn from(variant: Pselscl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pselscl {
    type Ux = u32;
}
impl crate::IsEnum for Pselscl {}
#[doc = "Field `PSELSCL` reader - Pin number configuration for TWI SCL signal"]
pub type PselsclR = crate::FieldReader<Pselscl>;
impl PselsclR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pselscl> {
        match self.bits {
            4294967295 => Some(Pselscl::Disconnected),
            _ => None,
        }
    }
    #[doc = "Disconnect"]
    #[inline(always)]
    pub fn is_disconnected(&self) -> bool {
        *self == Pselscl::Disconnected
    }
}
#[doc = "Field `PSELSCL` writer - Pin number configuration for TWI SCL signal"]
pub type PselsclW<'a, REG> = crate::FieldWriter<'a, REG, 32, Pselscl>;
impl<'a, REG> PselsclW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Disconnect"]
    #[inline(always)]
    pub fn disconnected(self) -> &'a mut crate::W<REG> {
        self.variant(Pselscl::Disconnected)
    }
}
impl R {
    #[doc = "Bits 0:31 - Pin number configuration for TWI SCL signal"]
    #[inline(always)]
    pub fn pselscl(&self) -> PselsclR {
        PselsclR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Pin number configuration for TWI SCL signal"]
    #[inline(always)]
    pub fn pselscl(&mut self) -> PselsclW<'_, PselsclSpec> {
        PselsclW::new(self, 0)
    }
}
#[doc = "Pin select for SCL\n\nYou can [`read`](crate::Reg::read) this register and get [`pselscl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pselscl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PselsclSpec;
impl crate::RegisterSpec for PselsclSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pselscl::R`](R) reader structure"]
impl crate::Readable for PselsclSpec {}
#[doc = "`write(|w| ..)` method takes [`pselscl::W`](W) writer structure"]
impl crate::Writable for PselsclSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSELSCL to value 0xffff_ffff"]
impl crate::Resettable for PselsclSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
