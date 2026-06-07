#[doc = "Register `SRAM0_EIBIT0` reader"]
pub type R = crate::R<Sram0Eibit0Spec>;
#[doc = "Register `SRAM0_EIBIT0` writer"]
pub type W = crate::W<Sram0Eibit0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "desc SRAM0_EIBIT0\n\nYou can [`read`](crate::Reg::read) this register and get [`sram0_eibit0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram0_eibit0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sram0Eibit0Spec;
impl crate::RegisterSpec for Sram0Eibit0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sram0_eibit0::R`](R) reader structure"]
impl crate::Readable for Sram0Eibit0Spec {}
#[doc = "`write(|w| ..)` method takes [`sram0_eibit0::W`](W) writer structure"]
impl crate::Writable for Sram0Eibit0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SRAM0_EIBIT0 to value 0"]
impl crate::Resettable for Sram0Eibit0Spec {}
