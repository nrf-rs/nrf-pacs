#[doc = "Register `ALIGN` reader"]
pub type R = crate::R<AlignSpec>;
#[doc = "Register `ALIGN` writer"]
pub type W = crate::W<AlignSpec>;
#[doc = "Alignment of sample within a frame\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Align {
    #[doc = "0: Left-aligned."]
    Left = 0,
    #[doc = "1: Right-aligned."]
    Right = 1,
}
impl From<Align> for bool {
    #[inline(always)]
    fn from(variant: Align) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALIGN` reader - Alignment of sample within a frame"]
pub type AlignR = crate::BitReader<Align>;
impl AlignR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Align {
        match self.bits {
            false => Align::Left,
            true => Align::Right,
        }
    }
    #[doc = "Left-aligned."]
    #[inline(always)]
    pub fn is_left(&self) -> bool {
        *self == Align::Left
    }
    #[doc = "Right-aligned."]
    #[inline(always)]
    pub fn is_right(&self) -> bool {
        *self == Align::Right
    }
}
#[doc = "Field `ALIGN` writer - Alignment of sample within a frame"]
pub type AlignW<'a, REG> = crate::BitWriter<'a, REG, Align>;
impl<'a, REG> AlignW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Left-aligned."]
    #[inline(always)]
    pub fn left(self) -> &'a mut crate::W<REG> {
        self.variant(Align::Left)
    }
    #[doc = "Right-aligned."]
    #[inline(always)]
    pub fn right(self) -> &'a mut crate::W<REG> {
        self.variant(Align::Right)
    }
}
impl R {
    #[doc = "Bit 0 - Alignment of sample within a frame"]
    #[inline(always)]
    pub fn align(&self) -> AlignR {
        AlignR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Alignment of sample within a frame"]
    #[inline(always)]
    pub fn align(&mut self) -> AlignW<'_, AlignSpec> {
        AlignW::new(self, 0)
    }
}
#[doc = "Alignment of sample within a frame\n\nYou can [`read`](crate::Reg::read) this register and get [`align::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`align::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AlignSpec;
impl crate::RegisterSpec for AlignSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`align::R`](R) reader structure"]
impl crate::Readable for AlignSpec {}
#[doc = "`write(|w| ..)` method takes [`align::W`](W) writer structure"]
impl crate::Writable for AlignSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ALIGN to value 0"]
impl crate::Resettable for AlignSpec {}
