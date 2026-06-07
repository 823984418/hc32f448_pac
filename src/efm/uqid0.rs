#[doc = "Register `UQID0` reader"]
pub type R = crate::R<Uqid0Spec>;
#[doc = "Field `Y_LOCATION` reader - desc Y_LOCATION"]
pub type YLocationR = crate::FieldReader;
#[doc = "Field `X_LOCATION` reader - desc X_LOCATION"]
pub type XLocationR = crate::FieldReader;
#[doc = "Field `WAFER_ID` reader - desc WAFER_ID"]
pub type WaferIdR = crate::FieldReader;
#[doc = "Field `LOT_ID` reader - desc LOT_ID"]
pub type LotIdR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - desc Y_LOCATION"]
    #[inline(always)]
    pub fn y_location(&self) -> YLocationR {
        YLocationR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - desc X_LOCATION"]
    #[inline(always)]
    pub fn x_location(&self) -> XLocationR {
        XLocationR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - desc WAFER_ID"]
    #[inline(always)]
    pub fn wafer_id(&self) -> WaferIdR {
        WaferIdR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - desc LOT_ID"]
    #[inline(always)]
    pub fn lot_id(&self) -> LotIdR {
        LotIdR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UQID0")
            .field("y_location", &self.y_location())
            .field("x_location", &self.x_location())
            .field("wafer_id", &self.wafer_id())
            .field("lot_id", &self.lot_id())
            .finish()
    }
}
#[doc = "desc UQID0\n\nYou can [`read`](crate::Reg::read) this register and get [`uqid0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uqid0Spec;
impl crate::RegisterSpec for Uqid0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uqid0::R`](R) reader structure"]
impl crate::Readable for Uqid0Spec {}
#[doc = "`reset()` method sets UQID0 to value 0"]
impl crate::Resettable for Uqid0Spec {}
