#[doc = "Register `REPORTPER` reader"]
pub type R = crate::R<ReportperSpec>;
#[doc = "Register `REPORTPER` writer"]
pub type W = crate::W<ReportperSpec>;
#[doc = "Number of samples to generate an EVENT_REPORTRDY.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Reportper {
    #[doc = "0: 10 samples per report."]
    _10smpl = 0,
    #[doc = "1: 40 samples per report."]
    _40smpl = 1,
    #[doc = "2: 80 samples per report."]
    _80smpl = 2,
    #[doc = "3: 120 samples per report."]
    _120smpl = 3,
    #[doc = "4: 160 samples per report."]
    _160smpl = 4,
    #[doc = "5: 200 samples per report."]
    _200smpl = 5,
    #[doc = "6: 240 samples per report."]
    _240smpl = 6,
    #[doc = "7: 280 samples per report."]
    _280smpl = 7,
}
impl From<Reportper> for u8 {
    #[inline(always)]
    fn from(variant: Reportper) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Reportper {
    type Ux = u8;
}
impl crate::IsEnum for Reportper {}
#[doc = "Field `REPORTPER` reader - Number of samples to generate an EVENT_REPORTRDY."]
pub type ReportperR = crate::FieldReader<Reportper>;
impl ReportperR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Reportper {
        match self.bits {
            0 => Reportper::_10smpl,
            1 => Reportper::_40smpl,
            2 => Reportper::_80smpl,
            3 => Reportper::_120smpl,
            4 => Reportper::_160smpl,
            5 => Reportper::_200smpl,
            6 => Reportper::_240smpl,
            7 => Reportper::_280smpl,
            _ => unreachable!(),
        }
    }
    #[doc = "10 samples per report."]
    #[inline(always)]
    pub fn is_10smpl(&self) -> bool {
        *self == Reportper::_10smpl
    }
    #[doc = "40 samples per report."]
    #[inline(always)]
    pub fn is_40smpl(&self) -> bool {
        *self == Reportper::_40smpl
    }
    #[doc = "80 samples per report."]
    #[inline(always)]
    pub fn is_80smpl(&self) -> bool {
        *self == Reportper::_80smpl
    }
    #[doc = "120 samples per report."]
    #[inline(always)]
    pub fn is_120smpl(&self) -> bool {
        *self == Reportper::_120smpl
    }
    #[doc = "160 samples per report."]
    #[inline(always)]
    pub fn is_160smpl(&self) -> bool {
        *self == Reportper::_160smpl
    }
    #[doc = "200 samples per report."]
    #[inline(always)]
    pub fn is_200smpl(&self) -> bool {
        *self == Reportper::_200smpl
    }
    #[doc = "240 samples per report."]
    #[inline(always)]
    pub fn is_240smpl(&self) -> bool {
        *self == Reportper::_240smpl
    }
    #[doc = "280 samples per report."]
    #[inline(always)]
    pub fn is_280smpl(&self) -> bool {
        *self == Reportper::_280smpl
    }
}
#[doc = "Field `REPORTPER` writer - Number of samples to generate an EVENT_REPORTRDY."]
pub type ReportperW<'a, REG> = crate::FieldWriter<'a, REG, 3, Reportper, crate::Safe>;
impl<'a, REG> ReportperW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "10 samples per report."]
    #[inline(always)]
    pub fn _10smpl(self) -> &'a mut crate::W<REG> {
        self.variant(Reportper::_10smpl)
    }
    #[doc = "40 samples per report."]
    #[inline(always)]
    pub fn _40smpl(self) -> &'a mut crate::W<REG> {
        self.variant(Reportper::_40smpl)
    }
    #[doc = "80 samples per report."]
    #[inline(always)]
    pub fn _80smpl(self) -> &'a mut crate::W<REG> {
        self.variant(Reportper::_80smpl)
    }
    #[doc = "120 samples per report."]
    #[inline(always)]
    pub fn _120smpl(self) -> &'a mut crate::W<REG> {
        self.variant(Reportper::_120smpl)
    }
    #[doc = "160 samples per report."]
    #[inline(always)]
    pub fn _160smpl(self) -> &'a mut crate::W<REG> {
        self.variant(Reportper::_160smpl)
    }
    #[doc = "200 samples per report."]
    #[inline(always)]
    pub fn _200smpl(self) -> &'a mut crate::W<REG> {
        self.variant(Reportper::_200smpl)
    }
    #[doc = "240 samples per report."]
    #[inline(always)]
    pub fn _240smpl(self) -> &'a mut crate::W<REG> {
        self.variant(Reportper::_240smpl)
    }
    #[doc = "280 samples per report."]
    #[inline(always)]
    pub fn _280smpl(self) -> &'a mut crate::W<REG> {
        self.variant(Reportper::_280smpl)
    }
}
impl R {
    #[doc = "Bits 0:2 - Number of samples to generate an EVENT_REPORTRDY."]
    #[inline(always)]
    pub fn reportper(&self) -> ReportperR {
        ReportperR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Number of samples to generate an EVENT_REPORTRDY."]
    #[inline(always)]
    pub fn reportper(&mut self) -> ReportperW<'_, ReportperSpec> {
        ReportperW::new(self, 0)
    }
}
#[doc = "Number of samples to generate an EVENT_REPORTRDY.\n\nYou can [`read`](crate::Reg::read) this register and get [`reportper::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reportper::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ReportperSpec;
impl crate::RegisterSpec for ReportperSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reportper::R`](R) reader structure"]
impl crate::Readable for ReportperSpec {}
#[doc = "`write(|w| ..)` method takes [`reportper::W`](W) writer structure"]
impl crate::Writable for ReportperSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REPORTPER to value 0"]
impl crate::Resettable for ReportperSpec {}
