use crate::model::Task;

pub(crate) enum Action {
    Quit,
    ToggleHelp,
    OpenDashboard,
    Run(Task),
    SelectOneShot(Task),
    Kill(usize),
}
