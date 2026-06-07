#[doc = "Register `MONDAR4` reader"]
pub type R = crate::R<Mondar4Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "desc MONDAR4\n\nYou can [`read`](crate::Reg::read) this register and get [`mondar4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mondar4Spec;
impl crate::RegisterSpec for Mondar4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mondar4::R`](R) reader structure"]
impl crate::Readable for Mondar4Spec {}
#[doc = "`reset()` method sets MONDAR4 to value 0"]
impl crate::Resettable for Mondar4Spec {}
