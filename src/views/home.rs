use crate::components::{
    Hero, OSSContributionsSection, ProjectsSection,
    WorkExperience, WorkExperienceSection, Project, RecentProjects, RecentProject,
    OSSContribution, 
};
use dioxus::prelude::*;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    // Recent projects data
    let recent_projects = RecentProject::from_json();

    let work_experiences = WorkExperience::from_json();

    let oss_contributions = OSSContribution::from_json();

    let projects = Project::from_json();

    rsx! {
        Hero {},
        RecentProjects { recent_projects: recent_projects },
        WorkExperienceSection { experiences: work_experiences },
        OSSContributionsSection { contributions: oss_contributions },
        ProjectsSection { projects: projects },
    }
}
