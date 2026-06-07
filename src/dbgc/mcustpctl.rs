#[doc = "Register `MCUSTPCTL` reader"]
pub type R = crate::R<McustpctlSpec>;
#[doc = "Register `MCUSTPCTL` writer"]
pub type W = crate::W<McustpctlSpec>;
#[doc = "Field `SWDTSTP` reader - desc SWDTSTP"]
pub type SwdtstpR = crate::BitReader;
#[doc = "Field `SWDTSTP` writer - desc SWDTSTP"]
pub type SwdtstpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDTSTP` reader - desc WDTSTP"]
pub type WdtstpR = crate::BitReader;
#[doc = "Field `WDTSTP` writer - desc WDTSTP"]
pub type WdtstpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCSTP` reader - desc RTCSTP"]
pub type RtcstpR = crate::BitReader;
#[doc = "Field `RTCSTP` writer - desc RTCSTP"]
pub type RtcstpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `M06STP` reader - desc M06STP"]
pub type M06stpR = crate::BitReader;
#[doc = "Field `M06STP` writer - desc M06STP"]
pub type M06stpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `M07STP` reader - desc M07STP"]
pub type M07stpR = crate::BitReader;
#[doc = "Field `M07STP` writer - desc M07STP"]
pub type M07stpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `M12STP` reader - desc M12STP"]
pub type M12stpR = crate::BitReader;
#[doc = "Field `M12STP` writer - desc M12STP"]
pub type M12stpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `M13STP` reader - desc M13STP"]
pub type M13stpR = crate::BitReader;
#[doc = "Field `M13STP` writer - desc M13STP"]
pub type M13stpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `M14STP` reader - desc M14STP"]
pub type M14stpR = crate::BitReader;
#[doc = "Field `M14STP` writer - desc M14STP"]
pub type M14stpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `M15STP` reader - desc M15STP"]
pub type M15stpR = crate::BitReader;
#[doc = "Field `M15STP` writer - desc M15STP"]
pub type M15stpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `M16STP` reader - desc M16STP"]
pub type M16stpR = crate::BitReader;
#[doc = "Field `M16STP` writer - desc M16STP"]
pub type M16stpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc SWDTSTP"]
    #[inline(always)]
    pub fn swdtstp(&self) -> SwdtstpR {
        SwdtstpR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc WDTSTP"]
    #[inline(always)]
    pub fn wdtstp(&self) -> WdtstpR {
        WdtstpR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc RTCSTP"]
    #[inline(always)]
    pub fn rtcstp(&self) -> RtcstpR {
        RtcstpR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - desc M06STP"]
    #[inline(always)]
    pub fn m06stp(&self) -> M06stpR {
        M06stpR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc M07STP"]
    #[inline(always)]
    pub fn m07stp(&self) -> M07stpR {
        M07stpR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 12 - desc M12STP"]
    #[inline(always)]
    pub fn m12stp(&self) -> M12stpR {
        M12stpR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - desc M13STP"]
    #[inline(always)]
    pub fn m13stp(&self) -> M13stpR {
        M13stpR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc M14STP"]
    #[inline(always)]
    pub fn m14stp(&self) -> M14stpR {
        M14stpR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc M15STP"]
    #[inline(always)]
    pub fn m15stp(&self) -> M15stpR {
        M15stpR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - desc M16STP"]
    #[inline(always)]
    pub fn m16stp(&self) -> M16stpR {
        M16stpR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MCUSTPCTL")
            .field("swdtstp", &self.swdtstp())
            .field("wdtstp", &self.wdtstp())
            .field("rtcstp", &self.rtcstp())
            .field("m06stp", &self.m06stp())
            .field("m07stp", &self.m07stp())
            .field("m12stp", &self.m12stp())
            .field("m13stp", &self.m13stp())
            .field("m14stp", &self.m14stp())
            .field("m15stp", &self.m15stp())
            .field("m16stp", &self.m16stp())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - desc SWDTSTP"]
    #[inline(always)]
    pub fn swdtstp(&mut self) -> SwdtstpW<'_, McustpctlSpec> {
        SwdtstpW::new(self, 0)
    }
    #[doc = "Bit 1 - desc WDTSTP"]
    #[inline(always)]
    pub fn wdtstp(&mut self) -> WdtstpW<'_, McustpctlSpec> {
        WdtstpW::new(self, 1)
    }
    #[doc = "Bit 2 - desc RTCSTP"]
    #[inline(always)]
    pub fn rtcstp(&mut self) -> RtcstpW<'_, McustpctlSpec> {
        RtcstpW::new(self, 2)
    }
    #[doc = "Bit 6 - desc M06STP"]
    #[inline(always)]
    pub fn m06stp(&mut self) -> M06stpW<'_, McustpctlSpec> {
        M06stpW::new(self, 6)
    }
    #[doc = "Bit 7 - desc M07STP"]
    #[inline(always)]
    pub fn m07stp(&mut self) -> M07stpW<'_, McustpctlSpec> {
        M07stpW::new(self, 7)
    }
    #[doc = "Bit 12 - desc M12STP"]
    #[inline(always)]
    pub fn m12stp(&mut self) -> M12stpW<'_, McustpctlSpec> {
        M12stpW::new(self, 12)
    }
    #[doc = "Bit 13 - desc M13STP"]
    #[inline(always)]
    pub fn m13stp(&mut self) -> M13stpW<'_, McustpctlSpec> {
        M13stpW::new(self, 13)
    }
    #[doc = "Bit 14 - desc M14STP"]
    #[inline(always)]
    pub fn m14stp(&mut self) -> M14stpW<'_, McustpctlSpec> {
        M14stpW::new(self, 14)
    }
    #[doc = "Bit 15 - desc M15STP"]
    #[inline(always)]
    pub fn m15stp(&mut self) -> M15stpW<'_, McustpctlSpec> {
        M15stpW::new(self, 15)
    }
    #[doc = "Bit 16 - desc M16STP"]
    #[inline(always)]
    pub fn m16stp(&mut self) -> M16stpW<'_, McustpctlSpec> {
        M16stpW::new(self, 16)
    }
}
#[doc = "desc MCUSTPCTL\n\nYou can [`read`](crate::Reg::read) this register and get [`mcustpctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcustpctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McustpctlSpec;
impl crate::RegisterSpec for McustpctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcustpctl::R`](R) reader structure"]
impl crate::Readable for McustpctlSpec {}
#[doc = "`write(|w| ..)` method takes [`mcustpctl::W`](W) writer structure"]
impl crate::Writable for McustpctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MCUSTPCTL to value 0x3b"]
impl crate::Resettable for McustpctlSpec {
    const RESET_VALUE: u32 = 0x3b;
}
