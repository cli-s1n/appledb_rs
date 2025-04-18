export const DRAWER_WIDTH = 240;

//////////////////////////
// BACKEND ENDPOINTS
const SERVER_URL = import.meta.env.PROD ? "" : "http://127.0.0.1:4000"
const API_VERSION = "/api/v1"
export const API_URL = `${SERVER_URL}${API_VERSION}`

export const GET_ALL_DEVICES_ENDPOINT = `${API_URL}/devices/all`
export const GET_ALL_OPERATING_SYSTEM_VERSIONS_ENDPOINT = `${API_URL}/operating_system_versions/all`
export const GET_EXTENDED_OPERATING_SYSTEM_VERSIONS = `${API_URL}/operating_system_versions/extended`
export const GET_ALL_EXECUTABLES_ENDPOINT = `${API_URL}/executables/all`
export const GET_ALL_FRAMEWORKS_ENDPOINT = `${API_URL}/frameworks/all`
export const GET_RUNNING_TASKS = `${API_URL}/tasks/running`
//////////////////////////

//////////////////////////
// LOCAL WEB-UI ROUTES
export const MAIN_ROUTE = "/"
export const ENTITLEMENTS_SEARCH_ROUTE = "/entitlements/search"
export const ENTITLEMENTS_DIFF_ROUTE = "/entitlements/diff"

export const EXECUTABLES_DIFF_ROUTE = "/executables/diff"
export const EXECUTABLES_FRAMEWORKS_ROUTE = "/executables/frameworks"

export const FRAMEWORKS_DIFF_ROUTE = "/frameworks/diff"
export const FRAMEWORKS_EXECUTABLES_ROUTE = "/frameworks/executables"
export const TASKS_ROUTE = "/tasks"
//////////////////////////