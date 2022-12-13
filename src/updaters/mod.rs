use crate::KubeMonGUI;

pub(crate) mod namespace;
pub(crate) mod pods;

pub(crate) fn start_all(ui_info: &mut KubeMonGUI) -> Result<(), ()> {
    namespace::start(ui_info)?;
    pods::start(ui_info)?;

    Ok(())
}
