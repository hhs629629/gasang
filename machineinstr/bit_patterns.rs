use crate::utils::extract_bits32;

use num_traits::FromPrimitive;

use std::marker::PhantomData;
use std::ops::Range;

pub struct BitPatternMatcher<O> {
    matches: Vec<Box<dyn MatchHelper<Output = Option<O>> + Send + Sync + 'static>>,
}

impl<O> BitPatternMatcher<O> {
    pub fn new() -> Self {
        Self {
            matches: Vec::new(),
        }
    }

    pub fn bind<H, A>(&mut self, pattern: &str, handler: H) -> &mut Self
    where
        A: Send + Sync + 'static,
        H: Send + Sync + 'static,
        H: Handler<A, Output = O>,
    {
        self.matches.push(Box::new(Match::new(pattern, handler)));
        self
    }

    pub fn handle(&self, raw_instr: u32) -> Option<O> {
        for matcher in &self.matches {
            if let Some(result) = matcher.handle(raw_instr) {
                return Some(result);
            }
        }
        None
    }
}

trait MatchHelper {
    type Output;
    fn handle(&self, raw_instr: u32) -> Self::Output;
}

impl<H, A> MatchHelper for Match<H, A>
where
    H: Handler<A>,
{
    type Output = Option<H::Output>;

    fn handle(&self, raw_instr: u32) -> Self::Output {
        self.handle(raw_instr)
    }
}

pub struct Match<H, A> {
    pattern: u32,
    mask: u32,
    handler: H,
    __p: PhantomData<A>,
}

impl<H, A> Match<H, A>
where
    H: Handler<A>,
{
    pub fn new<'s>(pattern: &'s str, handler: H) -> Self {
        let (pattern, mask) = Self::parse_pattern(pattern);

        Self {
            pattern,
            mask,
            handler,
            __p: PhantomData,
        }
    }

    pub fn handle(&self, raw_instr: u32) -> Option<H::Output> {
        if (!(raw_instr ^ self.pattern) & self.mask) == self.mask {
            Some(self.handler.handle(raw_instr))
        } else {
            None
        }
    }

    fn parse_pattern(pattern: &str) -> (u32, u32) {
        let mut pattern_result = 0b0;
        let mut mask_result = 0b0;

        for char in pattern.chars() {
            let (pat, mask) = match char {
                'x' => (0, 0),
                '0' => (0, 1),
                '1' => (1, 1),
                '_' | ' ' => {
                    continue;
                }
                _ => unreachable!("Bad parse pattern!"),
            };
            pattern_result <<= 1;
            pattern_result |= pat;

            mask_result <<= 1;
            mask_result |= mask;
        }

        (pattern_result, mask_result)
    }
}

trait BitRangeHelper {
    fn range() -> Range<usize>;
}

pub struct BitRange<const B: usize, const E: usize>();
impl<const B: usize, const E: usize> BitRangeHelper for BitRange<B, E> {
    fn range() -> Range<usize> {
        B..E
    }
}

pub struct Extract<R, T: FromPrimitive> {
    __p: PhantomData<R>,
    pub value: T,
}

pub trait Handler<Args> {
    type Output;

    fn handle(&self, raw_instr: u32) -> Self::Output;
}

impl<F, O> Handler<()> for F
where
    F: Fn(u32) -> O,
{
    type Output = O;

    fn handle(&self, raw_instr: u32) -> Self::Output {
        (self)(raw_instr)
    }
}

impl<F, O, R1, T1> Handler<(R1, T1)> for F
where
    F: Fn(u32, Extract<R1, T1>) -> O,
    R1: BitRangeHelper,
    T1: FromPrimitive,
{
    type Output = O;

    fn handle(&self, raw_instr: u32) -> Self::Output {
        let op1: u64 = extract_bits32(R1::range(), raw_instr) as u64;
        (self)(
            raw_instr,
            Extract {
                __p: PhantomData,
                value: T1::from_u64(op1).unwrap(),
            },
        )
    }
}

