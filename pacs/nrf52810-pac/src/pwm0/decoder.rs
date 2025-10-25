#[doc = "Register `DECODER` reader"]
pub type R = crate::R<DecoderSpec>;
#[doc = "Register `DECODER` writer"]
pub type W = crate::W<DecoderSpec>;
#[doc = "How a sequence is read from RAM and spread to the compare register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Load {
    #[doc = "0: 1st half word (16-bit) used in all PWM channels 0..3"]
    Common = 0,
    #[doc = "1: 1st half word (16-bit) used in channel 0..1; 2nd word in channel 2..3"]
    Grouped = 1,
    #[doc = "2: 1st half word (16-bit) in ch.0; 2nd in ch.1; ...; 4th in ch.3"]
    Individual = 2,
    #[doc = "3: 1st half word (16-bit) in ch.0; 2nd in ch.1; ...; 4th in COUNTERTOP"]
    WaveForm = 3,
}
impl From<Load> for u8 {
    #[inline(always)]
    fn from(variant: Load) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Load {
    type Ux = u8;
}
impl crate::IsEnum for Load {}
#[doc = "Field `LOAD` reader - How a sequence is read from RAM and spread to the compare register"]
pub type LoadR = crate::FieldReader<Load>;
impl LoadR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Load {
        match self.bits {
            0 => Load::Common,
            1 => Load::Grouped,
            2 => Load::Individual,
            3 => Load::WaveForm,
            _ => unreachable!(),
        }
    }
    #[doc = "1st half word (16-bit) used in all PWM channels 0..3"]
    #[inline(always)]
    pub fn is_common(&self) -> bool {
        *self == Load::Common
    }
    #[doc = "1st half word (16-bit) used in channel 0..1; 2nd word in channel 2..3"]
    #[inline(always)]
    pub fn is_grouped(&self) -> bool {
        *self == Load::Grouped
    }
    #[doc = "1st half word (16-bit) in ch.0; 2nd in ch.1; ...; 4th in ch.3"]
    #[inline(always)]
    pub fn is_individual(&self) -> bool {
        *self == Load::Individual
    }
    #[doc = "1st half word (16-bit) in ch.0; 2nd in ch.1; ...; 4th in COUNTERTOP"]
    #[inline(always)]
    pub fn is_wave_form(&self) -> bool {
        *self == Load::WaveForm
    }
}
#[doc = "Field `LOAD` writer - How a sequence is read from RAM and spread to the compare register"]
pub type LoadW<'a, REG> = crate::FieldWriter<'a, REG, 2, Load, crate::Safe>;
impl<'a, REG> LoadW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1st half word (16-bit) used in all PWM channels 0..3"]
    #[inline(always)]
    pub fn common(self) -> &'a mut crate::W<REG> {
        self.variant(Load::Common)
    }
    #[doc = "1st half word (16-bit) used in channel 0..1; 2nd word in channel 2..3"]
    #[inline(always)]
    pub fn grouped(self) -> &'a mut crate::W<REG> {
        self.variant(Load::Grouped)
    }
    #[doc = "1st half word (16-bit) in ch.0; 2nd in ch.1; ...; 4th in ch.3"]
    #[inline(always)]
    pub fn individual(self) -> &'a mut crate::W<REG> {
        self.variant(Load::Individual)
    }
    #[doc = "1st half word (16-bit) in ch.0; 2nd in ch.1; ...; 4th in COUNTERTOP"]
    #[inline(always)]
    pub fn wave_form(self) -> &'a mut crate::W<REG> {
        self.variant(Load::WaveForm)
    }
}
#[doc = "Selects source for advancing the active sequence\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    #[doc = "0: SEQ\\[n\\].REFRESH is used to determine loading internal compare registers"]
    RefreshCount = 0,
    #[doc = "1: NEXTSTEP task causes a new value to be loaded to internal compare registers"]
    NextStep = 1,
}
impl From<Mode> for bool {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODE` reader - Selects source for advancing the active sequence"]
pub type ModeR = crate::BitReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode {
        match self.bits {
            false => Mode::RefreshCount,
            true => Mode::NextStep,
        }
    }
    #[doc = "SEQ\\[n\\].REFRESH is used to determine loading internal compare registers"]
    #[inline(always)]
    pub fn is_refresh_count(&self) -> bool {
        *self == Mode::RefreshCount
    }
    #[doc = "NEXTSTEP task causes a new value to be loaded to internal compare registers"]
    #[inline(always)]
    pub fn is_next_step(&self) -> bool {
        *self == Mode::NextStep
    }
}
#[doc = "Field `MODE` writer - Selects source for advancing the active sequence"]
pub type ModeW<'a, REG> = crate::BitWriter<'a, REG, Mode>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SEQ\\[n\\].REFRESH is used to determine loading internal compare registers"]
    #[inline(always)]
    pub fn refresh_count(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::RefreshCount)
    }
    #[doc = "NEXTSTEP task causes a new value to be loaded to internal compare registers"]
    #[inline(always)]
    pub fn next_step(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::NextStep)
    }
}
impl R {
    #[doc = "Bits 0:1 - How a sequence is read from RAM and spread to the compare register"]
    #[inline(always)]
    pub fn load(&self) -> LoadR {
        LoadR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 8 - Selects source for advancing the active sequence"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - How a sequence is read from RAM and spread to the compare register"]
    #[inline(always)]
    pub fn load(&mut self) -> LoadW<'_, DecoderSpec> {
        LoadW::new(self, 0)
    }
    #[doc = "Bit 8 - Selects source for advancing the active sequence"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, DecoderSpec> {
        ModeW::new(self, 8)
    }
}
#[doc = "Configuration of the decoder\n\nYou can [`read`](crate::Reg::read) this register and get [`decoder::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`decoder::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DecoderSpec;
impl crate::RegisterSpec for DecoderSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`decoder::R`](R) reader structure"]
impl crate::Readable for DecoderSpec {}
#[doc = "`write(|w| ..)` method takes [`decoder::W`](W) writer structure"]
impl crate::Writable for DecoderSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DECODER to value 0"]
impl crate::Resettable for DecoderSpec {}
