#[doc = "Register `FORMAT` reader"]
pub type R = crate::R<FormatSpec>;
#[doc = "Register `FORMAT` writer"]
pub type W = crate::W<FormatSpec>;
#[doc = "Frame format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Format {
    #[doc = "0: Original I2S format."]
    I2s = 0,
    #[doc = "1: Alternate (left- or right-aligned) format."]
    Aligned = 1,
}
impl From<Format> for bool {
    #[inline(always)]
    fn from(variant: Format) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FORMAT` reader - Frame format"]
pub type FormatR = crate::BitReader<Format>;
impl FormatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Format {
        match self.bits {
            false => Format::I2s,
            true => Format::Aligned,
        }
    }
    #[doc = "Original I2S format."]
    #[inline(always)]
    pub fn is_i2s(&self) -> bool {
        *self == Format::I2s
    }
    #[doc = "Alternate (left- or right-aligned) format."]
    #[inline(always)]
    pub fn is_aligned(&self) -> bool {
        *self == Format::Aligned
    }
}
#[doc = "Field `FORMAT` writer - Frame format"]
pub type FormatW<'a, REG> = crate::BitWriter<'a, REG, Format>;
impl<'a, REG> FormatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Original I2S format."]
    #[inline(always)]
    pub fn i2s(self) -> &'a mut crate::W<REG> {
        self.variant(Format::I2s)
    }
    #[doc = "Alternate (left- or right-aligned) format."]
    #[inline(always)]
    pub fn aligned(self) -> &'a mut crate::W<REG> {
        self.variant(Format::Aligned)
    }
}
impl R {
    #[doc = "Bit 0 - Frame format"]
    #[inline(always)]
    pub fn format(&self) -> FormatR {
        FormatR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Frame format"]
    #[inline(always)]
    pub fn format(&mut self) -> FormatW<'_, FormatSpec> {
        FormatW::new(self, 0)
    }
}
#[doc = "Frame format\n\nYou can [`read`](crate::Reg::read) this register and get [`format::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`format::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FormatSpec;
impl crate::RegisterSpec for FormatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`format::R`](R) reader structure"]
impl crate::Readable for FormatSpec {}
#[doc = "`write(|w| ..)` method takes [`format::W`](W) writer structure"]
impl crate::Writable for FormatSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FORMAT to value 0"]
impl crate::Resettable for FormatSpec {}
