#[doc = "Register `FRAMEDELAYMODE` reader"]
pub type R = crate::R<FramedelaymodeSpec>;
#[doc = "Register `FRAMEDELAYMODE` writer"]
pub type W = crate::W<FramedelaymodeSpec>;
#[doc = "Configuration register for the Frame Delay Timer\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Framedelaymode {
    #[doc = "0: Transmission is independent of frame timer and will start when the STARTTX task is triggered. No timeout."]
    FreeRun = 0,
    #[doc = "1: Frame is transmitted between FRAMEDELAYMIN and FRAMEDELAYMAX"]
    Window = 1,
    #[doc = "2: Frame is transmitted exactly at FRAMEDELAYMAX"]
    ExactVal = 2,
    #[doc = "3: Frame is transmitted on a bit grid between FRAMEDELAYMIN and FRAMEDELAYMAX"]
    WindowGrid = 3,
}
impl From<Framedelaymode> for u8 {
    #[inline(always)]
    fn from(variant: Framedelaymode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Framedelaymode {
    type Ux = u8;
}
impl crate::IsEnum for Framedelaymode {}
#[doc = "Field `FRAMEDELAYMODE` reader - Configuration register for the Frame Delay Timer"]
pub type FramedelaymodeR = crate::FieldReader<Framedelaymode>;
impl FramedelaymodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Framedelaymode {
        match self.bits {
            0 => Framedelaymode::FreeRun,
            1 => Framedelaymode::Window,
            2 => Framedelaymode::ExactVal,
            3 => Framedelaymode::WindowGrid,
            _ => unreachable!(),
        }
    }
    #[doc = "Transmission is independent of frame timer and will start when the STARTTX task is triggered. No timeout."]
    #[inline(always)]
    pub fn is_free_run(&self) -> bool {
        *self == Framedelaymode::FreeRun
    }
    #[doc = "Frame is transmitted between FRAMEDELAYMIN and FRAMEDELAYMAX"]
    #[inline(always)]
    pub fn is_window(&self) -> bool {
        *self == Framedelaymode::Window
    }
    #[doc = "Frame is transmitted exactly at FRAMEDELAYMAX"]
    #[inline(always)]
    pub fn is_exact_val(&self) -> bool {
        *self == Framedelaymode::ExactVal
    }
    #[doc = "Frame is transmitted on a bit grid between FRAMEDELAYMIN and FRAMEDELAYMAX"]
    #[inline(always)]
    pub fn is_window_grid(&self) -> bool {
        *self == Framedelaymode::WindowGrid
    }
}
#[doc = "Field `FRAMEDELAYMODE` writer - Configuration register for the Frame Delay Timer"]
pub type FramedelaymodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Framedelaymode, crate::Safe>;
impl<'a, REG> FramedelaymodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Transmission is independent of frame timer and will start when the STARTTX task is triggered. No timeout."]
    #[inline(always)]
    pub fn free_run(self) -> &'a mut crate::W<REG> {
        self.variant(Framedelaymode::FreeRun)
    }
    #[doc = "Frame is transmitted between FRAMEDELAYMIN and FRAMEDELAYMAX"]
    #[inline(always)]
    pub fn window(self) -> &'a mut crate::W<REG> {
        self.variant(Framedelaymode::Window)
    }
    #[doc = "Frame is transmitted exactly at FRAMEDELAYMAX"]
    #[inline(always)]
    pub fn exact_val(self) -> &'a mut crate::W<REG> {
        self.variant(Framedelaymode::ExactVal)
    }
    #[doc = "Frame is transmitted on a bit grid between FRAMEDELAYMIN and FRAMEDELAYMAX"]
    #[inline(always)]
    pub fn window_grid(self) -> &'a mut crate::W<REG> {
        self.variant(Framedelaymode::WindowGrid)
    }
}
impl R {
    #[doc = "Bits 0:1 - Configuration register for the Frame Delay Timer"]
    #[inline(always)]
    pub fn framedelaymode(&self) -> FramedelaymodeR {
        FramedelaymodeR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Configuration register for the Frame Delay Timer"]
    #[inline(always)]
    pub fn framedelaymode(&mut self) -> FramedelaymodeW<'_, FramedelaymodeSpec> {
        FramedelaymodeW::new(self, 0)
    }
}
#[doc = "Configuration register for the Frame Delay Timer\n\nYou can [`read`](crate::Reg::read) this register and get [`framedelaymode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`framedelaymode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FramedelaymodeSpec;
impl crate::RegisterSpec for FramedelaymodeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`framedelaymode::R`](R) reader structure"]
impl crate::Readable for FramedelaymodeSpec {}
#[doc = "`write(|w| ..)` method takes [`framedelaymode::W`](W) writer structure"]
impl crate::Writable for FramedelaymodeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FRAMEDELAYMODE to value 0x01"]
impl crate::Resettable for FramedelaymodeSpec {
    const RESET_VALUE: u32 = 0x01;
}
