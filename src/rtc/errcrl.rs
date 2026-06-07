#[doc = "Register `ERRCRL` reader"]
pub type R = crate::R<ErrcrlSpec>;
#[doc = "Register `ERRCRL` writer"]
pub type W = crate::W<ErrcrlSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc ERRCRL\n\nYou can [`read`](crate::Reg::read) this register and get [`errcrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`errcrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrcrlSpec;
impl crate::RegisterSpec for ErrcrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`errcrl::R`](R) reader structure"]
impl crate::Readable for ErrcrlSpec {}
#[doc = "`write(|w| ..)` method takes [`errcrl::W`](W) writer structure"]
impl crate::Writable for ErrcrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ERRCRL to value 0x20"]
impl crate::Resettable for ErrcrlSpec {
    const RESET_VALUE: u8 = 0x20;
}
