use lapce_xi_rope::Interval;
use lapce_xi_rope::{DeltaElement, Rope, RopeDelta};
use anyhow::Result;
use log::debug;

#[derive(Copy, Clone, Debug, Default)]
pub enum Offset {
    #[default]
    None,
    Add(usize),
    Minus(usize),
}

impl Offset {
    pub fn new(origin: usize, new: usize) -> Self {
        if origin > new {
            Self::minus(origin - new)
        } else {
            Self::add(new - origin)
        }
    }
    pub fn add(offset: usize) -> Self {
        if offset == 0 {
            Self::None
        } else {
            Self::Add(offset)
        }
    }
    pub fn minus(offset: usize) -> Self {
        if offset == 0 {
            Self::None
        } else {
            Self::Minus(offset)
        }
    }

    pub fn adjust(&self, num: &mut usize) {
        match self {
            Offset::None => {}
            Offset::Add(offset) => { *num += *offset}
            Offset::Minus(offset) => {*num -= offset}
        }
    }

    pub fn adjust_new(&self, num: usize) -> usize {
        match self {
            Offset::None => {num}
            Offset::Add(offset) => { num + *offset}
            Offset::Minus(offset) => {num - offset}
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct OffsetDelta {
    pub copy_start: Interval,
    pub internal_len: usize,
    pub copy_end: Interval,
}

impl Default for OffsetDelta {
    fn default() -> Self {
        Self {
            copy_start: Interval::new(0, 0),
            internal_len: 0,
            copy_end: Interval::new(0, 0),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct OriginLinesDelta {
    pub copy_line_start: CopyStartDelta,
    pub internal_len: usize,
    pub recompute_line_start: usize,
    pub recompute_offset_end: usize,
    pub copy_line_end: CopyEndDelta,
}

impl OriginLinesDelta {
    pub fn delta(&self) -> (bool, Offset, Offset, Interval, usize, usize, Interval, Offset, Offset, bool) {
        (self.copy_line_start.recompute_first_line, self.copy_line_start.offset, self.copy_line_start.line_offset, self.copy_line_start.copy_line, self.recompute_line_start, self.recompute_offset_end, self.copy_line_end.copy_line, self.copy_line_end.offset, self.copy_line_end.line_offset,self.copy_line_end.recompute_last_line)
    }
}

pub fn origin_lines_delta(line_delta: &Option<OriginLinesDelta>) -> (bool, Offset, Offset, Interval, usize, usize, Interval, Offset, Offset,bool) {
    match line_delta {
        None => {(false, Offset::None, Offset::None, Interval::new(0, 0), 0, usize::MAX, Interval::new(0, 0), Offset::None, Offset::None, false)}
        Some(val) => {val.delta()}
    }
}

#[derive(Copy, Clone, Debug)]
pub struct CopyStartDelta {
    /// 首行如果不完整则需要重新计算
    pub recompute_first_line: bool,
    /// 相对的旧buffer的偏移
    pub offset: Offset,
    /// 相对的旧buffer的偏移
    pub line_offset: Offset,
    pub internal_len: usize,
    pub copy_line: Interval,
}
#[derive(Copy, Clone, Debug)]
pub struct CopyEndDelta {
    pub offset: Offset,
    pub line_offset: Offset,
    pub recompute_last_line: bool,
    pub copy_line: Interval,
}
pub fn resolve_delta_rs(
    rope: &Rope,
    delta: &RopeDelta
) -> Result<OriginLinesDelta> {
    let delta_compute = resolve_delta_compute(delta).unwrap();
    debug!("{delta_compute:?}");
    resolve_line_delta(rope, delta_compute)
}

fn resolve_line_delta(
    rope: &Rope,
    offset_delta_compute: OffsetDelta,
) -> Result<OriginLinesDelta> {
    let mut copy_line_start= CopyStartDelta {
        recompute_first_line: true,
        internal_len: 0,
        offset: Offset::None,
        copy_line: Interval::new(0, 0),
        line_offset: Default::default(),
    };
    let mut offset_end = 0;
    let mut global_internal_len = offset_delta_compute.internal_len;
    let mut line_start = 0;
    if !offset_delta_compute.copy_start.is_empty() {
        let copy_line_start_info = resolve_line_complete_by_start_offset(rope, offset_delta_compute.copy_start.start)?;
        let copy_line_end_info = resolve_line_complete_by_end_offset(rope, offset_delta_compute.copy_start.end)?;
        offset_end += offset_delta_compute.copy_start.size();
        if copy_line_end_info.0 > copy_line_start_info.0 {
            let recompute_first_line = copy_line_start_info.2;
            let internal_len = copy_line_end_info.1 - copy_line_start_info.1;
            let copy_line = Interval::new(copy_line_start_info.0, copy_line_end_info.0);

            if recompute_first_line {
                line_start += 1;
            }
            let line_offset = Offset::new(copy_line_start_info.0, line_start);
            copy_line_start = CopyStartDelta {
                recompute_first_line,
                offset: Offset::minus(offset_delta_compute.copy_start.start),
                line_offset,
                internal_len,
                copy_line,
            };
            line_start += copy_line_end_info.0 - copy_line_start_info.0;
            global_internal_len += offset_delta_compute.copy_start.end - copy_line_end_info.1;
        } else {
            global_internal_len += offset_delta_compute.copy_start.size();
        }
    }
    offset_end += offset_delta_compute.internal_len;
    let mut copy_line_end= CopyEndDelta {
        recompute_last_line: true,
        offset: Offset::None,
        copy_line: Interval::new(0, 0),
        line_offset: Default::default(),
    };
    if !offset_delta_compute.copy_end.is_empty() {
        let copy_line_start_info = resolve_line_complete_by_start_offset(rope, offset_delta_compute.copy_end.start)?;
        let copy_line_end_info = resolve_line_complete_by_end_offset(rope, offset_delta_compute.copy_end.end)?;
        if copy_line_end_info.0 > copy_line_start_info.0 {
            // offset_end += copy_line_start_info.1 - offset_delta_compute.copy_end.start;
            let recompute_last_line = copy_line_end_info.2;
            let copy_line =  if copy_line_end_info.2 {
                Interval::new(copy_line_start_info.0, copy_line_end_info.0)
            } else {
                Interval::new(copy_line_start_info.0, copy_line_end_info.0 + 1)
            };
            offset_end += copy_line_start_info.1 - offset_delta_compute.copy_end.start;
            copy_line_end = CopyEndDelta {
                offset: Offset::new(copy_line_start_info.1, offset_end),
                copy_line,
                recompute_last_line,
                line_offset: Default::default(),
            };
            global_internal_len += copy_line_start_info.1 - offset_delta_compute.copy_end.start;
        } else {
            global_internal_len += offset_delta_compute.copy_end.size();
            offset_end += offset_delta_compute.copy_end.size();
        }
    }
    Ok(OriginLinesDelta {
        copy_line_start,
        internal_len: global_internal_len,
        recompute_line_start: line_start,
        recompute_offset_end: offset_end,
        copy_line_end,
    })
}

/// return (line, offset_line, recompute)
fn resolve_line_complete_by_start_offset(
    rope: &Rope,
    offset: usize,
) -> Result<(usize, usize, bool)> {
    let mut line = rope.line_of_offset(offset);
    let mut line_offset = rope.offset_of_line(line)?;
    let recompute = offset != line_offset;
    if recompute {
        line += 1;
        line_offset = rope.offset_of_line(line)?;
    }
    Ok((line, line_offset, recompute))
}

/// return (line, offset_line, recompute)
fn resolve_line_complete_by_end_offset(
    rope: &Rope,
    offset: usize,
) -> Result<(usize, usize, bool)> {
    let line = rope.line_of_offset(offset);
    let offset_line = rope.offset_of_line(line)?;
    let recompute = offset != offset_line;
    Ok((line, offset_line, recompute))
}

fn resolve_delta_compute(
    delta: &RopeDelta
) -> Option<OffsetDelta> {
    let mut rs = OffsetDelta::default();
    let len = delta.els.len();
    debug!("{:?}", delta);
    match len {
        0 => {
        }
        1 => {
            let first = delta.els.first()?;
            match first {
                DeltaElement::Copy(start, end) => {
                    rs.copy_start = Interval::new(*start, *end);
                }
                DeltaElement::Insert(val) => {
                    rs.internal_len = val.len();
                }
            }
        }
        _ => {
            let (first,last) = (delta.els.first()?, delta.els.last()?);
            match (first, last) {
                (DeltaElement::Copy(start, end), DeltaElement::Copy(start_end, end_end)) => {
                    rs.copy_start = Interval::new(*start, *end);
                    rs.copy_end = Interval::new(*start_end, *end_end);
                }
                (DeltaElement::Copy(start, end), DeltaElement::Insert(val_end)) => {
                    rs.copy_start = Interval::new(*start, *end);
                    rs.internal_len = val_end.len();
                }
                (DeltaElement::Insert(val), DeltaElement::Copy(start_end, end_end)) => {
                    rs.internal_len = val.len();
                    rs.copy_end = Interval::new(*start_end, *end_end);
                }
                (DeltaElement::Insert(val), DeltaElement::Insert(val_end)) => {
                    rs.internal_len = val.len() + val_end.len();
                }
            }
            if len > 2 {
                let iter = delta.els[1..len-1].iter();
                for delta in iter {
                    match delta {
                        DeltaElement::Copy(start, end) => {
                            rs.internal_len += *end - *start;
                        }
                        DeltaElement::Insert(val) => {
                            rs.internal_len += val.len();
                        }
                    }
                }
            }

        }
    }
    Some(rs)
}