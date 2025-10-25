#[doc = "Register `PSELCTS` reader"]
pub type R = crate::R<PselctsSpec>;
#[doc = "Register `PSELCTS` writer"]
pub type W = crate::W<PselctsSpec>;
#[doc = "Pin number configuration for UART CTS signal\n\nValue on reset: 4294967295"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Pselcts {
    #[doc = "4294967295: Disconnect"]
    Disconnected = 4294967295,
}
impl From<Pselcts> for u32 {
    #[inline(always)]
    fn from(variant: Pselcts) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pselcts {
    type Ux = u32;
}
impl crate::IsEnum for Pselcts {}
#[doc = "Field `PSELCTS` reader - Pin number configuration for UART CTS signal"]
pub type PselctsR = crate::FieldReader<Pselcts>;
impl PselctsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pselcts> {
        match self.bits {
            4294967295 => Some(Pselcts::Disconnected),
            _ => None,
        }
    }
    #[doc = "Disconnect"]
    #[inline(always)]
    pub fn is_disconnected(&self) -> bool {
        *self == Pselcts::Disconnected
    }
}
#[doc = "Field `PSELCTS` writer - Pin number configuration for UART CTS signal"]
pub type PselctsW<'a, REG> = crate::FieldWriter<'a, REG, 32, Pselcts>;
impl<'a, REG> PselctsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Disconnect"]
    #[inline(always)]
    pub fn disconnected(self) -> &'a mut crate::W<REG> {
        self.variant(Pselcts::Disconnected)
    }
}
impl R {
    #[doc = "Bits 0:31 - Pin number configuration for UART CTS signal"]
    #[inline(always)]
    pub fn pselcts(&self) -> PselctsR {
        PselctsR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Pin number configuration for UART CTS signal"]
    #[inline(always)]
    pub fn pselcts(&mut self) -> PselctsW<'_, PselctsSpec> {
        PselctsW::new(self, 0)
    }
}
#[doc = "Pin select for CTS\n\nYou can [`read`](crate::Reg::read) this register and get [`pselcts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pselcts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PselctsSpec;
impl crate::RegisterSpec for PselctsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pselcts::R`](R) reader structure"]
impl crate::Readable for PselctsSpec {}
#[doc = "`write(|w| ..)` method takes [`pselcts::W`](W) writer structure"]
impl crate::Writable for PselctsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSELCTS to value 0xffff_ffff"]
impl crate::Resettable for PselctsSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
