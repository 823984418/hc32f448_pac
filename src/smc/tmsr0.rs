#[doc = "Register `TMSR0` reader"]
pub type R = crate::R<Tmsr0Spec>;
#[doc = "Field `T_RC` reader - desc T_RC"]
pub type TRcR = crate::FieldReader;
#[doc = "Field `T_WC` reader - desc T_WC"]
pub type TWcR = crate::FieldReader;
#[doc = "Field `T_CEOE` reader - desc T_CEOE"]
pub type TCeoeR = crate::FieldReader;
#[doc = "Field `T_WP` reader - desc T_WP"]
pub type TWpR = crate::FieldReader;
#[doc = "Field `T_TR` reader - desc T_TR"]
pub type TTrR = crate::FieldReader;
#[doc = "Field `T_ADV` reader - desc T_ADV"]
pub type TAdvR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - desc T_RC"]
    #[inline(always)]
    pub fn t_rc(&self) -> TRcR {
        TRcR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - desc T_WC"]
    #[inline(always)]
    pub fn t_wc(&self) -> TWcR {
        TWcR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - desc T_CEOE"]
    #[inline(always)]
    pub fn t_ceoe(&self) -> TCeoeR {
        TCeoeR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - desc T_WP"]
    #[inline(always)]
    pub fn t_wp(&self) -> TWpR {
        TWpR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 20:22 - desc T_TR"]
    #[inline(always)]
    pub fn t_tr(&self) -> TTrR {
        TTrR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - desc T_ADV"]
    #[inline(always)]
    pub fn t_adv(&self) -> TAdvR {
        TAdvR::new(((self.bits >> 24) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TMSR0")
            .field("t_rc", &self.t_rc())
            .field("t_wc", &self.t_wc())
            .field("t_ceoe", &self.t_ceoe())
            .field("t_wp", &self.t_wp())
            .field("t_tr", &self.t_tr())
            .field("t_adv", &self.t_adv())
            .finish()
    }
}
#[doc = "desc TMSR0\n\nYou can [`read`](crate::Reg::read) this register and get [`tmsr0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tmsr0Spec;
impl crate::RegisterSpec for Tmsr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmsr0::R`](R) reader structure"]
impl crate::Readable for Tmsr0Spec {}
#[doc = "`reset()` method sets TMSR0 to value 0x0012_63cc"]
impl crate::Resettable for Tmsr0Spec {
    const RESET_VALUE: u32 = 0x0012_63cc;
}
