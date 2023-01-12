use crate::KubeMonGUI;

pub(crate) mod cronjobs;
pub(crate) mod namespace;
pub(crate) mod nodes;
pub(crate) mod pods;

pub(crate) fn start_all(ui_info: &mut KubeMonGUI) -> Result<(), ()> {
    cronjobs::start(ui_info)?;
    namespace::start(ui_info)?;
    nodes::start(ui_info)?;
    pods::start(ui_info)?;

    Ok(())
}
