#[doc = "Register `CMDR` writer"]
pub type W = crate::W<CmdrSpec>;
#[doc = "Field `CMDADD` writer - desc CMDADD"]
pub type CmdaddW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `CRES` writer - desc CRES"]
pub type CresW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD` writer - desc CMD"]
pub type CmdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CMDCHIP` writer - desc CMDCHIP"]
pub type CmdchipW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl core::fmt::Debug for crate::generic::Reg<CmdrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:19 - desc CMDADD"]
    #[inline(always)]
    pub fn cmdadd(&mut self) -> CmdaddW<'_, CmdrSpec> {
        CmdaddW::new(self, 0)
    }
    #[doc = "Bit 20 - desc CRES"]
    #[inline(always)]
    pub fn cres(&mut self) -> CresW<'_, CmdrSpec> {
        CresW::new(self, 20)
    }
    #[doc = "Bits 21:22 - desc CMD"]
    #[inline(always)]
    pub fn cmd(&mut self) -> CmdW<'_, CmdrSpec> {
        CmdW::new(self, 21)
    }
    #[doc = "Bits 23:25 - desc CMDCHIP"]
    #[inline(always)]
    pub fn cmdchip(&mut self) -> CmdchipW<'_, CmdrSpec> {
        CmdchipW::new(self, 23)
    }
}
#[doc = "desc CMDR\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmdr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdrSpec;
impl crate::RegisterSpec for CmdrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cmdr::W`](W) writer structure"]
impl crate::Writable for CmdrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMDR to value 0"]
impl crate::Resettable for CmdrSpec {}
