use crate::components::{
    Hero, OSSContributionsSection, PinnedProjectsSection, RecentProjectsSection,
    WorkExperience, WorkExperienceSection, Project,
    OSSContribution, 
};
use dioxus::prelude::*;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    // Recent projects data
    let recent_projects = Project::from_recent_projects_json();

    let pinned_projects = Project::from_pinned_projects_json();

    let work_experiences = WorkExperience::from_json();

    let oss_contributions = OSSContribution::from_json();


    rsx! {
        Hero {},
        PinnedProjectsSection { projects: pinned_projects },
        WorkExperienceSection { experiences: work_experiences },
        OSSContributionsSection { contributions: oss_contributions },
        RecentProjectsSection { projects: recent_projects },
    }
}
