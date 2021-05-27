var uri = location.protocol + '//' + location.host + '/sse';
var sse = new ReconnectingEventSource(uri);
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
