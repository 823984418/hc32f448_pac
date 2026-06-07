#[doc = "Register `AWD1DR1` reader"]
pub type R = crate::R<Awd1dr1Spec>;
#[doc = "Register `AWD1DR1` writer"]
pub type W = crate::W<Awd1dr1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc AWD1DR1\n\nYou can [`read`](crate::Reg::read) this register and get [`awd1dr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awd1dr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Awd1dr1Spec;
impl crate::RegisterSpec for Awd1dr1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`awd1dr1::R`](R) reader structure"]
impl crate::Readable for Awd1dr1Spec {}
#[doc = "`write(|w| ..)` method takes [`awd1dr1::W`](W) writer structure"]
impl crate::Writable for Awd1dr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AWD1DR1 to value 0xffff"]
impl crate::Resettable for Awd1dr1Spec {
    const RESET_VALUE: u16 = 0xffff;
}
