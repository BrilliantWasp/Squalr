use anyhow::{Result};
use squalr_engine_api::{
    commands::unprivileged_command::{UnprivilegedCommand},
    commands::project::create::project_create_request::{ProjectCreateRequest},
};

#[tokio::test]
async fn test_project_create_command() -> Result<()> {
    let project_name = "test_project_bwasp".to_string();
    let project_create_request = ProjectCreateRequest { project_name: project_name.clone() };

    let command = UnprivilegedCommand::Project(squalr_engine_api::commands::project::project_command::ProjectCommand::Create {
        project_create_request,
    });

    // In a real scenario, you would send this command to the engine
    // and then assert the response. For now, we're just testing command construction.
    println!("Created command: {:?}", command);

    // Assert that the command was constructed correctly (e.g., check the variant)
    match command {
        UnprivilegedCommand::Project(squalr_engine_api::commands::project::project_command::ProjectCommand::Create { project_create_request }) => {
            assert_eq!(project_create_request.project_name, "test_project_bwasp");
        }
        _ => panic!("Unexpected command type"),
    }

    Ok(())
}
