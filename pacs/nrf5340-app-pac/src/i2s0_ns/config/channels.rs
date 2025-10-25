#[doc = "Register `CHANNELS` reader"]
pub type R = crate::R<ChannelsSpec>;
#[doc = "Register `CHANNELS` writer"]
pub type W = crate::W<ChannelsSpec>;
#[doc = "Enable channels\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Channels {
    #[doc = "0: Stereo."]
    Stereo = 0,
    #[doc = "1: Left only."]
    Left = 1,
    #[doc = "2: Right only."]
    Right = 2,
}
impl From<Channels> for u8 {
    #[inline(always)]
    fn from(variant: Channels) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Channels {
    type Ux = u8;
}
impl crate::IsEnum for Channels {}
#[doc = "Field `CHANNELS` reader - Enable channels"]
pub type ChannelsR = crate::FieldReader<Channels>;
impl ChannelsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Channels> {
        match self.bits {
            0 => Some(Channels::Stereo),
            1 => Some(Channels::Left),
            2 => Some(Channels::Right),
            _ => None,
        }
    }
    #[doc = "Stereo."]
    #[inline(always)]
    pub fn is_stereo(&self) -> bool {
        *self == Channels::Stereo
    }
    #[doc = "Left only."]
    #[inline(always)]
    pub fn is_left(&self) -> bool {
        *self == Channels::Left
    }
    #[doc = "Right only."]
    #[inline(always)]
    pub fn is_right(&self) -> bool {
        *self == Channels::Right
    }
}
#[doc = "Field `CHANNELS` writer - Enable channels"]
pub type ChannelsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Channels>;
impl<'a, REG> ChannelsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Stereo."]
    #[inline(always)]
    pub fn stereo(self) -> &'a mut crate::W<REG> {
        self.variant(Channels::Stereo)
    }
    #[doc = "Left only."]
    #[inline(always)]
    pub fn left(self) -> &'a mut crate::W<REG> {
        self.variant(Channels::Left)
    }
    #[doc = "Right only."]
    #[inline(always)]
    pub fn right(self) -> &'a mut crate::W<REG> {
        self.variant(Channels::Right)
    }
}
impl R {
    #[doc = "Bits 0:1 - Enable channels"]
    #[inline(always)]
    pub fn channels(&self) -> ChannelsR {
        ChannelsR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Enable channels"]
    #[inline(always)]
    pub fn channels(&mut self) -> ChannelsW<'_, ChannelsSpec> {
        ChannelsW::new(self, 0)
    }
}
#[doc = "Enable channels\n\nYou can [`read`](crate::Reg::read) this register and get [`channels::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`channels::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChannelsSpec;
impl crate::RegisterSpec for ChannelsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`channels::R`](R) reader structure"]
impl crate::Readable for ChannelsSpec {}
#[doc = "`write(|w| ..)` method takes [`channels::W`](W) writer structure"]
impl crate::Writable for ChannelsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHANNELS to value 0"]
impl crate::Resettable for ChannelsSpec {}
