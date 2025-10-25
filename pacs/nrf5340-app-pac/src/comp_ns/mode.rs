#[doc = "Register `MODE` reader"]
pub type R = crate::R<ModeSpec>;
#[doc = "Register `MODE` writer"]
pub type W = crate::W<ModeSpec>;
#[doc = "Speed and power modes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sp {
    #[doc = "0: Low-power mode"]
    Low = 0,
    #[doc = "1: Normal mode"]
    Normal = 1,
    #[doc = "2: High-speed mode"]
    High = 2,
}
impl From<Sp> for u8 {
    #[inline(always)]
    fn from(variant: Sp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sp {
    type Ux = u8;
}
impl crate::IsEnum for Sp {}
#[doc = "Field `SP` reader - Speed and power modes"]
pub type SpR = crate::FieldReader<Sp>;
impl SpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sp> {
        match self.bits {
            0 => Some(Sp::Low),
            1 => Some(Sp::Normal),
            2 => Some(Sp::High),
            _ => None,
        }
    }
    #[doc = "Low-power mode"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Sp::Low
    }
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Sp::Normal
    }
    #[doc = "High-speed mode"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Sp::High
    }
}
#[doc = "Field `SP` writer - Speed and power modes"]
pub type SpW<'a, REG> = crate::FieldWriter<'a, REG, 2, Sp>;
impl<'a, REG> SpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low-power mode"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Sp::Low)
    }
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Sp::Normal)
    }
    #[doc = "High-speed mode"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Sp::High)
    }
}
#[doc = "Main operation modes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Main {
    #[doc = "0: Single-ended mode"]
    Se = 0,
    #[doc = "1: Differential mode"]
    Diff = 1,
}
impl From<Main> for bool {
    #[inline(always)]
    fn from(variant: Main) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MAIN` reader - Main operation modes"]
pub type MainR = crate::BitReader<Main>;
impl MainR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Main {
        match self.bits {
            false => Main::Se,
            true => Main::Diff,
        }
    }
    #[doc = "Single-ended mode"]
    #[inline(always)]
    pub fn is_se(&self) -> bool {
        *self == Main::Se
    }
    #[doc = "Differential mode"]
    #[inline(always)]
    pub fn is_diff(&self) -> bool {
        *self == Main::Diff
    }
}
#[doc = "Field `MAIN` writer - Main operation modes"]
pub type MainW<'a, REG> = crate::BitWriter<'a, REG, Main>;
impl<'a, REG> MainW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Single-ended mode"]
    #[inline(always)]
    pub fn se(self) -> &'a mut crate::W<REG> {
        self.variant(Main::Se)
    }
    #[doc = "Differential mode"]
    #[inline(always)]
    pub fn diff(self) -> &'a mut crate::W<REG> {
        self.variant(Main::Diff)
    }
}
impl R {
    #[doc = "Bits 0:1 - Speed and power modes"]
    #[inline(always)]
    pub fn sp(&self) -> SpR {
        SpR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 8 - Main operation modes"]
    #[inline(always)]
    pub fn main(&self) -> MainR {
        MainR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Speed and power modes"]
    #[inline(always)]
    pub fn sp(&mut self) -> SpW<'_, ModeSpec> {
        SpW::new(self, 0)
    }
    #[doc = "Bit 8 - Main operation modes"]
    #[inline(always)]
    pub fn main(&mut self) -> MainW<'_, ModeSpec> {
        MainW::new(self, 8)
    }
}
#[doc = "Mode configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ModeSpec;
impl crate::RegisterSpec for ModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mode::R`](R) reader structure"]
impl crate::Readable for ModeSpec {}
#[doc = "`write(|w| ..)` method takes [`mode::W`](W) writer structure"]
impl crate::Writable for ModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MODE to value 0"]
impl crate::Resettable for ModeSpec {}