impl<F, O, R1, T1, R2, T2> Handler<(R1, T1, R2, T2)> for F
where
    F: Fn(u32, Extract<R1, T1>, Extract<R2, T2>) -> O,
    R1: BitRangeHelper,
    R2: BitRangeHelper,
    T1: FromPrimitive,
    T2: FromPrimitive,
{
    type Output = O;

    fn handle(&self, raw_instr: u32) -> Self::Output {
        let op1: u64 = extract_bits32(R1::range(), raw_instr) as u64;
        let op2: u64 = extract_bits32(R2::range(), raw_instr) as u64;
        (self)(
            raw_instr,
            Extract {
                __p: PhantomData,
                value: T1::from_u64(op1).unwrap(),
            },
            Extract {
                __p: PhantomData,
                value: T2::from_u64(op2).unwrap(),
            },
        )
    }
}

impl<F, O, R1, T1, R2, T2, R3, T3> Handler<(R1, T1, R2, T2, R3, T3)> for F
where
    F: Fn(u32, Extract<R1, T1>, Extract<R2, T2>, Extract<R3, T3>) -> O,
    R1: BitRangeHelper,
    T1: FromPrimitive,
    R2: BitRangeHelper,
    T2: FromPrimitive,
    R3: BitRangeHelper,
    T3: FromPrimitive,
{
    type Output = O;

    fn handle(&self, raw_instr: u32) -> Self::Output {
        let op1: u64 = extract_bits32(R1::range(), raw_instr) as u64;
        let op2: u64 = extract_bits32(R2::range(), raw_instr) as u64;
        let op3: u64 = extract_bits32(R3::range(), raw_instr) as u64;

        (self)(
            raw_instr,
            Extract {
                __p: PhantomData,
                value: T1::from_u64(op1).unwrap(),
            },
            Extract {
                __p: PhantomData,
                value: T2::from_u64(op2).unwrap(),
            },
            Extract {
                __p: PhantomData,
                value: T3::from_u64(op3).unwrap(),
            },
        )
    }
}

impl<F, O, R1, T1, R2, T2, R3, T3, R4, T4> Handler<(R1, T1, R2, T2, R3, T3, R4, T4)> for F
where
    F: Fn(u32, Extract<R1, T1>, Extract<R2, T2>, Extract<R3, T3>, Extract<R4, T4>) -> O,
    R1: BitRangeHelper,
    T1: FromPrimitive,
    R2: BitRangeHelper,
    T2: FromPrimitive,
    R3: BitRangeHelper,
    T3: FromPrimitive,
    R4: BitRangeHelper,
    T4: FromPrimitive,
{
    type Output = O;

    fn handle(&self, raw_instr: u32) -> Self::Output {
        let op1: u64 = extract_bits32(R1::range(), raw_instr) as u64;
        let op2: u64 = extract_bits32(R2::range(), raw_instr) as u64;
        let op3: u64 = extract_bits32(R3::range(), raw_instr) as u64;
        let op4: u64 = extract_bits32(R4::range(), raw_instr) as u64;

        (self)(
            raw_instr,
            Extract {
                __p: PhantomData,
                value: T1::from_u64(op1).unwrap(),
            },
            Extract {
                __p: PhantomData,
                value: T2::from_u64(op2).unwrap(),
            },
            Extract {
                __p: PhantomData,
                value: T3::from_u64(op3).unwrap(),
            },
            Extract {
                __p: PhantomData,
                value: T4::from_u64(op4).unwrap(),
            },
        )
    }
}

impl<F, O, R1, T1, R2, T2, R3, T3, R4, T4, R5, T5> Handler<(R1, T1, R2, T2, R3, T3, R4, T4, R5, T5)>
    for F
