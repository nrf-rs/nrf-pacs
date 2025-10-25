#[doc = "Register `XIPEN` reader"]
pub type R = crate::R<XipenSpec>;
#[doc = "Register `XIPEN` writer"]
pub type W = crate::W<XipenSpec>;
#[doc = "Enable XIP AHB Slave interface and access to XIP memory range\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Xipen {
    #[doc = "0: Disable XIP interface"]
    Disable = 0,
    #[doc = "1: Enable XIP interface"]
    Enable = 1,
}
impl From<Xipen> for bool {
    #[inline(always)]
    fn from(variant: Xipen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `XIPEN` reader - Enable XIP AHB Slave interface and access to XIP memory range"]
pub type XipenR = crate::BitReader<Xipen>;
impl XipenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Xipen {
        match self.bits {
            false => Xipen::Disable,
            true => Xipen::Enable,
        }
    }
    #[doc = "Disable XIP interface"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Xipen::Disable
    }
    #[doc = "Enable XIP interface"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Xipen::Enable
    }
}
#[doc = "Field `XIPEN` writer - Enable XIP AHB Slave interface and access to XIP memory range"]
pub type XipenW<'a, REG> = crate::BitWriter<'a, REG, Xipen>;
impl<'a, REG> XipenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable XIP interface"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Xipen::Disable)
    }
    #[doc = "Enable XIP interface"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Xipen::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - Enable XIP AHB Slave interface and access to XIP memory range"]
    #[inline(always)]
    pub fn xipen(&self) -> XipenR {
        XipenR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable XIP AHB Slave interface and access to XIP memory range"]
    #[inline(always)]
    pub fn xipen(&mut self) -> XipenW<'_, XipenSpec> {
        XipenW::new(self, 0)
    }
}
#[doc = "Enable Execute in Place operation.\n\nYou can [`read`](crate::Reg::read) this register and get [`xipen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xipen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XipenSpec;
impl crate::RegisterSpec for XipenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xipen::R`](R) reader structure"]
impl crate::Readable for XipenSpec {}
#[doc = "`write(|w| ..)` method takes [`xipen::W`](W) writer structure"]
impl crate::Writable for XipenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets XIPEN to value 0x01"]
impl crate::Resettable for XipenSpec {
    const RESET_VALUE: u32 = 0x01;
}
