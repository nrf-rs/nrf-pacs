#[doc = "Register `CLEARPATTERN` reader"]
pub type R = crate::R<ClearpatternSpec>;
#[doc = "Register `CLEARPATTERN` writer"]
pub type W = crate::W<ClearpatternSpec>;
#[doc = "Clears GPIO pattern array for antenna control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clearpattern {
    #[doc = "1: Clear the GPIO pattern"]
    Clear = 1,
}
impl From<Clearpattern> for bool {
    #[inline(always)]
    fn from(variant: Clearpattern) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLEARPATTERN` reader - Clears GPIO pattern array for antenna control"]
pub type ClearpatternR = crate::BitReader<Clearpattern>;
impl ClearpatternR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Clearpattern> {
        match self.bits {
            true => Some(Clearpattern::Clear),
            _ => None,
        }
    }
    #[doc = "Clear the GPIO pattern"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Clearpattern::Clear
    }
}
#[doc = "Field `CLEARPATTERN` writer - Clears GPIO pattern array for antenna control"]
pub type ClearpatternW<'a, REG> = crate::BitWriter1C<'a, REG, Clearpattern>;
impl<'a, REG> ClearpatternW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear the GPIO pattern"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Clearpattern::Clear)
    }
}
impl R {
    #[doc = "Bit 0 - Clears GPIO pattern array for antenna control"]
    #[inline(always)]
    pub fn clearpattern(&self) -> ClearpatternR {
        ClearpatternR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clears GPIO pattern array for antenna control"]
    #[inline(always)]
    pub fn clearpattern(&mut self) -> ClearpatternW<'_, ClearpatternSpec> {
        ClearpatternW::new(self, 0)
    }
}
#[doc = "Clear the GPIO pattern array for antenna control\n\nYou can [`read`](crate::Reg::read) this register and get [`clearpattern::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clearpattern::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClearpatternSpec;
impl crate::RegisterSpec for ClearpatternSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clearpattern::R`](R) reader structure"]
impl crate::Readable for ClearpatternSpec {}
#[doc = "`write(|w| ..)` method takes [`clearpattern::W`](W) writer structure"]
impl crate::Writable for ClearpatternSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x01;
}
#[doc = "`reset()` method sets CLEARPATTERN to value 0"]
impl crate::Resettable for ClearpatternSpec {}
