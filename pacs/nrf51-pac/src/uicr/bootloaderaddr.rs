#[doc = "Register `BOOTLOADERADDR` reader"]
pub type R = crate::R<BootloaderaddrSpec>;
#[doc = "Register `BOOTLOADERADDR` writer"]
pub type W = crate::W<BootloaderaddrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Bootloader start address.\n\nYou can [`read`](crate::Reg::read) this register and get [`bootloaderaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bootloaderaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BootloaderaddrSpec;
impl crate::RegisterSpec for BootloaderaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bootloaderaddr::R`](R) reader structure"]
impl crate::Readable for BootloaderaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`bootloaderaddr::W`](W) writer structure"]
impl crate::Writable for BootloaderaddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BOOTLOADERADDR to value 0xffff_ffff"]
impl crate::Resettable for BootloaderaddrSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
