export const DRAWER_WIDTH = 240;

export const API_URL = import.meta.env.PROD ? "" : "http://127.0.0.1:4000"

export const GET_ALL_DEVICES_ENDPOINT = `${API_URL}/api/v1/devices/all`

export const GET_ALL_EXECUTABLES_ENDPOINT = `${API_URL}/api/v1/executables/all`

// ROUTES
export const MAIN_ROUTE = "/"
export const STATS_ROUTE = "/stats"
export const DIFF_ROUTE = "/diff"
export const MODELS = "/model/:modelId"
export const ENTITLEMENTS_VERSION = "/model/:modelId/version/:versionId"