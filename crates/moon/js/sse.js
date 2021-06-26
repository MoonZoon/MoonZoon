// https://github.com/fanout/reconnecting-eventsource
var sse = new ReconnectingEventSource('/_api/reload_sse', {
    withCredentials: false,
    max_retry_time: 5000,
});
var backendBuildId = null;
sse.addEventListener("backend_build_id", function (msg) {
    var newBackendBuildId = msg.data;
    if (backendBuildId === null) {
        backendBuildId = newBackendBuildId;
    } else if (backendBuildId !== newBackendBuildId) {
        sse.close();
        location.reload();
    }
});
sse.addEventListener("reload", function (msg) {
    sse.close();
    location.reload();
});
