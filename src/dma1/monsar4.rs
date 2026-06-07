#[doc = "Register `MONSAR4` reader"]
pub type R = crate::R<Monsar4Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "desc MONSAR4\n\nYou can [`read`](crate::Reg::read) this register and get [`monsar4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Monsar4Spec;
impl crate::RegisterSpec for Monsar4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`monsar4::R`](R) reader structure"]
impl crate::Readable for Monsar4Spec {}
#[doc = "`reset()` method sets MONSAR4 to value 0"]
impl crate::Resettable for Monsar4Spec {}
