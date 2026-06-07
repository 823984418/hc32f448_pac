#[doc = "Register `PR` reader"]
pub type R = crate::R<PrSpec>;
#[doc = "Register `PR` writer"]
pub type W = crate::W<PrSpec>;
#[doc = "Field `PSC` reader - desc PSC"]
pub type PscR = crate::FieldReader;
#[doc = "Field `PSC` writer - desc PSC"]
pub type PscW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LBMPSC` reader - desc LBMPSC"]
pub type LbmpscR = crate::FieldReader;
#[doc = "Field `LBMPSC` writer - desc LBMPSC"]
pub type LbmpscW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ULBREN` reader - desc ULBREN"]
pub type UlbrenR = crate::BitReader;
#[doc = "Field `ULBREN` writer - desc ULBREN"]
pub type UlbrenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - desc PSC"]
    #[inline(always)]
    pub fn psc(&self) -> PscR {
        PscR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - desc LBMPSC"]
    #[inline(always)]
    pub fn lbmpsc(&self) -> LbmpscR {
        LbmpscR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - desc ULBREN"]
    #[inline(always)]
    pub fn ulbren(&self) -> UlbrenR {
        UlbrenR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PR")
            .field("psc", &self.psc())
            .field("lbmpsc", &self.lbmpsc())
            .field("ulbren", &self.ulbren())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - desc PSC"]
    #[inline(always)]
    pub fn psc(&mut self) -> PscW<'_, PrSpec> {
        PscW::new(self, 0)
    }
    #[doc = "Bits 2:3 - desc LBMPSC"]
    #[inline(always)]
    pub fn lbmpsc(&mut self) -> LbmpscW<'_, PrSpec> {
        LbmpscW::new(self, 2)
    }
    #[doc = "Bit 4 - desc ULBREN"]
    #[inline(always)]
    pub fn ulbren(&mut self) -> UlbrenW<'_, PrSpec> {
        UlbrenW::new(self, 4)
    }
}
#[doc = "desc PR\n\nYou can [`read`](crate::Reg::read) this register and get [`pr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrSpec;
impl crate::RegisterSpec for PrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr::R`](R) reader structure"]
impl crate::Readable for PrSpec {}
#[doc = "`write(|w| ..)` method takes [`pr::W`](W) writer structure"]
impl crate::Writable for PrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PR to value 0"]
impl crate::Resettable for PrSpec {}