where
    F: Fn(
        u32,
        Extract<R1, T1>,
        Extract<R2, T2>,
        Extract<R3, T3>,
        Extract<R4, T4>,
        Extract<R5, T5>,
    ) -> O,
    R1: BitRangeHelper,
    T1: FromPrimitive,
    R2: BitRangeHelper,
    T2: FromPrimitive,
    R3: BitRangeHelper,
    T3: FromPrimitive,
    R4: BitRangeHelper,
    T4: FromPrimitive,
    R5: BitRangeHelper,
    T5: FromPrimitive,
{
    type Output = O;

    fn handle(&self, raw_instr: u32) -> Self::Output {
        let op1: u64 = extract_bits32(R1::range(), raw_instr) as u64;
        let op2: u64 = extract_bits32(R2::range(), raw_instr) as u64;
        let op3: u64 = extract_bits32(R3::range(), raw_instr) as u64;
        let op4: u64 = extract_bits32(R4::range(), raw_instr) as u64;
        let op5: u64 = extract_bits32(R5::range(), raw_instr) as u64;

        (self)(
            raw_instr,
            Extract {
                __p: PhantomData,
                value: T1::from_u64(op1).unwrap(),
            },
            Extract {
                __p: PhantomData,
                value: T2::from_u64(op2).unwrap(),
            },
            Extract {
                __p: PhantomData,
                value: T3::from_u64(op3).unwrap(),
            },
            Extract {
                __p: PhantomData,
                value: T4::from_u64(op4).unwrap(),
            },
            Extract {
                __p: PhantomData,
                value: T5::from_u64(op5).unwrap(),
            },
        )
    }
}

impl<F, O, R1, T1, R2, T2, R3, T3, R4, T4, R5, T5, R6, T6>
    Handler<(R1, T1, R2, T2, R3, T3, R4, T4, R5, T5, R6, T6)> for F
where
    F: Fn(
        u32,
        Extract<R1, T1>,
        Extract<R2, T2>,
        Extract<R3, T3>,
        Extract<R4, T4>,
        Extract<R5, T5>,
        Extract<R6, T6>,
    ) -> O,
    R1: BitRangeHelper,
    T1: FromPrimitive,
    R2: BitRangeHelper,
    T2: FromPrimitive,
    R3: BitRangeHelper,
    T3: FromPrimitive,
    R4: BitRangeHelper,
    T4: FromPrimitive,
    R5: BitRangeHelper,
    T5: FromPrimitive,
    R6: BitRangeHelper,
    T6: FromPrimitive,
{
    type Output = O;

    fn handle(&self, raw_instr: u32) -> Self::Output {
        let op1: u64 = extract_bits32(R1::range(), raw_instr) as u64;
        let op2: u64 = extract_bits32(R2::range(), raw_instr) as u64;
        let op3: u64 = extract_bits32(R3::range(), raw_instr) as u64;
        let op4: u64 = extract_bits32(R4::range(), raw_instr) as u64;
        let op5: u64 = extract_bits32(R5::range(), raw_instr) as u64;
        let op6: u64 = extract_bits32(R6::range(), raw_instr) as u64;

        (self)(
            raw_instr,
            Extract {
                __p: PhantomData,
                value: T1::from_u64(op1).unwrap(),
            },
            Extract {
                __p: PhantomData,
                value: T2::from_u64(op2).unwrap(),
            },
            Extract {
                __p: PhantomData,
                value: T3::from_u64(op3).unwrap(),
            },
            Extract {
                __p: PhantomData,
                value: T4::from_u64(op4).unwrap(),
            },
            Extract {
                __p: PhantomData,
                value: T5::from_u64(op5).unwrap(),
            },
            Extract {
                __p: PhantomData,
                value: T6::from_u64(op6).unwrap(),
            },
        )
    }
}


impl<F, O, R1, T1, R2, T2, R3, T3, R4, T4, R5, T5, R6, T6, R7, T7>
    Handler<(R1, T1, R2, T2, R3, T3, R4, T4, R5, T5, R6, T6, R7, T7)> for F
