#[doc = "Register `MONDAR5` reader"]
pub type R = crate::R<Mondar5Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "desc MONDAR5\n\nYou can [`read`](crate::Reg::read) this register and get [`mondar5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mondar5Spec;
impl crate::RegisterSpec for Mondar5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mondar5::R`](R) reader structure"]
impl crate::Readable for Mondar5Spec {}
#[doc = "`reset()` method sets MONDAR5 to value 0"]
impl crate::Resettable for Mondar5Spec {}
