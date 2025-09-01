(function() {
    fetch('https://api.ipify.org?format=json')
        .then(response => response.json())
        .then(data => {
            console.log("Your public IP address is:", data.ip);
        })
        .catch(error => {
            console.error("Could not retrieve IP address:", error);
        });
})();