where
    F: Fn(
        u32,
        Extract<R1, T1>,
        Extract<R2, T2>,
        Extract<R3, T3>,
        Extract<R4, T4>,
        Extract<R5, T5>,
        Extract<R6, T6>,
        Extract<R7, T7>,
    ) -> O,
    R1: BitRangeHelper,
    T1: FromPrimitive,
    R2: BitRangeHelper,
    T2: FromPrimitive,
    R3: BitRangeHelper,
    T3: FromPrimitive,
    R4: BitRangeHelper,
    T4: FromPrimitive,
    R5: BitRangeHelper,
    T5: FromPrimitive,
    R6: BitRangeHelper,
    T6: FromPrimitive,
    R7: BitRangeHelper,
    T7: FromPrimitive,
{
    type Output = O;

    fn handle(&self, raw_instr: u32) -> Self::Output {
        let op1: u64 = extract_bits32(R1::range(), raw_instr) as u64;
        let op2: u64 = extract_bits32(R2::range(), raw_instr) as u64;
        let op3: u64 = extract_bits32(R3::range(), raw_instr) as u64;
        let op4: u64 = extract_bits32(R4::range(), raw_instr) as u64;
        let op5: u64 = extract_bits32(R5::range(), raw_instr) as u64;
        let op6: u64 = extract_bits32(R6::range(), raw_instr) as u64;
        let op7: u64 = extract_bits32(R7::range(), raw_instr) as u64;

        (self)(
            raw_instr,
            Extract {
                __p: PhantomData,
                value: T1::from_u64(op1).unwrap(),
            },
            Extract {
                __p: PhantomData,
                value: T2::from_u64(op2).unwrap(),
            },
            Extract {
                __p: PhantomData,
                value: T3::from_u64(op3).unwrap(),
            },
            Extract {
                __p: PhantomData,
                value: T4::from_u64(op4).unwrap(),
            },
            Extract {
                __p: PhantomData,
                value: T5::from_u64(op5).unwrap(),
            },
            Extract {
                __p: PhantomData,
                value: T6::from_u64(op6).unwrap(),
            },
            Extract {
                __p: PhantomData,
                value: T7::from_u64(op7).unwrap(),
            },
        )
    }
}


impl<F, O, R1, T1, R2, T2, R3, T3, R4, T4, R5, T5, R6, T6, R7, T7, R8, T8>
    Handler<(R1, T1, R2, T2, R3, T3, R4, T4, R5, T5, R6, T6, R7, T7, R8, T8)> for F
where
    F: Fn(
        u32,
        Extract<R1, T1>,
        Extract<R2, T2>,
        Extract<R3, T3>,
        Extract<R4, T4>,
        Extract<R5, T5>,
        Extract<R6, T6>,
        Extract<R7, T7>,
        Extract<R8, T8>,
    ) -> O,
    R1: BitRangeHelper,
    T1: FromPrimitive,
    R2: BitRangeHelper,
    T2: FromPrimitive,
    R3: BitRangeHelper,
    T3: FromPrimitive,
    R4: BitRangeHelper,
    T4: FromPrimitive,
    R5: BitRangeHelper,
    T5: FromPrimitive,
    R6: BitRangeHelper,
    T6: FromPrimitive,
    R7: BitRangeHelper,
    T7: FromPrimitive,
    R8: BitRangeHelper,
    T8: FromPrimitive,
{
    type Output = O;

    fn handle(&self, raw_instr: u32) -> Self::Output {
        let op1: u64 = extract_bits32(R1::range(), raw_instr) as u64;
        let op2: u64 = extract_bits32(R2::range(), raw_instr) as u64;
        let op3: u64 = extract_bits32(R3::range(), raw_instr) as u64;
        let op4: u64 = extract_bits32(R4::range(), raw_instr) as u64;
        let op5: u64 = extract_bits32(R5::range(), raw_instr) as u64;
        let op6: u64 = extract_bits32(R6::range(), raw_instr) as u64;
        let op7: u64 = extract_bits32(R7::range(), raw_instr) as u64;
        let op8: u64 = extract_bits32(R8::range(), raw_instr) as u64;

        (self)(
            raw_instr,
            Extract {
                __p: PhantomData,
                value: T1::from_u64(op1).unwrap(),
            },
            Extract {
                __p: PhantomData,
                value: T2::from_u64(op2).unwrap(),
            },
            Extract {
                __p: PhantomData,
                value: T3::from_u64(op3).unwrap(),
            },
            Extract {
                __p: PhantomData,
                value: T4::from_u64(op4).unwrap(),
            },
            Extract {
                __p: PhantomData,
                value: T5::from_u64(op5).unwrap(),
            },
            Extract {
                __p: PhantomData,
                value: T6::from_u64(op6).unwrap(),
            },
            Extract {
                __p: PhantomData,
                value: T7::from_u64(op7).unwrap(),
            },
            Extract {
                __p: PhantomData,
                value: T8::from_u64(op8).unwrap(),
            },
        )
    }
}


