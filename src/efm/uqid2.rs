#[doc = "Register `UQID2` reader"]
pub type R = crate::R<Uqid2Spec>;
#[doc = "Field `LOT_ID` reader - desc LOT_ID"]
pub type LotIdR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - desc LOT_ID"]
    #[inline(always)]
    pub fn lot_id(&self) -> LotIdR {
        LotIdR::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UQID2")
            .field("lot_id", &self.lot_id())
            .finish()
    }
}
#[doc = "desc UQID2\n\nYou can [`read`](crate::Reg::read) this register and get [`uqid2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uqid2Spec;
impl crate::RegisterSpec for Uqid2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uqid2::R`](R) reader structure"]
impl crate::Readable for Uqid2Spec {}
#[doc = "`reset()` method sets UQID2 to value 0"]
impl crate::Resettable for Uqid2Spec {}
