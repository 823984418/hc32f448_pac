#[doc = "Register `FLAGCLR` writer"]
pub type W = crate::W<FlagclrSpec>;
#[doc = "Field `CLR_OP` writer - desc CLR_OP"]
pub type ClrOpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR_LS2` writer - desc CLR_LS2"]
pub type ClrLs2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR_EQ2` writer - desc CLR_EQ2"]
pub type ClrEq2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR_GT2` writer - desc CLR_GT2"]
pub type ClrGt2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR_LS1` writer - desc CLR_LS1"]
pub type ClrLs1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR_EQ1` writer - desc CLR_EQ1"]
pub type ClrEq1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR_GT1` writer - desc CLR_GT1"]
pub type ClrGt1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR_RLD` writer - desc CLR_RLD"]
pub type ClrRldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR_BTM` writer - desc CLR_BTM"]
pub type ClrBtmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR_TOP` writer - desc CLR_TOP"]
pub type ClrTopW<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<FlagclrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - desc CLR_OP"]
    #[inline(always)]
    pub fn clr_op(&mut self) -> ClrOpW<'_, FlagclrSpec> {
        ClrOpW::new(self, 0)
    }
    #[doc = "Bit 1 - desc CLR_LS2"]
    #[inline(always)]
    pub fn clr_ls2(&mut self) -> ClrLs2W<'_, FlagclrSpec> {
        ClrLs2W::new(self, 1)
    }
    #[doc = "Bit 2 - desc CLR_EQ2"]
    #[inline(always)]
    pub fn clr_eq2(&mut self) -> ClrEq2W<'_, FlagclrSpec> {
        ClrEq2W::new(self, 2)
    }
    #[doc = "Bit 3 - desc CLR_GT2"]
    #[inline(always)]
    pub fn clr_gt2(&mut self) -> ClrGt2W<'_, FlagclrSpec> {
        ClrGt2W::new(self, 3)
    }
    #[doc = "Bit 4 - desc CLR_LS1"]
    #[inline(always)]
    pub fn clr_ls1(&mut self) -> ClrLs1W<'_, FlagclrSpec> {
        ClrLs1W::new(self, 4)
    }
    #[doc = "Bit 5 - desc CLR_EQ1"]
    #[inline(always)]
    pub fn clr_eq1(&mut self) -> ClrEq1W<'_, FlagclrSpec> {
        ClrEq1W::new(self, 5)
    }
    #[doc = "Bit 6 - desc CLR_GT1"]
    #[inline(always)]
    pub fn clr_gt1(&mut self) -> ClrGt1W<'_, FlagclrSpec> {
        ClrGt1W::new(self, 6)
    }
    #[doc = "Bit 9 - desc CLR_RLD"]
    #[inline(always)]
    pub fn clr_rld(&mut self) -> ClrRldW<'_, FlagclrSpec> {
        ClrRldW::new(self, 9)
    }
    #[doc = "Bit 10 - desc CLR_BTM"]
    #[inline(always)]
    pub fn clr_btm(&mut self) -> ClrBtmW<'_, FlagclrSpec> {
        ClrBtmW::new(self, 10)
    }
    #[doc = "Bit 11 - desc CLR_TOP"]
    #[inline(always)]
    pub fn clr_top(&mut self) -> ClrTopW<'_, FlagclrSpec> {
        ClrTopW::new(self, 11)
    }
}
#[doc = "desc FLAGCLR\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flagclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlagclrSpec;
impl crate::RegisterSpec for FlagclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`flagclr::W`](W) writer structure"]
impl crate::Writable for FlagclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLAGCLR to value 0"]
impl crate::Resettable for FlagclrSpec {}
