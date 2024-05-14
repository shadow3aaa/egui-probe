use core::ops::{RangeFrom, RangeFull, RangeInclusive, RangeToInclusive};

use crate::{option::option_probe_with, EguiProbe, Style};

/// Bundles value and a range to show probbing UI to edit the value in that range.
pub struct EguiProbeRange<'a, T, R> {
    pub value: &'a mut T,
    pub range: R,
}

pub fn non_negative<'a>(value: &'a mut f32) -> EguiProbeRange<'a, f32, RangeFrom<f32>> {
    EguiProbeRange {
        value,
        range: 0.0..,
    }
}

// pub fn range_from<'a, T>(value: &'a mut T, from: T) -> EguiProbeRange<'a, T, RangeFrom<T>> {
//     EguiProbeRange {
//         value,
//         range: from..,
//     }
// }

// pub fn range_to<'a, T>(value: &'a mut T, to: T) -> EguiProbeRange<'a, T, RangeToInclusive<T>> {
//     EguiProbeRange {
//         value,
//         range: ..=to,
//     }
// }

// pub fn range<'a, T>(
//     value: &'a mut T,
//     range: RangeInclusive<T>,
// ) -> EguiProbeRange<'a, T, RangeInclusive<T>> {
//     EguiProbeRange { value, range }
// }

macro_rules! impl_for_num_types {
    ($num_type:ident) => {
        impl EguiProbe for $num_type {
            #[inline(always)]
            fn probe(&mut self, ui: &mut egui::Ui, _: &Style) -> egui::Response {
                ui.add(egui::DragValue::new(self))
            }
        }

        impl EguiProbe for EguiProbeRange<'_, $num_type, RangeFull> {
            #[inline(always)]
            fn probe(&mut self, ui: &mut egui::Ui, _: &Style) -> egui::Response {
                let range = $num_type::MIN..=$num_type::MAX;
                ui.add(egui::DragValue::new(self.value).clamp_range(range))
            }
        }

        impl EguiProbe for EguiProbeRange<'_, $num_type, RangeFrom<$num_type>> {
            #[inline(always)]
            fn probe(&mut self, ui: &mut egui::Ui, _: &Style) -> egui::Response {
                let range = self.range.start..=$num_type::MAX;
                let mut changed = false;
                let mut r = ui.horizontal(|ui| {
                    changed |= ui.add(egui::DragValue::new(self.value).clamp_range(range)).changed();
                    ui.weak(format!("{}..", self.range.start));
                }).response;

                if changed {
                    r.mark_changed();
                }

                r
            }
        }

        impl EguiProbe for EguiProbeRange<'_, $num_type, RangeToInclusive<$num_type>> {
            #[inline(always)]
            fn probe(&mut self, ui: &mut egui::Ui, _: &Style) -> egui::Response {
                let range = $num_type::MIN..=self.range.end;
                let mut changed = false;
                let mut r = ui.horizontal(|ui| {
                    changed |= ui.add(egui::DragValue::new(self.value).clamp_range(range)).changed();
                    ui.weak(format!("..={}", self.range.end));
                }).response;

                if changed {
                    r.mark_changed();
                }

                r
            }
        }

        impl EguiProbe for EguiProbeRange<'_, $num_type, RangeInclusive<$num_type>> {
            #[inline(always)]
            fn probe(&mut self, ui: &mut egui::Ui, _: &Style) -> egui::Response {
                let range = self.range.clone();
                let mut changed = false;
                let mut r = ui.horizontal(|ui| {
                    changed |= ui.add(egui::DragValue::new(self.value).clamp_range(range)).changed();
                    ui.weak(format!("{}..={}", self.range.start(), self.range.end()));
                }).response;

                if changed {
                    r.mark_changed();
                }

                r
            }
        }

        impl EguiProbe for EguiProbeRange<'_, Option<$num_type>, RangeFull> {
            #[inline(always)]
            fn probe(&mut self, ui: &mut egui::Ui, style: &Style) -> egui::Response {
                let range = $num_type::MIN..=$num_type::MAX;
                option_probe_with(self.value, ui, style, $num_type::default, |value, ui, _| {
                    ui.add(egui::DragValue::new(value).clamp_range(range))
                })
            }
        }

        impl EguiProbe for EguiProbeRange<'_, Option<$num_type>, RangeFrom<$num_type>> {
            #[inline(always)]
            fn probe(&mut self, ui: &mut egui::Ui, style: &Style) -> egui::Response {
                let range = self.range.start..=$num_type::MAX;
                option_probe_with(self.value, ui, style, $num_type::default, |value, ui, _| {
                    let r = ui.add(egui::DragValue::new(value).clamp_range(range));
                    ui.weak(format!("{}..", self.range.start));
                    r
                })
            }
        }

        impl EguiProbe for EguiProbeRange<'_, Option<$num_type>, RangeToInclusive<$num_type>> {
            #[inline(always)]
            fn probe(&mut self, ui: &mut egui::Ui, style: &Style) -> egui::Response {
                let range = $num_type::MIN..=self.range.end;
                option_probe_with(self.value, ui, style, $num_type::default, |value, ui, _| {
                    let r = ui.add(egui::DragValue::new(value).clamp_range(range));
                    ui.weak(format!("..={}", self.range.end));
                    r
                })
            }
        }

        impl EguiProbe for EguiProbeRange<'_, Option<$num_type>, RangeInclusive<$num_type>> {
            #[inline(always)]
            fn probe(&mut self, ui: &mut egui::Ui, style: &Style) -> egui::Response {
                let range = self.range.clone();
                option_probe_with(self.value, ui, style, $num_type::default, |value, ui, _| {
                    let r = ui.add(egui::DragValue::new(value).clamp_range(range));
                    ui.weak(format!("{}..={}", self.range.start(), self.range.end()));
                    r
                })
            }
        }
    };

    ($($num_type:ident),*) => {
        $(impl_for_num_types!($num_type);)*
    };
}

impl_for_num_types!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, f32, f64);
