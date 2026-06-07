#[doc = "Register `AWD1DR0` reader"]
pub type R = crate::R<Awd1dr0Spec>;
#[doc = "Register `AWD1DR0` writer"]
pub type W = crate::W<Awd1dr0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc AWD1DR0\n\nYou can [`read`](crate::Reg::read) this register and get [`awd1dr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awd1dr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Awd1dr0Spec;
impl crate::RegisterSpec for Awd1dr0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`awd1dr0::R`](R) reader structure"]
impl crate::Readable for Awd1dr0Spec {}
#[doc = "`write(|w| ..)` method takes [`awd1dr0::W`](W) writer structure"]
impl crate::Writable for Awd1dr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AWD1DR0 to value 0"]
impl crate::Resettable for Awd1dr0Spec {}
