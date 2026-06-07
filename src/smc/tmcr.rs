#[doc = "Register `TMCR` writer"]
pub type W = crate::W<TmcrSpec>;
#[doc = "Field `T_RC` writer - desc T_RC"]
pub type TRcW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `T_WC` writer - desc T_WC"]
pub type TWcW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `T_CEOE` writer - desc T_CEOE"]
pub type TCeoeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `T_WP` writer - desc T_WP"]
pub type TWpW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `T_TR` writer - desc T_TR"]
pub type TTrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `T_ADV` writer - desc T_ADV"]
pub type TAdvW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl core::fmt::Debug for crate::generic::Reg<TmcrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:3 - desc T_RC"]
    #[inline(always)]
    pub fn t_rc(&mut self) -> TRcW<'_, TmcrSpec> {
        TRcW::new(self, 0)
    }
    #[doc = "Bits 4:7 - desc T_WC"]
    #[inline(always)]
    pub fn t_wc(&mut self) -> TWcW<'_, TmcrSpec> {
        TWcW::new(self, 4)
    }
    #[doc = "Bits 8:10 - desc T_CEOE"]
    #[inline(always)]
    pub fn t_ceoe(&mut self) -> TCeoeW<'_, TmcrSpec> {
        TCeoeW::new(self, 8)
    }
    #[doc = "Bits 12:14 - desc T_WP"]
    #[inline(always)]
    pub fn t_wp(&mut self) -> TWpW<'_, TmcrSpec> {
        TWpW::new(self, 12)
    }
    #[doc = "Bits 20:22 - desc T_TR"]
    #[inline(always)]
    pub fn t_tr(&mut self) -> TTrW<'_, TmcrSpec> {
        TTrW::new(self, 20)
    }
    #[doc = "Bits 24:26 - desc T_ADV"]
    #[inline(always)]
    pub fn t_adv(&mut self) -> TAdvW<'_, TmcrSpec> {
        TAdvW::new(self, 24)
    }
}
#[doc = "desc TMCR\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TmcrSpec;
impl crate::RegisterSpec for TmcrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tmcr::W`](W) writer structure"]
impl crate::Writable for TmcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TMCR to value 0"]
impl crate::Resettable for TmcrSpec {}
