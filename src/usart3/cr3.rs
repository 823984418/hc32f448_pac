#[doc = "Register `CR3` reader"]
pub type R = crate::R<Cr3Spec>;
#[doc = "Register `CR3` writer"]
pub type W = crate::W<Cr3Spec>;
#[doc = "Field `HDSEL` reader - desc HDSEL"]
pub type HdselR = crate::BitReader;
#[doc = "Field `HDSEL` writer - desc HDSEL"]
pub type HdselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOOP` reader - desc LOOP"]
pub type LoopR = crate::BitReader;
#[doc = "Field `LOOP` writer - desc LOOP"]
pub type LoopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTSE` reader - desc RTSE"]
pub type RtseR = crate::BitReader;
#[doc = "Field `RTSE` writer - desc RTSE"]
pub type RtseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSE` reader - desc CTSE"]
pub type CtseR = crate::BitReader;
#[doc = "Field `CTSE` writer - desc CTSE"]
pub type CtseW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - desc HDSEL"]
    #[inline(always)]
    pub fn hdsel(&self) -> HdselR {
        HdselR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc LOOP"]
    #[inline(always)]
    pub fn loop_(&self) -> LoopR {
        LoopR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - desc RTSE"]
    #[inline(always)]
    pub fn rtse(&self) -> RtseR {
        RtseR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc CTSE"]
    #[inline(always)]
    pub fn ctse(&self) -> CtseR {
        CtseR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR3")
            .field("hdsel", &self.hdsel())
            .field("loop_", &self.loop_())
            .field("rtse", &self.rtse())
            .field("ctse", &self.ctse())
            .finish()
    }
}
impl W {
    #[doc = "Bit 3 - desc HDSEL"]
    #[inline(always)]
    pub fn hdsel(&mut self) -> HdselW<'_, Cr3Spec> {
        HdselW::new(self, 3)
    }
    #[doc = "Bit 4 - desc LOOP"]
    #[inline(always)]
    pub fn loop_(&mut self) -> LoopW<'_, Cr3Spec> {
        LoopW::new(self, 4)
    }
    #[doc = "Bit 8 - desc RTSE"]
    #[inline(always)]
    pub fn rtse(&mut self) -> RtseW<'_, Cr3Spec> {
        RtseW::new(self, 8)
    }
    #[doc = "Bit 9 - desc CTSE"]
    #[inline(always)]
    pub fn ctse(&mut self) -> CtseW<'_, Cr3Spec> {
        CtseW::new(self, 9)
    }
}
#[doc = "desc CR3\n\nYou can [`read`](crate::Reg::read) this register and get [`cr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr3Spec;
impl crate::RegisterSpec for Cr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr3::R`](R) reader structure"]
impl crate::Readable for Cr3Spec {}
#[doc = "`write(|w| ..)` method takes [`cr3::W`](W) writer structure"]
impl crate::Writable for Cr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR3 to value 0"]
impl crate::Resettable for Cr3Spec {}
