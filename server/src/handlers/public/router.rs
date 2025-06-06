use std::sync::Arc;
use strum::EnumCount;

use crate::handlers::public::{
    devices::{
        __path_get_device_operating_system_versions, __path_get_devices,
        get_device_operating_system_versions, get_devices,
    },
    entitlements::{__path_diff_entitlements_for_executables, diff_entitlements_for_executables},
    executables::{
        __path_diff_executables_for_versions, __path_get_all_executables,
        __path_get_all_executables_entitlements, __path_get_executable_entitlements,
        __path_get_executable_versions, diff_executables_for_versions, get_all_executables,
        get_all_executables_entitlements, get_executable_entitlements, get_executable_versions,
    },
    frameworks::{
        __path_diff_frameworks_for_executables, __path_get_all_frameworks,
        __path_get_executable_frameworks, __path_get_framework_executables,
        __path_get_framework_versions,
    },
    operating_system_versions::{
        __path_get_extended_operating_system_versions,
        __path_get_operating_system_version_executables,
        __path_get_operating_system_version_frameworks, __path_get_operating_system_versions,
        __path_get_operating_system_versions_by_id, get_extended_operating_system_versions,
    },
    operating_systems::{__path_get_operating_system_by_id, __path_get_operating_systems},
    stats::__path_get_stats,
};
use appledb_common::routes::PublicRoutes;
use axum::{Router, routing::get};
use utoipa::OpenApi;
use utoipa_swagger_ui::{Config, SwaggerUi};

use crate::models::AppState;

use super::{
    frameworks::{
        diff_frameworks_for_executables, get_all_frameworks, get_executable_frameworks,
        get_framework_executables, get_framework_versions,
    },
    operating_system_versions::{
        get_operating_system_version_executables, get_operating_system_version_frameworks,
        get_operating_system_versions, get_operating_system_versions_by_id,
    },
    operating_systems::{get_operating_system_by_id, get_operating_systems},
    stats::get_stats,
    tasks::{__path_get_running_tasks, get_running_tasks},
};

pub fn setup_public_openapi_router(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    log::info!("Serve public openapi documentation");
    #[derive(OpenApi)]
    #[openapi(paths(
        get_stats,
        get_operating_systems,
        get_operating_system_by_id,
        get_devices,
        get_device_operating_system_versions,
        get_operating_system_versions,
        get_operating_system_versions_by_id,
        get_extended_operating_system_versions,
        get_operating_system_version_executables,
        get_operating_system_version_frameworks,
        get_executable_versions,
        get_all_executables,
        get_all_executables_entitlements,
        get_executable_entitlements,
        diff_executables_for_versions,
        diff_entitlements_for_executables,
        diff_frameworks_for_executables,
        get_executable_frameworks,
        get_all_frameworks,
        get_framework_versions,
        get_framework_executables,
        get_running_tasks
    ))]
    struct ApiDoc;

    // Update each path to add PUBLIC_ROUTES prefix
    let mut openapi = ApiDoc::openapi();
    openapi.info.title = format!("{} - public API documentation", env!("CARGO_PKG_NAME"));
    openapi.paths.paths = openapi
        .paths
        .paths
        .iter_mut()
        .map(|(path, item)| {
            (
                format!("{}{}", PublicRoutes::route_prefix(), path),
                item.to_owned(),
            )
        })
        .collect();

    // Check that every registered endpoint is documented (only in debug builds)
    debug_assert_eq!(
        openapi.paths.paths.len(),
        PublicRoutes::COUNT,
        "all public handlers aren't documented..."
    );

    router.merge(
        SwaggerUi::new("/swagger")
            .config(Config::from(
                PublicRoutes::route_prefix().to_owned() + "/openapi.json",
            ))
            .url("/openapi.json", openapi),
    )
}

pub fn get_public_router(with_openapi: bool) -> Router<Arc<AppState>> {
    let mut router = Router::new();

    if with_openapi {
        router = setup_public_openapi_router(router);
    }

    router
        // ##################
        // Stats
        // ##################
        .route(&PublicRoutes::GetStats.to_string(), get(get_stats))
        // ##################
        // Operating systems
        // ##################
        .route(
            &PublicRoutes::GetOperatingSystems.to_string(),
            get(get_operating_systems),
        )
        .route(
            &PublicRoutes::GetOperatingSystemById.to_string(),
            get(get_operating_system_by_id),
        )
        // ##################
        // Devices
        // ##################
        .route(&PublicRoutes::GetDevices.to_string(), get(get_devices))
        .route(
            &PublicRoutes::GetDeviceVersions.to_string(),
            get(get_device_operating_system_versions),
        )
        // ##################
        // Operating system versions
        // ##################
        .route(
            &PublicRoutes::GetOperatingSystemVersions.to_string(),
            get(get_operating_system_versions),
        )
        .route(
            &PublicRoutes::GetOperatingSystemVersionsById.to_string(),
            get(get_operating_system_versions_by_id),
        )
        .route(
            &PublicRoutes::GetOperatingSystemVersionsExtended.to_string(),
            get(get_extended_operating_system_versions),
        )
        .route(
            &PublicRoutes::GetOperatingSystemVersionsExecutables.to_string(),
            get(get_operating_system_version_executables),
        )
        .route(
            &PublicRoutes::GetOperatingSystemVersionsFrameworks.to_string(),
            get(get_operating_system_version_frameworks),
        )
        // ##################
        // Executables
        // ##################
        .route(
            &PublicRoutes::GetAllExecutablesEntitlements.to_string(),
            get(get_all_executables_entitlements),
        )
        .route(
            &PublicRoutes::GetExecutableVersions.to_string(),
            get(get_executable_versions),
        )
        .route(
            &PublicRoutes::GetAllExecutables.to_string(),
            get(get_all_executables),
        )
        .route(
            &PublicRoutes::GetDiffExecutablesOperatingSystemVersion.to_string(),
            get(diff_executables_for_versions),
        )
        // ##################
        // Entitlements
        // ##################
        .route(
            &PublicRoutes::GetExecutableEntitlements.to_string(),
            get(get_executable_entitlements),
        )
        .route(
            &PublicRoutes::GetDiffEntitlementsExecutables.to_string(),
            get(diff_entitlements_for_executables),
        )
        // ##################
        // Frameworks
        // ##################
        .route(
            &PublicRoutes::GetDiffFrameworksExecutables.to_string(),
            get(diff_frameworks_for_executables),
        )
        .route(
            &PublicRoutes::GetExecutableFrameworks.to_string(),
            get(get_executable_frameworks),
        )
        .route(
            &PublicRoutes::GetAllFrameworks.to_string(),
            get(get_all_frameworks),
        )
        .route(
            &PublicRoutes::GetFrameworkVersions.to_string(),
            get(get_framework_versions),
        )
        .route(
            &PublicRoutes::GetFrameworkExecutables.to_string(),
            get(get_framework_executables),
        )
        // ##################
        // Tasks
        // ##################
        .route(
            &PublicRoutes::GetRunningTasks.to_string(),
            get(get_running_tasks),
        )
}
