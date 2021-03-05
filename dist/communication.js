document
    .getElementById("connect")
    .addEventListener("click", function () {
        var address = document.getElementById("address").value;
        window.__TAURI__.tauri
            .promisified({
                cmd: "connect",
                address: address,
            })
            .then(function (response) {
                document.getElementById("content").innerHTML =
                    typeof response === "object" ? JSON.stringify(response) : response;
            });
    });


window.__TAURI__.event
    .listen("rust-event", function (res) {
        console.log("asdas");
    });