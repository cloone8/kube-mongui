use crate::KubeMonGUI;

pub(crate) mod namespace;
pub(crate) mod pods;
pub(crate) mod nodes;

pub(crate) fn start_all(ui_info: &mut KubeMonGUI) -> Result<(), ()> {
    namespace::start(ui_info)?;
    pods::start(ui_info)?;
    nodes::start(ui_info)?;

    Ok(())
}