impl<F, O, R1, T1, R2, T2, R3, T3, R4, T4, R5, T5, R6, T6, R7, T7, R8, T8, R9, T9>
    Handler<(R1, T1, R2, T2, R3, T3, R4, T4, R5, T5, R6, T6, R7, T7, R8, T8, R9, T9)> for F
where
    F: Fn(
        u32,
        Extract<R1, T1>,
        Extract<R2, T2>,
        Extract<R3, T3>,
        Extract<R4, T4>,
        Extract<R5, T5>,
        Extract<R6, T6>,
        Extract<R7, T7>,
        Extract<R8, T8>,
        Extract<R9, T9>,
    ) -> O,
    R1: BitRangeHelper,
    T1: FromPrimitive,
    R2: BitRangeHelper,
    T2: FromPrimitive,
    R3: BitRangeHelper,
    T3: FromPrimitive,
    R4: BitRangeHelper,
    T4: FromPrimitive,
    R5: BitRangeHelper,
    T5: FromPrimitive,
    R6: BitRangeHelper,
    T6: FromPrimitive,
    R7: BitRangeHelper,
    T7: FromPrimitive,
    R8: BitRangeHelper,
    T8: FromPrimitive,
    R9: BitRangeHelper,
    T9: FromPrimitive,
{
    type Output = O;

    fn handle(&self, raw_instr: u32) -> Self::Output {
        let op1: u64 = extract_bits32(R1::range(), raw_instr) as u64;
        let op2: u64 = extract_bits32(R2::range(), raw_instr) as u64;
        let op3: u64 = extract_bits32(R3::range(), raw_instr) as u64;
        let op4: u64 = extract_bits32(R4::range(), raw_instr) as u64;
        let op5: u64 = extract_bits32(R5::range(), raw_instr) as u64;
        let op6: u64 = extract_bits32(R6::range(), raw_instr) as u64;
        let op7: u64 = extract_bits32(R7::range(), raw_instr) as u64;
        let op8: u64 = extract_bits32(R8::range(), raw_instr) as u64;
        let op9: u64 = extract_bits32(R9::range(), raw_instr) as u64;

        (self)(
            raw_instr,
            Extract {
                __p: PhantomData,
                value: T1::from_u64(op1).unwrap(),
            },
            Extract {
                __p: PhantomData,
                value: T2::from_u64(op2).unwrap(),
            },
            Extract {
                __p: PhantomData,
                value: T3::from_u64(op3).unwrap(),
            },
            Extract {
                __p: PhantomData,
                value: T4::from_u64(op4).unwrap(),
            },
            Extract {
                __p: PhantomData,
                value: T5::from_u64(op5).unwrap(),
            },
            Extract {
                __p: PhantomData,
                value: T6::from_u64(op6).unwrap(),
            },
            Extract {
                __p: PhantomData,
                value: T7::from_u64(op7).unwrap(),
            },
            Extract {
                __p: PhantomData,
                value: T8::from_u64(op8).unwrap(),
            },
            Extract {
                __p: PhantomData,
                value: T9::from_u64(op9).unwrap(),
            },
        )
    }
}