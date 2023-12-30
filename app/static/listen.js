function fetchAndUpdate() {
    var url = "/event"
    fetch(url)
        .then(response => {
            return response.text();
        })
        .then(htmlContent => {
            document.getElementById("message-box").innerHTML = htmlContent;
        })
        .catch(error => {
            console.error('Error fetching HTML:', error);
        });
}

setInterval(fetchAndUpdate, 500);
fetchAndUpdate();