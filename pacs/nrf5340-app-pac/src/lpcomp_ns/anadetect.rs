#[doc = "Register `ANADETECT` reader"]
pub type R = crate::R<AnadetectSpec>;
#[doc = "Register `ANADETECT` writer"]
pub type W = crate::W<AnadetectSpec>;
#[doc = "Analog detect configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Anadetect {
    #[doc = "0: Generate ANADETECT on crossing, both upward crossing and downward crossing"]
    Cross = 0,
    #[doc = "1: Generate ANADETECT on upward crossing only"]
    Up = 1,
    #[doc = "2: Generate ANADETECT on downward crossing only"]
    Down = 2,
}
impl From<Anadetect> for u8 {
    #[inline(always)]
    fn from(variant: Anadetect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Anadetect {
    type Ux = u8;
}
impl crate::IsEnum for Anadetect {}
#[doc = "Field `ANADETECT` reader - Analog detect configuration"]
pub type AnadetectR = crate::FieldReader<Anadetect>;
impl AnadetectR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Anadetect> {
        match self.bits {
            0 => Some(Anadetect::Cross),
            1 => Some(Anadetect::Up),
            2 => Some(Anadetect::Down),
            _ => None,
        }
    }
    #[doc = "Generate ANADETECT on crossing, both upward crossing and downward crossing"]
    #[inline(always)]
    pub fn is_cross(&self) -> bool {
        *self == Anadetect::Cross
    }
    #[doc = "Generate ANADETECT on upward crossing only"]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == Anadetect::Up
    }
    #[doc = "Generate ANADETECT on downward crossing only"]
    #[inline(always)]
    pub fn is_down(&self) -> bool {
        *self == Anadetect::Down
    }
}
#[doc = "Field `ANADETECT` writer - Analog detect configuration"]
pub type AnadetectW<'a, REG> = crate::FieldWriter<'a, REG, 2, Anadetect>;
impl<'a, REG> AnadetectW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Generate ANADETECT on crossing, both upward crossing and downward crossing"]
    #[inline(always)]
    pub fn cross(self) -> &'a mut crate::W<REG> {
        self.variant(Anadetect::Cross)
    }
    #[doc = "Generate ANADETECT on upward crossing only"]
    #[inline(always)]
    pub fn up(self) -> &'a mut crate::W<REG> {
        self.variant(Anadetect::Up)
    }
    #[doc = "Generate ANADETECT on downward crossing only"]
    #[inline(always)]
    pub fn down(self) -> &'a mut crate::W<REG> {
        self.variant(Anadetect::Down)
    }
}
impl R {
    #[doc = "Bits 0:1 - Analog detect configuration"]
    #[inline(always)]
    pub fn anadetect(&self) -> AnadetectR {
        AnadetectR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Analog detect configuration"]
    #[inline(always)]
    pub fn anadetect(&mut self) -> AnadetectW<'_, AnadetectSpec> {
        AnadetectW::new(self, 0)
    }
}
#[doc = "Analog detect configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`anadetect::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`anadetect::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnadetectSpec;
impl crate::RegisterSpec for AnadetectSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`anadetect::R`](R) reader structure"]
impl crate::Readable for AnadetectSpec {}
#[doc = "`write(|w| ..)` method takes [`anadetect::W`](W) writer structure"]
impl crate::Writable for AnadetectSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ANADETECT to value 0"]
impl crate::Resettable for AnadetectSpec {}
