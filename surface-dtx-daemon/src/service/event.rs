use crate::logic::CancelReason;
use crate::service::arg::DbusArg;

use dbus::arg::{Append, Dict, RefArg, Variant};



pub enum Event {
    DetachmentInhibited { reason: CancelReason },
    DetachmentStart,
    DetachmentComplete,
    DetachmentTimeout,
    DetachmentCancelStart { reason: CancelReason },
    DetachmentCancelComplete,
    DetachmentCancelTimeout,
    DetachmentUnexpected,
    AttachmentStart,
    AttachmentComplete,
    AttachmentTimeout,
}

impl dbus::arg::AppendAll for Event {
    fn append(&self, ia: &mut dbus::arg::IterAppend) {
        match self {
            Self::DetachmentInhibited { reason }   => append1(ia, "detachment:inhibited", "reason", reason),
            Self::DetachmentStart                  => append0(ia, "detachment:start"),
            Self::DetachmentComplete               => append0(ia, "detachment:complete"),
            Self::DetachmentTimeout                => append0(ia, "detachment:timeout"),
            Self::DetachmentCancelStart { reason } => append1(ia, "detachment:cancel:start", "reason", reason),
            Self::DetachmentCancelComplete         => append0(ia, "detachment:cancel:complete"),
            Self::DetachmentCancelTimeout          => append0(ia, "detachment:cancel:timeout"),
            Self::DetachmentUnexpected             => append0(ia, "detachment:unexpected"),
            Self::AttachmentStart                  => append0(ia, "attachment:start"),
            Self::AttachmentComplete               => append0(ia, "attachment:complete"),
            Self::AttachmentTimeout                => append0(ia, "attachment:timeout"),
        }
    }
}

fn append0(ia: &mut dbus::arg::IterAppend, ty: &'static str) {
    let values: Dict<String, Variant<Box<dyn RefArg>>, _> = Dict::new(std::iter::empty());

    ty.append(ia);
    values.append(ia);
}

fn append1<T>(ia: &mut dbus::arg::IterAppend, ty: &'static str, name: &'static str, value: &T)
where
    T: DbusArg,
{
    ty.append(ia);

    ia.append_dict(&"s".into(), &"v".into(), |ia| {
        ia.append_dict_entry(|ia| {
            ia.append(name.to_owned());
            ia.append(value.as_variant());
        })
    });
}
