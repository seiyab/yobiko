use crate::model::Task;

use super::components::LaunchMode;

pub(crate) enum Action {
    Quit,
    ToggleHelp,
    OpenDashboard,
    Run(Task),
    SelectOneShot(Task),
    Edit(Task, LaunchMode),
    Kill(usize),
}
