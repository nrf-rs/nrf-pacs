#[doc = "Register `PSELRTS` reader"]
pub type R = crate::R<PselrtsSpec>;
#[doc = "Register `PSELRTS` writer"]
pub type W = crate::W<PselrtsSpec>;
#[doc = "Pin number configuration for UART RTS signal\n\nValue on reset: 4294967295"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Pselrts {
    #[doc = "4294967295: Disconnect"]
    Disconnected = 4294967295,
}
impl From<Pselrts> for u32 {
    #[inline(always)]
    fn from(variant: Pselrts) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pselrts {
    type Ux = u32;
}
impl crate::IsEnum for Pselrts {}
#[doc = "Field `PSELRTS` reader - Pin number configuration for UART RTS signal"]
pub type PselrtsR = crate::FieldReader<Pselrts>;
impl PselrtsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pselrts> {
        match self.bits {
            4294967295 => Some(Pselrts::Disconnected),
            _ => None,
        }
    }
    #[doc = "Disconnect"]
    #[inline(always)]
    pub fn is_disconnected(&self) -> bool {
        *self == Pselrts::Disconnected
    }
}
#[doc = "Field `PSELRTS` writer - Pin number configuration for UART RTS signal"]
pub type PselrtsW<'a, REG> = crate::FieldWriter<'a, REG, 32, Pselrts>;
impl<'a, REG> PselrtsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Disconnect"]
    #[inline(always)]
    pub fn disconnected(self) -> &'a mut crate::W<REG> {
        self.variant(Pselrts::Disconnected)
    }
}
impl R {
    #[doc = "Bits 0:31 - Pin number configuration for UART RTS signal"]
    #[inline(always)]
    pub fn pselrts(&self) -> PselrtsR {
        PselrtsR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Pin number configuration for UART RTS signal"]
    #[inline(always)]
    pub fn pselrts(&mut self) -> PselrtsW<'_, PselrtsSpec> {
        PselrtsW::new(self, 0)
    }
}
#[doc = "Pin select for RTS\n\nYou can [`read`](crate::Reg::read) this register and get [`pselrts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pselrts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PselrtsSpec;
impl crate::RegisterSpec for PselrtsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pselrts::R`](R) reader structure"]
impl crate::Readable for PselrtsSpec {}
#[doc = "`write(|w| ..)` method takes [`pselrts::W`](W) writer structure"]
impl crate::Writable for PselrtsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSELRTS to value 0xffff_ffff"]
impl crate::Resettable for PselrtsSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
