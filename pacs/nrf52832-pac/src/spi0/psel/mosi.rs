#[doc = "Register `MOSI` reader"]
pub type R = crate::R<MosiSpec>;
#[doc = "Register `MOSI` writer"]
pub type W = crate::W<MosiSpec>;
#[doc = "Pin number configuration for SPI MOSI signal\n\nValue on reset: 4294967295"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Pselmosi {
    #[doc = "4294967295: Disconnect"]
    Disconnected = 4294967295,
}
impl From<Pselmosi> for u32 {
    #[inline(always)]
    fn from(variant: Pselmosi) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pselmosi {
    type Ux = u32;
}
impl crate::IsEnum for Pselmosi {}
#[doc = "Field `PSELMOSI` reader - Pin number configuration for SPI MOSI signal"]
pub type PselmosiR = crate::FieldReader<Pselmosi>;
impl PselmosiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pselmosi> {
        match self.bits {
            4294967295 => Some(Pselmosi::Disconnected),
            _ => None,
        }
    }
    #[doc = "Disconnect"]
    #[inline(always)]
    pub fn is_disconnected(&self) -> bool {
        *self == Pselmosi::Disconnected
    }
}
#[doc = "Field `PSELMOSI` writer - Pin number configuration for SPI MOSI signal"]
pub type PselmosiW<'a, REG> = crate::FieldWriter<'a, REG, 32, Pselmosi>;
impl<'a, REG> PselmosiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Disconnect"]
    #[inline(always)]
    pub fn disconnected(self) -> &'a mut crate::W<REG> {
        self.variant(Pselmosi::Disconnected)
    }
}
impl R {
    #[doc = "Bits 0:31 - Pin number configuration for SPI MOSI signal"]
    #[inline(always)]
    pub fn pselmosi(&self) -> PselmosiR {
        PselmosiR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Pin number configuration for SPI MOSI signal"]
    #[inline(always)]
    pub fn pselmosi(&mut self) -> PselmosiW<'_, MosiSpec> {
        PselmosiW::new(self, 0)
    }
}
#[doc = "Pin select for MOSI\n\nYou can [`read`](crate::Reg::read) this register and get [`mosi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mosi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MosiSpec;
impl crate::RegisterSpec for MosiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mosi::R`](R) reader structure"]
impl crate::Readable for MosiSpec {}
#[doc = "`write(|w| ..)` method takes [`mosi::W`](W) writer structure"]
impl crate::Writable for MosiSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MOSI to value 0xffff_ffff"]
impl crate::Resettable for MosiSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
