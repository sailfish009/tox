#![deny(warnings)]


// duckling links
// - https://github.com/wit-ai/duckling_old/blob/6b7e2e1bdbd50299cee4075ff48d7323c05758bc/src/duckling/time/pred.clj#L57-L72
// - https://duckling.wit.ai/#limitations
// - https://github.com/wit-ai/duckling_old/blob/6b7e2e1bdbd50299cee4075ff48d7323c05758bc/src/duckling/time/pred.clj#L333

//
// - interval based (weekend, mon 2.30pm to tues 3am)
//   - monday to friday
//   - afternoon (13hs - 19hs)
//
// - union
//   - (mon, wed and fri)
//   - (mon 2.30pm to tue 1am) and (fri 4 to 5pm)
//
// - intersect
//   - mondays of march
//   - tuesday 29th
//   - Q: what if alignment is not exact?
//   - Q: what if no intersection? empty set FUSE?
//
// - difference
//   - Days except Friday
//
//
// * composite durations: 3hs and 20 minutes -> grains
// - interval
//   - monday to friday (range of 5 days)
// - multiple-base eg: 2 days -> mon+tue, wed+thu, fri+sat ...
//
// filters:
// - ever other month
// - shift-by-2 (eg 2 days after monday)
//



extern crate chrono;

pub type DateTime = chrono::NaiveDateTime;
pub type Date = chrono::NaiveDate;
pub type Duration = chrono::Duration;


// TODO: Fortnight is not aligned to any known frame its just 14 nights
// TOOD: distinguish between Grain and Resolution (that of Range)

#[derive(Debug,PartialEq,Eq,PartialOrd,Ord,Clone,Copy)]
pub enum Grain {
    Second,
    Minute,
    Hour,
    Day,
    Week,
    Month,
    Quarter,
    Half,
    Year,
    Lustrum,
    Decade,
    Century,
    Millenium,
}


// Ranges are right-open intervals of time, ie: [start, end)
#[derive(Clone,Debug,PartialEq)]
pub struct Range {
    pub start: DateTime, // included
    pub end: DateTime,   // excluded
    pub grain: Grain,    // resolution of start/end
}

impl Range {
    pub fn intersect(&self, other: &Range) -> Option<Range> {
        use std::cmp;
        if self.start < other.end && self.end > other.start {
            return Some(Range{
                start: cmp::max(self.start, other.start),
                end: cmp::min(self.end, other.end),
                grain: cmp::min(self.grain, other.grain)
            });
        }
        None
    }

    pub fn len(&self) -> Duration {
        self.end.signed_duration_since(self.start)
    }
}


// TimeSequence is a floating description of a set of time Ranges.
// They can be evaluated in the context of an instant to produce time Ranges.

pub trait TimeSequence<'a> {
    // Resolution of Ranges produced by this sequence
    // TODO: remove or add Mixed
    fn resolution(&self) -> Grain;

    // Yield instances of this sequence into the future.
    // End-time of Ranges must be greater than reference t0 DateTime.
    // NOTE: First Range may start after t0 if for example discontinuous.
    fn _future_raw(&self, t0: &DateTime) -> Box<dyn Iterator<Item=Range> + 'a>;

    // Yield instances of this sequence into the past
    // Start-time of emited Ranges must be less-or-equal than reference t0.
    fn _past_raw(&self, t0: &DateTime) -> Box<Iterator<Item=Range> + 'a>;

    // NOTE: past_raw and future_raw are mainly used internaly.
    // Their first elements may overlap and are needed for composing NthOf.
    // End-user wants future + past which have no overlap in emitted Ranges

    fn future(&self, t0: &DateTime) -> Box<dyn Iterator<Item=Range> + 'a> {
        let t0 = t0.clone();
        Box::new(self._future_raw(&t0)
            .skip_while(move |range| range.end <= t0))
    }

    // End-time of emited Ranges must be less-or-equal than reference DateTime.
    // Complement of "future" where end-time must be greater than t0.
    fn past(&self, t0: &DateTime) -> Box<Iterator<Item=Range> + 'a> {
        let t0 = t0.clone();
        Box::new(self._past_raw(&t0)
            .skip_while(move |range| range.end > t0))
    }
}
