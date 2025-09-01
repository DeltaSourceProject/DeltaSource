(function() {
    const rtcSupported = !!(window.RTCPeerConnection || window.mozRTCPeerConnection || window.webkitRTCPeerConnection);
    if (rtcSupported) {
        console.warn("WebRTC is enabled in your browser. For privacy, consider disabling it.");
    } else {
        console.log("WebRTC is not available in this browser.");
    }
})();