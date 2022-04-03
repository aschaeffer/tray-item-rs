mod api;
mod error;

pub use error::TIError;
use uuid::Uuid;

pub struct TrayItem(api::TrayItemImpl);

#[derive(Clone)]
pub enum IconSource {
    Resource(&'static str),
    #[cfg(target_os = "linux")]
    Data {
        height: i32,
        width: i32,
        data: Vec<u8>,
    },
}

impl IconSource {
    pub fn as_str(&self) -> &str {
        match self {
            IconSource::Resource(res) => res,
            #[allow(unreachable_patterns)]
            _ => unimplemented!(),
        }
    }
}

impl TrayItem {
    pub fn new(title: &str, icon: IconSource) -> Result<Self, TIError> {
        Ok(Self(api::TrayItemImpl::new(title, icon)?))
    }

    pub fn set_icon(&mut self, icon: IconSource) -> Result<(), TIError> {
        self.0.set_icon(icon)
    }

    pub fn add_label(&mut self, id: Uuid, label: &str) -> Result<(), TIError> {
        self.0.add_label(id, label)
    }

    pub fn add_menu_item<F>(&mut self, id: Uuid, label: &str, cb: F) -> Result<(), TIError>
    where
        F: Fn() -> () + Send + Sync + 'static,
    {
        self.0.add_menu_item(id, label, cb)
    }

    pub fn remove(&mut self, id: Uuid) -> Result<(), TIError> {
        self.0.remove(id)
    }

    pub fn inner_mut(&mut self) -> &mut api::TrayItemImpl {
        &mut self.0
    }
}
